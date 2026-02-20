use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use std::time::Instant;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::json;
use tracing::warn;

use super::types::{SopRun, SopRunStatus, SopStepStatus};
use crate::memory::traits::{Memory, MemoryCategory};

/// Maximum recent runs kept in each ring buffer (global + per-SOP).
/// Covers ~90-day window at ~11 runs/day. If throughput exceeds this,
/// windowed metrics gracefully undercount rather than error.
const MAX_RECENT_RUNS: usize = 1000;

/// Stale pending-approval entries older than this are evicted.
const PENDING_EVICT_SECS: u64 = 3600;

// ── RunSnapshot ────────────────────────────────────────────────

/// Lightweight snapshot of a terminal run for windowed metric computation.
#[derive(Debug, Clone)]
struct RunSnapshot {
    completed_at: DateTime<Utc>,
    terminal_status: SopRunStatus,
    steps_executed: u64,
    steps_defined: u64,
    steps_failed: u64,
    steps_skipped: u64,
    had_human_approval: bool,
    had_timeout_approval: bool,
}

// ── SopCounters ────────────────────────────────────────────────

/// Accumulated counters for a single SOP (or global aggregate).
#[derive(Debug, Default)]
struct SopCounters {
    runs_completed: u64,
    runs_failed: u64,
    runs_cancelled: u64,
    steps_executed: u64,
    steps_defined: u64,
    steps_failed: u64,
    steps_skipped: u64,
    human_approvals: u64,
    timeout_auto_approvals: u64,
    recent_runs: VecDeque<RunSnapshot>,
}

// ── CollectorState ─────────────────────────────────────────────

#[derive(Debug, Default)]
struct CollectorState {
    global: SopCounters,
    per_sop: HashMap<String, SopCounters>,
    /// Pending human approvals: run_id → insertion time.
    pending_approvals: HashMap<String, Instant>,
    /// Pending timeout auto-approvals: run_id → insertion time.
    pending_timeout_approvals: HashMap<String, Instant>,
}

// ── SopMetricsCollector ────────────────────────────────────────

/// Thread-safe SOP metrics aggregator.
///
/// Bridges raw SOP audit events into queryable metrics for gate evaluation,
/// health endpoints, and diagnostics.
pub struct SopMetricsCollector {
    inner: RwLock<CollectorState>,
}

