# Fork Architecture Brief

Timestamp: 2026-03-22T10:02:37+02:00

## Status

This document is the first architecture bridge between:

- the fork product definition in [`PLAN_TO_FUTURE_PRODUCT.md`](../../PLAN_TO_FUTURE_PRODUCT.md)
- the current upstream-shaped ZeroClaw codebase

It exists because the product thesis is now coherent, but is not yet the controlling architectural frame of the repository.

This brief should now be read together with [formal-core-implementation-map.md](formal-core-implementation-map.md), which records the current code-facing status of the formal core and the remaining gap to architectural convergence.

## Purpose

The purpose of this brief is to prevent the fork from drifting into:

- product thesis as document only
- upstream feature accumulation as de facto architecture
- repeated conflict resolution without seam ownership

This brief should answer one question:

`where does governed response actually live in the architecture of this fork`

## Source Basis

This brief is constrained by the current fork product definition:

- `primary signal -> meaning -> governed response`
- PDCA framing: `primary signal -> Check -> Act -> update Plan`
- SOP is the current core of `Plan`
- signal type matters more than delivery channel
- cognition should be selective, not maximal by default
- governed response is better described by a `response_envelope` than by a single response mode
- `update Plan` should become reviewable change admission, not silent runtime self-mutation

It is also shaped by the fork audit conclusion current at the time this document version was written:

- the repo is partially drifting but recoverable
- the thesis is coherent
- the codebase remains upstream-shaped
- governance components are present but architecturally orphaned

## Architectural Judgment

The current repository should be understood as:

- an upstream-provided runtime substrate
- plus fork-specific governance and procedure components
- without yet having a fork-owned integration layer that turns one into the other

This is the key architectural problem.

The fork does **not** need to replace the upstream runtime.
But it does need explicit fork-owned seams where governed response enters the system.

## Canonical Pipeline

The fork architecture should be oriented around this pipeline:

1. signal ingress
2. signal emergence / normalization
3. signal classification and operational meaning
4. governed case formation or case update
5. response-envelope decision
6. bounded response execution
7. evidence and audit capture
8. PDCA feedback and reviewable plan-change proposal

In compact form:

`ingress -> meaning -> governed case -> response envelope -> bounded response -> evidence -> PDCA feedback`

This pipeline is architectural, not transport-specific.

## Fork-Owned Seams

The fork's architectural posture is orchestration over upstream substrate. Each seam below should be understood as an orchestration concern — the fork connects existing upstream components, not replaces them. Where a seam can be satisfied by configuring an upstream component, configuration is preferred over custom code.

The fork needs explicit ownership of the following seams.

### 1. Signal Classification Seam

Location in flow:

- after ingress
- before generic agent-loop behavior

Purpose:

- determine whether an incoming event is only observation
- determine whether it becomes a primary signal
- attach initial operational meaning
- decide whether case creation or case update is warranted

This seam is the first point where the fork stops being a generic inbox/runtime substrate and becomes a governed response runtime.

**Implementation posture:** primarily configuration. Upstream classifier already provides rule-based classification with configurable keywords, patterns, and priorities. Fork work: define classification rules that carry signal-type meaning. Orchestration code needed only for the routing decision that acts on classifier output.

### 2. Governed Case Seam

Location in flow:

- after signal classification
- before agent/tool execution chooses motion

Purpose:

- open a governed case
- update an existing governed case
- bind context, SOP, evidence, approvals, and response envelope
- become the durable operational unit of the fork rather than a transport-local draft

This seam should become the central operational unit of the fork.

**Implementation posture:** orchestration code required. No upstream component provides governed case identity, lifecycle, or state. This is the primary fork-owned code surface — minimal but irreducible.

### 3. Pre-Planning / Response-Envelope Seam

Location in flow:

- after case formation
- before LLM dispatch or tool execution

Purpose:

- derive the governed response envelope for the case
- make explicit which bounded next steps are currently allowed, such as:
  - observe only
  - request evidence
  - route or assign
  - apply SOP directly
  - stage for approval
  - permit constrained execution
  - escalate to richer cognition
