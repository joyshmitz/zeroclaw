# SOP Foundation Plan (feat/sop-foundation) — 2026-02-19

Repository:
- `/data/projects/zeroclaw`

Branch:
- `feat/sop-foundation`

## Status

Phases 1-5 and Phase 6 (webhook, cron) are implemented on this branch.
Phase 6 peripheral fan-in has a dispatch helper but no runtime wiring (deferred — needs firmware callback protocol).

### Phase 5 — MQTT Channel (Implemented)

Files:
- `src/channels/mqtt.rs` — MQTT SOP listener (`run_mqtt_sop_listener`)
- `src/config/schema.rs` — `MqttConfig` struct with `validate()`
- `Cargo.toml` — `rumqttc = "0.24"` dependency

Deliverables (complete):
1. MQTT channel module with connect/subscribe lifecycle and health behavior.
2. Config wiring for broker URL/client id/topics/qos/auth/tls.
3. SOP launch path from MQTT events into shared SOP engine via `dispatch_sop_event`.
4. Tests for config validation and topic routing.

### Phase 6A — Webhook Fan-In (Implemented)

Files:
- `src/sop/dispatch.rs` — unified dispatch helper, cron cache, peripheral helper
- `src/gateway/mod.rs` — webhook SOP fan-in (non-blocking, returns HTTP 202)
- `src/daemon/mod.rs` — single shared SOP engine wired to all components

Deliverables (complete):
1. Webhook -> `SopEvent { source: Webhook, topic: uri_path, ... }` via `handle_webhook` non-blocking dispatch.
2. Unified dispatch helper with batch-lock pattern and `SopRunAction` propagation.
3. Unit tests for dispatch, webhook routing, and action capture.

### Phase 6B — Cron Fan-In (Implemented)

Files:
- `src/cron/scheduler.rs` — window-based cron SOP tick + approval timeout polling
- `src/sop/dispatch.rs` — `SopCronCache`, `check_sop_cron_triggers`

Deliverables (complete):
1. Cron scheduler tick -> `SopEvent { source: Cron, ... }` via window-based `check_sop_cron_triggers`.
2. Approval timeout polling integrated into scheduler tick loop.
3. Pre-parsed cron cache built once at daemon startup.
4. Unit tests for cron cache, window-based evaluation.

### Phase 6C — Peripheral Fan-In (Dispatch Helper Only — Runtime Deferred)

Files:
- `src/sop/dispatch.rs` — `dispatch_peripheral_signal` helper

Status: The dispatch helper function exists and is tested. **No runtime wiring** — there is
no callback bus, event loop, or daemon integration for peripheral signals. Real wiring requires
a firmware callback protocol that does not yet exist. This phase should not be claimed as
"implemented" until a peripheral can trigger a SOP at runtime without manual invocation.

Deliverables (partial):
1. `dispatch_peripheral_signal()` helper builds `SopEvent { source: Peripheral, topic: "{board}/{signal}" }` and routes through `dispatch_sop_event`.
2. Unit tests for peripheral signal dispatch.
3. **NOT delivered**: runtime callback bus, daemon wiring, integration test with real or simulated hardware.

## Known Limitations

1. **Pre-existing clippy warnings**: The following clippy warnings exist on `main` and are outside SOP scope:
   - `option_option` in `proxy_config.rs`
   - `identical_match_arms` in `reliable.rs`
   - `returning_let_binding` in `wizard.rs`
   - `manual_string_new` in `matrix.rs`
2. **Pre-existing test failure**: `onboard::wizard::tests::run_models_refresh_rejects_unsupported_provider` fails on `main` (Venice provider behavior changed; test expectation is stale).
3. **Standalone gateway runs SOP-less**: The `gateway` CLI command (`src/main.rs`) starts the gateway without a SOP engine. This is intentional — only `daemon` mode creates and shares the SOP engine. Standalone gateway is for testing/development.
4. **SopCronCache not hot-reloaded**: Built once at daemon startup. Adding/changing SOP cron triggers requires daemon restart.
5. **MQTT TLS**: URL scheme (`mqtts://`) controls TLS via `Transport::tls_with_default_config()`. Client certificate auth is deferred.
6. **Headless fan-in cannot execute SOP steps**: Fan-in callers (MQTT, webhook, cron) start SOP runs and audit them, but cannot execute `ExecuteStep` actions without an agent loop. Runs are started in the engine and logged via `process_headless_results`. Approval timeout polling handles `WaitApproval` actions.
7. **Custom SOP webhook paths**: Use `POST /sop/{path}` for SOP triggers with custom paths (e.g., `POST /sop/deploy` matches trigger `Webhook { path: "/sop/deploy" }`). The `/webhook` route matches triggers with `path: "/webhook"`.

## P0 Gate Before Phase 5/6

Before Phase 5/6 routing, close the current runtime gaps:

1. Deterministic trigger `condition` evaluation.
- Files: `src/sop/engine.rs`, `src/sop/mod.rs`, `src/sop/condition.rs` (new)

2. SOP audit logger wiring in real tool flow.
- Files: `src/sop/audit.rs`, `src/tools/sop_execute.rs`, `src/tools/sop_approve.rs`, `src/tools/sop_advance.rs`, `src/tools/mod.rs`

3. Runtime scheduling for approval timeouts.
- Files: `src/agent/loop_.rs`, `src/sop/engine.rs`
- Note: `src/channels/mod.rs` and `src/tools/traits.rs` were evaluated during P0 review and required no changes — channels pass `None` for sop_engine (correct for Phase 4), and tool traits already support the SOP tool interface.

Reason:
- Without P0, Phase 5/6 event fan-in can create non-deterministic or untracked SOP transitions.

## Verification

Run in `/data/projects/zeroclaw`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test sop:: --lib
cargo test tools::sop --lib
cargo test
```

## Ownership Split

- `zeroclaw`: collective implementation (this plan)
- `ampersona`: separate repo, separately owned and planned
