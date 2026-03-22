# Formal Core Implementation Map

Timestamp: 2026-03-22T09:56:06+02:00

## Status

This document maps the formal core from [`PLAN_TO_FUTURE_PRODUCT.md`](../../PLAN_TO_FUTURE_PRODUCT.md) onto the current codebase.

It is not a new product-definition document.
It is a code-facing ground-truth note for maintainers.

## Purpose

The purpose of this document is to answer:

`where does each formal core function live in code today, and what is still missing before the fork can claim architectural convergence`

It exists to keep three things aligned:

- the product definition
- the implementation briefs
- the actual current module seams

## Scope

This document covers only the current formal core:

- `Emergence_k`
- `Interpret_k`
- `CaseBind`
- `Envelope`
- `Execute`
- `Feedback`

It also summarizes the remaining distance to:

- Milestone A: Semantic Convergence
- Milestone B: Durable Case Convergence
- Milestone C: PDCA Convergence

## Summary Judgment

Current code already implements a real first governed seam, but only for a narrow incident-handling slice.

The most accurate short description today is:

`incident-scoped admission -> deterministic draft -> SOP dispatch -> audit`

This is enough to prove a governed seam on the gateway-managed path (webhook + WhatsApp, Linq, WATI, Nextcloud Talk).
It is not yet enough to claim a full fork-owned governed-response runtime contract, because the generic channel-listener path still bypasses it.

## Formal Core Mapping

### 1. `Emergence_k`

Current best fit:

- [`src/agent/governed.rs`](../../src/agent/governed.rs)
  - `draft_incident_case(...)`
  - `draft_from_envelope(...)`
  - `draft_from_marker(...)`
- [`src/agent/loop_.rs`](../../src/agent/loop_.rs)
  - `process_message(...)`

What exists:

- explicit incident admission exists as a real seam
- the current admission contract is intentionally narrow
- the code admits either:
  - a structured incident envelope
  - an explicit textual marker such as `INCIDENT:`
- if neither path matches, the message falls through to ordinary generic motion

What this means in practice:

- this is already a concrete `observation -> primary signal or not` gate
- the negative branch is currently implicit through `Option<GovernedCaseDraft>`

What is still missing:

- a typed `observation_only` result rather than implicit `None`
- emergence rules beyond explicit incidents
- shared admission parity across all governed-capable ingress paths

Important current coverage and limit:

- the governed seam already covers the full `gateway -> process_message(...)` path, which includes:
  - the main webhook endpoint
  - WhatsApp, Linq, WATI, and Nextcloud Talk handlers in [`src/gateway/mod.rs`](../../src/gateway/mod.rs) via `run_gateway_chat_with_tools(...)`
- the bypass is specifically the generic channel-listener path in [`src/channels/mod.rs`](../../src/channels/mod.rs), which goes directly into `run_tool_call_loop(...)` without passing through `process_message(...)`
- this means the governed seam is broader than "webhook only" but narrower than "all ingress"

### 2. `Interpret_k`

Current best fit:

- [`src/agent/governed.rs`](../../src/agent/governed.rs)
  - `draft_from_envelope(...)`
  - `draft_from_marker(...)`
  - `build_draft(...)`

What exists:

- deterministic first-pass interpretation already happens during draft construction
- the current code derives:
  - `case_type`
  - `severity`
  - `evidence_present`
  - `webhook_path`
  - initial response selection

What is still missing:

- a first-class typed `interpretation` object
- explicit cognition-admission wording in code as a sub-decision of interpretation
- broader interpretation logic for signal classes beyond incidents

Clarification:

- [`src/agent/classifier.rs`](../../src/agent/classifier.rs) is query-hint routing for generic model selection
- it is not the governed interpretation seam described by the formal core

### 3. `CaseBind`

Current best fit:

- [`src/agent/governed.rs`](../../src/agent/governed.rs)
  - `GovernedCaseDraft`

What exists:

- only an in-flight draft object used before generic motion
- the draft binds enough for first-pass incident handling:
  - admission method
  - case type
  - summary
  - severity
  - evidence present flag
  - current response mode
  - webhook path

What is still missing:

- a durable governed-case model
- `case_id`
- open vs update semantics
- case history
- owner or routing target
- approval state
- outcome trail
- feedback disposition

Judgment:

`GovernedCaseDraft` is a useful intake artifact.
It is not yet a governed case in the product sense.

### 4. `Envelope`

Current best fit:

- [`src/agent/governed.rs`](../../src/agent/governed.rs)
  - `GovernedResponseMode`
  - `build_draft(...)`
- [`src/approval/mod.rs`](../../src/approval/mod.rs)
  - `ApprovalManager`
- [`src/security/policy.rs`](../../src/security/policy.rs)
  - `SecurityPolicy`
- [`src/sop/engine.rs`](../../src/sop/engine.rs)
  - `resolve_step_action(...)`

What exists:

- envelope logic is present, but distributed
- current code already holds real pieces of a future envelope:
  - initial response choice
  - approval requirements
  - autonomy and risk boundaries
  - SOP step execution gating