impl SopMetricsCollector {
    /// Create an empty collector (cold start).
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(CollectorState::default()),
        }
    }

    // ── Push methods (sync, write lock) ────────────────────────

    /// Record a terminal run (Completed/Failed/Cancelled).
    ///
    /// Call after `audit.log_run_complete()`.
    pub fn record_run_complete(&self, run: &SopRun) {
        let Ok(mut state) = self.inner.write() else {
            warn!("SOP metrics collector lock poisoned in record_run_complete");
            return;
        };

        // Evict stale pending entries (>1h)
        let now = Instant::now();
        state
            .pending_approvals
            .retain(|_, ts| now.duration_since(*ts).as_secs() < PENDING_EVICT_SECS);
        state
            .pending_timeout_approvals
            .retain(|_, ts| now.duration_since(*ts).as_secs() < PENDING_EVICT_SECS);

        let had_human = state.pending_approvals.remove(&run.run_id).is_some();
        let had_timeout = state
            .pending_timeout_approvals
            .remove(&run.run_id)
            .is_some();

        let snapshot = build_snapshot(run, had_human, had_timeout);
        apply_run(&mut state.global, &snapshot);
        let counters = state.per_sop.entry(run.sop_name.clone()).or_default();
        apply_run(counters, &snapshot);
    }

    /// Record a human approval event.
    ///
    /// Call after `audit.log_approval()`.
    pub fn record_approval(&self, sop_name: &str, run_id: &str) {
        let Ok(mut state) = self.inner.write() else {
            warn!("SOP metrics collector lock poisoned in record_approval");
            return;
        };
        state.global.human_approvals += 1;
        state
            .per_sop
            .entry(sop_name.to_string())
            .or_default()
            .human_approvals += 1;
        state
            .pending_approvals
            .insert(run_id.to_string(), Instant::now());
    }

    /// Record a timeout auto-approval event.
    ///
    /// Call after `audit.log_timeout_auto_approve()`.
    pub fn record_timeout_auto_approve(&self, sop_name: &str, run_id: &str) {
        let Ok(mut state) = self.inner.write() else {
            warn!("SOP metrics collector lock poisoned in record_timeout_auto_approve");
            return;
        };
        state.global.timeout_auto_approvals += 1;
        state
            .per_sop
            .entry(sop_name.to_string())
            .or_default()
            .timeout_auto_approvals += 1;
        state
            .pending_timeout_approvals
            .insert(run_id.to_string(), Instant::now());
    }

    // ── Warm-start (async) ─────────────────────────────────────

    /// Rebuild collector state from Memory backend (single-pass O(n)).
    ///
    /// Scans all entries in `MemoryCategory::Custom("sop")`.
    /// Falls back to empty collector on failure.
    pub async fn rebuild_from_memory(memory: &dyn Memory) -> anyhow::Result<Self> {
        let category = MemoryCategory::Custom("sop".into());
        let entries = memory.list(Some(&category), None).await?;

        // Pass 1: collect terminal runs
        let mut runs: HashMap<String, SopRun> = HashMap::new();
        // Track approval/timeout approval run_ids
        let mut approval_run_ids: Vec<String> = Vec::new();
        let mut timeout_approval_run_ids: Vec<String> = Vec::new();

        for entry in &entries {
            if entry.key.starts_with("sop_run_") {
                if let Ok(run) = serde_json::from_str::<SopRun>(&entry.content) {
                    // Only keep terminal runs
                    if matches!(
                        run.status,
                        SopRunStatus::Completed | SopRunStatus::Failed | SopRunStatus::Cancelled
                    ) {
                        runs.insert(run.run_id.clone(), run);
                    }
                }
            } else if entry.key.starts_with("sop_approval_") {
                // Extract run_id from the stored content (SopRun JSON)
                if let Ok(run) = serde_json::from_str::<SopRun>(&entry.content) {
                    approval_run_ids.push(run.run_id);
                }
            } else if entry.key.starts_with("sop_timeout_approve_") {
                if let Ok(run) = serde_json::from_str::<SopRun>(&entry.content) {
                    timeout_approval_run_ids.push(run.run_id);
                }
            }
        }

        // Pass 2: match approvals to known terminal runs
        let approval_set: std::collections::HashSet<&str> = approval_run_ids
            .iter()
            .filter(|id| runs.contains_key(id.as_str()))
            .map(|s| s.as_str())
            .collect();
        let timeout_set: std::collections::HashSet<&str> = timeout_approval_run_ids
            .iter()
            .filter(|id| runs.contains_key(id.as_str()))
            .map(|s| s.as_str())
            .collect();

        // Build state
        let mut state = CollectorState::default();
        for (run_id, run) in &runs {
            let had_human = approval_set.contains(run_id.as_str());
            let had_timeout = timeout_set.contains(run_id.as_str());
            let snapshot = build_snapshot(run, had_human, had_timeout);
            apply_run(&mut state.global, &snapshot);
            let counters = state.per_sop.entry(run.sop_name.clone()).or_default();
            apply_run(counters, &snapshot);
        }

        // Count all approval events (not just those matching terminal runs)
        // for accurate all-time counters
        for entry in &entries {
            if entry.key.starts_with("sop_approval_") {
                if let Ok(run) = serde_json::from_str::<SopRun>(&entry.content) {
                    state.global.human_approvals += 1;
                    state
                        .per_sop
                        .entry(run.sop_name.clone())
                        .or_default()
                        .human_approvals += 1;
                }
            } else if entry.key.starts_with("sop_timeout_approve_") {
                if let Ok(run) = serde_json::from_str::<SopRun>(&entry.content) {
                    state.global.timeout_auto_approvals += 1;
                    state
                        .per_sop
                        .entry(run.sop_name.clone())
                        .or_default()
                        .timeout_auto_approvals += 1;
                }
            }
        }

        Ok(Self {
            inner: RwLock::new(state),
        })
    }

    // ── Internal metric API ────────────────────────────────────

    /// Resolve a metric name to its current value.
    ///
    /// Format: `sop.<metric>` (global) or `sop.<sop_name>.<metric>` (per-SOP).
    /// Per-SOP resolution uses longest-match-first to prevent shorter SOP
    /// names from shadowing longer ones.
    pub fn get_metric_value(&self, name: &str) -> Option<serde_json::Value> {
        let Ok(state) = self.inner.read() else {
            return None;
        };

        let rest = name.strip_prefix("sop.")?;

        // Try global first (no dot-separated SOP name prefix)
        if let Some(val) = resolve_metric(&state.global, rest) {
            return Some(val);
        }

        // Per-SOP: longest-match-first
        let mut best_key: Option<&str> = None;
        let mut best_len = 0;
        for key in state.per_sop.keys() {
            if rest.starts_with(key.as_str()) {
                let next_char_idx = key.len();
                // Must be followed by '.' to be a valid SOP name match
                if rest.len() > next_char_idx
                    && rest.as_bytes()[next_char_idx] == b'.'
                    && key.len() > best_len
                {
                    best_key = Some(key.as_str());
                    best_len = key.len();
                }
            }
        }

        if let Some(sop_key) = best_key {
            let suffix = &rest[sop_key.len() + 1..]; // skip "sop_name."
            if let Some(counters) = state.per_sop.get(sop_key) {
                return resolve_metric(counters, suffix);
            }
        }

        None
    }

    // ── Diagnostics ────────────────────────────────────────────

    /// Return a full snapshot of collector state for health/debug purposes.
    pub fn snapshot(&self) -> serde_json::Value {
        let Ok(state) = self.inner.read() else {
            return json!({"error": "lock poisoned"});
        };

        let per_sop: serde_json::Map<String, serde_json::Value> = state
            .per_sop
            .iter()
            .map(|(name, c)| (name.clone(), counters_to_json(c)))
            .collect();

        json!({
            "global": counters_to_json(&state.global),
            "per_sop": per_sop,
            "pending_approvals": state.pending_approvals.len(),
            "pending_timeout_approvals": state.pending_timeout_approvals.len(),
        })
    }
}

