# Session Context — SOP Foundation (Current) — 2026-02-19

Use this context file to start the next session without ambiguity.

## Canonical Context

repo_path: `/data/projects/zeroclaw`  
branch: `feat/sop-foundation`  
base_branch: `main`  
head_commit: (pending — Phase 5/6 implementation uncommitted)

objective:
- Phase 5, 6A (webhook), and 6B (cron) event fan-in implemented. MQTT, webhook, and cron route to SOP engine at runtime.
- Phase 6C (peripheral) has dispatch helper only — no runtime wiring (deferred, needs firmware callback protocol).

done_definition:
- Trigger `condition` is evaluated deterministically in SOP matching.
- SOP audit is wired to real tool flow (`sop_execute/sop_approve/sop_advance`).
- Approval timeout check runs in active runtime loop (not tests-only).
- Unified dispatch helper routes events from all sources to SOP engine (`src/sop/dispatch.rs`).
- MQTT listener subscribes to configured topics and dispatches to SOP (`src/channels/mqtt.rs`).
- Webhook handler performs non-blocking SOP dispatch, returns HTTP 202 Accepted (`src/gateway/mod.rs`).
- Cron scheduler performs window-based SOP trigger evaluation on each tick (`src/cron/scheduler.rs`).
- Peripheral signal dispatch helper exists but has no runtime wiring — helper only (`dispatch_peripheral_signal`).
- Single shared SOP engine is created once in daemon and passed to all components (`src/daemon/mod.rs`).
- Approval timeout polling runs in scheduler tick loop (daemon-safe, not just interactive loop).
- Webhook uses actual URI path for SOP trigger matching (not hardcoded `/webhook`).
- Custom SOP paths reachable via `/sop/{*rest}` gateway route (e.g., `POST /sop/deploy`).
- `SopRunAction` is propagated through `DispatchResult::Started`, not silently dropped.
- All headless callers process dispatch results via `process_headless_results` (no `let _ =`).
- MQTT `use_tls` flag is validated for consistency with URL scheme (`mqtt://` vs `mqtts://`).
- MQTT TLS transport configured via `Transport::tls_with_default_config()` when `use_tls` is true.
- Scheduler timeout mirrors interactive loop: snapshots runs, audits `log_timeout_auto_approve`.
- Daemon `large_future` clippy lint fixed with `Box::pin`.
- Headless log output redacted: only `step.number` and `step.title` logged, no raw context/payload/PII.
- `/sop/{*rest}` endpoint has idempotency check (X-Idempotency-Key header dedup via IdempotencyStore).
- MQTT `broker_port()` defaults to 8883 for `mqtts://` scheme (not 1883).
- Lock-poison returns 500 Internal Server Error in both `/webhook` and `/sop/*` handlers (not silent fallthrough).
- Missing SOP engine in `/sop/*` handler returns 503 Service Unavailable.
- Idempotency keys namespaced per endpoint (`webhook:{key}` vs `sop:{key}`) — no cross-endpoint collision.
- Scheduler lock-poison logs `error!` and marks `sop_scheduler` component error via health system.
- `/sop/*` handler covered by 5 dedicated tests: match→202, no-match→404, engine-missing→503, idempotency dedup, cross-endpoint namespace isolation.

in_scope_paths:
- `src/sop/**`
- `src/tools/sop_*.rs`
- `src/tools/mod.rs`
- `src/tools/traits.rs`
- `src/agent/loop_.rs`
- `src/channels/mod.rs`
- `src/channels/mqtt.rs`
- `src/config/schema.rs`
- `src/gateway/mod.rs`
- `src/cron/scheduler.rs`
- `src/daemon/mod.rs`
- `docs/project/sop-foundation-plan-2026-02-19.md`

out_of_scope_paths:
- `src/providers/**` (unless explicitly requested)
- unrelated docs or localization files not tied to SOP runtime

authoritative_docs:
- `docs/project/sop-foundation-plan-2026-02-19.md`
- `docs/project/session-context-template.md`
- `docs/sop-necessity-research-2026-02-19.md`

constraints:
- Do not touch unrelated dirty files.
- No scope expansion beyond `in_scope_paths` without explicit confirmation.
- Keep changes small and reversible.

verification:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test sop:: --lib`
- `cargo test tools::sop --lib`
- `cargo test`

open_risks:
- SopCronCache is built once at daemon startup; hot-reload of SOP definitions requires daemon restart.
- MQTT TLS client cert auth deferred (needs config schema extension).
- Peripheral event streaming deferred (needs firmware callback protocol) — dispatch helper exists but no runtime wiring.
- Cross-SOP priority preemption during fan-in deferred to Phase 7+.
- Standalone `gateway` CLI command runs SOP-less (intentional — only `daemon` creates shared SOP engine).

known_pre_existing_failures:
- clippy: `option_option` (proxy_config.rs), `identical_match_arms` (reliable.rs), `returning_let_binding` (wizard.rs), `manual_string_new` (matrix.rs) — all on `main`, outside SOP scope.
- test: `onboard::wizard::tests::run_models_refresh_rejects_unsupported_provider` — stale expectation on `main`.
