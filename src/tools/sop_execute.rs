use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;
use tracing::warn;

use super::traits::{Tool, ToolResult};
use crate::sop::types::{SopEvent, SopRunAction, SopTriggerSource};
use crate::sop::{SopAuditLogger, SopEngine};

/// Manually trigger an SOP by name. Returns the run ID and first step instruction.
pub struct SopExecuteTool {
    engine: Arc<Mutex<SopEngine>>,
    audit: Option<Arc<SopAuditLogger>>,
}

impl SopExecuteTool {
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
impl Tool for SopExecuteTool {
    fn name(&self) -> &str {
        "sop_execute"
    }

    fn description(&self) -> &str {
        "Manually trigger a Standard Operating Procedure (SOP) by name. Returns the run ID and first step instruction. Use sop_list to see available SOPs."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Name of the SOP to execute"
                },
                "payload": {
                    "type": "string",
                    "description": "Optional trigger payload (JSON string)"
                }
            },
            "required": ["name"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let sop_name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'name' parameter"))?;

        let payload = args
            .get("payload")
            .and_then(|v| v.as_str())
            .map(String::from);

        let event = SopEvent {
            source: SopTriggerSource::Manual,
            topic: None,
            payload,
            timestamp: now_iso8601(),
        };

        // Lock engine, start run, snapshot run for audit, then drop lock
        let (action, run_snapshot) = {
            let mut engine = self
                .engine
                .lock()
                .map_err(|e| anyhow::anyhow!("Engine lock poisoned: {e}"))?;

            match engine.start_run(sop_name, event) {
                Ok(action) => {
                    let run_id = action_run_id(&action);
                    let snapshot = run_id.and_then(|id| engine.get_run(id).cloned());
                    (Ok(action), snapshot)
                }
                Err(e) => (Err(e), None),
            }
        };

        // Audit log (engine lock dropped, safe to await)
        if let Some(ref audit) = self.audit {
            if let Some(ref run) = run_snapshot {
                if let Err(e) = audit.log_run_start(run).await {
                    warn!("SOP audit log_run_start failed: {e}");
                }
            }
        }

        match action {
            Ok(action) => {
                let output = match action {
                    SopRunAction::ExecuteStep {
                        run_id, context, ..
                    } => {
                        format!("SOP run started: {run_id}\n\n{context}")
                    }
                    SopRunAction::WaitApproval {
                        run_id, context, ..
                    } => {
                        format!("SOP run started: {run_id} (waiting for approval)\n\n{context}")
                    }
                    SopRunAction::Completed { run_id, sop_name } => {
                        format!("SOP '{sop_name}' run {run_id} completed immediately (no steps).")
                    }
                    SopRunAction::Failed { run_id, reason, .. } => {
                        format!("SOP run {run_id} failed: {reason}")
                    }
                };
                Ok(ToolResult {
                    success: true,
                    output,
                    error: None,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to start SOP: {e}")),
            }),
        }
    }
}

/// Extract run_id from any SopRunAction variant.
fn action_run_id(action: &SopRunAction) -> Option<&str> {
    match action {
        SopRunAction::ExecuteStep { run_id, .. }
        | SopRunAction::WaitApproval { run_id, .. }
        | SopRunAction::Completed { run_id, .. }
        | SopRunAction::Failed { run_id, .. } => Some(run_id),
    }
}

/// Simple UTC timestamp (same as engine's internal helper, kept local to avoid pub exposure).
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
    use crate::sop::engine::SopEngine;
    use crate::sop::types::*;

    fn test_sop(name: &str, mode: SopExecutionMode) -> Sop {
        Sop {
            name: name.into(),
            description: format!("Test SOP: {name}"),
            version: "1.0.0".into(),
            priority: SopPriority::Normal,
            execution_mode: mode,
            triggers: vec![SopTrigger::Manual],
            steps: vec![
                SopStep {
                    number: 1,
                    title: "Step one".into(),
                    body: "Do step one".into(),
                    suggested_tools: vec!["shell".into()],
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

    fn engine_with_sops(sops: Vec<Sop>) -> Arc<Mutex<SopEngine>> {
        let mut engine = SopEngine::new(SopConfig::default());
        engine.set_sops_for_test(sops);
        Arc::new(Mutex::new(engine))
    }

    #[tokio::test]
    async fn execute_auto_sop() {
        let engine = engine_with_sops(vec![test_sop("test-sop", SopExecutionMode::Auto)]);
        let tool = SopExecuteTool::new(engine);
        let result = tool.execute(json!({"name": "test-sop"})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("run-000001"));
        assert!(result.output.contains("Step one"));
    }

    #[tokio::test]
    async fn execute_supervised_sop() {
        let engine = engine_with_sops(vec![test_sop("test-sop", SopExecutionMode::Supervised)]);
        let tool = SopExecuteTool::new(engine);
        let result = tool.execute(json!({"name": "test-sop"})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("waiting for approval"));
    }

    #[tokio::test]
    async fn execute_unknown_sop() {
        let engine = engine_with_sops(vec![]);
        let tool = SopExecuteTool::new(engine);
        let result = tool.execute(json!({"name": "nonexistent"})).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("Failed to start SOP"));
    }

    #[tokio::test]
    async fn execute_missing_name() {
        let engine = engine_with_sops(vec![]);
        let tool = SopExecuteTool::new(engine);
        let result = tool.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_with_payload() {
        let engine = engine_with_sops(vec![test_sop("test-sop", SopExecutionMode::Auto)]);
        let tool = SopExecuteTool::new(engine);
        let result = tool
            .execute(json!({"name": "test-sop", "payload": "{\"value\": 87.3}"}))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("87.3"));
    }

    #[test]
    fn name_and_schema() {
        let engine = engine_with_sops(vec![]);
        let tool = SopExecuteTool::new(engine);
        assert_eq!(tool.name(), "sop_execute");
        assert!(tool.parameters_schema()["required"].is_array());
    }
}
