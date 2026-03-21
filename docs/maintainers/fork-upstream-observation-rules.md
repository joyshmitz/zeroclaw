# Fork Upstream Observation Rules

Timestamp: 2026-03-21T13:50:25+02:00

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
- on every new upstream beta tag or stable release tag

Recommended cadence:

- one lightweight observation pass after each `git fetch upstream --prune`
- one deeper pass before creating or refreshing a `merge/*` intake branch

## Beta And Release Lens

The fork should watch upstream betas and stable releases deliberately.

Why this matters:

- beta tags are a useful signal of what upstream is actively stabilizing
- stable releases are a useful signal of what upstream considers ready to ship broadly
- neither should be treated as the only observation surface

Current factual tag pattern already shows why a release lens matters:

- recent tags include `v0.5.4`
- the same minor line also continues with later beta tags such as `v0.5.4-beta.487`, `v0.5.4-beta.490`, `v0.5.4-beta.491`, and `v0.5.4-beta.504`

This means beta watching should be treated as a rolling release-vector signal, not only as a one-time pre-release freeze window.

## How To Read Betas Correctly

Beta watching is useful, but it is not enough by itself.

The fork should treat:

- seam watch on `master` and high-signal PR flow as the early-warning layer
- beta and stable tags as the stabilization and release-preparation layer

Do not wait for beta or stable tags to discover architecture pressure in:

- `src/agent/agent.rs`
- `src/gateway/mod.rs`
- `src/tools/mod.rs`
- `src/config/schema.rs`
- `src/main.rs`

By the time pressure is visible only through a beta tag, the merge debt is already older.

## What To Watch In Betas And Releases

For each new beta tag or stable release, ask:

- which conflict surfaces are touched most by the commits leading into this tag
- whether the release vector is converging around channels, gateway, tools, config, hardware, web, or orchestration
- whether the fork's known conflict surfaces are getting hotter or cooler
- whether upstream is stabilizing something the fork should reuse as-is
- whether upstream is stabilizing something that challenges fork-owned seams or SOP positioning

Useful signals here include:

- new tags
- version bumps
- release-related workflows or scripts
- recurring release-note themes
- repeated commit clusters in the same architectural surfaces

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
- `release-surface`
  - indicates what upstream is actively stabilizing for beta or stable shipment
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
- `prepare release intake`
  - release or beta movement is strong enough that the fork should plan around the incoming stabilized surface

## Revisit Triggers

Escalate from routine observation to explicit planning review when upstream does one of the following:

- introduces meaningful SOP, workflow, gating, approval, or orchestration primitives
- expands hardware or edge support enough to change the practical meaning of deployment scope
- changes tool registration or runtime construction in conflict-prone seams
- materially shifts gateway, channel, or provider responsibilities
- introduces governance, audit, policy, or approval capabilities the fork could reuse
- creates repeated merge debt in the same fork-owned areas
- repeatedly ships betas or release candidates concentrated in the same fork-sensitive surfaces

## What To Record Per Observation Pass

For each non-trivial observation pass, record:

- upstream commit or range reviewed
- high-signal files or modules touched
- observed change classes
- expected conflict surfaces
- likely impact on fork architecture
- likely impact on product definition
- likely impact on release intake timing or preparation
- whether a follow-up artifact is required

Recommended result states:

- `no action`
- `watch next intake`
- `watch next beta`
- `update architecture brief`
- `update product document`
- `prepare merge branch`
- `prepare release intake`

## Release Preparation Rule

When upstream beta or stable movement repeatedly concentrates in a fork-sensitive surface:

- do not panic-merge
- do not treat release motion as automatic adoption pressure
- do prepare a fork review pass before the next sync

The right question is not:

- "should we follow the release because it is a release"

The right question is:

- "what does this release vector tell us about substrate we can reuse, conflict we should expect, and seams we should strengthen before intake"

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
- a first governed-case implementation brief

These artifacts are the bridge between product definition and implementation direction.
