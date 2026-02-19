use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;

use super::traits::{Tool, ToolResult};
use crate::sop::types::SopRunAction;
use crate::sop::SopEngine;

/// Approve a pending SOP step that is waiting for operator approval.
pub struct SopApproveTool {
    engine: Arc<Mutex<SopEngine>>,
}

impl SopApproveTool {
    pub fn new(engine: Arc<Mutex<SopEngine>>) -> Self {
        Self { engine }
    }
}

#[async_trait]
impl Tool for SopApproveTool {
    fn name(&self) -> &str {
        "sop_approve"
    }

    fn description(&self) -> &str {
        "Approve a pending SOP step that is waiting for operator approval. Returns the step instruction to execute. Use sop_status to see which runs are waiting."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "run_id": {
                    "type": "string",
                    "description": "The run ID to approve"
                }
            },
            "required": ["run_id"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let run_id = args
            .get("run_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'run_id' parameter"))?;

        let mut engine = self
            .engine
            .lock()
            .map_err(|e| anyhow::anyhow!("Engine lock poisoned: {e}"))?;

        match engine.approve_step(run_id) {
            Ok(action) => {
                let output = match action {
                    SopRunAction::ExecuteStep {
                        run_id, context, ..
                    } => {
                        format!("Approved. Proceeding with run {run_id}.\n\n{context}")
                    }
                    other => format!("Approved. Action: {other:?}"),
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
                error: Some(format!("Approval failed: {e}")),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SopConfig;
    use crate::sop::engine::SopEngine;
    use crate::sop::types::*;

    fn test_sop() -> Sop {
        Sop {
            name: "test-sop".into(),
            description: "Test SOP".into(),
            version: "1.0.0".into(),
            priority: SopPriority::Normal,
            execution_mode: SopExecutionMode::Supervised,
            triggers: vec![SopTrigger::Manual],
            steps: vec![SopStep {
                number: 1,
                title: "Step one".into(),
                body: "Do it".into(),
                suggested_tools: vec![],
                requires_confirmation: false,
            }],
            cooldown_secs: 0,
            max_concurrent: 1,
            location: None,
        }
    }

    fn engine_with_run() -> Arc<Mutex<SopEngine>> {
        let mut engine = SopEngine::new(SopConfig::default());
        engine.set_sops_for_test(vec![test_sop()]);
        let event = SopEvent {
            source: SopTriggerSource::Manual,
            topic: None,
            payload: None,
            timestamp: "2026-02-19T12:00:00Z".into(),
        };
        // Start run — Supervised mode → WaitApproval
        engine.start_run("test-sop", event).unwrap();
        Arc::new(Mutex::new(engine))
    }

    #[tokio::test]
    async fn approve_waiting_run() {
        let engine = engine_with_run();
        let tool = SopApproveTool::new(engine);
        let result = tool.execute(json!({"run_id": "run-000001"})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Approved"));
        assert!(result.output.contains("Step one"));
    }

    #[tokio::test]
    async fn approve_nonexistent_run() {
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tool = SopApproveTool::new(engine);
        let result = tool
            .execute(json!({"run_id": "nonexistent"}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("Approval failed"));
    }

    #[tokio::test]
    async fn approve_missing_run_id() {
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tool = SopApproveTool::new(engine);
        let result = tool.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[test]
    fn name_and_schema() {
        let engine = Arc::new(Mutex::new(SopEngine::new(SopConfig::default())));
        let tool = SopApproveTool::new(engine);
        assert_eq!(tool.name(), "sop_approve");
        assert!(tool.parameters_schema()["required"].is_array());
    }
}
