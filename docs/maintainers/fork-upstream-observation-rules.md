# Fork Upstream Observation Rules

Rules for watching `upstream` without letting upstream velocity silently redefine fork direction.

## Purpose

The fork should stay observant of upstream, but not subordinate to upstream feature accumulation.

Observation has three goals:

- detect upstream changes that create future merge conflict surfaces
- detect upstream changes that materially affect fork architecture choices
- detect upstream changes that challenge or strengthen fork product identity

Observation is not adoption.
Not every upstream feature, integration, or transport should enter the fork thesis.

## Core Rule

Treat upstream changes as signals requiring interpretation, not as self-justifying roadmap input.

For every meaningful upstream change, ask:

- does it change conflict surfaces
- does it change available architectural seams
- does it change what the fork can reuse as-is
- does it challenge `primary signal -> meaning -> governed response`
- does it force a revisit of current product or architecture assumptions

## Product Identity Guardrail

Do not let upstream observation drift into:

- transport-first thinking
- feature-catalog thinking
- hardware-first thinking
- workflow-engine-first thinking
- certification-first thinking

Upstream may add capabilities.
The fork still defines product identity by governed interpretation and response to heterogeneous primary signals.

## Observation Cadence

Minimum cadence:

- on every planned upstream intake
- after any upstream merge that touches known fork conflict surfaces
- after any upstream release or feature batch that materially expands tools, transports, hardware, runtime, or gateway behavior

Recommended cadence:

- one lightweight observation pass after each `git fetch upstream --prune`
- one deeper pass before creating or refreshing a `merge/*` intake branch

## What To Watch

Prioritize observation of changes in these surfaces:

- `src/agent/agent.rs`
- `src/agent/loop_.rs`
- `src/gateway/mod.rs`
- `src/tools/mod.rs`
- `src/config/schema.rs`
- `src/lib.rs`
- `src/main.rs`
- `src/security/**`
- `src/runtime/**`
- bootstrap and prompt-shaping files such as `AGENTS.md`, `CLAUDE.md`, onboarding, and wizard flows

These are likely repeat-conflict zones because they sit near:

- tool registration
- agent construction
- gateway wiring
- config surface growth
- prompt/bootstrap behavior
- security and runtime boundaries

## Change Classes

Classify each meaningful upstream change into one or more buckets:

- `merge-surface`
  - likely to create or repeat git conflicts during intake
- `architecture-surface`
  - changes seams, ownership boundaries, extension points, or lifecycle wiring
- `product-surface`
  - changes the practical meaning of edge, tools, channels, workflows, or runtime scope
- `policy-surface`
  - changes security, approvals, runtime boundaries, data handling, or governance assumptions
- `noise`
  - no material effect on fork direction beyond normal upkeep

## Decision Rules

For each observed upstream change, decide one of:

- `observe only`
  - note it, but do not change fork planning or code because of it
- `prepare for merge`
  - track the conflict surface and expected resolution pattern
- `reuse as-is`
  - upstream change aligns with fork direction and likely enters the fork intact
- `adapt in fork`
  - upstream change is useful, but needs fork-specific wiring or constraints
- `revisit thesis`
  - upstream change is strong enough to challenge current product or architecture assumptions

## Revisit Triggers

Escalate from routine observation to explicit planning review when upstream does one of the following:

- introduces meaningful SOP, workflow, gating, approval, or orchestration primitives
- expands hardware or edge support enough to change the practical meaning of deployment scope
- changes tool registration or runtime construction in conflict-prone seams
- materially shifts gateway, channel, or provider responsibilities
- introduces governance, audit, policy, or approval capabilities the fork could reuse
- creates repeated merge debt in the same fork-owned areas

## What To Record Per Observation Pass

For each non-trivial observation pass, record:

- upstream commit or range reviewed
- high-signal files or modules touched
- observed change classes
- expected conflict surfaces
- likely impact on fork architecture
- likely impact on product definition
- whether a follow-up artifact is required

Recommended result states:

- `no action`
- `watch next intake`
- `update architecture brief`
- `update product document`
- `prepare merge branch`

## First Known Watchlist

Based on current fork state, keep special attention on:

- `src/agent/agent.rs`
  - agent construction and tool/runtime wiring
- `src/gateway/mod.rs`
  - gateway runtime assembly and tool registry wiring
- `src/tools/mod.rs`
  - tool registration surface
- `src/config/schema.rs`
  - config growth and cross-cutting feature pressure
- `AGENTS.md`
  - fork working contract
- `PLAN_TO_FUTURE_PRODUCT.md`
  - canonical product framing

Known repeated-risk pattern:

- upstream adds cross-cutting capability wiring in agent/gateway setup
- fork adds SOP or governed-response-specific wiring in the same setup zones
- the merge conflict is then tactical, but the real issue is shared ownership of the same seam

## Practical Rule During Intake

When syncing from upstream:

- merge first
- resolve conflicts faithfully
- only then ask whether the repeated conflict means a fork-owned seam needs a dedicated abstraction or architecture decision

Do not use every painful merge as an excuse for immediate refactor.
But do not ignore repeated conflict surfaces either.

## Output Artifact

When observation repeatedly shows the same seams, create or update:

- a fork architecture brief
- a conflict surface map
- a first governed-case MVP plan

These artifacts are the bridge between product definition and implementation direction.
