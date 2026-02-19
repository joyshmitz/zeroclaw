use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

// ── Priority ────────────────────────────────────────────────────

/// SOP priority level, used for execution mode resolution and scheduling.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SopPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

impl fmt::Display for SopPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Normal => write!(f, "normal"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

// ── Execution Mode ──────────────────────────────────────────────

/// How much autonomy the agent has when executing an SOP.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SopExecutionMode {
    /// Execute all steps without human approval.
    Auto,
    /// Request approval before starting, then execute all steps.
    #[default]
    Supervised,
    /// Request approval before each step.
    StepByStep,
    /// Critical/High → Auto, Normal/Low → Supervised.
    PriorityBased,
}

impl fmt::Display for SopExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Supervised => write!(f, "supervised"),
            Self::StepByStep => write!(f, "step_by_step"),
            Self::PriorityBased => write!(f, "priority_based"),
        }
    }
}

// ── Trigger ─────────────────────────────────────────────────────

/// What event can activate an SOP.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SopTrigger {
    Mqtt {
        topic: String,
        #[serde(default)]
        condition: Option<String>,
    },
    Webhook {
        path: String,
    },
    Cron {
        expression: String,
    },
    Peripheral {
        board: String,
        signal: String,
        #[serde(default)]
        condition: Option<String>,
    },
    Manual,
}

impl fmt::Display for SopTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mqtt { topic, .. } => write!(f, "mqtt:{topic}"),
            Self::Webhook { path } => write!(f, "webhook:{path}"),
            Self::Cron { expression } => write!(f, "cron:{expression}"),
            Self::Peripheral { board, signal, .. } => write!(f, "peripheral:{board}/{signal}"),
            Self::Manual => write!(f, "manual"),
        }
    }
}

// ── Step ────────────────────────────────────────────────────────

/// A single step in an SOP procedure, parsed from SOP.md.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SopStep {
    pub number: u32,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub suggested_tools: Vec<String>,
    #[serde(default)]
    pub requires_confirmation: bool,
}

// ── SOP ─────────────────────────────────────────────────────────

/// A complete Standard Operating Procedure definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sop {
    pub name: String,
    pub description: String,
    pub version: String,
    pub priority: SopPriority,
    pub execution_mode: SopExecutionMode,
    pub triggers: Vec<SopTrigger>,
    pub steps: Vec<SopStep>,
    #[serde(default = "default_cooldown_secs")]
    pub cooldown_secs: u64,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: u32,
    #[serde(skip)]
    pub location: Option<PathBuf>,
}

fn default_cooldown_secs() -> u64 {
    0
}

fn default_max_concurrent() -> u32 {
    1
}

// ── TOML manifest (internal parse target) ───────────────────────

/// Top-level SOP.toml structure.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SopManifest {
    pub sop: SopMeta,
    #[serde(default)]
    pub triggers: Vec<SopTrigger>,
}

/// The `[sop]` table in SOP.toml.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SopMeta {
    pub name: String,
    pub description: String,
    #[serde(default = "default_sop_version")]
    pub version: String,
    #[serde(default)]
    pub priority: SopPriority,
    #[serde(default)]
    pub execution_mode: SopExecutionMode,
    #[serde(default = "default_cooldown_secs")]
    pub cooldown_secs: u64,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: u32,
}

fn default_sop_version() -> String {
    "0.1.0".to_string()
}

// ── Run state (Phase 2, types defined here for forward compat) ──

/// Status of an SOP execution run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SopRunStatus {
    Pending,
    Running,
    WaitingApproval,
    Completed,
    Failed,
    Cancelled,
}

impl fmt::Display for SopRunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Running => write!(f, "running"),
            Self::WaitingApproval => write!(f, "waiting_approval"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_display() {
        assert_eq!(SopPriority::Critical.to_string(), "critical");
        assert_eq!(SopPriority::Low.to_string(), "low");
    }

    #[test]
    fn execution_mode_display() {
        assert_eq!(SopExecutionMode::Auto.to_string(), "auto");
        assert_eq!(
            SopExecutionMode::PriorityBased.to_string(),
            "priority_based"
        );
    }

    #[test]
    fn trigger_display() {
        let mqtt = SopTrigger::Mqtt {
            topic: "sensors/temp".into(),
            condition: Some("$.value > 85".into()),
        };
        assert_eq!(mqtt.to_string(), "mqtt:sensors/temp");

        let manual = SopTrigger::Manual;
        assert_eq!(manual.to_string(), "manual");
    }

    #[test]
    fn priority_serde_roundtrip() {
        let json = serde_json::to_string(&SopPriority::Critical).unwrap();
        assert_eq!(json, "\"critical\"");
        let parsed: SopPriority = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, SopPriority::Critical);
    }

    #[test]
    fn execution_mode_serde_roundtrip() {
        let json = serde_json::to_string(&SopExecutionMode::PriorityBased).unwrap();
        assert_eq!(json, "\"priority_based\"");
        let parsed: SopExecutionMode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, SopExecutionMode::PriorityBased);
    }

    #[test]
    fn trigger_toml_roundtrip() {
        let toml_str = r#"
type = "mqtt"
topic = "facility/pump/pressure"
condition = "$.value > 85"
"#;
        let trigger: SopTrigger = toml::from_str(toml_str).unwrap();
        assert!(
            matches!(trigger, SopTrigger::Mqtt { ref topic, .. } if topic == "facility/pump/pressure")
        );
    }

    #[test]
    fn trigger_manual_toml() {
        let toml_str = r#"type = "manual""#;
        let trigger: SopTrigger = toml::from_str(toml_str).unwrap();
        assert_eq!(trigger, SopTrigger::Manual);
    }

    #[test]
    fn run_status_display() {
        assert_eq!(
            SopRunStatus::WaitingApproval.to_string(),
            "waiting_approval"
        );
    }

    #[test]
    fn step_defaults() {
        let step: SopStep =
            serde_json::from_str(r#"{"number": 1, "title": "Check", "body": "Verify readings"}"#)
                .unwrap();
        assert!(step.suggested_tools.is_empty());
        assert!(!step.requires_confirmation);
    }

    #[test]
    fn manifest_parse() {
        let toml_str = r#"
[sop]
name = "test-sop"
description = "A test SOP"

[[triggers]]
type = "manual"

[[triggers]]
type = "webhook"
path = "/sop/test"
"#;
        let manifest: SopManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.sop.name, "test-sop");
        assert_eq!(manifest.triggers.len(), 2);
        assert_eq!(manifest.sop.priority, SopPriority::Normal);
        assert_eq!(manifest.sop.execution_mode, SopExecutionMode::Supervised);
    }
}
