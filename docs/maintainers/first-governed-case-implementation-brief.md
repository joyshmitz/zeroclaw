# First Governed Case Implementation Brief

Timestamp: 2026-03-21T20:10:07+02:00

## Status

This document is the first module-level implementation brief for the chosen first governed case:

- scenario: `quality or operational incident`
- planning basis: [first-governed-case-mvp-plan.md](first-governed-case-mvp-plan.md)
- architecture basis: [fork-architecture-brief.md](fork-architecture-brief.md)

It does not define a broad refactor.
It defines the smallest code-facing implementation address for the first governed path.

## Purpose

The purpose of this brief is to close the remaining gap between:

- a coherent product thesis
- a coherent architecture brief
- a coherent first MVP scenario
- and the actual current module layout of the repository

The goal is to say, in concrete codebase terms:

`where should the first governed case enter the runtime`

## Current Code Facts

The current repo already provides several useful footholds.

### 1. `process_message(...)` is the current gateway/webhook seam, not a universal non-interactive seam

Current gateway tool-enabled handling flows through:

- [src/gateway/mod.rs](../../src/gateway/mod.rs)
  - `run_gateway_chat_with_tools(...)`
- [src/agent/loop_.rs](../../src/agent/loop_.rs)
  - `process_message(...)`

Current channel handling does **not** flow through `process_message(...)`.
In [src/channels/mod.rs](../../src/channels/mod.rs), channel messages currently do their own:

- memory recall
- system-prompt construction
- history assembly
- direct `run_tool_call_loop(...)` invocation

This makes `process_message(...)` the clearest first governed seam for the current gateway/webhook path, but not yet a shared seam across all non-interactive ingress.

### 2. Generic agent behavior currently starts before governed handling exists

In [src/agent/loop_.rs](../../src/agent/loop_.rs), the current flow still centers on:

- context enrichment
- system prompt construction
- provider dispatch
- `run_tool_call_loop(...)`

That means the current runtime is still upstream-shaped:
message in, LLM/tool motion out.

### 3. SOP substrate already exists and is wired in

The current repo already has:

- [src/sop/mod.rs](../../src/sop/mod.rs)
  - `create_sop_engine(...)`
- [src/tools/mod.rs](../../src/tools/mod.rs)
  - conditional SOP tool registration
- [src/agent/agent.rs](../../src/agent/agent.rs)
  - SOP engine passed into tool construction
- [src/gateway/mod.rs](../../src/gateway/mod.rs)
  - SOP engine passed into gateway tool construction

So bounded procedure execution substrate is present, even though governed-case handling is not.

### 4. Approval behavior already exists, but matters for scope

The current repo already has:

- [src/approval/mod.rs](../../src/approval/mod.rs)
  - `ApprovalManager`
  - `ApprovalManager::for_non_interactive(...)`

Fact that matters for this MVP:

- in non-interactive runs, tools that require interactive approval are auto-denied rather than prompted

This means the first governed case should not assume rich live approval UX in channel/gateway paths.

### 5. SOP audit persistence already exists

The current repo already has:

- [src/sop/audit.rs](../../src/sop/audit.rs)
  - `SopAuditLogger`

It persists SOP run and step records to the configured memory backend.

This is not yet full governed-case persistence.
But it is a real evidence foothold the first MVP can reuse.

## Implementation Judgment

The first governed case should enter the runtime at:

- [src/agent/loop_.rs](../../src/agent/loop_.rs)
  - `process_message(...)`

For the first MVP, it should enter there:

- after transport-specific ingress has already been normalized into a message/event payload
- before system-prompt-driven LLM/tool motion begins

This is the narrowest current seam that is both:

- real in the codebase
- already on the gateway/webhook tool-enabled path

## MVP Path Scope

The first code pass should explicitly scope the first governed seam to gateway/webhook-originated signals that already reach `process_message(...)`.

The current channel runtime in [src/channels/mod.rs](../../src/channels/mod.rs) is a parallel non-interactive path.
It currently performs its own memory recall, system-prompt assembly, history construction, and `run_tool_call_loop(...)` call without passing through `process_message(...)`.

That means channel-path governed handling is a follow-on second pass, not silently covered by the first `process_message(...)` insertion.
This is acceptable for the MVP because the fork first needs one honest narrow seam that makes the thesis operational without expanding immediately into one of the repo's largest ingress surfaces.

