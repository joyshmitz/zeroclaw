# Fork Architecture Brief

Timestamp: 2026-03-21

## Status

This document is the first architecture bridge between:

- the fork product definition in [`PLAN_TO_FUTURE_PRODUCT.md`](/data/projects/zeroclaw/PLAN_TO_FUTURE_PRODUCT.md)
- the current upstream-shaped ZeroClaw codebase

It exists because the product thesis is now coherent, but is not yet the controlling architectural frame of the repository.

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

It is also shaped by the 2026-03-21 fork audit conclusion:

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
5. pre-planning decision
6. bounded response execution
7. evidence and audit capture
8. PDCA feedback into `Plan`

In compact form:

`ingress -> meaning -> governed case -> bounded response -> evidence -> update Plan`

This pipeline is architectural, not transport-specific.

## Fork-Owned Seams

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

### 2. Governed Case Seam

Location in flow:

- after signal classification
- before agent/tool execution chooses motion

Purpose:

- open a governed case
- update an existing governed case
- bind context, SOP, evidence, approvals, and response mode

This seam should become the central operational unit of the fork.

### 3. Pre-Planning Decision Seam

Location in flow:

- after case formation
- before LLM dispatch or tool execution

Purpose:

- decide whether the next step is:
  - observe only
  - request evidence
  - route or assign
  - apply SOP directly
  - stage for approval
  - permit constrained execution
  - escalate to richer cognition

This seam is where selective cognition becomes architectural rather than rhetorical.

### 4. Evidence and Governance Seam

Location in flow:

- during and after action selection

Purpose:

- enforce autonomy boundaries
- capture decision path
- capture evidence sufficiency
- preserve auditability

Without this seam, the fork collapses back into generic tool-calling behavior.

### 5. PDCA Feedback Seam

Location in flow:

- after outcome capture

Purpose:

- decide whether outcome only closes a case
- or whether it should also trigger `update Plan`
- connect repeated outcomes back to SOP, policy, thresholds, routing, or autonomy

This seam turns governed response into a loop instead of a one-off reaction.

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
- response-bounding mechanism
- later-stage procedure execution and approval/gating logic

Current problem:

- SOP exists in the repo but not yet as the natural continuation of signal classification and governed case handling
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

This brief does **not** define code changes yet.
It only defines the required next architectural moves:

1. define the signal classification seam
2. define the governed case seam
3. define the pre-planning decision seam
4. map SOP, autonomy, evidence, and approvals onto those seams
5. define which seams are fork-owned and which remain upstream-owned

If those five things become explicit, the fork thesis stops being document-only.

## Immediate Non-Goals

This brief does not attempt to:

- redesign the whole upstream runtime
- replace all workflow, telemetry, or edge systems
- define final storage models for governed cases
- define final persistence architecture
- refactor all conflict surfaces immediately
- turn the fork into an orchestration platform first

## Required Follow-On Artifacts

This brief makes the following next artifacts mandatory:

1. First Governed Case MVP Plan
   - pick one scenario
   - define the smallest path through signal -> case -> bounded response -> evidence
2. Conflict Surface Map
   - document the repeated fork/upstream collision zones
   - distinguish tactical conflicts from ownership problems

## Revisit Triggers

Revisit this brief when one of these becomes true:

- a fork-owned seam is identified more precisely
- upstream introduces orchestration or workflow primitives that challenge SOP positioning
- repeated merges continue hitting the same cross-cutting setup zones
- the first governed-case MVP forces a different placement of SOP, autonomy, or evidence logic

## Bottom Line

The architectural task is not to replace upstream.

It is to insert a fork-owned governed-response layer between generic runtime ingress and generic runtime action.

Until that layer exists, the fork thesis remains correct but only partially operationalized.
