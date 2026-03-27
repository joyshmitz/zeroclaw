#[allow(clippy::module_inception)]
pub mod agent;
pub mod classifier;
pub mod context_analyzer;
pub mod context_compressor;
pub mod cost;
pub mod dispatcher;
pub mod eval;
pub(crate) mod governed;
pub mod history;
pub mod history_pruner;
pub mod loop_;
pub mod loop_detector;
pub mod tool_execution;
pub mod memory_loader;
pub mod personality;
pub mod prompt;
pub mod thinking;

pub(crate) fn current_datetime_context() -> String {
    let now = chrono::Local::now();
    format!(
        "[CURRENT DATE & TIME: {} ({})]",
        now.format("%Y-%m-%d %H:%M:%S"),
        now.format("%Z")
    )
}

pub(crate) fn enrich_user_message(context: &str, user_message: &str) -> String {
    let datetime_context = current_datetime_context();
    if context.is_empty() {
        format!("{datetime_context}\n{user_message}")
    } else if context.ends_with('\n') {
        format!("{datetime_context}\n{context}{user_message}")
    } else {
        format!("{datetime_context}\n{context}\n{user_message}")
    }
}

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use agent::{Agent, AgentBuilder, TurnEvent};
#[allow(unused_imports)]
pub use loop_::{process_message, run};

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn enrich_user_message_places_datetime_context_first() {
        let enriched = enrich_user_message("", "status?");
        assert!(enriched.starts_with("[CURRENT DATE & TIME: "));
        assert!(enriched.ends_with("status?"));
    }

    #[test]
    fn enrich_user_message_keeps_existing_context_separate() {
        let enriched = enrich_user_message("[Memory context]\n- prior note\n", "status?");
        let expected = "[Memory context]\n- prior note\nstatus?";
        assert!(enriched.starts_with("[CURRENT DATE & TIME: "));
        assert!(enriched.contains(expected));
    }
}
