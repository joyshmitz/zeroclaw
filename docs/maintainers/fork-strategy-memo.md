# Fork Strategy Memo

Timestamp: 2026-03-26T05:54:45+02:00

## Status

This document is the canonical starting point for maintainer-level strategy in
the fork.

It is intentionally short.
It does not replace:

- the product thesis in [`../../PLAN_TO_FUTURE_PRODUCT.md`](../../PLAN_TO_FUTURE_PRODUCT.md)
- the fork control rules in [fork-operating-model.md](fork-operating-model.md)
- the architecture bridge in [fork-architecture-brief.md](fork-architecture-brief.md)
- the implementation planning in
  [first-governed-case-mvp-plan.md](first-governed-case-mvp-plan.md)

Its role is to keep those documents aligned under one controlling frame.

## Purpose

This memo exists to answer four questions quickly:

1. What is the fork trying to become?
2. What should remain upstream substrate?
3. What should the fork intentionally own?
4. What should change over the next 2-3 upstream intake cycles?

Use this memo before making non-trivial decisions about:

- fork-owned architecture
- upstream intake adaptation
- maintainer roadmap priority
- release and control policy tradeoffs

## Product Identity

The fork should be treated as a governed-response runtime for heterogeneous
primary signals.

Its compact thesis remains:

`primary signal -> meaning -> governed response`

Its governing loop remains:

`primary signal -> Check -> Act -> update Plan`

The fork is not defined by:

- a transport catalog
- a personal-assistant framing
- a generic workflow engine
- a standards or certification story
- maximal use of cognition

The strongest near-term product form is still:

- quality or operational incident handling

That scenario is the best first proof that the fork can:

- interpret an operationally meaningful signal
- form a governed case
- choose a bounded response before generic agent motion
- capture evidence and outcome
- emit reviewable PDCA feedback

Working rule:

- upstream can keep growing the runtime
- the fork must keep sharpening the meaning and governance layer

## Architectural Boundary

The architectural boundary should stay explicit:

- upstream owns reusable runtime substrate
- the fork owns governed meaning and response seams

### Keep Upstream As Substrate

Treat these areas as upstream-owned by default:

- channels and ingress adapters
- gateway and server machinery
- provider integrations and routing wrappers
- generic tool registry mechanics
- memory backends and retrieval substrate
- runtime adapters
- peripherals, media, desktop, and broad surface expansion
- generic observability and service lifecycle

These surfaces are important.
They are not the fork's product identity by themselves.

### Intentionally Own These Seams

The fork should intentionally own:

- signal admission and classification
- governed case formation and update
- response-envelope choice before generic motion
- evidence, approval, and audit capture
- PDCA feedback and reviewable plan-change proposal

Compact rule:

`upstream owns motion; the fork owns meaning`

### Ownership Guardrail

Avoid broad fork ownership in the busiest shared files unless repeated pressure
proves it is necessary.

Current caution surfaces:

- `src/tools/mod.rs`
- `src/config/schema.rs`
- `src/channels/mod.rs`
- `src/main.rs`
- `src/lib.rs`

Take ownership there only when a fork seam is clear enough to reduce future
merge cost rather than increase it.

## Next 3 Intake Cycles

### Cycle 1: Seam Convergence

Goal:

- make governed admission a shared pre-motion seam rather than a one-off path

Moves:

- converge non-interactive ingress paths on one governed admission helper
- keep the seam above transport and below generic motion
- make channel-path parity an explicit decision instead of an accidental gap

Exit signal:

- equivalent governed admission happens before generic motion on each
  governed-capable ingress path

### Cycle 2: Durable Case Convergence

Goal:

- make the governed case a durable operational unit rather than a local draft

Moves:

- introduce a durable `GovernedCase`
- bind SOP, evidence state, approval state, outcome trail, and routing to that
  case
- treat SOP execution as case-bounded behavior rather than the primary unit

Exit signal:

- cases, not transient transport-local state, become the durable center of
  governed handling

### Cycle 3: Controlled Expansion

Goal:

- prove the fork can expand by signal class without becoming a generic
  orchestration engine

Moves:

- add one adjacent signal class after the first incident path is stable
- materialize missing response-envelope modes such as `ObserveOnly` or
  `RouteOrAssign`
- keep richer cognition as an exception path rather than the default

Exit signal:

- the fork can add one new governed signal class without taking broad ownership
  of upstream substrate

## Explicit Non-Goals

The fork should not drift into:

- transport-first product identity
- certification-first product identity
- workflow-engine-first product identity
- edge-first product identity
- LLM-first product identity
- broad parallel replacement of upstream runtime substrate
- broad fork-specific CLI or UI divergence before seam ownership is clear

## Read Order

Use this reading order for strategic work:

1. this memo
2. [fork-operating-model.md](fork-operating-model.md)
3. [fork-architecture-brief.md](fork-architecture-brief.md)
4. [formal-core-implementation-map.md](formal-core-implementation-map.md)
5. [first-governed-case-mvp-plan.md](first-governed-case-mvp-plan.md)
6. [seam-ownership-roadmap.md](seam-ownership-roadmap.md)

## Update Rule

Update this memo when any of the following changes materially:

- the fork's product identity
- the fork/upstream architectural boundary
- the priority of the next 2-3 intake cycles
- the fork's explicit non-goals

If a change does not alter one of those four points, update the more detailed
supporting document instead of this memo.