impl Default for SopMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ── Conditional MetricsProvider impl ───────────────────────────

#[cfg(feature = "ampersona-gates")]
impl ampersona_core::traits::MetricsProvider for SopMetricsCollector {
    fn get_metric(
        &self,
        query: &ampersona_core::traits::MetricQuery,
    ) -> Result<ampersona_core::traits::MetricSample, ampersona_core::errors::MetricError> {
        if self.inner.is_poisoned() {
            return Err(ampersona_core::errors::MetricError::ProviderUnavailable);
        }
        self.get_metric_value(&query.name)
            .map(|value| ampersona_core::traits::MetricSample {
                name: query.name.clone(),
                value,
                sampled_at: Utc::now(),
            })
            .ok_or_else(|| ampersona_core::errors::MetricError::NotFound(query.name.clone()))
    }
}

// ── Helpers ────────────────────────────────────────────────────

fn build_snapshot(run: &SopRun, had_human: bool, had_timeout: bool) -> RunSnapshot {
    let completed_at = run
        .completed_at
        .as_deref()
        .and_then(parse_completed_at)
        .unwrap_or_else(Utc::now);

    let steps_executed = run.step_results.len() as u64;
    let steps_failed = run
        .step_results
        .iter()
        .filter(|s| s.status == SopStepStatus::Failed)
        .count() as u64;
    let steps_skipped = run
        .step_results
        .iter()
        .filter(|s| s.status == SopStepStatus::Skipped)
        .count() as u64;

    RunSnapshot {
        completed_at,
        terminal_status: run.status,
        steps_executed,
        steps_defined: u64::from(run.total_steps),
        steps_failed,
        steps_skipped,
        had_human_approval: had_human,
        had_timeout_approval: had_timeout,
    }
}