That follow-on choice has now been made (2026-03-22):

- **Decision: shared pre-motion helper (Variant A)**
- Extract governed admission logic from `process_message(...)` into a standalone function
- Call that function from both `process_message(...)` and the generic channel-listener path in `src/channels/mod.rs`
- Admission logic stays single source of truth — no duplication, no ongoing sync burden
- Touch in `channels/mod.rs` should be minimal: one call before `run_tool_call_loop(...)`
- This is the next code step toward Milestone A (Semantic Convergence)

## When To Start

The first governed seam should start after the current document set passes review.

That means:

- the product definition remains accepted as the controlling frame
- the architecture brief remains accepted as the codebase bridge
- the first governed-case MVP plan remains accepted as the chosen scenario
- the conflict surface map remains accepted as the merge-pressure baseline
- this implementation brief remains accepted as the first insertion address

This first code step should **not** wait for a future stable release or beta tag by default.

Why:

- the current insertion point is already clear enough
- waiting does not reduce the current thesis-to-code gap
- upstream beta and release watching should continue in parallel, not replace the first fork-owned seam

The first code step should be delayed only if one of the following becomes true before work starts:

- upstream materially rewrites the `process_message(...)` path
- upstream introduces orchestration or approval primitives that change the meaning of the chosen seam
- review finds a conceptual contradiction in the current document chain

## Minimal First Code Step

The first code step should be a narrow insertion in:

- [src/agent/loop_.rs](../../src/agent/loop_.rs)
  - `process_message(...)`

The insertion point should be:

- after the incoming payload has reached `process_message(...)`
- before `build_context(...)`
- before `ChatMessage::user(&enriched)` is created
- before `agent_turn(...)` begins generic provider/tool motion

The first pass should do only three new things there:

1. detect whether the incoming payload is an explicit incident candidate
2. build a minimal in-flight governed-case draft for that scenario
3. choose the first bounded response mode before generic motion

That is the smallest code step that makes the fork thesis operational rather than documentary.

## MVP Signal Admission Contract

The first pass should not rely on broad keyword spotting across ordinary chat text.
`process_message(...)` currently receives only `message: &str`, so the admission rule needs to stay explicit.

The preferred target signal format for this MVP is a structured incident envelope with a top-level `signal_type` field, for example `signal_type = "incident"`, plus only the minimal fields needed for first-pass handling such as incident kind, severity, summary, and optional evidence hints.

Because the current webhook handler still normalizes `{\"message\": \"...\"}` into a raw string, the bootstrap implementation may admit either:

- a structured JSON envelope carried inside the existing `message` string
- a clearly marked textual submission such as `INCIDENT:` or `[incident]`

Free-form messages without an explicit marker or structured envelope should remain observation-only in this first pass.
That keeps false positives bounded and avoids silently redefining normal gateway traffic as governed-case admission.

## Minimal File Footprint

The preferred first code footprint should be:

- one narrow call-site change in [src/agent/loop_.rs](../../src/agent/loop_.rs)
- one new fork-owned helper module for incident classification and first governed-case drafting

The first pass should avoid touching these shared central surfaces unless the compiler forces it:

- [src/agent/agent.rs](../../src/agent/agent.rs)
- [src/gateway/mod.rs](../../src/gateway/mod.rs)
- [src/channels/mod.rs](../../src/channels/mod.rs)
- [src/tools/mod.rs](../../src/tools/mod.rs)
- [src/config/schema.rs](../../src/config/schema.rs)
- [src/lib.rs](../../src/lib.rs)
- [src/main.rs](../../src/main.rs)

This matters because the first seam should reduce architectural uncertainty without inflating known merge surfaces.

## First Implementation Scope

The first implementation scope should be limited to the gateway/webhook non-interactive incident-handling path.

Working target:

- explicit incident-like input arrives through the existing gateway/webhook path that reaches `process_message(...)`
- `process_message(...)` recognizes it as an incident candidate before generic loop execution
- the runtime chooses governed handling before open-ended agent/tool behavior

Channel-path governed handling remains a second-pass concern after the first seam is proven on the gateway path.

This means the first pass should **not** try to cover:

- interactive CLI mode first
- channel-path parity first
- telemetry-window anomaly emergence
- broad inbox classification
- final case database design

## Minimal Seam Placement

The first governed case should be implemented through four minimal placements.

### 1. Classification seam

