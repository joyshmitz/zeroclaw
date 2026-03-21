# First Governed Case MVP Plan

Timestamp: 2026-03-21T13:50:25+02:00

## Status

This document is the first implementation-planning bridge after:

- the final product definition in [`PLAN_TO_FUTURE_PRODUCT.md`](/data/projects/zeroclaw/PLAN_TO_FUTURE_PRODUCT.md)
- the architectural bridge in [fork-architecture-brief.md](fork-architecture-brief.md)

It does not define code yet.
It defines the smallest governed-response path that should become real first.

## Purpose

The purpose of this MVP plan is to make the fork thesis testable in one concrete path:

`primary signal -> governed case -> bounded response -> evidence -> PDCA feedback`

This artifact exists because the current repo has:

- a coherent product thesis
- a coherent architecture brief
- no first implementation target that turns the thesis into a controlled architectural move

## Chosen Scenario

The first governed case MVP should be:

`quality or operational incident`

Working signal examples:

- failed quality check
- quality alert
- stop condition
- explicit operator incident submission

Working meaning:

- corrective workflow required
- bounded handling required
- evidence and approvals may be required

## Why This Scenario First

This scenario is the best first MVP because it exercises the strongest current fork assets:

- SOP as the current core of `Plan`
- evidence and audit expectations
- approval and autonomy boundaries
- PDCA feedback into improved procedure

It also avoids two early traps:

- making the first MVP transport-specific
- making the first MVP dependent on heavy cognition

This scenario can begin from an explicit incident signal that is already operationally meaningful.
That means the fork can prove governed case handling first, without requiring full telemetry-window logic on day one.

## Why Not The Other Anchor Scenarios First

### Not Technical Anomaly First

`technical anomaly` remains a key validation scenario, but it is not the best first MVP because it pushes the fork immediately into:

- temporal emergence semantics
- anomaly thresholds and drift logic
- gateway or edge placement questions
- multi-observation signal formation

Those are important, but they are better treated after the fork proves governed case handling in a simpler operational path.

### Not Business Inbound Request First

`business inbound request` is valid, but as the first MVP it risks pulling the fork too early toward:

- inbox-first thinking
- request-routing copilot framing
- natural-language-heavy cognition as the default move

That would make it easier to slide back into upstream-shaped assistant logic.

## MVP Objective

The first MVP should prove that the fork can:

1. receive one operationally meaningful incident signal
2. classify it deterministically enough to open a governed case
3. bind that case to SOP, evidence expectations, and response constraints
4. choose a bounded response mode before generic agent action
5. capture outcome and decide whether `Plan` should be updated

If the fork can do those five things in one scenario, the thesis stops being document-only.

## Canonical MVP Path

The first governed case MVP should follow this path.

### 1. Signal Ingress

Ingress may arrive through any adapter that can carry an explicit incident:

- channel message
- gateway event
- internal submission
- external callback

Transport is not part of product identity.
For MVP purposes, the signal only needs to arrive in a form that can be interpreted as an explicit incident candidate.

### 2. Signal Emergence

The first MVP should avoid complex emergence logic.

The working rule should be:

- an explicit failed check, quality alert, stop condition, or operator incident submission already counts as an operationally meaningful primary signal

This keeps the MVP focused on governed handling rather than telemetry preprocessing.

### 3. Signal Classification And Meaning

The fork should classify the signal into a small deterministic incident lens, for example:

- `quality incident`
- `operational incident`
- `stop-condition incident`

The classification step should also determine:

- severity or urgency
- whether evidence is immediately required
- whether approval is immediately required
- whether a new case should open or an existing case should update

The first evaluator should be the cheapest useful rule set for this signal type.
Richer cognition should only be admitted if the submission is too ambiguous for deterministic classification.

### 4. Governed Case Formation

The signal should open a governed case when it introduces a distinct corrective situation requiring bounded handling.

The minimum governed case should bind:

- signal reference
- working interpretation
- case type
- response mode
- applicable SOP
- evidence state
- approval state
- owner or routing target
- action and outcome trail
- PDCA feedback disposition

This is the first point where the fork becomes a governed response runtime rather than a generic agent loop.

### 5. Pre-Planning Decision

Before generic tool use or LLM motion, the fork should choose one of a small set of response modes:

- observe only
- request missing evidence
- route or assign ownership
- stage for approval
- apply SOP in bounded mode
- escalate for richer interpretation

The MVP should prove this choice happens before open-ended agent behavior.

### 6. Bounded Response

The first MVP does not need broad automation.
It only needs one bounded path such as:

- select the relevant SOP
- stage the next corrective step
- request the required evidence
- require approval if severity or policy demands it
- record the chosen next step

This bounded response may remain supervised.
The goal is not autonomy first.
The goal is governed handling first.

### 7. Evidence And Outcome Capture

The first MVP should explicitly capture:

- what signal opened the case
- what interpretation was chosen
- what SOP or procedure was selected
- what evidence was requested or received
- whether approval was required, granted, or denied
- what action was staged or executed
- what outcome occurred

Without this, the fork collapses back into generic tool-use with no durable governance value.

### 8. PDCA Feedback

The first MVP should end with an explicit decision:

- close the case only
- or propose `update Plan`

`update Plan` should be considered when the incident reveals:

- missing or weak SOP coverage
- evidence expectations that were insufficient
- approval boundaries that were wrong
- routing or ownership ambiguity
- repeated incident patterns that justify procedural revision

This is the minimum move that makes the runtime PDCA-governed rather than merely case-tracking.

## Minimal Fork-Owned Decisions

The first MVP requires the fork to make these ownership decisions explicit:

1. where explicit incident signals are classified
2. where governed cases are opened or updated
3. where response mode is selected before generic agent behavior
4. where SOP attaches to a case rather than to raw tool execution
5. where evidence and approval state are recorded logically, even if persistence stays minimal
6. where PDCA feedback is emitted

These decisions matter more than the exact transport or storage implementation.

## What The MVP Can Reuse From Upstream

The first MVP should continue to reuse upstream substrate for:

- ingress adapters
- runtime assembly
- generic provider and tool infrastructure
- generic daemon and service lifecycle

The fork should add meaning at the governed seams, not replace the whole runtime.

## Immediate Non-Goals

This MVP should not try to:

- solve telemetry-window emergence
- become a workflow engine
- generalize immediately across all signal classes
- define final governed-case persistence architecture
- automate high-consequence actions by default
- prove full edge deployment strategy
- absorb business inbound request handling into the same first pass

## Success Criteria

The MVP is successful if it makes all of the following true in one concrete scenario:

- an explicit incident signal becomes a governed case
- the case is not treated as self-justifying free-form execution
- SOP, evidence, and approval expectations shape the response
- the next step is chosen through bounded response mode selection
- outcome is recorded in a form that supports PDCA review

## Required Follow-On After This Plan

Once this MVP plan is accepted, the next artifacts should be:

1. a conflict surface map for the implementation seams it will touch
2. a first implementation brief mapping this MVP onto current modules
3. validation criteria showing how this one governed path will be reviewed

The first two are now represented by:

- [conflict-surface-map.md](conflict-surface-map.md)
- [first-governed-case-implementation-brief.md](first-governed-case-implementation-brief.md)

## Bottom Line

The first governed case MVP should not start with the most technically impressive signal.

It should start with the cleanest case where the fork can prove:

`signal -> case -> bounded response -> evidence -> PDCA feedback`

Right now, that case is `quality or operational incident`.