fn apply_run(counters: &mut SopCounters, snap: &RunSnapshot) {
    match snap.terminal_status {
        SopRunStatus::Completed => counters.runs_completed += 1,
        SopRunStatus::Failed => counters.runs_failed += 1,
        SopRunStatus::Cancelled => counters.runs_cancelled += 1,
        _ => {}
    }
    counters.steps_executed += snap.steps_executed;
    counters.steps_defined += snap.steps_defined;
    counters.steps_failed += snap.steps_failed;
    counters.steps_skipped += snap.steps_skipped;

    counters.recent_runs.push_back(snap.clone());
    if counters.recent_runs.len() > MAX_RECENT_RUNS {
        counters.recent_runs.pop_front();
    }
}

fn parse_completed_at(ts: &str) -> Option<DateTime<Utc>> {
    // Primary: RFC 3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(ts) {
        return Some(dt.with_timezone(&Utc));
    }
    // Fallback: naive without timezone suffix
    if let Ok(n) = NaiveDateTime::parse_from_str(ts.trim_end_matches('Z'), "%Y-%m-%dT%H:%M:%S") {
        return Some(n.and_utc());
    }
    // Last resort
    warn!("SOP metrics: could not parse completed_at timestamp: {ts}");
    None
}

/// Resolve a metric suffix against a counters struct.
fn resolve_metric(counters: &SopCounters, suffix: &str) -> Option<serde_json::Value> {
    // Check for windowed variant
    let (base, window_days) = if let Some(base) = suffix.strip_suffix("_7d") {
        (base, Some(7i64))
    } else if let Some(base) = suffix.strip_suffix("_30d") {
        (base, Some(30i64))
    } else if let Some(base) = suffix.strip_suffix("_90d") {
        (base, Some(90i64))
    } else {
        (suffix, None)
    };

    if let Some(days) = window_days {
        resolve_windowed(counters, base, days)
    } else {
        resolve_alltime(counters, base)
    }
}

fn resolve_alltime(c: &SopCounters, metric: &str) -> Option<serde_json::Value> {
    match metric {
        "runs_completed" => Some(json!(c.runs_completed)),
        "runs_failed" => Some(json!(c.runs_failed)),
        "runs_cancelled" => Some(json!(c.runs_cancelled)),
        "deviation_rate" => {
            if c.steps_executed == 0 {
                Some(json!(0.0))
            } else {
                Some(json!(
                    (c.steps_failed + c.steps_skipped) as f64 / c.steps_executed as f64
                ))
            }
        }
        "protocol_adherence_rate" => {
            if c.steps_defined == 0 {
                Some(json!(0.0))
            } else {
                let good = c
                    .steps_executed
                    .saturating_sub(c.steps_failed)
                    .saturating_sub(c.steps_skipped);
                Some(json!(good as f64 / c.steps_defined as f64))
            }
        }
        "human_intervention_count" => Some(json!(c.human_approvals)),
        "human_intervention_rate" => Some(json!(
            c.human_approvals as f64 / c.runs_completed.max(1) as f64
        )),
        "timeout_auto_approvals" => Some(json!(c.timeout_auto_approvals)),
        "timeout_approval_rate" => Some(json!(
            c.timeout_auto_approvals as f64 / c.runs_completed.max(1) as f64
        )),
        "completion_rate" => {
            let total = c.runs_completed + c.runs_failed + c.runs_cancelled;
            Some(json!(c.runs_completed as f64 / total.max(1) as f64))
        }
        _ => None,
    }
}