- bind evidence requirements, approval conditions, allowed actions, escalation paths, and closure conditions before generic motion

This seam is where selective cognition becomes architectural rather than rhetorical.

**Implementation posture:** orchestration code with configuration. The decision logic is fork code; the enforcement uses upstream SecurityPolicy, ApprovalManager, and AutonomyLevel. Configuration defines thresholds and boundaries.

### 4. Evidence and Governance Seam

Location in flow:

- during and after action selection

Purpose:

- enforce autonomy boundaries
- capture decision path
- capture evidence sufficiency
- preserve the evidence and approval state that justifies the current response envelope
- preserve auditability

Without this seam, the fork collapses back into generic tool-calling behavior.

**Implementation posture:** primarily configuration over upstream substrate. ApprovalManager, SecurityPolicy, SOP audit, and Observer already provide the mechanisms. Fork work: configure what evidence is required per case type and ensure the orchestration layer routes outcomes through existing audit paths.

### 5. PDCA Feedback Seam

Location in flow:

- after outcome capture

Purpose:

- decide whether outcome is `close_only`
- or whether it should emit a reviewable `plan_change_proposal`
- connect repeated outcomes back to SOP, policy, thresholds, routing, or autonomy

This seam turns governed response into a loop instead of a one-off reaction.

**Implementation posture:** orchestration code required. TrustTracker provides the scoring substrate but the feedback disposition decision (close_only vs plan_change_proposal) and the wiring from SOP outcomes to trust corrections are fork-owned logic.

## What Upstream Provides As Substrate

The fork should treat these areas primarily as upstream substrate unless a concrete fork need proves otherwise:

- channels and message ingress
- gateway/server machinery
- provider integrations
- generic tool registry mechanics
- memory backends
- runtime adapters
- broad hardware/peripheral support
- general daemon/service lifecycle

These are valid runtime surfaces for the fork.
They are not, by themselves, the fork’s product identity.

## Existing Components Mapped To The Pipeline

### Upstream-Centered Core

Current repository reality is still centered on:

- message ingress
- agent loop
- provider dispatch
- tool execution
- response return

This is why the repo still behaves architecturally like an upstream LLM runtime.

### SOP

Current best fit:

- core of `Plan`
- current primary bounded-execution substrate once the response envelope permits SOP handling
- later-stage procedure execution and approval/gating logic

Current problem:

- SOP exists in the repo but not yet as the natural continuation of signal classification, governed case handling, and response-envelope derivation
- it is wired in, but not yet architecturally seated in the canonical pipeline

### Hands

Current likely fit:

- bounded action substrate
- execution modality under governance

Current problem:

- present conceptually, but not yet mapped clearly into the fork pipeline

### Verifiable Intent

Current likely fit:

- governance/evidence seam
- intent checking before high-consequence action

Current problem:

- not yet presented as part of a coherent fork-owned response boundary

### Autonomy / Security / Observability

Current fit:

- governance seam
- evidence seam
- audit seam
- partial substrate for a future response envelope

These are compatible with the fork thesis, but need to be addressed explicitly through the fork pipeline rather than treated as generic runtime accessories.

## Current Architectural Tension

The fork currently has this tension:

- upstream architecture assumes a generic multi-channel agent runtime
- fork product definition assumes a governed response runtime

This tension is still recoverable because:

- the fork delta is not yet enormous
- the thesis is coherent
- the substrate is still reusable

But it will become expensive if the fork keeps adding product-definition detail without creating fork-owned seams.

The current implementation map makes the present code-facing reality sharper:

- the repo already proves one narrow governed seam
- that seam is best described as `incident-scoped admission -> deterministic draft -> SOP dispatch -> audit`
- this is enough to validate first entry into runtime
- it is not yet enough to claim full architectural convergence

## Architectural Convergence Framing

Architectural convergence should be treated here as a milestone contract, not as a vague aspiration.

