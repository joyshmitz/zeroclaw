# Fork Operating Model

Timestamp: 2026-03-23T06:51:06+02:00

## Status

This document defines the current operating model for the fork while it remains in the upstream fork network.

Current judgment:

- treat the fork as an independent product
- keep upstream as an input source and reusable runtime substrate
- do not detach into a separate repository yet

## Purpose

This document exists to prevent three failure modes:

- upstream roadmap drift becoming the de facto fork roadmap
- release or CI behavior being inherited accidentally instead of owned explicitly
- repeated merge pressure being handled tactically without creating clearer fork-owned seams

## Core Stance

The fork should be run as:

- product-independent
- release-policy-independent
- documentation-policy-independent
- selectively upstream-fed

The fork should not be run as:

- a passive mirror of upstream
- a feature bucket for every upstream capability
- a release surface driven implicitly by upstream workflow defaults

In compact form:

`independent product -> controlled intake -> selective reuse -> explicit fork ownership where pressure repeats`

## Decision Hierarchy

When signals conflict, use this priority order:

1. fork product identity and governed-response thesis
2. fork-owned policy and operating docs
3. current fork runtime and release reality
4. upstream changes and upstream release motion

Upstream remains important, but it does not outrank fork identity.

## Operating Rules

### 1. Fork Trunk

- `origin/main` is the fork trunk
- product work branches from the current fork trunk
- upstream intake happens in `merge/*` branches and is merged back explicitly

### 2. Upstream Intake

- observe first, merge second
- classify upstream changes before treating them as candidates for adoption
- record meaningful observation and intake results in [upstream-intake-log.md](upstream-intake-log.md)
- prefer `reuse as-is`
- choose `adapt in fork` or `reject for fork` when reuse would distort fork direction

### 3. Release And CI

- release semantics are fork-owned, even if workflow files still came from upstream
- no release automation change should be treated as routine upkeep
- workflow, tag, publish, and package-identity changes require policy review against [fork-release-policy.md](fork-release-policy.md)

### 4. Seam Ownership

- repeated pressure in shared hot zones should trigger seam review
- do not refactor on the first painful merge
- do not normalize repeated merge debt forever either
- use [seam-ownership-roadmap.md](seam-ownership-roadmap.md) to decide when a shared surface now needs a fork-owned boundary

### 5. Documentation

- maintainer docs are part of runtime control, not commentary only
- if upstream intake changes policy, release behavior, or seam ownership expectations, update the relevant maintainer docs in the same workstream

## Intake Decision States

Use one or more of these states for each meaningful upstream change:

- `observe only`
- `prepare for merge`
- `reuse as-is`
- `adapt in fork`
- `reject for fork`
- `revisit thesis`
- `prepare release intake`

## Maintainer Loop

Each non-trivial upstream cycle should follow this loop:

1. fetch and observe upstream
2. classify the changes
3. record the observation in [upstream-intake-log.md](upstream-intake-log.md)
4. create a `merge/*` branch if intake is warranted
5. merge and resolve conflicts explicitly
6. validate by current risk tier
7. update maintainer docs if semantics changed
8. merge back into fork trunk

## Required Artifacts

Keep these documents aligned:

- [fork-operating-model.md](fork-operating-model.md)
- [fork-release-policy.md](fork-release-policy.md)
- [upstream-intake-log.md](upstream-intake-log.md)
- [seam-ownership-roadmap.md](seam-ownership-roadmap.md)
- [repo-separation-criteria.md](repo-separation-criteria.md)
- [fork-independence-backlog.md](fork-independence-backlog.md)

## Review Triggers

Revisit this document when any of the following becomes true:

- upstream starts repeatedly changing fork-sensitive release or runtime seams
- the fork begins rejecting or heavily adapting most upstream intake
- release automation on the fork changes materially
- the fork starts preparing for repository separation

## Current Working Interpretation

As of this document version:

- the fork should behave as an independent product inside a shared repo graph
- upstream is still materially useful substrate
- the immediate need is stronger fork control, not repository detachment
