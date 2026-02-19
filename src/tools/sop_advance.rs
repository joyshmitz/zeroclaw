use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;
use tracing::warn;

use super::traits::{Tool, ToolResult};
use crate::sop::types::{SopRunAction, SopStepResult, SopStepStatus};
use crate::sop::{SopAuditLogger, SopEngine};

/// Report a step result and advance an SOP run to the next step.
pub struct SopAdvanceTool {
    engine: Arc<Mutex<SopEngine>>,
    audit: Option<Arc<SopAuditLogger>>,
}

impl SopAdvanceTool {
    pub fn new(engine: Arc<Mutex<SopEngine>>) -> Self {
        Self {
            engine,
            audit: None,
        }
    }

    pub fn with_audit(mut self, audit: Arc<SopAuditLogger>) -> Self {
        self.audit = Some(audit);
        self
    }
}

#[async_trait]
impl Tool for SopAdvanceTool {
    fn name(&self) -> &str {
        "sop_advance"
    }

    fn description(&self) -> &str {
        "Report the result of the current SOP step and advance to the next step. Provide the run_id, whether the step succeeded or failed, and a brief output summary."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "run_id": {
                    "type": "string",
                    "description": "The run ID to advance"
                },
                "status": {
                    "type": "string",
                    "enum": ["completed", "failed", "skipped"],
                    "description": "Result status of the current step"
                },
                "output": {
                    "type": "string",
                    "description": "Brief summary of what happened in this step"
                }
            },
            "required": ["run_id", "status", "output"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let run_id = args
            .get("run_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'run_id' parameter"))?;

        let status_str = args
            .get("status")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'status' parameter"))?;

        let output = args
            .get("output")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'output' parameter"))?;

        let step_status = match status_str {
            "completed" => SopStepStatus::Completed,
            "failed" => SopStepStatus::Failed,
            "skipped" => SopStepStatus::Skipped,
            other => {
                return Ok(ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!(
                        "Invalid status '{other}'. Must be: completed, failed, or skipped"
                    )),
                });
            }
        };

        // Lock engine, advance step, snapshot data for audit, then drop lock
        let (action, step_result_ok, finished_run) = {
            let mut engine = self
                .engine
                .lock()
                .map_err(|e| anyhow::anyhow!("Engine lock poisoned: {e}"))?;

            let current_step = engine
                .get_run(run_id)
                .map(|r| r.current_step)
                .ok_or_else(|| anyhow::anyhow!("Run not found: {run_id}"))?;

            let now = now_iso8601();
            let step_result = SopStepResult {
                step_number: current_step,
                status: step_status,
                output: output.to_string(),
                started_at: now.clone(),
                completed_at: Some(now),
            };
            let step_result_clone = step_result.clone();

            match engine.advance_step(run_id, step_result) {
                Ok(action) => {
                    // Snapshot finished run for audit (Completed/Failed/Cancelled)
                    let finished = match &action {
                        SopRunAction::Completed { run_id, .. }
                        | SopRunAction::Failed { run_id, .. } => engine.get_run(run_id).cloned(),
                        _ => None,
                    };
                    // Only audit step result when advance succeeded
                    (Ok(action), Some(step_result_clone), finished)
                }
                Err(e) => (Err(e), None, None),
            }
        };

        // Audit logging (engine lock dropped, safe to await)
        if let Some(ref audit) = self.audit {
            if let Some(ref sr) = step_result_ok {
                if let Err(e) = audit.log_step_result(run_id, sr).await {
                    warn!("SOP audit log_step_result failed: {e}");
                }
            }
            if let Some(ref run) = finished_run {
                if let Err(e) = audit.log_run_complete(run).await {
                    warn!("SOP audit log_run_complete failed: {e}");
                }
            }
        }

        match action {
            Ok(action) => {
                let result_output = match action {
                    SopRunAction::ExecuteStep {
                        run_id, context, ..
                    } => {
                        format!("Step recorded. Next step for run {run_id}:\n\n{context}")
                    }
                    SopRunAction::WaitApproval {
                        run_id, context, ..
                    } => {
                        format!(
                            "Step recorded. Next step for run {run_id} (waiting for approval):\n\n{context}"
                        )
                    }
                    SopRunAction::Completed { run_id, sop_name } => {
                        format!("SOP '{sop_name}' run {run_id} completed successfully.")
                    }
                    SopRunAction::Failed {
                        run_id,
                        sop_name,
                        reason,
                    } => {
                        format!("SOP '{sop_name}' run {run_id} failed: {reason}")
                    }
                };
                Ok(ToolResult {
                    success: true,
                    output: result_output,
                    error: None,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to advance step: {e}")),
            }),
        }
    }
}

