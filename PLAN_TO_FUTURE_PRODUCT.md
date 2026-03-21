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

The fork is best understood as a **primary-signal-first** system.

Its differentiating value is:

`primary signal -> meaning -> governed response`

When viewed through PDCA, the more complete form is:

`primary signal -> Check -> Act -> update Plan`

This means:

- primary signals and their operational meaning are central
- signal providers are ingress mechanisms, not product identity
- SOP is central
- policy and governance are central
- agentic cognition is valuable only when bounded by procedure, policy, and review

This thesis is currently more important than any specific provider or implementation detail choice.

## Invariant Product Principles

The following principles should remain true even as architecture, transports, and use cases evolve:

- a signal is not the same thing as an action
- the system should add meaning before it adds motion
- procedure and policy should bound agentic behavior
- context should not be discarded when moving from signal to response
- evidence is part of the product value, not an afterthought
- approvals and autonomy boundaries should stay explicit
- the system should help close a PDCA loop, not only produce one-off reactions
- product identity should be defined by governed interpretation and response
- signal type should matter more than delivery channel
- cognition should be applied selectively, not maximally by default

These principles are intended to constrain future design choices without prematurely freezing implementation.

## What The Fork Is

The fork is currently best understood as an internal runtime for governed response to heterogeneous primary signals, with optional evolution toward a product-grade edge and enterprise workflow runtime.

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

In this framing, `edge` means the point of primary signal origination in a process.

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

The long-term architectural horizon is **signal-provider-first**.

Working distinctions:

- `signal source`
  - the process locus where a condition, request, event, or action originates
- `primary signal`
  - the first operationally meaningful event emitted from that source
- `signal provider`
  - the mechanism or system that brings that primary signal into the fork

Examples of signal providers:

- webhook
- cron
- peripheral or board event
- email
- external API callback
- MQTT adapter
- Fledge or FledgePower-style edge/OT systems

Key principle:

- providers matter because they shape ingestion constraints and context
- governed interpretation and response is the product value

An additional strategic axis, separate from source/provider, is **signal type**.

Over time, the fork will likely need a clearer typology of signals, for example:

- observation
- anomaly
- alert
- request
- command
- state change
- approval-relevant event

This typology is not yet fully defined, but it is expected to matter more than any single transport choice.

## Signal Emergence Gates

Not every raw event emitted by a source should count as a primary signal for the fork.

This distinction matters most in high-volume environments.
A sensor, meter, controller, or gateway may emit observations continuously, but the runtime does not gain value from treating each raw emission as a first-class governed event.

The working principle should be:

- raw telemetry is not automatically a primary signal
- a primary signal emerges only when some condition makes it operationally meaningful
- emergence criteria belong to `Plan` because they define what deserves `Check`

In practical terms, a primary signal may emerge through one or more gates such as:

- threshold breach
- state transition
- persistence over time
- rate-of-change or drift beyond tolerance
- correlation across multiple observations
- missing expected event or silence
- explicit human submission
- policy-defined exception or priority trigger

This keeps the fork centered on governed response rather than raw transport volume.

For many environments, especially device or gateway-heavy ones, the better model is:

`stream of observations -> emergence gate -> primary signal -> governed interpretation`

This reduces noise and preserves the idea that the fork is not primarily a telemetry firehose processor.

## Temporal Semantics Of Emergence

For many instrument, gateway, and controller-originated signals, emergence should be evaluated over time rather than on a single raw observation.

This makes your intuition directionally correct:
for some signal types, `Plan` should define a time window in which observations are analyzed for change, persistence, drift, or trend before a primary signal is emitted.

However, this should not become a universal rule for all signals.
The better framing is:

- each signal type may have its own temporal semantics
- some signals are instantaneous
- some are windowed
- some are stateful across a rolling history
- some emerge from absence or silence rather than presence

A useful working distinction is:

- instantaneous emergence
  - explicit human submission
  - approval or rejection event
  - hard safety stop
  - direct policy-defined exception
- windowed emergence
  - threshold breach sustained over a period
  - drift across successive measurements
  - unstable oscillation or flapping
  - rate-of-change beyond tolerance
- stateful emergence
  - repeated weak deviations that become significant cumulatively
  - fault patterns that only matter across multiple episodes
- absence-based emergence
  - expected heartbeat missing
  - required confirmation not received
  - expected follow-up event absent within a policy-defined interval

For windowed signal types, `Plan` should define emergence semantics such as:

- evaluation window
- aggregation or smoothing method
- baseline or expected range
- trend or drift criteria
- persistence requirement
- hysteresis or cooldown behavior
- deduplication and re-alert conditions