Placement:

- near the beginning of `process_message(...)`
- before generic context injection and before `run_tool_call_loop(...)`

Responsibility:

- detect whether the payload is an explicit incident candidate
- classify it with the cheapest useful deterministic rules
- decide observation-only vs primary-signal handling

Why here:

- this is the earliest narrow point on the current gateway/webhook path where fork meaning can enter without changing transport handling

### 2. Governed case draft seam

Placement:

- immediately after incident classification in the same first pass

Responsibility:

- create the minimal in-flight governed case representation for this scenario
- bind:
  - signal reference
  - working interpretation
  - severity
  - response mode
  - SOP candidate
  - evidence requirement
  - approval requirement
  - outcome/update disposition

Why here:

- the first MVP needs case semantics before tool motion
- it does not yet need final persistence architecture

### 3. Pre-planning decision seam

Placement:

- after case draft creation
- before generic LLM dispatch and before free-form tool motion

Responsibility:

- choose one bounded response mode such as:
  - request evidence
  - route or assign
  - stage for approval
  - apply SOP in bounded mode
  - escalate for richer interpretation

Why here:

- this is the exact point where selective cognition becomes real behavior instead of doctrine

### 4. Evidence and outcome seam

Placement:

- during bounded response handling and at case outcome

Responsibility:

- reuse existing SOP audit surfaces where possible
- record enough structured outcome to support PDCA review

Why here:

- the first governed case only proves value if it leaves a reviewable trail

## What The First Pass Should Reuse As-Is

The first implementation brief should continue to reuse upstream substrate for:

- gateway ingress
- channel ingress
- runtime assembly
- provider dispatch
- generic tool infrastructure
- daemon/service lifecycle

The fork should add governed meaning before generic motion, not replace the upstream runtime.

## What The First Pass Should Not Touch First

To keep scope bounded, the first governed case should **not** start by redesigning:

- [src/gateway/mod.rs](../../src/gateway/mod.rs)
- [src/main.rs](../../src/main.rs)
- [src/lib.rs](../../src/lib.rs)
- global config shape in [src/config/schema.rs](../../src/config/schema.rs)
- the generic tool registry model in [src/tools/mod.rs](../../src/tools/mod.rs)

Those surfaces matter, but they are not the best first insertion point for proving the first governed case.

## Approval Implication

One fact should constrain the first implementation:

- non-interactive approval currently auto-denies tools that need interactive approval

So for this MVP:

- approval-requiring incident handling should usually become `stage for approval`
- not "attempt tool execution and hope approval appears later"

This is a strong reason to keep the first governed case centered on bounded supervised handling.

## SOP Implication

Another fact should constrain the first implementation:

- SOP substrate already exists
- but it is currently attached to tool/runtime assembly, not to governed-case semantics

So the first pass should not try to prove SOP as a generic workflow engine.
It should prove a narrower claim:

- once a governed incident case exists, SOP can become the bounded procedure selected for that case

## Evidence Implication

Current evidence capability is good enough for a first pass if the fork uses it honestly:

- SOP audit can already persist run and step records
- approval logic can already record decisions

What is still missing is broader governed-case persistence.

That means the first pass should:

- reuse SOP audit for execution evidence
- keep case persistence minimal
- avoid pretending the final evidence model is already solved

## Review Criteria

This implementation brief is successful if it leads to a first code pass where all of the following become true:

- an explicit incident candidate is recognized before generic tool-loop execution
- a minimal governed case exists before open-ended motion
- response mode is selected before provider/tool dispatch
- approval-required motion is staged rather than accidentally free-running
- SOP and evidence surfaces are used as bounded execution substrate
- a PDCA-relevant outcome can be reviewed afterward

## Relationship To Upstream

This brief should be read in the fork's intended collaboration posture:

- upstream remains the runtime substrate
- the fork is not trying to fork away from every upstream architectural decision
- the fork is trying to add one controlled governed-response seam at the smallest viable place

That is why the first target is a narrow insertion in the existing path, not a competing runtime architecture.

## Bottom Line

The first governed case should not begin by changing the whole repo.

It should begin by inserting one fork-owned governed-response seam into the existing non-interactive path centered on:

- [src/agent/loop_.rs](../../src/agent/loop_.rs)
  - `process_message(...)`

That is the smallest current implementation address where the thesis can start becoming runtime behavior on the gateway/webhook path.
