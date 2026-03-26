# Technology Watchlist Framework

Timestamp: 2026-03-26T07:20:00+02:00

## Status

This document defines how maintainers should watch external technology
candidates that might become future replacement options for fork-owned seams.

It is intentionally narrow.
It is not a commitment to adopt any watched project.

## Purpose

Use this framework to answer three questions consistently:

1. What exactly are we watching?
2. How do we compare trajectory and implementation fitness over time?
3. When should a watched project be revisited seriously?

The goal is to prevent two failure modes:

- curiosity becoming roadmap by accident
- dismissing future options without tracking whether they are maturing

## What We Watch

For each watched project, record these fields:

- target seam
  what it could replace for the fork, if anything
- category
  storage engine, runtime, data layer, or other
- current status
  `ignore`, `watch`, `watch seriously`, `prototype candidate`, or
  `adoption candidate`
- latest reviewed version
  stable tag when available; otherwise exact commit plus note that stable is
  missing
- critique
  the clearest current reason it is not ready

Track these dimensions at each meaningful review point:

- release hygiene
  tagged releases, changelog discipline, semver stability
- toolchain fit
  stable Rust versus nightly, platform assumptions, heavy custom runtime needs
- seam fit
  whether it can land behind one narrow fork-owned seam instead of forcing a
  broad rewrite
- implementation burden
  adapter-scale insertion versus subsystem rewrite versus architecture rewrite
- coupling discipline
  whether it can be consumed as a clean component rather than a tightly coupled
  stack
- operational evidence
  migration story, compatibility claims, documentation quality, production or
  serious internal-use signals

## How We Score

Score each watched project on a `0-2` scale for each dimension:

- `0`
  weak or missing
- `1`
  partially present, still risky
- `2`
  strong and credible

Use this table:

| Dimension | 0 | 1 | 2 |
|---|---|---|---|
| Release hygiene | no stable release line | some releases, still noisy/inconsistent | stable release line with credible semver and notes |
| Toolchain fit | nightly-only or unusual platform constraints | stable path exists but with caveats | stable toolchain and normal target assumptions |
| Seam fit | requires core rewrite | narrow pilot possible with visible pressure | clean insertion behind one fork-owned seam |
| Implementation burden | architecture rewrite | subsystem rewrite | adapter-scale or small sidecar insertion |
| Coupling discipline | bundled stack or path/git-tight coupling | some clean packaging, still opinionated | clean consumable component boundary |
| Operational evidence | mostly thesis/demo | partial compatibility or internal-use evidence | strong migration, compatibility, and ops evidence |

Total score: `0-12`.

Interpretation:

- `0-3`
  ignore for roadmap, optional occasional recheck
- `4-6`
  watch only
- `7-9`
  prototype candidate, but only behind a narrow seam
- `10-12`
  adoption candidate

## Hard Blockers

A high score does not override these blockers:

- the candidate requires broad ownership of current upstream substrate
- the candidate has no narrow seam where the fork could adopt it incrementally
- the candidate's maturity is lower than the maturity of the fork component it
  would replace
- the fork has not yet matured the target seam enough to benefit from the
  replacement

## Revisit Gates

Review a watched project when any of these happens:

- a new minor release
- a major architecture announcement
- stable-toolchain support arrives
- a migration guide or compatibility claim becomes concrete
- our own target seam matures materially

Fork-side revisit triggers:

- `frankensqlite`-type storage bets
  revisit when a durable local `GovernedCase` or evidence store exists and
  local SQLite write contention is a real pain point
- `sqlmodel_rust`-type data-layer bets
  revisit when `GovernedCase`, evidence, approval, and outcome become a stable
  typed domain model
- `asupersync`-type runtime bets
  revisit only when a new fork-owned service boundary exists and greenfield
  runtime choice is on the table

## Current Seeded Watchlist

| Project | Category | Target seam | Current status | Current critique |
|---|---|---|---|---|
| [frankensqlite](https://github.com/Dicklesworthstone/frankensqlite) | storage engine | future durable local case/evidence store | `watch seriously` | most credible storage candidate, but still nightly-only, no stable release line, and live path is still compatibility-first |
| [sqlmodel_rust](https://github.com/Dicklesworthstone/sqlmodel_rust) | data layer | future typed `GovernedCase` relational layer | `watch` | interesting only after domain stabilization; still active development, nightly-only, and tightly coupled to its own runtime assumptions |
| [asupersync](https://github.com/Dicklesworthstone/asupersync) | runtime | possible future greenfield governed-core service | `watch` | conceptually strong, but not a practical migration target for the current ZeroClaw core |

## Current Working Judgment

As of this document version:

- none of the seeded projects should enter the next 2 intake cycles as an
  adoption commitment
- `frankensqlite` is the only one that currently looks like a plausible future
  replacement candidate for a storage seam
- `sqlmodel_rust` only becomes relevant after a stable fork-owned relational
  domain emerges
- `asupersync` should be treated as a runtime philosophy to observe, not a core
  migration plan

## Update Rule

Update this document when:

- a watched project crosses a new decision band
- a new watched project is added or removed
- a revisit gate changes
- the fork matures a target seam enough that a previously irrelevant candidate
  becomes strategically relevant
