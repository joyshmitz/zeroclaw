use std::fmt::Write;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;

use super::traits::{Tool, ToolResult};
use crate::sop::SopEngine;

/// Query SOP execution status — active runs, finished runs, or a specific run by ID.
pub struct SopStatusTool {
    engine: Arc<Mutex<SopEngine>>,
}

impl SopStatusTool {
    pub fn new(engine: Arc<Mutex<SopEngine>>) -> Self {
        Self { engine }
    }
}

#[async_trait]
impl Tool for SopStatusTool {
    fn name(&self) -> &str {
        "sop_status"
    }

    fn description(&self) -> &str {
        "Query SOP execution status. Provide run_id for a specific run, or sop_name to list runs for that SOP. With no arguments, shows all active runs."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "run_id": {
                    "type": "string",
                    "description": "Specific run ID to query"
                },
                "sop_name": {
                    "type": "string",
                    "description": "SOP name to list runs for"
                }
            }
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let run_id = args.get("run_id").and_then(|v| v.as_str());
        let sop_name = args.get("sop_name").and_then(|v| v.as_str());

        let engine = self
            .engine
            .lock()
            .map_err(|e| anyhow::anyhow!("Engine lock poisoned: {e}"))?;

        // Query specific run
        if let Some(run_id) = run_id {
            return match engine.get_run(run_id) {
                Some(run) => {
                    let mut output = format!(
                        "Run: {}\nSOP: {}\nStatus: {}\nStep: {} of {}\nStarted: {}\n",
                        run.run_id,
                        run.sop_name,
                        run.status,
                        run.current_step,
                        run.total_steps,
                        run.started_at,
                    );
                    if let Some(ref completed) = run.completed_at {
                        let _ = writeln!(output, "Completed: {completed}");
                    }
                    if !run.step_results.is_empty() {
                        let _ = writeln!(output, "\nStep results:");
                        for step in &run.step_results {
                            let _ = writeln!(
                                output,
                                "  Step {}: {} — {}",
                                step.step_number, step.status, step.output
                            );
                        }
                    }
                    Ok(ToolResult {
                        success: true,
                        output,
                        error: None,
                    })
                }
                None => Ok(ToolResult {
                    success: true,
                    output: format!("No run found with ID '{run_id}'."),
                    error: None,
                }),
            };
        }

        // List runs for a specific SOP or all active runs
        let mut output = String::new();

        // Active runs
        let active: Vec<_> = engine
            .active_runs()
            .values()
            .filter(|r| sop_name.map_or(true, |name| r.sop_name == name))
            .collect();

        if active.is_empty() {
            let scope = sop_name.map_or("".into(), |n| format!(" for '{n}'"));
            let _ = writeln!(output, "No active runs{scope}.");
        } else {
            let _ = writeln!(output, "Active runs ({}):", active.len());
            for run in &active {
                let _ = writeln!(
                    output,
                    "  {} — {} [{}] step {}/{}",
                    run.run_id, run.sop_name, run.status, run.current_step, run.total_steps
                );
            }
        }

        // Finished runs
        let finished = engine.finished_runs(sop_name);
        if !finished.is_empty() {
            let _ = writeln!(output, "\nFinished runs ({}):", finished.len());
            for run in finished.iter().rev().take(10) {
                let _ = writeln!(
                    output,
                    "  {} — {} [{}] ({})",
                    run.run_id,
                    run.sop_name,
                    run.status,
                    run.completed_at.as_deref().unwrap_or("?")
                );
            }
        }

        Ok(ToolResult {
            success: true,
            output,
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SopConfig;
    use crate::sop::engine::SopEngine;
    use crate::sop::types::*;

    fn test_sop(name: &str) -> Sop {
        Sop {
            name: name.into(),
            description: format!("Test SOP: {name}"),
            version: "1.0.0".into(),
            priority: SopPriority::Normal,
            execution_mode: SopExecutionMode::Auto,
            triggers: vec![SopTrigger::Manual],
            steps: vec![SopStep {
                number: 1,
                title: "Step one".into(),
                body: "Do it".into(),
                suggested_tools: vec![],
                requires_confirmation: false,
            }],
            cooldown_secs: 0,
            max_concurrent: 2,
            location: None,
        }
    }

    fn engine_with_sops(sops: Vec<Sop>) -> Arc<Mutex<SopEngine>> {
        let mut engine = SopEngine::new(SopConfig::default());
        engine.set_sops_for_test(sops);
        Arc::new(Mutex::new(engine))
    }

    fn manual_event() -> SopEvent {
        SopEvent {
            source: SopTriggerSource::Manual,
            topic: None,
            payload: None,
            timestamp: "2026-02-19T12:00:00Z".into(),
        }
    }

    #[tokio::test]
    async fn status_no_runs() {
        let engine = engine_with_sops(vec![test_sop("s1")]);
        let tool = SopStatusTool::new(engine);
        let result = tool.execute(json!({})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("No active runs"));
    }

    #[tokio::test]
    async fn status_with_active_run() {
        let engine = engine_with_sops(vec![test_sop("s1")]);
        {
            let mut e = engine.lock().unwrap();
            e.start_run("s1", manual_event()).unwrap();
        }
        let tool = SopStatusTool::new(engine);
        let result = tool.execute(json!({})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Active runs (1)"));
        assert!(result.output.contains("run-000001"));
    }

    #[tokio::test]
    async fn status_specific_run() {
        let engine = engine_with_sops(vec![test_sop("s1")]);
        {
            let mut e = engine.lock().unwrap();
            e.start_run("s1", manual_event()).unwrap();
        }
        let tool = SopStatusTool::new(engine);
        let result = tool.execute(json!({"run_id": "run-000001"})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Run: run-000001"));
        assert!(result.output.contains("Status: running"));
    }

    #[tokio::test]
    async fn status_unknown_run() {
        let engine = engine_with_sops(vec![]);
        let tool = SopStatusTool::new(engine);
        let result = tool
            .execute(json!({"run_id": "nonexistent"}))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("No run found"));
    }

    #[tokio::test]
    async fn status_filter_by_sop_name() {
        let engine = engine_with_sops(vec![test_sop("s1"), test_sop("s2")]);
        {
            let mut e = engine.lock().unwrap();
            e.start_run("s1", manual_event()).unwrap();
            e.start_run("s2", manual_event()).unwrap();
        }
        let tool = SopStatusTool::new(engine);
        let result = tool.execute(json!({"sop_name": "s1"})).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("s1"));
        // s2's run shouldn't show
        assert!(!result.output.contains(" s2 "));
    }

    #[test]
    fn name_and_schema() {
        let engine = engine_with_sops(vec![]);
        let tool = SopStatusTool::new(engine);
        assert_eq!(tool.name(), "sop_status");
        let schema = tool.parameters_schema();
        assert!(schema["properties"]["run_id"].is_object());
        assert!(schema["properties"]["sop_name"].is_object());
    }
}