fn now_iso8601() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    days += 719_468;
    let era = days / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SopConfig;
    use crate::memory::Memory;
    use crate::sop::engine::SopEngine;
    use crate::sop::types::*;

    fn test_sop() -> Sop {
        Sop {
            name: "test-sop".into(),
            description: "Test SOP".into(),
            version: "1.0.0".into(),
            priority: SopPriority::Normal,
            execution_mode: SopExecutionMode::Auto,
            triggers: vec![SopTrigger::Manual],
            steps: vec![
                SopStep {
                    number: 1,
                    title: "Step one".into(),
                    body: "Do step one".into(),
                    suggested_tools: vec![],
                    requires_confirmation: false,
                },
                SopStep {
                    number: 2,
                    title: "Step two".into(),
                    body: "Do step two".into(),
                    suggested_tools: vec![],
                    requires_confirmation: false,
                },
            ],
            cooldown_secs: 0,
            max_concurrent: 1,
            location: None,
        }
    }

    fn engine_with_active_run() -> Arc<Mutex<SopEngine>> {
        let mut engine = SopEngine::new(SopConfig::default());
        engine.set_sops_for_test(vec![test_sop()]);
        let event = SopEvent {
            source: SopTriggerSource::Manual,
            topic: None,
            payload: None,
            timestamp: "2026-02-19T12:00:00Z".into(),
        };
        engine.start_run("test-sop", event).unwrap();
        Arc::new(Mutex::new(engine))
    }

    #[tokio::test]
    async fn advance_to_next_step() {
        let engine = engine_with_active_run();
        let tool = SopAdvanceTool::new(engine);
        let result = tool
            .execute(json!({
                "run_id": "run-000001",
                "status": "completed",
                "output": "Step 1 done successfully"
            }))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("Next step"));
        assert!(result.output.contains("Step two"));
    }

    #[tokio::test]
    async fn advance_to_completion() {
        let engine = engine_with_active_run();
        let tool = SopAdvanceTool::new(engine.clone());

        // Complete step 1
        tool.execute(json!({
            "run_id": "run-000001",
            "status": "completed",
            "output": "Step 1 done"
        }))
        .await
        .unwrap();

        // Complete step 2
        let result = tool
            .execute(json!({
                "run_id": "run-000001",
                "status": "completed",
                "output": "Step 2 done"
            }))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("completed successfully"));
    }

    #[tokio::test]
    async fn advance_with_failure() {
        let engine = engine_with_active_run();
        let tool = SopAdvanceTool::new(engine);
        let result = tool
            .execute(json!({
                "run_id": "run-000001",
                "status": "failed",
                "output": "Valve stuck open"
            }))
            .await
            .unwrap();
        assert!(result.success); // tool succeeded, SOP failed
        assert!(result.output.contains("failed"));
        assert!(result.output.contains("Valve stuck open"));
    }

    #[tokio::test]
    async fn advance_invalid_status() {
        let engine = engine_with_active_run();
        let tool = SopAdvanceTool::new(engine);
        let result = tool
            .execute(json!({
                "run_id": "run-000001",
                "status": "invalid",
                "output": "whatever"
            }))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("Invalid status"));
    }

    #[tokio::test]
    async fn advance_unknown_run() {
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tool = SopAdvanceTool::new(engine);
        let result = tool
            .execute(json!({
                "run_id": "nonexistent",
                "status": "completed",
                "output": "done"
            }))
            .await;
        assert!(result.is_err());
    }

    #[test]
    fn name_and_schema() {
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tool = SopAdvanceTool::new(engine);
        assert_eq!(tool.name(), "sop_advance");
        let schema = tool.parameters_schema();
        assert!(schema["properties"]["run_id"].is_object());
        assert!(schema["properties"]["status"]["enum"].is_array());
    }

    #[tokio::test]
    async fn advance_error_does_not_write_step_audit() {
        // Use a run_id that doesn't exist — advance_step will fail
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tmp = tempfile::tempdir().unwrap();
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let memory: Arc<dyn Memory> =
            Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());
        let audit = Arc::new(SopAuditLogger::new(memory.clone()));

        let tool = SopAdvanceTool::new(engine).with_audit(audit.clone());
        let result = tool
            .execute(json!({
                "run_id": "nonexistent",
                "status": "completed",
                "output": "done"
            }))
            .await;
        // advance_step on nonexistent run returns Err (anyhow)
        assert!(result.is_err());

        // Verify no phantom audit entries were written
        let runs = audit.list_runs().await.unwrap();
        assert!(
            runs.is_empty(),
            "no audit entries should exist after advance error"
        );
    }

    #[tokio::test]
    async fn advance_success_writes_step_audit() {
        let engine = engine_with_active_run();
        let tmp = tempfile::tempdir().unwrap();
        let mem_cfg = crate::config::MemoryConfig {
            backend: "sqlite".into(),
            ..crate::config::MemoryConfig::default()
        };
        let memory: Arc<dyn Memory> =
            Arc::from(crate::memory::create_memory(&mem_cfg, tmp.path(), None).unwrap());
        let audit = Arc::new(SopAuditLogger::new(memory.clone()));

        let tool = SopAdvanceTool::new(engine).with_audit(audit.clone());
        let result = tool
            .execute(json!({
                "run_id": "run-000001",
                "status": "completed",
                "output": "Step 1 done"
            }))
            .await
            .unwrap();
        assert!(result.success);

        // Verify step audit was written
        let entries = memory
            .list(
                Some(&crate::memory::traits::MemoryCategory::Custom("sop".into())),
                None,
            )
            .await
            .unwrap();
        let step_keys: Vec<_> = entries
            .iter()
            .filter(|e| e.key.starts_with("sop_step_"))
            .collect();
        assert!(
            !step_keys.is_empty(),
            "step audit should be written on success"
        );
    }
}
