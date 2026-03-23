# Fork Independence Backlog

Timestamp: 2026-03-23T06:51:06+02:00

## Purpose

This backlog converts the fork operating model into concrete maintainer work.
It is the execution companion to:

- [fork-operating-model.md](fork-operating-model.md)
- [fork-release-policy.md](fork-release-policy.md)
- [seam-ownership-roadmap.md](seam-ownership-roadmap.md)
- [repo-separation-criteria.md](repo-separation-criteria.md)

## Priority Backlog

| ID | Priority | Status | Item | Scope | Exit condition |
|---|---|---|---|---|---|
| FK-001 | P0 | done | Publish the maintainer fork-control doc set | docs | operating model, release policy, intake log, roadmap, criteria, and backlog are present and indexed |
| FK-002 | P0 | ready | Decide and record fork release mode | policy | maintainers explicitly choose development-only or release-capable mode in [fork-release-policy.md](fork-release-policy.md) |
| FK-003 | P0 | ready | Make CI and PR protections track fork trunk | `.github/workflows/**` | `origin/main` is the protected branch in CI and PR workflows |
| FK-004 | P0 | ready | Audit and adapt publish workflows to fork policy | `.github/workflows/**` | publish paths are either intentionally enabled for the fork or intentionally disabled |
| FK-005 | P0 | ready | Replace or rewrite fork-unsafe tag-cutting flow | `scripts/release/**` | release tooling no longer assumes upstream branch or tag semantics |
| FK-006 | P1 | ready | Make intake logging part of upstream sync discipline | docs and maintainer process | every non-trivial observation pass and every intake lands with a corresponding ledger update |
| FK-007 | P1 | next | Define the first narrow signal-classification seam | `src/gateway/**`, `src/agent/**` | primary signal classification has an explicit pre-motion home |
| FK-008 | P1 | next | Define governed case lifecycle ownership | `src/gateway/**`, `src/agent/**` | governed case creation and update stop depending on ad hoc shared bootstrap wiring |
| FK-009 | P1 | next | Define response-envelope decision seam | agent dispatch and tool path | allowed response modes are decided before generic tool/provider motion |
| FK-010 | P2 | next | Reduce shared tool-registry merge pressure | `src/tools/mod.rs` and adjacent wiring | fork governance logic no longer rides directly on the busiest shared registry seam |
| FK-011 | P2 | next | Reduce shared config-schema merge pressure | `src/config/schema.rs` and config loading | fork governance config has a clearer ownership boundary and admission path |
| FK-012 | P2 | later | Decide channel-path governed parity strategy | `src/channels/mod.rs` | maintainers have an explicit decision to defer, avoid, or implement channel-path parity |
| FK-013 | P2 | ready | Add separation review to maintainer cadence | docs and process | every 1-2 intakes the fork is scored against [repo-separation-criteria.md](repo-separation-criteria.md) |
| FK-014 | P3 | later | Prepare independent-repo fallback plan | repo settings, release, comms | a lightweight migration checklist exists before separation becomes urgent |

## Branching Guidance

Use these branch shapes while executing this backlog:

- `docs/*` for operating docs and policy updates
- `ops/*` or `infra/*` for release and CI adaptation
- `feat/*` or `fix/*` for runtime seam work
- `merge/*` only for upstream intake

## Review Rule

Reorder this backlog after each upstream intake that changes:

- release semantics
- fork-owned seam pressure
- repository separation likelihood