This brief follows the convergence framing in [`PLAN_TO_FUTURE_PRODUCT.md`](../../PLAN_TO_FUTURE_PRODUCT.md) and the code-facing grounding in [formal-core-implementation-map.md](formal-core-implementation-map.md).

### Milestone A: Semantic Convergence

Meaning:

- every governed-capable ingress path evaluates an equivalent admission, case, and response-envelope contract before generic motion
- this is semantic parity, not a demand that every ingress path share one identical code path

Under orchestration posture: achieved when upstream classifier is configured with signal-type rules on every governed-capable ingress path, and the orchestration layer evaluates classifier output before generic motion on each path.

### Milestone B: Durable Case Convergence

Meaning:

- governed case becomes a durable operational unit
- case identity and history outlive transport-local messages, webhook calls, and individual SOP runs

Under orchestration posture: achieved when GovernedCase exists as orchestration-layer code (not upstream substrate) with identity and state persisted through upstream session/memory mechanisms.

### Milestone C: PDCA Convergence

Meaning:

- meaningful outcomes record explicit feedback disposition
- the runtime can emit reviewable `plan_change_proposal` artifacts rather than leaving PDCA only in prose or logs

Under orchestration posture: achieved when the orchestration layer produces plan_change_proposal artifacts by wiring SOP audit outcomes through TrustTracker and feedback disposition logic.

Current status:

- the implementation map confirms that none of A, B, or C are fully reached yet

## Conflict Surface Implications

Known repeated-risk seams already suggest where architectural ownership is missing:

- `src/agent/agent.rs`
- `src/gateway/mod.rs`
- `src/tools/mod.rs`
- `src/config/schema.rs`
- `src/lib.rs`
- `src/main.rs`

These are not just git-conflict hotspots.
They are signs that fork-specific meaning is entering the system only through cross-cutting wiring points.

That is acceptable temporarily.
It is not a stable long-term architecture.

## First Architecture Moves

This brief does **not** define code changes line-by-line.
It defines the required next architectural moves:

1. define the signal classification seam
2. define the governed case seam as a durable operational unit
3. define the response-envelope seam before generic motion
4. map SOP, autonomy, evidence, approvals, and closure conditions onto that envelope
5. define the PDCA feedback seam so outcomes can emit reviewable `plan_change_proposal` artifacts
6. define which seams are fork-owned and which remain upstream-owned

If those six things become explicit, the fork thesis stops being document-only.

## Immediate Non-Goals

This brief does not attempt to:

- redesign the whole upstream runtime
- replace all workflow, telemetry, or edge systems
- define final storage models for governed cases
- define final persistence architecture
- refactor all conflict surfaces immediately
- turn the fork into an orchestration platform first

## Required Follow-On And Alignment Artifacts

This brief makes the following next artifacts mandatory:

1. First Governed Case MVP Plan
   - pick one scenario
   - define the smallest path through signal -> case -> bounded response -> evidence
2. Conflict Surface Map
   - document the repeated fork/upstream collision zones
   - distinguish tactical conflicts from ownership problems
3. Formal Core Implementation Map
   - map each formal-core function to the current code seams
   - make the remaining distance to Milestones A, B, and C explicit

## Revisit Triggers

Revisit this brief when one of these becomes true:

- a fork-owned seam is identified more precisely
- upstream introduces orchestration or workflow primitives that challenge SOP positioning
- repeated merges continue hitting the same cross-cutting setup zones
- the first governed-case MVP forces a different placement of SOP, autonomy, or evidence logic

## Bottom Line

The architectural task is not to replace upstream.

It is to insert a fork-owned governed-response layer between generic runtime ingress and generic runtime action, and to carry that layer through semantic convergence, durable case handling, and PDCA closure.

The fork achieves this not by building custom parallel systems, but by configuring and orchestrating existing upstream components through a thin orchestration layer. The product identity is in the chain and its configuration, not in reimplemented mechanisms.

Until that layer exists, the fork thesis remains correct but only partially operationalized.
