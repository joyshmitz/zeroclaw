use crate::sop::dispatch::DispatchResult;
use crate::sop::{SopEvent, SopRunAction, SopTriggerSource};
use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernedResponseMode {
    RequestEvidence,
    StageForApproval,
    ApplySop,
    Escalate,
}

impl fmt::Display for GovernedResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestEvidence => write!(f, "request_evidence"),
            Self::StageForApproval => write!(f, "stage_for_approval"),
            Self::ApplySop => write!(f, "apply_sop"),
            Self::Escalate => write!(f, "escalate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncidentSeverity {
    Low,
    Normal,
    High,
    Critical,
}

impl fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Normal => write!(f, "normal"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedCaseDraft {
    pub admission: &'static str,
    pub case_type: String,
    pub summary: String,
    pub severity: IncidentSeverity,
    pub evidence_present: bool,
    pub response_mode: GovernedResponseMode,
    pub webhook_path: String,
}

impl GovernedCaseDraft {
    pub fn needs_sop_dispatch(&self) -> bool {
        self.response_mode == GovernedResponseMode::ApplySop
    }

    pub fn to_sop_event(&self, raw_message: &str) -> SopEvent {
        SopEvent {
            source: SopTriggerSource::Webhook,
            topic: Some(self.webhook_path.clone()),
            payload: Some(raw_message.to_string()),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedDispatchSummary {
    pub response_mode: GovernedResponseMode,
    pub detail: String,
    pub sop_name: Option<String>,
    pub run_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IncidentEnvelope {
    signal_type: String,
    #[serde(default)]
    incident_type: Option<String>,
    #[serde(default)]
    severity: Option<String>,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    evidence: Option<Value>,
    #[serde(default)]
    evidence_hint: Option<String>,
    #[serde(default)]
    webhook_path: Option<String>,
    #[serde(default)]
    path: Option<String>,
}

pub fn draft_incident_case(message: &str) -> Option<GovernedCaseDraft> {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return None;
    }

    draft_from_envelope(trimmed).or_else(|| draft_from_marker(trimmed))
}

pub fn summarize_dispatch_results(results: &[DispatchResult]) -> GovernedDispatchSummary {
    if let Some(DispatchResult::Started {
        run_id,
        sop_name,
        action,
    }) = results
        .iter()
        .find(|result| matches!(result, DispatchResult::Started { .. }))
    {
        return match action.as_ref() {
            SopRunAction::WaitApproval { step, .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::StageForApproval,
                detail: format!(
                    "matched SOP '{sop_name}' (run {run_id}); step {} '{}' requires approval",
                    step.number, step.title
                ),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
            SopRunAction::ExecuteStep { step, .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::ApplySop,
                detail: format!(
                    "matched SOP '{sop_name}' (run {run_id}); step {} '{}' is ready for bounded execution",
                    step.number, step.title
                ),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
            SopRunAction::DeterministicStep { step, .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::ApplySop,
                detail: format!(
                    "matched SOP '{sop_name}' (run {run_id}); deterministic step {} '{}' is ready for bounded execution",
                    step.number, step.title
                ),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
            SopRunAction::CheckpointWait { step, .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::StageForApproval,
                detail: format!(
                    "matched SOP '{sop_name}' (run {run_id}); checkpoint step {} '{}' is waiting for approval",
                    step.number, step.title
                ),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
            SopRunAction::Completed { .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::ApplySop,
                detail: format!("matched SOP '{sop_name}' (run {run_id}) and it completed immediately"),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
            SopRunAction::Failed { reason, .. } => GovernedDispatchSummary {
                response_mode: GovernedResponseMode::Escalate,
                detail: format!("matched SOP '{sop_name}' (run {run_id}) but startup failed: {reason}"),
                sop_name: Some(sop_name.clone()),
                run_id: Some(run_id.clone()),
            },
        };
    }

    if let Some(DispatchResult::Skipped { sop_name, reason }) = results
        .iter()
        .find(|result| matches!(result, DispatchResult::Skipped { .. }))
    {
        return GovernedDispatchSummary {
            response_mode: GovernedResponseMode::Escalate,
            detail: format!("matching SOP '{sop_name}' could not start: {reason}"),
            sop_name: Some(sop_name.clone()),
            run_id: None,
        };
    }

    GovernedDispatchSummary {
        response_mode: GovernedResponseMode::Escalate,
        detail: "no matching webhook-triggered SOP is currently loaded".to_string(),
        sop_name: None,
        run_id: None,
    }
}

pub fn render_gateway_response(
    draft: &GovernedCaseDraft,
    dispatch: Option<&GovernedDispatchSummary>,
) -> String {
    let final_mode = dispatch.map_or(draft.response_mode, |summary| summary.response_mode);
    let mut response = String::from("Governed incident intake accepted.\n");
    let _ = writeln!(response, "Ingress: gateway_webhook");
    let _ = writeln!(response, "Admission: {}", draft.admission);
    let _ = writeln!(response, "Case type: {}", draft.case_type);
    let _ = writeln!(response, "Severity: {}", draft.severity);
    let _ = writeln!(response, "Response mode: {final_mode}");
    let _ = writeln!(
        response,
        "Evidence present: {}",
        yes_no(draft.evidence_present)
    );
    let _ = writeln!(response, "Webhook path: {}", draft.webhook_path);
    let _ = writeln!(response, "Summary: {}", draft.summary);

    if let Some(summary) = dispatch {
        let _ = writeln!(response, "SOP dispatch: {}", summary.detail);
    } else if final_mode == GovernedResponseMode::RequestEvidence {
        let _ = writeln!(
            response,
            "SOP dispatch: not attempted because evidence collection gates the first pass"
        );
    } else if final_mode == GovernedResponseMode::StageForApproval {
        let _ = writeln!(
            response,
            "SOP dispatch: not attempted because approval staging gates the first pass"
        );
    }

    let _ = write!(
        response,
        "Next step: {}",
        match final_mode {
            GovernedResponseMode::RequestEvidence => {
                "resubmit with structured evidence or add evidence details to the incident envelope"
            }
            GovernedResponseMode::StageForApproval => {
                "review the incident and approve or route it before any bounded execution begins"
            }
            GovernedResponseMode::ApplySop => {
                "continue through the matched SOP using the existing bounded SOP tools"
            }
            GovernedResponseMode::Escalate => {
                "route for governed review because no bounded SOP path is currently ready"
            }
        }
    );

    response
}

pub fn disabled_sop_summary() -> GovernedDispatchSummary {
    GovernedDispatchSummary {
        response_mode: GovernedResponseMode::Escalate,
        detail: "SOP engine is disabled on this runtime, so the incident must be routed for governed review".to_string(),
        sop_name: None,
        run_id: None,
    }
}

fn draft_from_envelope(message: &str) -> Option<GovernedCaseDraft> {
    let envelope: IncidentEnvelope = serde_json::from_str(message).ok()?;
    let signal_type = envelope.signal_type.trim().to_ascii_lowercase();
    if signal_type != "incident"
        && !signal_type.ends_with("_incident")
        && !signal_type.ends_with("-incident")
    {
        return None;
    }

    let summary = envelope
        .summary
        .as_deref()
        .or(envelope.message.as_deref())
        .map(str::trim)
        .filter(|summary| !summary.is_empty())?
        .to_string();

    let case_type = normalize_case_type(
        envelope
            .incident_type
            .as_deref()
            .or_else(|| signal_type_as_case_type(&signal_type)),
    );
    let severity = parse_severity(envelope.severity.as_deref());
    let evidence_present = has_evidence(&envelope);
    let webhook_path = normalize_webhook_path(
        envelope
            .webhook_path
            .as_deref()
            .or(envelope.path.as_deref()),
        &case_type,
    );

    Some(build_draft(
        "structured_envelope",
        case_type,
        summary,
        severity,
        evidence_present,
        webhook_path,
    ))
}

fn draft_from_marker(message: &str) -> Option<GovernedCaseDraft> {
    let remainder = strip_explicit_marker(message)?.trim();
    if remainder.is_empty() {
        return None;
    }

    let (severity, summary) = parse_marker_severity(remainder);
    let summary = summary.trim();
    if summary.is_empty() {
        return None;
    }

    let case_type = normalize_case_type(Some("operational_incident"));
    let webhook_path = normalize_webhook_path(None, &case_type);
    Some(build_draft(
        "explicit_marker",
        case_type,
        summary.to_string(),
        severity,
        false,
        webhook_path,
    ))
}

fn build_draft(
    admission: &'static str,
    case_type: String,
    summary: String,
    severity: IncidentSeverity,
    evidence_present: bool,
    webhook_path: String,
) -> GovernedCaseDraft {
    let response_mode = if !evidence_present {
        GovernedResponseMode::RequestEvidence
    } else if matches!(
        severity,
        IncidentSeverity::High | IncidentSeverity::Critical
    ) {
        GovernedResponseMode::StageForApproval
    } else {
        GovernedResponseMode::ApplySop
    };

    GovernedCaseDraft {
        admission,
        case_type,
        summary,
        severity,
        evidence_present,
        response_mode,
        webhook_path,
    }
}

fn has_evidence(envelope: &IncidentEnvelope) -> bool {
    if envelope
        .evidence_hint
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return true;
    }

    match envelope.evidence.as_ref() {
        None | Some(Value::Null) => false,
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(_)) => true,
        Some(Value::String(value)) => !value.trim().is_empty(),
        Some(Value::Array(values)) => !values.is_empty(),
        Some(Value::Object(values)) => !values.is_empty(),
    }
}

fn parse_severity(raw: Option<&str>) -> IncidentSeverity {
    match raw.unwrap_or_default().trim().to_ascii_lowercase().as_str() {
        "low" => IncidentSeverity::Low,
        "high" => IncidentSeverity::High,
        "critical" => IncidentSeverity::Critical,
        _ => IncidentSeverity::Normal,
    }
}

fn normalize_case_type(raw: Option<&str>) -> String {
    let fallback = "operational_incident";
    let candidate = raw.unwrap_or(fallback).trim();
    if candidate.is_empty() {
        return fallback.to_string();
    }

    let normalized: String = candidate
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    let collapsed = normalized
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if collapsed.is_empty() {
        fallback.to_string()
    } else {
        collapsed
    }
}

fn normalize_webhook_path(raw: Option<&str>, case_type: &str) -> String {
    match raw.map(str::trim).filter(|value| !value.is_empty()) {
        Some(path) if path.starts_with('/') => path.to_string(),
        Some(path) => format!("/{path}"),
        None => format!("/governed/incident/{case_type}"),
    }
}

fn signal_type_as_case_type(signal_type: &str) -> Option<&str> {
    match signal_type {
        "incident" => None,
        other => Some(other),
    }
}

fn strip_explicit_marker(message: &str) -> Option<&str> {
    for prefix in ["INCIDENT:", "[incident]"] {
        if message
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        {
            return Some(&message[prefix.len()..]);
        }
    }
    None
}

fn parse_marker_severity(message: &str) -> (IncidentSeverity, &str) {
    for (prefix, severity) in [
        ("[critical]", IncidentSeverity::Critical),
        ("[high]", IncidentSeverity::High),
        ("[normal]", IncidentSeverity::Normal),
        ("[low]", IncidentSeverity::Low),
        ("critical:", IncidentSeverity::Critical),
        ("high:", IncidentSeverity::High),
        ("normal:", IncidentSeverity::Normal),
        ("low:", IncidentSeverity::Low),
    ] {
        if message
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        {
            return (severity, message[prefix.len()..].trim());
        }
    }

    (IncidentSeverity::Normal, message)
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_incident_envelope_creates_draft() {
        let draft = draft_incident_case(
            r#"{
                "signal_type": "incident",
                "incident_type": "quality_incident",
                "severity": "high",
                "summary": "Pump 7 failed pressure check",
                "evidence": {"photo": "attached"},
                "webhook_path": "/incident/quality"
            }"#,
        )
        .expect("structured incident draft");

        assert_eq!(draft.admission, "structured_envelope");
        assert_eq!(draft.case_type, "quality_incident");
        assert_eq!(draft.severity, IncidentSeverity::High);
        assert!(draft.evidence_present);
        assert_eq!(draft.response_mode, GovernedResponseMode::StageForApproval);
        assert_eq!(draft.webhook_path, "/incident/quality");
    }

    #[test]
    fn explicit_marker_defaults_to_request_evidence() {
        let draft = draft_incident_case("INCIDENT: conveyor unexpectedly stopped")
            .expect("marker-based incident draft");

        assert_eq!(draft.admission, "explicit_marker");
        assert_eq!(draft.case_type, "operational_incident");
        assert_eq!(draft.severity, IncidentSeverity::Normal);
        assert!(!draft.evidence_present);
        assert_eq!(draft.response_mode, GovernedResponseMode::RequestEvidence);
    }

    #[test]
    fn ordinary_message_does_not_open_governed_case() {
        assert!(draft_incident_case("hello, can you summarize this log?").is_none());
    }
}
