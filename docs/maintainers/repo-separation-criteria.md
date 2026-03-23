# Repository Separation Criteria

Timestamp: 2026-03-23T06:51:06+02:00

## Status

This document defines when the fork should remain in managed independence and when it should separate into an independent repository.

Current judgment:

- do not split the repository yet
- run the fork as an independent product inside the current repo graph

## Purpose

Repository separation should happen because the operating model demands it, not because a branch-model mismatch or a single painful intake felt irritating.

## Decision Bands

| State | Meaning | Action |
|---|---|---|
| managed independence | upstream still provides meaningful substrate and intake cost is acceptable | stay in current topology |
| separation preparation | upstream is still useful, but drift is rising enough that independent repo prep should begin | prepare migration plan and controls, but keep importing from upstream |
| repository separation | the fork now needs full repo autonomy to protect product, release, or governance direction | create independent repository and treat upstream only as an import source |

## Core Criteria

Count how many criteria below are materially true.

1. Most useful upstream changes now require `adapt in fork` or selective cherry-pick rather than `reuse as-is`.
2. Two or more consecutive intakes create semantic conflicts in fork-owned seams, not just mechanical merge conflicts.
3. Fork release, CI, packaging, and security policy are now substantially different from upstream and expensive to keep aligned.
4. Fork product language, governance model, and roadmap no longer map cleanly onto upstream product framing.
5. Very little work is intended to flow back upstream.
6. GitHub fork identity now creates user, branding, or trust confusion that materially harms the fork.
7. Repository settings, branch protections, issue policy, or security controls needed by the fork are awkward or risky in the current topology.
8. The cost of preserving easy upstream intake is now greater than the value the fork gets from that ease.

## Thresholds

Use these thresholds unless a critical blocker appears earlier:

- `0-2` criteria: stay in managed independence
- `3-4` criteria: start separation preparation
- `5+` criteria: plan repository separation

Critical blocker rule:

- a single hard blocker in legal, security, release identity, or brand trust can justify separation earlier

## Evidence To Review

Do not score this document from intuition only.
Review evidence from:

- [upstream-intake-log.md](upstream-intake-log.md)
- [conflict-surface-map.md](conflict-surface-map.md)
- [seam-ownership-roadmap.md](seam-ownership-roadmap.md)
- [fork-release-policy.md](fork-release-policy.md)
- the last 2-3 upstream intakes

## Preparation Checklist

If the fork enters separation preparation:

1. define package and release identity clearly
2. confirm CI, release, and secret ownership
3. decide how upstream changes will be imported after separation
4. prepare repository settings, protections, and issue templates
5. prepare communication for contributors and users

## Review Cadence

Re-score these criteria after every 1-2 upstream intakes and after any major release-policy change.
