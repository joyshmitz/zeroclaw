# Seam Ownership Roadmap

Timestamp: 2026-03-23T06:51:06+02:00

## Status

This roadmap turns the current conflict map and architecture brief into a prioritized sequence of fork-owned seam work.

## Purpose

Repeated merge pressure is a signal about ownership boundaries.
This document identifies where the fork should keep reusing upstream substrate and where it should gradually establish clearer fork-owned seams.

## Priority Rules

- prioritize seams that protect product direction or release control
- prefer small fork-owned boundaries over broad parallel rewrites
- do not refactor a shared hot zone unless repeated conflict or semantic mismatch justifies it
- treat release and CI control as a seam, not just as repository housekeeping

## Current Seam Table

| Seam | Current shared surfaces | Why it matters | Next ownership move | Priority |
|---|---|---|---|---|
| release and CI control | `.github/workflows/**`, `scripts/release/**` | release semantics can redefine fork behavior without code conflicts | fork-owned release policy, fork-safe triggers, fork-safe release scripts | P0 |
| signal classification ingress | `src/gateway/mod.rs`, `src/agent/agent.rs` | fork meaning currently enters through shared bootstrap zones | define a narrow pre-motion classification seam before generic runtime motion | P1 |
| governed case lifecycle | `src/gateway/mod.rs`, `src/agent/loop_.rs`, `src/agent/agent.rs` | governed response needs a durable operational unit, not transport-local state | establish case formation and update boundary | P1 |
| response envelope / pre-planning | `src/tools/mod.rs`, agent dispatch path | selective cognition remains rhetorical without an explicit decision seam | derive allowed response modes before generic tool or provider motion | P1 |
| SOP and tool registry isolation | `src/tools/mod.rs` | fork governance currently shares the same registry seam as all upstream tool growth | reduce direct pressure on the shared registry surface | P2 |
| fork config isolation | `src/config/schema.rs` | SOP and governance config live in a high-churn shared schema file | carve out clearer fork-owned config grouping and admission rules | P2 |
| channel-path governed parity | `src/channels/mod.rs` | future governed handling for channel traffic will collide in a high-churn ingress surface | decide whether to defer, avoid, or design a parity seam explicitly | P3 |
| CLI and product surface ownership | `src/main.rs`, `src/lib.rs` | fork product identity currently leaks through the same top-level command hub as upstream growth | keep CLI deltas tight and isolate fork product commands where practical | P3 |

## Phases

### Phase 0: Fork Control Artifacts

Deliverables:

- [fork-operating-model.md](fork-operating-model.md)
- [fork-release-policy.md](fork-release-policy.md)
- [upstream-intake-log.md](upstream-intake-log.md)
- [fork-independence-backlog.md](fork-independence-backlog.md)

Exit signal:

- maintainers can explain fork control without relying on chat history

### Phase 1: Operational Ownership

Target surfaces:

- `.github/workflows/**`
- `scripts/release/**`

Exit signal:

- CI and release semantics reflect fork intent rather than inherited upstream defaults

### Phase 2: Governed Entry Seams

Target surfaces:

- `src/gateway/mod.rs`
- `src/agent/agent.rs`
- `src/agent/loop_.rs`

Exit signal:

- primary signal classification and governed case formation have an explicit home

### Phase 3: Shared Registry And Schema Pressure

Target surfaces:

- `src/tools/mod.rs`
- `src/config/schema.rs`

Exit signal:

- repeated intake in these files is reduced because fork-specific logic no longer depends on the busiest shared seams

### Phase 4: Optional Parity And Separation Prep

Target surfaces:

- `src/channels/mod.rs`
- `src/main.rs`
- `src/lib.rs`

Exit signal:

- maintainers know whether channel-path parity is needed and whether repository separation prep should start

## Review Triggers

Update this roadmap when:

- the same shared hot zone causes repeated semantic conflict
- a fork-owned seam is created
- the fork rejects an upstream change because the seam is now intentionally owned
- repository separation becomes a realistic near-term option