fn resolve_windowed(c: &SopCounters, metric: &str, days: i64) -> Option<serde_json::Value> {
    let cutoff = Utc::now() - chrono::Duration::days(days);
    let window: Vec<&RunSnapshot> = c
        .recent_runs
        .iter()
        .filter(|r| r.completed_at >= cutoff)
        .collect();

    // Accumulate windowed counters
    let mut wc = WindowedCounters::default();
    for snap in &window {
        match snap.terminal_status {
            SopRunStatus::Completed => wc.runs_completed += 1,
            SopRunStatus::Failed => wc.runs_failed += 1,
            SopRunStatus::Cancelled => wc.runs_cancelled += 1,
            _ => {}
        }
        wc.steps_executed += snap.steps_executed;
        wc.steps_defined += snap.steps_defined;
        wc.steps_failed += snap.steps_failed;
        wc.steps_skipped += snap.steps_skipped;
        if snap.had_human_approval {
            wc.human_approvals += 1;
        }
        if snap.had_timeout_approval {
            wc.timeout_auto_approvals += 1;
        }
    }

    match metric {
        "runs_completed" => Some(json!(wc.runs_completed)),
        "runs_failed" => Some(json!(wc.runs_failed)),
        "runs_cancelled" => Some(json!(wc.runs_cancelled)),
        "deviation_rate" => {
            if wc.steps_executed == 0 {
                Some(json!(0.0))
            } else {
                Some(json!(
                    (wc.steps_failed + wc.steps_skipped) as f64 / wc.steps_executed as f64
                ))
            }
        }
        "protocol_adherence_rate" => {
            if wc.steps_defined == 0 {
                Some(json!(0.0))
            } else {
                let good = wc
                    .steps_executed
                    .saturating_sub(wc.steps_failed)
                    .saturating_sub(wc.steps_skipped);
                Some(json!(good as f64 / wc.steps_defined as f64))
            }
        }
        "human_intervention_count" => Some(json!(wc.human_approvals)),
        "human_intervention_rate" => Some(json!(
            wc.human_approvals as f64 / wc.runs_completed.max(1) as f64
        )),
        "timeout_auto_approvals" => Some(json!(wc.timeout_auto_approvals)),
        "timeout_approval_rate" => Some(json!(
            wc.timeout_auto_approvals as f64 / wc.runs_completed.max(1) as f64
        )),
        "completion_rate" => {
            let total = wc.runs_completed + wc.runs_failed + wc.runs_cancelled;
            Some(json!(wc.runs_completed as f64 / total.max(1) as f64))
        }
        _ => None,
    }
}

#[derive(Default)]
struct WindowedCounters {
    runs_completed: u64,
    runs_failed: u64,
    runs_cancelled: u64,
    steps_executed: u64,
    steps_defined: u64,
    steps_failed: u64,
    steps_skipped: u64,
    human_approvals: u64,
    timeout_auto_approvals: u64,
}

fn counters_to_json(c: &SopCounters) -> serde_json::Value {
    json!({
        "runs_completed": c.runs_completed,
        "runs_failed": c.runs_failed,
        "runs_cancelled": c.runs_cancelled,
        "steps_executed": c.steps_executed,
        "steps_defined": c.steps_defined,
        "steps_failed": c.steps_failed,
        "steps_skipped": c.steps_skipped,
        "human_approvals": c.human_approvals,
        "timeout_auto_approvals": c.timeout_auto_approvals,
        "recent_runs_depth": c.recent_runs.len(),
    })
}

