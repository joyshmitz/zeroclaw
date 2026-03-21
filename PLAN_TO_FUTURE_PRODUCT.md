# Plan To Future Product

## Status

This document captures the current working vision for the `zeroclaw` fork.

It is intentionally strategic, not implementation-prescriptive.
The goal is to stabilize product thinking before committing to large architectural work.

This document is informed by:

- internal fork strategy discussions
- the future-system explanation in [`/data/projects/odoov19/EXPLAIN.md`](/data/projects/odoov19/EXPLAIN.md)
- Fledge and FledgePower as reference points for signal-provider ecosystems

## Planning Posture

At this stage, heavy investment in planning is intentional.

We are explicitly willing to spend the majority of time on planning before broad implementation work because:

- planning tokens are cheaper than implementation tokens
- system-wide corrections are cheaper in plan form than in code form
- agentic coding quality increases when execution follows a dense, coherent, constrained design
- premature implementation tends to create local optimizations, accidental abstractions, and cross-cutting slop

For this fork, planning is not treated as delay.
It is treated as the cheapest place to buy:

- correctness
- coherence
- ambition
- bounded execution

This document is therefore allowed to be broader and more detailed than a near-term implementation brief.
Refinement and trimming are expected later.

Current fork status:

- SOP wiring is merged into `fork/main`
- The fork is maintained `fork-first`
- Large upstream PRs are not the primary path for cross-cutting work
- Upstream contributions should be atomic and low-conflict

## Core Thesis

The fork is **not** primarily a transport-first system.

Its differentiating value is:

`primary signal -> meaning -> governed response`

When viewed through PDCA, the more complete form is:

`primary signal -> Check -> Act -> update Plan`

This means:

- transports and signal adapters are secondary
- SOP is central
- policy and governance are central
- agentic cognition is valuable only when bounded by procedure, policy, and review

This thesis is currently more important than transport or implementation detail choices.

## Invariant Product Principles

The following principles should remain true even as architecture, transports, and use cases evolve:

- a signal is not the same thing as an action
- the system should add meaning before it adds motion
- procedure and policy should bound agentic behavior
- context should not be discarded when moving from signal to response
- evidence is part of the product value, not an afterthought
- approvals and autonomy boundaries should stay explicit
- the system should help close a PDCA loop, not only produce one-off reactions
- transport should not define product identity
- signal type should matter more than delivery channel
- cognition should be applied selectively, not maximally by default

These principles are intended to constrain future design choices without prematurely freezing implementation.

## What The Fork Is

The fork is currently best understood as:

**an internal runtime for governed response to heterogeneous primary signals, with optional evolution toward a product-grade edge and enterprise workflow runtime**

This means the fork may serve:

- internal enterprise workflows
- process rehearsal and SOP validation
- operator-assist workflows
- future edge/product scenarios

It does **not** yet imply that the fork is already a production orchestration substrate.

## Actors

The fork may involve several distinct actors:

- human operator
  - executes or supervises a procedure
  - provides evidence
  - confirms or rejects suggested action
- reviewer / approver
  - authorizes gated transitions
  - approves high-risk actions or policy changes
- bounded AI assistant
  - interprets signal context
  - suggests procedure steps
  - drafts response or corrective action
  - operates only within policy and approval boundaries
- runtime executor
  - carries out allowed automated steps
  - may exist in central or edge form
- external system
  - emits signals
  - receives outcomes
  - may serve as system of record for domain data
- edge endpoint
  - the place where the primary signal originates
  - can be physical, digital, or human-process based

## PDCA Position

PDCA remains a first-class framing principle.

The PDCA framing here is strongly reinforced by the future-system description in
[`/data/projects/odoov19/EXPLAIN.md`](/data/projects/odoov19/EXPLAIN.md),
especially its treatment of SOP, quality checks, alerts, CAPA, and Go/No-Go gates.

- `Plan`
  - SOP
  - policy
  - thresholds
  - gates
- `Do`
  - execution of allowed actions
  - operator or agent-assisted workflow progress
- `Check`
  - interpretation of incoming signal
  - evidence gathering
  - pass/fail or deviation detection
- `Act`
  - corrective action
  - escalation
  - CAPA-style response
  - SOP or policy refinement

The fork should be evaluated not only by whether it reacts to a signal, but whether it supports a usable PDCA loop.

## SOP As Plan Core

SOP should be treated as the most concrete current embodiment of `Plan`.

In this fork, SOP is important because it can serve as:

- the procedural expression of intended behavior
- the bridge between signal interpretation and governed execution
- the thing that makes agentic behavior bounded instead of improvisational
- the artifact that can be reviewed, corrected, versioned, and improved over time

SOP is therefore not “just another feature”.
It is currently the strongest available mechanism for turning:

`signal -> meaning`

into:

`governed response`

and then feeding the result back into:

`updated plan`

However, SOP should not be mistaken for the whole system.
The broader fork still includes other essential layers:

- signal providers
- normalization
- policy and routing
- evidence and audit
- approvals and autonomy boundaries
- execution context
- future persistence or runtime ownership concerns

So the working view is:

- SOP is the current core of `Plan`
- not the entirety of the future product
- but the best current anchor for disciplined execution design

## Autonomy Levels

At a high level, the fork should support multiple autonomy modes:

- advisory
  - interpret signal
  - explain context
  - suggest SOP or next step
  - never act directly
- supervised
  - prepare or stage actions
  - require explicit approval before execution
- constrained automatic
  - execute pre-authorized low-risk steps
  - always log evidence and decision path
  - escalate when confidence, policy, or context is insufficient
- prohibited
  - actions that must never occur without human authorization

The exact autonomy boundary is use-case specific and should remain explicit.

## What Counts As Edge

For this vision, `edge` is not limited to IoT hardware.

`Edge` means:

**the point of primary signal origination in a process**

Examples:

- sensor reading
- power meter anomaly
- quality instrument event
- PLC/controller signal
- operator action
- inbound email
- customer request
- webhook from an external system
- any other first-order event that begins or changes a governed workflow

This definition keeps industrial, enterprise, and hybrid workflows within one conceptual model.

## Signal-Provider Horizon

The long-term architectural horizon is **signal-provider-first**, not transport-first.

Examples of signal providers:

- MQTT
- webhook
- cron
- peripheral or board event
- email
- external API callback
- Fledge or FledgePower-style edge/OT systems

Key principle:

- transport is not the product
- ingestion is not the product
- governed interpretation and response is the product value

An additional strategic axis, separate from provider/transport, is **signal type**.

Over time, the fork will likely need a clearer typology of signals, for example:

- observation
- anomaly
- alert
- request
- command
- state change
- approval-relevant event

This typology is not yet fully defined, but it is expected to matter more than any single transport choice.

## System Boundary

The fork should currently be treated primarily as:

- interpretation layer
- governed response layer
- procedure execution layer
- audit/evidence layer

The fork should **not** be assumed, by default, to be:

- the system of record for all domain data
- the only workflow engine in the environment
- the only event transport or integration hub

In many realistic deployments, domain truth may live elsewhere while the fork provides governed interpretation and response.

## Role Relative To Signal Ecosystems

Systems like Fledge/FledgePower are useful reference points.

They are strong at:

- data collection
- protocol translation
- edge pipelines
- normalization and transport

The fork should not be assumed to compete at that layer first.

The fork’s likely higher-layer value is:

- interpret what a signal means in operational context
- map signal to procedure
- enforce policy and approval boundaries
- collect evidence
- support governed response
- feed learning and correction back into the plan

Reference points:

- [Fledge Introduction](https://fledge-iot.readthedocs.io/en/v3.1.0/introduction.html)
- [fledge-power GitHub organization](https://github.com/fledge-power)

## Evidence And Audit Value

The fork’s value is not limited to “reacting to a signal”.

It should also help preserve:

- why the signal mattered
- which procedure applied
- what decision was proposed
- what action was taken
- who approved it, if approval was required
- what evidence supports the outcome

This is especially important in enterprise, quality, industrial, and edge-adjacent contexts.

## From Signal To Governed Response

The fork should be thought of as a system that receives primary signals and progressively turns them into bounded, explainable, reviewable responses.

The conceptual flow is:

1. a primary signal originates at the edge of some process
2. the signal is received from a provider
3. the signal is interpreted in operational context
4. policy, autonomy boundaries, and procedure are consulted
5. a governed response path is selected
6. evidence, approvals, and outcomes are recorded
7. the result feeds back into the PDCA loop

In compact form:

`signal -> interpretation -> policy/procedure selection -> governed execution -> evidence -> PDCA feedback`

This should not be read as a strict implementation sequence yet.
It is a product-thinking sequence that keeps the system coherent.

Within this flow:

- signal providers bring the event into the system
- interpretation gives the event meaning
- SOP and policy constrain what may happen next
- autonomy determines who or what may act
- evidence and audit make the response explainable
- PDCA closes the loop so the system can improve over time

This flow is broader than SOP alone, but SOP is currently the most concrete anchor inside it.

## Interpretation Layer

The system needs an interpretation layer between raw signal receipt and governed response.

This layer exists to answer questions such as:

- what kind of situation is this
- why does it matter
- what process or procedure does it belong to
- how risky or urgent is it
- is there enough certainty to act
- does this require escalation, approval, or only observation

This interpretation layer should be understood as an operationalization of `Check`, not as a competing framework to PDCA.

It should also **not** be treated as a single expensive cognitive bottleneck.

The preferred direction is a graded interpretation cascade:

- cheap interpretation
  - schema validation
  - deduplication
  - threshold or rule match
  - obvious routing
- structured contextual interpretation
  - process association
  - SOP or policy association
  - risk or priority enrichment
- cognitive interpretation
  - ambiguity resolution
  - human-language understanding
  - explanation synthesis
  - difficult response recommendation

Guiding principle:

`every signal should receive enough interpretation for its risk and ambiguity, not maximum cognition by default`

This helps preserve:

- throughput
- latency
- reliability
- bounded cost
- architectural clarity

## Scenario Table

| Scenario | Primary Signal | Meaning | Governed Response | PDCA Role | Classification |
|---|---|---|---|---|---|
| Technical anomaly | sensor, meter, controller, gateway event | deviation, risk, incident | classify, choose SOP, alert/escalate, collect evidence, decide next step | `Check -> Act` | `internal -> dual-use/product seed` |
| Business inbound request | email, form, CRM lead, quote request, complaint | start of commercial or service workflow | classify request, assign owner, apply SOP, prepare response, track escalation | `Do` or pre-`Plan` trigger | `internal` |
| Quality or operational incident | failed check, quality alert, stop condition | corrective workflow required | run SOP, require evidence, approvals, corrective steps, phase/gate control | `Check -> Act -> Plan` | `internal` |
| Edge agent in product | local device or gateway event | local constrained response needed | execute allowed local SOP step, escalate when needed, record decision trail | `Do/Check -> Act` | `product` |
| Mixed-signal response hub | technical, human, business, system events | need one governed response model | normalize, route, execute SOP, audit, escalate, revise process | full loop | `product seed` |

## Immediate Anchor Scenarios

The most useful anchor scenarios right now are:

1. quality or operational incident
2. technical anomaly
3. business inbound request

Why these:

- they are concrete
- they test the full value chain from signal to response
- they do not require immediate commitment to full product architecture
- they exercise SOP in realistic ways

These anchor scenarios should guide future narrowing of the plan.
They are more important than trying to prematurely optimize for a single final architecture.

## Current Non-Goals

The following are explicitly not immediate goals unless a concrete use case forces them:

- process-wide daemon singleton as a mandatory architecture rule
- universal transport support
- transport-first productization
- full persistence layer for every SOP run
- full edge fleet orchestration
- broad product release process overhead

These may become necessary later, but they are not assumed by default.

## Explicit Non-Replacement Boundary

The fork is not currently intended to replace, wholesale:

- ERP systems
- SCADA systems
- historians
- dedicated telemetry gateways
- generic ticketing systems
- every existing workflow engine

Its value should come from governed interpretation and response, not from trying to subsume every surrounding platform.

## Failure Model

Important failure modes to keep visible:

- missed signal
- duplicate signal handled incorrectly
- signal interpreted incorrectly
- wrong SOP selected
- action executed outside allowed autonomy
- required approval bypassed
- insufficient evidence captured
- response completed without updating the learning/correction loop
- process drift between policy, SOP, and actual behavior

Architectural work should be justified by which of these failures it reduces.

## Fork Strategy Alignment

This planning direction assumes:

- `fork/main` stays close to upstream
- fork-specific integration work lives in the fork
- only small, atomic, low-conflict changes are proposed upstream
- strategy is revisited when product intent becomes clearer

## Triggers For Strategy Revisit

Revisit this document when one of these becomes true:

1. a real workflow repeatedly fails without shared SOP runtime state
2. a concrete signal-provider integration becomes high-value
3. a production-grade edge use case appears
4. upstream introduces meaningful SOP/DAG/orchestration primitives
5. fork drift cost becomes materially painful

## Working Conclusion

At this stage, the fork should be treated as:

- more than a playground
- not yet a full product runtime
- a practical internal runtime for PDCA-governed response to primary signals

This is enough clarity to guide near-term development without forcing premature architecture.

The current phase is idea formation and direction-setting.
Over-inclusiveness in the plan is acceptable at this stage if it preserves important strategic possibilities that may later be narrowed.

## Sources

- Internal future-system framing:
  - [`/data/projects/odoov19/EXPLAIN.md`](/data/projects/odoov19/EXPLAIN.md)
- Signal-provider ecosystem references:
  - [Fledge Introduction](https://fledge-iot.readthedocs.io/en/v3.1.0/introduction.html)
  - [fledge-power GitHub organization](https://github.com/fledge-power)
- Fork strategy and governance context:
  - local fork strategy discussion captured in `fork-first` working decisions
  - upstream repository: [zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw)
