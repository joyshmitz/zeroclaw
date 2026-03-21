# Conflict Surface Map

Timestamp: 2026-03-21

## Status

This document records the current fork/upstream conflict surfaces after:

- the fork product definition became explicit
- the first architecture brief was added
- the first governed-case MVP path was chosen
- `upstream/master` advanced beyond the last sync point

This is not a generic git-hygiene note.
It is a map of the architectural seams where fork meaning currently enters an upstream-shaped runtime.

## Reference Point

Current comparison baseline at the time of writing:

- fork branch head before this document: `7fb72afb`
- current `upstream/master`: `756c3cad`

The point is not the exact hashes.
The point is that the fork has stable thesis documents, while upstream continues to change runtime surfaces that the fork already touches.

## Purpose

This map exists to answer:

- where repeated merge conflicts are most likely
- which of those conflicts are merely tactical
- which of those conflicts indicate missing ownership boundaries
- where a future fork-owned seam or integration point would reduce repeated merge cost

## Upstream Collaboration Guardrail

This map should not be read as a case for adversarial divergence from upstream.

The fork's working stance remains:

- upstream is the main runtime substrate
- upstream movement is observed as signal, not treated as noise
- reuse as-is is preferred where it does not distort fork identity
- fork-owned seams should reduce repeated conflict without turning the fork into a parallel unrelated system

Repeated conflict should therefore trigger better seam ownership and cleaner collaboration choices.
It should not trigger reflexive rejection of upstream evolution.

## Core Reading

The same files keep mattering because they sit at the boundary between:

- generic upstream runtime assembly
- fork-specific governed-response wiring

That means these files are not just "busy files".
They are places where product identity is trying to enter the runtime.

## Primary Conflict Surfaces

### 1. `src/agent/agent.rs`

Current fork pressure:

- fork adds SOP engine creation
- fork passes shared SOP engine into tool construction
- upstream also modifies agent construction and MCP tool wiring here

Why this repeats:

- this file is a central runtime assembly point
- any cross-cutting capability tends to be wired here first

What this means architecturally:

- the fork does not yet have a dedicated governed-response entry seam before generic agent construction
- so SOP enters the system through a shared bootstrap zone

Current judgment:

- high repeated-conflict risk
- not yet a reason to refactor blindly
- but clearly a candidate for future integration consolidation

### 2. `src/gateway/mod.rs`

Current fork pressure:

- fork adds SOP engine creation during gateway runtime setup
- fork passes SOP state into tool registry creation
- upstream changes gateway routing and runtime assembly in the same area

Why this repeats:

- gateway assembly is another central bootstrap zone
- the fork currently injects meaning here because no earlier fork-owned seam exists

What this means architecturally:

- gateway-level runtime setup is serving as a proxy for signal-handling ownership
- that is acceptable temporarily, but unstable as a long-term seam

Current judgment:

- high repeated-conflict risk
- same underlying ownership problem as `src/agent/agent.rs`

### 3. `src/tools/mod.rs`

Current fork pressure:

- fork adds SOP tool modules and exports
- fork extends `all_tools` and `all_tools_with_runtime` with shared SOP engine state
- fork conditionally registers SOP tools

Why this repeats:

- this file is the generic capability registry
- upstream frequently changes tool registration, tool signatures, and runtime-aware tool composition

What this means architecturally:

- the fork currently expresses governed response partly by adding tools
- that is valid, but it also means governed-response logic shares a seam with every generic capability expansion upstream

Current judgment:

- high conflict risk
- especially sensitive because constructor signatures ripple into tests and call sites

### 4. `src/config/schema.rs`

Current fork pressure:

- fork adds `[sop]` configuration and defaults
- upstream keeps changing unrelated config areas in the same giant file

Observed upstream movement already visible in the current comparison:

- transcription provider changes
- WhatsApp Web policy shape changes
- validation allowlists shifting

Why this repeats:

- this is a large cross-cutting schema file
- many unrelated features collide here even when they are conceptually independent

What this means architecturally:

- the fork's SOP presence is currently represented as a config extension inside a high-churn shared schema surface
- that is structurally expensive even if functionally correct

