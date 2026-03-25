# AGENTS.md — ZeroClaw

Cross-tool agent instructions for any AI coding assistant working on this repository.
## Commands

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Full pre-PR validation (recommended):

```bash
./dev/ci.sh all
```

Docs-only changes: run markdown lint and link-integrity checks. If touching bootstrap scripts: `bash -n install.sh`.

## Project Snapshot

ZeroClaw is a Rust-first autonomous agent runtime optimized for performance, efficiency, stability, extensibility, sustainability, and security.

Core architecture is trait-driven and modular. Extend by implementing traits and registering in factory modules.

Key extension points:

- `src/providers/traits.rs` (`Provider`)
- `src/channels/traits.rs` (`Channel`)
- `src/tools/traits.rs` (`Tool`)
- `src/memory/traits.rs` (`Memory`)
- `src/observability/traits.rs` (`Observer`)
- `src/runtime/traits.rs` (`RuntimeAdapter`)
- `src/peripherals/traits.rs` (`Peripheral`) — hardware boards (STM32, RPi GPIO)

## Repository Map

- `src/main.rs` — CLI entrypoint and command routing
- `src/lib.rs` — module exports and shared command enums
- `src/config/` — schema + config loading/merging
- `src/agent/` — orchestration loop
- `src/gateway/` — webhook/gateway server
- `src/security/` — policy, pairing, secret store
- `src/memory/` — markdown/sqlite memory backends + embeddings/vector merge
- `src/providers/` — model providers and resilient wrapper
- `src/channels/` — Telegram/Discord/Slack/etc channels
- `src/tools/` — tool execution surface (shell, file, memory, browser)
- `src/peripherals/` — hardware peripherals (STM32, RPi GPIO)
- `src/runtime/` — runtime adapters (currently native)
- `docs/` — topic-based documentation (setup-guides, reference, ops, security, hardware, contributing, maintainers)
- `.github/` — CI, templates, automation workflows

## Risk Tiers

- **Low risk**: docs/chore/tests-only changes
- **Medium risk**: most `src/**` behavior changes without boundary/security impact
- **High risk**: `src/security/**`, `src/runtime/**`, `src/gateway/**`, `src/tools/**`, `.github/workflows/**`, access-control boundaries

When uncertain, classify as higher risk.

## Workflow

1. **Read before write** — inspect existing module, factory wiring, and adjacent tests before editing.
2. **One concern per PR** — avoid mixed feature+refactor+infra patches.
3. **Implement minimal patch** — no speculative abstractions, no config keys without a concrete use case.
4. **Validate by risk tier** — docs-only: lightweight checks. Code changes: full relevant checks.
5. **Document impact** — update PR notes for behavior, risk, side effects, and rollback.
6. **Queue hygiene** — stacked PR: declare `Depends on #...`. Replacing old PR: declare `Supersedes #...`.

Branch/commit/PR rules:
- Work from a non-`master` branch. Open a PR to `master`; do not push directly.
- Use conventional commit titles. Prefer small PRs (`size: XS/S/M`).
- Follow `.github/pull_request_template.md` fully.
- Never commit secrets, personal data, or real identity information (see `@docs/contributing/pr-discipline.md`).

### Fork Branch Hygiene

For fork maintenance, treat the branch model explicitly:

- `origin/main` is the fork trunk
- `upstream/master` is the current upstream intake source unless upstream changes its default branch
- topic work should branch from the current fork trunk, not directly from upstream intake branches

When upstream merges are expected to create conflicts:

- create a dedicated intake branch such as `merge/upstream-master-YYYYMMDD`
- merge `upstream/master` into that intake branch first
- resolve conflicts there with explicit conflict-resolution commits
- validate there before merging the intake branch back into the fork trunk

Do not mix these concerns in one branch:

- long-lived product work
- upstream intake/conflict resolution
- upstreamable low-conflict patches

Preferred split:

- `docs/*` or `plan/*` for product-definition and strategic docs work
- `feat/*` or `fix/*` for fork functionality
- `merge/*` for upstream intake and conflict resolution
- small atomic branches for anything intended to be proposed upstream

Conflict-aware working rule:

- keep fork-specific deltas isolated and easy to replay
- expect repeated conflicts in strategic docs, prompt/bootstrap files, and cross-cutting orchestration surfaces
- prefer explicit merge commits for upstream intake over hiding conflict resolution in unrelated feature history
- revisit branch boundaries when fork drift starts making conflict resolution materially expensive
## Anti-Patterns

- Do not add heavy dependencies for minor convenience.
- Do not silently weaken security policy or access constraints.
- Do not add speculative config/feature flags "just in case".
- Do not mix massive formatting-only changes with functional changes.
- Do not modify unrelated modules "while here".
- Do not bypass failing checks without explicit explanation.
- Do not hide behavior-changing side effects in refactor commits.
- Do not include personal identity or sensitive information in test data, examples, docs, or commits.

## Fork Product Guidance

When working on the fork’s product definition, planning documents, or related strategic materials:

- keep the center on `primary signal -> meaning -> governed response`
- use PDCA as the governing framing
- treat SOP as the current core of `Plan`
- keep signal type above delivery channel in product reasoning
- apply cognition selectively, not maximally by default

### Standards Guardrail

Standards have an important role in this fork, especially for:

- PDCA discipline
- vocabulary
- evidence and audit
- risk treatment
- review and continual improvement
- AI governance and security boundaries

Critical correction:

- standards should guide and tighten relevant sections of the documentation
- standards should **not** define product identity
- otherwise the fork drifts into certification-first thinking instead of product-first clarity

### Product Identity Guardrail

Do not let planning or documentation work slide into:

- transport-first framing
- certification-first framing
- workflow-engine-first framing
- edge-first framing
- LLM-first framing

The product is defined by governed interpretation and response to heterogeneous primary signals, not by any one standard, provider, transport, or deployment topology.
## Linked References

- `@docs/contributing/change-playbooks.md` — adding providers, channels, tools, peripherals; security/gateway changes; architecture boundaries
- `@docs/contributing/pr-discipline.md` — privacy rules, superseded-PR attribution/templates, handoff template
- `@docs/contributing/docs-contract.md` — docs system contract, i18n rules, locale parity
- `@docs/maintainers/conflict-surface-map.md` — current fork/upstream collision zones and what they imply about missing ownership seams
- `@docs/maintainers/fork-architecture-brief.md` — architecture bridge between fork thesis and upstream-shaped codebase
- `@docs/maintainers/first-governed-case-implementation-brief.md` — module-level implementation address for the first governed case without broad refactor pressure
- `@docs/maintainers/first-governed-case-mvp-plan.md` — first concrete path from primary signal to governed case, bounded response, evidence, and PDCA feedback
- `@docs/maintainers/fork-upstream-observation-rules.md` — how to watch upstream without letting upstream velocity redefine fork direction
