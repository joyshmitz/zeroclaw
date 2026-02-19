# Session Context — SOP Foundation (Current) — 2026-02-19

Use this context file to start the next session without ambiguity.

## Canonical Context

repo_path: `/data/projects/zeroclaw`  
branch: `feat/sop-foundation`  
base_branch: `main`  
head_commit: `c2f7cd0`

objective:
- Complete `P0` gate for SOP runtime before Phase 5/6 fan-in.

done_definition:
- Trigger `condition` is evaluated deterministically in SOP matching.
- SOP audit is wired to real tool flow (`sop_execute/sop_approve/sop_advance`).
- Approval timeout check runs in active runtime loop (not tests-only).

in_scope_paths:
- `src/sop/**`
- `src/tools/sop_*.rs`
- `src/tools/mod.rs`
- `src/tools/traits.rs`
- `src/agent/loop_.rs`
- `src/channels/mod.rs`
- `docs/project/sop-foundation-plan-2026-02-19.md`

out_of_scope_paths:
- `src/providers/**` (unless explicitly requested)
- unrelated docs or localization files not tied to SOP runtime

authoritative_docs:
- `docs/project/sop-foundation-plan-2026-02-19.md`
- `docs/project/session-context-template.md`
- `docs/sop-necessity-research-2026-02-19.md`

constraints:
- Do not touch unrelated dirty files.
- No scope expansion beyond `in_scope_paths` without explicit confirmation.
- Keep changes small and reversible.

verification:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test sop:: --lib`
- `cargo test tools::sop --lib`
- `cargo test`

open_risks:
- Shared SOP engine lifecycle across tool/runtime boundaries is currently local to tool registry.
- Event fan-in (phase 5/6) before P0 can amplify nondeterministic transitions.
