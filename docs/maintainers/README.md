# Maintainer Docs

Maintainer-facing documents for fork control, upstream intake, architecture pressure, documentation governance, and time-bound snapshots.

## Fork Control

- [fork-strategy-memo.md](fork-strategy-memo.md) - canonical strategic entry point for fork identity, boundaries, and near-term direction
- [fork-operating-model.md](fork-operating-model.md) - canonical fork-control model and decision hierarchy
- [fork-release-policy.md](fork-release-policy.md) - release, tag, publish, and CI ownership rules for the fork
- [repo-separation-criteria.md](repo-separation-criteria.md) - thresholds for staying in managed independence or preparing repo separation
- [fork-independence-backlog.md](fork-independence-backlog.md) - concrete backlog tasks that turn the operating model into work

## Intake And Ownership

- [upstream-intake-log.md](upstream-intake-log.md) - observation and intake ledger with validation and follow-up obligations
- [fork-upstream-observation-rules.md](fork-upstream-observation-rules.md) - upstream monitoring rules and decision states
- [conflict-surface-map.md](conflict-surface-map.md) - current fork/upstream collision zones and why they repeat
- [seam-ownership-roadmap.md](seam-ownership-roadmap.md) - phased roadmap for turning repeated pressure into fork-owned seams
- [fork-architecture-brief.md](fork-architecture-brief.md) - bridge between product thesis and current upstream-shaped runtime
- [formal-core-implementation-map.md](formal-core-implementation-map.md) - code-facing map from formal core to current implementation
- [first-governed-case-implementation-brief.md](first-governed-case-implementation-brief.md) - first bounded insertion point for governed handling
- [first-governed-case-mvp-plan.md](first-governed-case-mvp-plan.md) - first concrete governed-case implementation path

## Documentation Governance

- [docs-inventory.md](docs-inventory.md) - documentation classification and entry-point inventory
- [docs-path-and-gate-policy.md](docs-path-and-gate-policy.md) - repo-path versus user-path policy and corresponding docs gates
- [structure-README.md](structure-README.md) - structure map for documentation layout
- [i18n-coverage.md](i18n-coverage.md) - localization coverage status
- [repo-map.md](repo-map.md) - repository map for maintainer orientation
- [refactor-candidates.md](refactor-candidates.md) - current refactor candidates and cautions
- [trademark.md](trademark.md) - naming and trademark guidance

## Time-Bound Snapshots

- [project-triage-snapshot-2026-02-18.md](project-triage-snapshot-2026-02-18.md) - time-bound project triage snapshot

## How To Use This Set

- Use [fork-strategy-memo.md](fork-strategy-memo.md) first when the question is product identity, fork/upstream boundary, or the next strategic cycle.
- Use [fork-operating-model.md](fork-operating-model.md) as the canonical fork-control document.
- Use [fork-release-policy.md](fork-release-policy.md) before changing release automation, tags, publishing, or CI ownership.
- Update [upstream-intake-log.md](upstream-intake-log.md) on every non-trivial upstream observation pass and every upstream intake.
- Use [seam-ownership-roadmap.md](seam-ownership-roadmap.md) to decide which repeated conflict surfaces deserve fork-owned boundaries.
- Revisit [repo-separation-criteria.md](repo-separation-criteria.md) after every 1-2 upstream intakes or whenever release/product drift materially changes.

Time-bound maintainer artifacts should use a full ISO 8601 timestamp with UTC offset.
Date-only stamps are not enough for the current beta/release and upstream-intake cadence.