This matters because the product value is not in forwarding every reading.
It is in deciding when a pattern has become operationally meaningful enough to deserve governed interpretation.

In many deployments, especially high-volume edge environments, these temporal emergence rules may run close to the source, including at gateway level.
That is acceptable and often desirable.
But it still should not redefine product identity:
the gateway may host emergence logic, while the fork remains centered on meaning, governance, SOP, and response.

## Transport-Neutral Cognitive Admission

The decision to invoke heavier cognition should be treated as a separate gate from signal emergence.

In other words:

- not every raw event becomes a primary signal
- not every primary signal deserves cognitive interpretation
- not every cognitive interpretation should involve an LLM

The admission question is not only “is cognition affordable”.
It is “does cognition materially improve the governed response enough to justify its operational cost”.

A useful admission lens is:

- ambiguity
  - is the meaning unclear without richer interpretation
- unstructuredness
  - does the signal arrive in natural language, weak schema, or messy mixed context
- consequence
  - would better interpretation materially reduce risk of wrong routing or wrong action
- novelty
  - is this poorly covered by existing SOP, rules, or historical patterns
- evidence gap
  - does interpretation need synthesis across scattered context before action can be governed

This keeps cognition subordinate to operational need rather than source prestige.

The default posture should therefore be:

- signal-type-specific deterministic rules first
- richer contextual interpretation second
- cognitive escalation only when lower-cost rules leave meaningful uncertainty

This means the cheapest useful computation for a given signal type should usually be preferred as the first gate.
That includes threshold rules, temporal window rules, drift detection, state transition rules, silence detection, and other bounded evaluators defined by `Plan`.

An email often justifies cognition more often than device telemetry because it is frequently unstructured and compressed into human language.
But email should still not be assumed to require cognition by default:

- spam
- auto-replies
- routine structured requests
- obvious known intents

may all be handled without heavy interpretation.

Likewise, device and gateway signals usually justify cognition less often because they are structured and high-volume.
But they should not be excluded categorically:

- multi-signal anomalies
- repeated near-threshold drift
- ambiguous fault clusters
- incidents requiring explanation or recommendation

may justify cognitive escalation.

So the correct principle is not:

`email -> cognition`

or:

`gateway -> no cognition`

but rather:

`signal significance + ambiguity + consequence -> cognition if justified`

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

## Cost Is Not The Only Gate

Cheap versus expensive model calls are relevant, but they are not the main conceptual boundary.

Even a cheap LLM invocation can be operationally expensive if it adds:

- latency at the wrong point in a workflow
- nondeterminism where a rule would be safer
- audit burden without decision-value
- operator confusion or review fatigue
- broader policy or privacy exposure
- more surface area for subtle misinterpretation

The inverse is also true:
an expensive cognitive step may be justified if it materially reduces error, improves routing, protects autonomy boundaries, or prevents repeated human confusion in high-value cases.

So the core question should be:

`is cognition warranted here`

not merely:

`is cognition cheap enough`

This supports a stronger design posture:

- use cheap deterministic gates first
- escalate to richer interpretation only when those gates leave meaningful uncertainty
- treat cognition as selective operational leverage, not ambient background processing

Applied back to signal design, this means:

- the first evaluator should usually be the cheapest rule set that is appropriate for that signal type
- signal-type-specific emergence logic is a product asset, not an implementation detail
- LLM usage should usually begin where deterministic signal-type rules stop being sufficient

## Mathematical Framing

Mathematics should be introduced here as a language for formalizing boundaries, not as a substitute for product thinking.

Its strongest use in this fork is to make the following questions precise:

- when raw observations become a primary signal
- what uncertainty remains after interpretation
- which response modes are allowed under policy and autonomy constraints
- what evidence is sufficient for escalation, action, or closure
- when outcomes justify `update Plan`

A useful minimal formal core is:

- `E_k(history, state, plan) -> primary_signal | nil`
  - emergence function for signal class `k`
  - turns raw observations or events into an operationally meaningful signal only when emergence criteria are met
- `I_k(signal, context, plan) -> meaning, risk, uncertainty`
  - interpretation function for signal class `k`
  - produces the operational meaning needed for governed response
- `G(meaning, risk, uncertainty, evidence, policy, autonomy) -> response_mode`
  - governance function
  - determines whether the correct response is observation, evidence request, routing, staged action, constrained execution, or escalation
- `U(history, outcomes, plan) -> proposed_plan_change | nil`
  - feedback/update function
  - determines when repeated patterns or outcomes justify changing SOP, policy, thresholds, evidence requirements, or autonomy boundaries