What is still missing:

- a first-class `response_envelope` type
- one explicit seam that derives the envelope before execution begins
- explicit binding of:
  - evidence requirements
  - approval conditions
  - allowed actions
  - escalation path
  - closure conditions

Important current limit:

- current `GovernedResponseMode` has only four variants:
  - `RequestEvidence`
  - `StageForApproval`
  - `ApplySop`
  - `Escalate`
- `ObserveOnly` and `RouteOrAssign`, which exist in planning language, are not materialized as runtime response modes

Judgment:

the current implementation covers only a narrower incident-handling slice of the response space than the formal core describes.

### 5. `Execute`

Current best fit:

- [`src/sop/types.rs`](../../src/sop/types.rs)
  - `SopRunAction`
- [`src/sop/engine.rs`](../../src/sop/engine.rs)
  - `start_run(...)`
  - `advance_step(...)`
  - `approve_step(...)`
  - `finish_run(...)`
  - `resolve_step_action(...)`
- [`src/agent/loop_.rs`](../../src/agent/loop_.rs)
  - approval-aware tool execution

What exists:

- bounded execution is already the strongest implemented part of the formal core
- SOP runtime already provides typed next-step actions:
  - `ExecuteStep`
  - `WaitApproval`
  - `Completed`
  - `Failed`
- approval-aware tool execution already exists in the generic tool loop

What is still missing:

- explicit attachment of execution to a durable governed case
- a case-centric execution trail outside SOP run records
- one clear seam that says `execute this envelope for this case`

Judgment:

execution substrate is real and reusable.
What it lacks is case-centered ownership.

### 6. `Feedback`

Current best fit:

- [`src/sop/audit.rs`](../../src/sop/audit.rs)
  - `SopAuditLogger`
- [`src/sop/metrics.rs`](../../src/sop/metrics.rs)
  - `SopMetricsCollector`

What exists:

- durable SOP audit exists
- step, run, and approval records are persisted
- metrics already aggregate approval and run-completion signals

What is still missing:

- explicit feedback disposition on governed outcomes
- a first-class `plan_change_proposal` artifact
- code that decides:
  - close only
  - continue watching a pattern
  - escalate review
  - propose a plan change

Judgment:

the repo already has evidence capture.
It does not yet have PDCA closure in code.

## Current Path Reality

Today the first governed seam is real on the `gateway -> process_message(...)` path.

This path is broader than a single webhook endpoint.
It already covers:

- the main webhook endpoint in [`src/gateway/mod.rs`](../../src/gateway/mod.rs)
- gateway-managed channel handlers: WhatsApp, Linq, WATI, and Nextcloud Talk
- all of these reach [`src/agent/loop_.rs`](../../src/agent/loop_.rs) via `run_gateway_chat_with_tools(...)` → `process_message(...)`

The bypass is specifically the generic channel-listener path:

- [`src/channels/mod.rs`](../../src/channels/mod.rs)
  - direct history assembly
  - direct `run_tool_call_loop(...)`
  - used by Telegram, Discord, Slack, and other trait-based `Channel` listeners

This means the fork currently has:

- governed admission on the gateway-managed path (webhook + WhatsApp, Linq, WATI, Nextcloud Talk)
- no governed admission on the generic channel-listener path

## Architectural Convergence Gap Summary

### Milestone A: Semantic Convergence

Current status:

- not reached

Reason:

- governed admission already covers the gateway-managed path: webhook, WhatsApp, Linq, WATI, Nextcloud Talk
- the generic channel-listener path in `src/channels/mod.rs` (Telegram, Discord, Slack, and other trait-based `Channel` listeners) still bypasses `process_message(...)` and has no equivalent governed seam

Concrete gap:

- the remaining gap is narrower than previously described: it is specifically the generic channel-listener path, not "all channels"
- either a shared pre-motion helper is needed
- or an equivalent governed seam must be inserted into the generic channel-listener handling

### Milestone B: Durable Case Convergence

Current status:

- not reached

Reason:

- SOP runs are durable
- governed cases are not

Concrete gap:

- the code has persistent SOP evidence records
- it does not yet have one durable entity that binds:
  - signal
  - interpretation
  - envelope
  - approvals
  - SOP runs
  - outcomes

### Milestone C: PDCA Convergence

Current status:

- not reached

Reason:

- evidence and audit are present
- reviewable plan-change proposal flow is not

Concrete gap:

- the runtime still ends in run records, step records, and metrics
- it does not yet emit a maintained feedback decision such as `close_only` or `plan_change_proposal`

## Final Judgment

The current fork codebase already proves that the product thesis can enter the runtime without a broad refactor.

What it proves today is narrower than the full formal core:

- `Emergence_k` exists for explicit incidents
- `Interpret_k` exists as deterministic draft construction
- `Execute` exists strongly through SOP and approval-aware bounded tool execution

What still separates the current seam from a full governed-response runtime contract is:

- durable case binding
- first-class response envelope
- explicit PDCA feedback closure
