# SOP Foundation Plan (feat/sop-foundation) — 2026-02-19

Repository:
- `/data/projects/zeroclaw`

Branch:
- `feat/sop-foundation`

## Status

Phases 1-4 are implemented on this branch.  
The remaining planned scope is:

- Phase 5 — MQTT channel + SOP routing
- Phase 6 — webhook/cron/peripheral event-source routing to SOP

## Remaining By Plan

### Phase 5 — MQTT Channel

Target:
- `src/channels/mqtt.rs` (new)
- `rumqttc` integration
- `MqttConfig` (config schema + defaults)
- runtime route: MQTT message -> `SopEvent { source: Mqtt, ... }` -> `SopEngine::match_trigger`

Deliverables:
1. MQTT channel module with connect/subscribe lifecycle and health behavior.
2. Config wiring for broker URL/client id/topics/qos/auth/tls.
3. SOP launch path from MQTT events into shared SOP engine state.
4. Tests for topic routing and reconnect/error handling.

### Phase 6 — Other Event Sources

Target:
- webhook route integration
- cron trigger integration
- peripheral callback integration

Deliverables:
1. Webhook -> `SopEvent { source: Webhook, ... }` mapping into SOP engine.
2. Cron scheduler tick -> `SopEvent { source: Cron, ... }`.
3. Peripheral signal callbacks -> `SopEvent { source: Peripheral, ... }`.
4. End-to-end tests for source->match->run start.

## P0 Gate Before Phase 5/6

Before Phase 5/6 routing, close the current runtime gaps:

1. Deterministic trigger `condition` evaluation.
- Files: `src/sop/engine.rs`, `src/sop/mod.rs`, `src/sop/condition.rs` (new)

2. SOP audit logger wiring in real tool flow.
- Files: `src/sop/audit.rs`, `src/tools/sop_execute.rs`, `src/tools/sop_approve.rs`, `src/tools/sop_advance.rs`, `src/tools/mod.rs`

3. Runtime scheduling for approval timeouts.
- Files: `src/agent/loop_.rs`, `src/channels/mod.rs`, `src/tools/traits.rs`, `src/sop/engine.rs`

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