This is intentionally minimal.
It is enough to make the product core more exact without prematurely hard-coding one architectural style or one mathematical worldview.

## Mathematical Regimes By Signal Class

The strongest direction is not one universal formula for everything.
It is a small set of mathematical regimes matched to the nature of each signal class.

Examples:

- continuous or windowed observations
  - moving windows
  - smoothing or aggregation
  - drift or trend detection
  - rate-of-change checks
  - control limits
  - hysteresis
- discrete state transitions
  - finite-state logic
  - allowed transition predicates
  - guard conditions
- approval or gate events
  - boolean policy logic
  - authorization predicates
  - explicit threshold and signature requirements
- request or inbox-style signals
  - classification
  - confidence thresholds
  - routing scores
  - ambiguity flags
- silence or missing-event signals
  - timeout windows
  - expected-arrival intervals
  - missing-heartbeat detection
- repeated incidents and outcomes
  - recurrence rates
  - trend and control-chart style thinking
  - feedback thresholds for `update Plan`

This means the right pattern is:

`signal class -> suitable mathematical regime -> emergence rule -> governed response envelope`

That is stronger than either extreme:

- one grand universal formula for all signals
- a completely ad hoc bespoke logic for every individual case

## Product Boundary Clarified By Math

Mathematical framing helps clarify what is inside the product boundary and what is merely adjacent.

Inside the product boundary by default:

- emergence logic
- interpretation and uncertainty handling
- governance predicates
- evidence sufficiency rules
- response-mode selection
- feedback thresholds for `update Plan`

Outside the product boundary by default:

- raw transport mechanics for every possible provider
- full telemetry retention or historian behavior
- full system-of-record ownership for domain truth
- arbitrary cognition without a governed decision need

Seen through this lens, the strongest current candidate for the product core is:

`a runtime for signal-class-specific emergence, governed interpretation, constrained response, and auditable PDCA feedback`

## Operational Meaning

Meaning should be understood here as **operational meaning**, not semantic richness for its own sake.

A signal has enough meaning when the system can determine:

- what kind of situation it represents
- whether it opens a new governed case, updates an existing one, or only enriches context
- what process context and ownership domain it belongs to
- which procedure, policy, or gate may apply
- what risk, urgency, and autonomy envelope applies
- what evidence is already present and what evidence is still missing

In this framing, meaning matters because it changes the response envelope.
If interpretation does not change routing, procedure selection, evidence requirements, approval needs, or allowed action, it is not yet strong product-value interpretation.

## Working Signal-Type Lens

The signal-type axis should remain intentionally small until repeated real workflows force refinement.

A useful current working set is:

- `observation`
  - state, measurement, or condition report
  - may remain in `Check` unless thresholds or policy elevate it
- `deviation`
  - anomaly, failed check, or out-of-bounds condition
  - usually requires classification, evidence, and possible corrective flow
- `request`
  - inbound demand for service, decision, workflow initiation, or action preparation
  - typically requires routing, ownership, and procedure selection
- `decision/gate event`
  - approval, rejection, Go/No-Go, or other authorization-relevant transition
  - changes what actions are now allowed
- `outcome event`
  - completion, failure, or result of a governed step
  - may close a case or trigger `update Plan`

This is not a final taxonomy.
It is a minimal working lens that covers current anchor scenarios without overfitting to transport.

An inbound command should usually be treated first as a signal requiring interpretation and governance, not as self-justifying execution.

## Governed Response Modes

Not every meaningful signal should trigger the same kind of response.

A useful response lens is:

- record and observe
- enrich or request evidence
- route or assign
- recommend a procedure or next step
- stage an action for approval
- execute a pre-authorized constrained step
- initiate plan review

The correct response mode should be shaped by signal type, context, risk, policy, and autonomy boundaries.

In many cases, the correct governed response is not immediate motion.
It may be explicit non-action, observation, or evidence collection with a clear decision trail.

## What `update Plan` Means

The PDCA loop in this fork should not end at response execution.

`update Plan` should be understood as governed change to the artifacts that shape future response, such as:

- SOP content or branching
- policy rules
- thresholds or gate criteria
- evidence requirements
- routing or ownership rules
- autonomy boundaries

Plan update becomes justified when repeated signals or outcomes expose:

- missing or ambiguous SOP coverage
- repeated misclassification or wrong routing
- unstable thresholds or noisy alerts
- approval bottlenecks or autonomy mismatch
- evidence gaps that block reliable action
- divergence between written procedure and actual successful practice

This keeps learning concrete, reviewable, and bounded.
It does not imply unconstrained self-modifying behavior.

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
- premature commitment to any single provider-specific product shape
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