// ── Tests ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sop::types::{SopEvent, SopStepResult, SopTriggerSource};

    fn make_event() -> SopEvent {
        SopEvent {
            source: SopTriggerSource::Manual,
            topic: None,
            payload: None,
            timestamp: "2026-02-19T12:00:00Z".into(),
        }
    }

    fn make_run(
        run_id: &str,
        sop_name: &str,
        status: SopRunStatus,
        total_steps: u32,
        step_results: Vec<SopStepResult>,
    ) -> SopRun {
        SopRun {
            run_id: run_id.into(),
            sop_name: sop_name.into(),
            trigger_event: make_event(),
            status,
            current_step: total_steps,
            total_steps,
            started_at: "2026-02-19T12:00:00Z".into(),
            completed_at: Some("2026-02-19T12:05:00Z".into()),
            step_results,
            waiting_since: None,
        }
    }

    fn make_step(number: u32, status: SopStepStatus) -> SopStepResult {
        SopStepResult {
            step_number: number,
            status,
            output: format!("Step {number}"),
            started_at: "2026-02-19T12:00:00Z".into(),
            completed_at: Some("2026-02-19T12:01:00Z".into()),
        }
    }

    #[test]
    fn zero_state_baseline() {
        let c = SopMetricsCollector::new();
        assert_eq!(c.get_metric_value("sop.runs_completed"), Some(json!(0u64)));
        assert_eq!(c.get_metric_value("sop.runs_failed"), Some(json!(0u64)));
        assert_eq!(c.get_metric_value("sop.runs_cancelled"), Some(json!(0u64)));
        assert_eq!(c.get_metric_value("sop.deviation_rate"), Some(json!(0.0)));
        assert_eq!(c.get_metric_value("sop.completion_rate"), Some(json!(0.0)));
    }

    #[test]
    fn counter_arithmetic() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            3,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Completed),
                make_step(3, SopStepStatus::Completed),
            ],
        );
        c.record_run_complete(&run);

        assert_eq!(c.get_metric_value("sop.runs_completed"), Some(json!(1u64)));
        assert_eq!(c.get_metric_value("sop.runs_failed"), Some(json!(0u64)));
        assert_eq!(c.get_metric_value("sop.deviation_rate"), Some(json!(0.0)));
        assert_eq!(c.get_metric_value("sop.completion_rate"), Some(json!(1.0)));
    }

    #[test]
    fn windowed_filtering() {
        let c = SopMetricsCollector::new();
        // Completed run with recent timestamp
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            2,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Completed),
            ],
        );
        c.record_run_complete(&run);

        // 7-day window should include it (completed_at is recent)
        assert_eq!(
            c.get_metric_value("sop.runs_completed_7d"),
            Some(json!(1u64))
        );
        assert_eq!(
            c.get_metric_value("sop.runs_completed_30d"),
            Some(json!(1u64))
        );
        assert_eq!(
            c.get_metric_value("sop.runs_completed_90d"),
            Some(json!(1u64))
        );
    }

    #[test]
    fn deviation_rate_zero_steps() {
        let c = SopMetricsCollector::new();
        let run = make_run("r1", "test-sop", SopRunStatus::Completed, 0, vec![]);
        c.record_run_complete(&run);

        // Zero steps_executed → deviation_rate = 0.0
        assert_eq!(c.get_metric_value("sop.deviation_rate"), Some(json!(0.0)));
    }

    #[test]
    fn protocol_adherence_rate_partial_run() {
        let c = SopMetricsCollector::new();
        // 3 steps defined, 2 executed (1 completed, 1 failed)
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Failed,
            3,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Failed),
            ],
        );
        c.record_run_complete(&run);

        // adherence = (2 - 1 - 0) / 3 = 1/3
        let val = c
            .get_metric_value("sop.protocol_adherence_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((val - 1.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn protocol_adherence_rate_full_run() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            2,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Completed),
            ],
        );
        c.record_run_complete(&run);

        let val = c
            .get_metric_value("sop.protocol_adherence_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((val - 1.0).abs() < 1e-10);
    }

    #[test]
    fn protocol_adherence_rate_failed_run() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Failed,
            3,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Failed),
                make_step(3, SopStepStatus::Skipped),
            ],
        );
        c.record_run_complete(&run);

        // adherence = (3 - 1 - 1) / 3 = 1/3
        let val = c
            .get_metric_value("sop.protocol_adherence_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((val - 1.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn derived_rate_metrics() {
        let c = SopMetricsCollector::new();
        // Record approval then completed run
        c.record_approval("test-sop", "r1");
        c.record_timeout_auto_approve("test-sop", "r2");

        let run1 = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        let run2 = make_run(
            "r2",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        c.record_run_complete(&run1);
        c.record_run_complete(&run2);

        // human_intervention_rate = 1 / 2 = 0.5
        let hir = c
            .get_metric_value("sop.human_intervention_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((hir - 0.5).abs() < 1e-10);

        // timeout_approval_rate = 1 / 2 = 0.5
        let tar = c
            .get_metric_value("sop.timeout_approval_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((tar - 0.5).abs() < 1e-10);

        // completion_rate = 2 / 2 = 1.0
        assert_eq!(c.get_metric_value("sop.completion_rate"), Some(json!(1.0)));
    }

    #[test]
    fn per_sop_lookup() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "valve-shutdown",
            SopRunStatus::Completed,
            2,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Completed),
            ],
        );
        c.record_run_complete(&run);

        assert_eq!(
            c.get_metric_value("sop.valve-shutdown.runs_completed"),
            Some(json!(1u64))
        );
        assert_eq!(
            c.get_metric_value("sop.valve-shutdown.completion_rate"),
            Some(json!(1.0))
        );
    }

    #[test]
    fn longest_match_disambiguation() {
        let c = SopMetricsCollector::new();
        // Two SOPs: "valve" and "valve-shutdown"
        let r1 = make_run(
            "r1",
            "valve",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        let r2 = make_run(
            "r2",
            "valve-shutdown",
            SopRunStatus::Failed,
            2,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Failed),
            ],
        );
        c.record_run_complete(&r1);
        c.record_run_complete(&r2);

        // "valve-shutdown" is the longest match
        assert_eq!(
            c.get_metric_value("sop.valve-shutdown.runs_failed"),
            Some(json!(1u64))
        );
        assert_eq!(
            c.get_metric_value("sop.valve.runs_completed"),
            Some(json!(1u64))
        );
    }

    #[test]
    fn not_found_for_unknown_metric() {
        let c = SopMetricsCollector::new();
        assert_eq!(c.get_metric_value("sop.nonexistent"), None);
        assert_eq!(c.get_metric_value("other.runs_completed"), None);
        assert_eq!(c.get_metric_value("sop.no-sop.nonexistent"), None);
    }

    #[test]
    fn approval_flag_propagation() {
        let c = SopMetricsCollector::new();
        c.record_approval("test-sop", "r1");

        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        c.record_run_complete(&run);

        // Snapshot should show the run had human approval
        let snap = c.snapshot();
        let global = &snap["global"];
        assert_eq!(global["human_approvals"], json!(1u64));
        assert_eq!(global["runs_completed"], json!(1u64));

        // Windowed should reflect approval flag
        let hic = c
            .get_metric_value("sop.human_intervention_count_7d")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(hic, 1);
    }

    #[test]
    fn pending_approval_stale_eviction() {
        let c = SopMetricsCollector::new();
        // Record approval for a run that never completes
        c.record_approval("test-sop", "orphan-run");

        // The pending_approvals map has 1 entry
        {
            let state = c.inner.read().unwrap();
            assert_eq!(state.pending_approvals.len(), 1);
        }

        // Record a different run completing — this triggers eviction,
        // but since the orphan entry is fresh it survives
        let run = make_run(
            "r2",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        c.record_run_complete(&run);

        // Orphan entry still present (not stale yet)
        {
            let state = c.inner.read().unwrap();
            assert_eq!(state.pending_approvals.len(), 1);
        }
    }

    #[test]
    fn snapshot_diagnostic_output() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        c.record_run_complete(&run);

        let snap = c.snapshot();
        assert!(snap["global"].is_object());
        assert!(snap["per_sop"].is_object());
        assert_eq!(snap["global"]["runs_completed"], json!(1u64));
        assert_eq!(snap["global"]["recent_runs_depth"], json!(1));
        assert!(snap["per_sop"]["test-sop"].is_object());
    }

    #[test]
    fn runs_cancelled_tracking() {
        let c = SopMetricsCollector::new();
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Cancelled,
            2,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        c.record_run_complete(&run);

        assert_eq!(c.get_metric_value("sop.runs_cancelled"), Some(json!(1u64)));
        // Cancelled in denominator lowers completion rate
        let cr = c
            .get_metric_value("sop.completion_rate")
            .unwrap()
            .as_f64()
            .unwrap();
        assert!((cr - 0.0).abs() < 1e-10);
    }

    #[tokio::test]
    async fn warm_start_roundtrip() {
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let tmp = tempfile::tempdir().unwrap();
        let memory: std::sync::Arc<dyn Memory> =
            std::sync::Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());

        // Store a completed run + approval via audit logger
        let audit = crate::sop::SopAuditLogger::new(memory.clone());
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            2,
            vec![
                make_step(1, SopStepStatus::Completed),
                make_step(2, SopStepStatus::Completed),
            ],
        );
        audit.log_run_start(&run).await.unwrap();
        audit.log_run_complete(&run).await.unwrap();
        audit.log_approval(&run, 1).await.unwrap();

        // Rebuild from memory
        let collector = SopMetricsCollector::rebuild_from_memory(memory.as_ref())
            .await
            .unwrap();

        assert_eq!(
            collector.get_metric_value("sop.runs_completed"),
            Some(json!(1u64))
        );
        assert_eq!(
            collector.get_metric_value("sop.human_intervention_count"),
            Some(json!(1u64))
        );
        assert_eq!(
            collector.get_metric_value("sop.test-sop.runs_completed"),
            Some(json!(1u64))
        );
    }

    #[tokio::test]
    async fn warm_start_skips_running_runs() {
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let tmp = tempfile::tempdir().unwrap();
        let memory: std::sync::Arc<dyn Memory> =
            std::sync::Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());

        let audit = crate::sop::SopAuditLogger::new(memory.clone());
        // Store a Running (non-terminal) run
        let run = SopRun {
            run_id: "r1".into(),
            sop_name: "test-sop".into(),
            trigger_event: make_event(),
            status: SopRunStatus::Running,
            current_step: 1,
            total_steps: 3,
            started_at: "2026-02-19T12:00:00Z".into(),
            completed_at: None,
            step_results: vec![],
            waiting_since: None,
        };
        audit.log_run_start(&run).await.unwrap();

        let collector = SopMetricsCollector::rebuild_from_memory(memory.as_ref())
            .await
            .unwrap();

        // Running run should not be counted
        assert_eq!(
            collector.get_metric_value("sop.runs_completed"),
            Some(json!(0u64))
        );
    }

    #[tokio::test]
    async fn warm_start_empty_memory() {
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let tmp = tempfile::tempdir().unwrap();
        let memory: std::sync::Arc<dyn Memory> =
            std::sync::Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());

        let collector = SopMetricsCollector::rebuild_from_memory(memory.as_ref())
            .await
            .unwrap();

        assert_eq!(
            collector.get_metric_value("sop.runs_completed"),
            Some(json!(0u64))
        );
    }

    #[tokio::test]
    async fn warm_start_approval_matching() {
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let tmp = tempfile::tempdir().unwrap();
        let memory: std::sync::Arc<dyn Memory> =
            std::sync::Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());

        let audit = crate::sop::SopAuditLogger::new(memory.clone());

        // Run with timeout approval
        let run = make_run(
            "r1",
            "test-sop",
            SopRunStatus::Completed,
            1,
            vec![make_step(1, SopStepStatus::Completed)],
        );
        audit.log_run_start(&run).await.unwrap();
        audit.log_timeout_auto_approve(&run, 1).await.unwrap();
        audit.log_run_complete(&run).await.unwrap();

        let collector = SopMetricsCollector::rebuild_from_memory(memory.as_ref())
            .await
            .unwrap();

        assert_eq!(
            collector.get_metric_value("sop.timeout_auto_approvals"),
            Some(json!(1u64))
        );
        // Windowed variant should also reflect approval in snapshot
        let ta_7d = collector
            .get_metric_value("sop.timeout_auto_approvals_7d")
            .unwrap()
            .as_u64()
            .unwrap();
        assert_eq!(ta_7d, 1);
    }
}