Current judgment:

- high mechanical merge-risk
- medium architectural-risk
- likely to remain painful until either schema churn slows or fork-owned config seams are better isolated

### 5. `src/lib.rs`

Current fork pressure:

- fork exports `sop`
- fork adds `SopCommands`

Why this repeats:

- `lib.rs` is a central visibility and command-enum surface
- upstream also changes command topology over time

What this means architecturally:

- the fork currently claims product surface area partly through CLI exposure
- that is reasonable, but it also means the fork must keep reconciling with upstream command growth

Current judgment:

- medium repeated-conflict risk
- lower churn than `main.rs`, but still a shared control surface

### 6. `src/main.rs`

Current fork pressure:

- fork adds `mod sop`
- fork re-exports and handles `SopCommands`
- upstream keeps changing auth flows, status output, and command behavior here

Why this repeats:

- this file is the single biggest CLI and orchestration chokepoint
- almost every new user-facing capability eventually touches it

What this means architecturally:

- the fork currently exposes governed-response capability through the same top-level command hub that upstream uses for broad product evolution

Current judgment:

- high repeated-conflict risk
- central CLI assembly remains one of the most expensive shared seams in the repo

## Shared Conflict Patterns

The six surfaces above cluster into four repeat patterns.

### Pattern A: Shared Bootstrap Zones

Files:

- `src/agent/agent.rs`
- `src/gateway/mod.rs`

Meaning:

- fork-specific governed logic enters the runtime during central assembly
- upstream also adds runtime-wide capability wiring there

### Pattern B: Shared Registry Surface

Files:

- `src/tools/mod.rs`

Meaning:

- the fork uses the generic tool registry to carry governed-response capability
- upstream uses the same place for broad tool growth

### Pattern C: Shared Cross-Cutting Schema Surface

Files:

- `src/config/schema.rs`

Meaning:

- unrelated capabilities collide because one mega-schema file absorbs everything

### Pattern D: Shared CLI Control Surface

Files:

- `src/lib.rs`
- `src/main.rs`

Meaning:

- fork and upstream both express identity through central command wiring

## Tactical Conflicts vs Ownership Conflicts

Some future conflicts in these files will be merely tactical:

- function signature shifts
- route additions
- enum growth
- unrelated config additions

But repeated conflict in the same seam should be treated as an ownership signal when:

- the fork keeps re-introducing governed-response wiring in the same bootstrap zones
- upstream keeps evolving those same zones for unrelated runtime features
- merge resolution becomes "keep both" without reducing conceptual overlap

That is when the issue stops being git friction and becomes architecture pressure.

## What To Watch In Future Upstream Movement

Priority watch items:

- agent construction changes
- gateway runtime assembly changes
- tool-registry signature changes
- command-tree growth
- config-schema expansion
- any upstream orchestration, workflow, gating, or DAG work

Current examples already visible after the latest fetch:

- transcription/provider expansion touches `src/config/schema.rs`
- status/auth/CLI changes touch `src/main.rs`
- gateway behavior continues moving in `src/gateway/mod.rs`

## Working Reduction Strategy

This document does not prescribe immediate refactors.
It does define the reduction strategy the fork should prefer over time:

1. avoid spreading new fork meaning across additional shared bootstrap zones
2. identify the smallest future seam where governed-case handling can enter before generic tool motion
3. consolidate fork-specific SOP wiring when a stable integration point becomes available
4. treat repeated conflict in the same surface as evidence for seam ownership, not just merge pain

## Immediate Consequences

Based on the current map:

- `src/agent/agent.rs` and `src/gateway/mod.rs` are the clearest first-order tension points
- `src/tools/mod.rs` is the most likely signature-ripple surface
- `src/config/schema.rs` is the most likely slow-burn merge-cost amplifier
- `src/main.rs` remains the most exposed CLI conflict surface

## Bottom Line

The fork does not yet suffer because it has too many custom files.

It suffers because its product meaning currently enters the repo through upstream-owned central surfaces.

This map should therefore be read as:

- a merge-risk artifact
- an architecture-pressure artifact
- a guide for where future fork-owned seams should reduce repeated conflict
