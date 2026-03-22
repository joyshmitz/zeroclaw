# Docs Path And Gate Policy

Timestamp: 2026-03-22T10:25:00+02:00

## Status

This document defines the fork policy for repository-doc path usage and the corresponding docs gates in `scripts/ci`.

It is fork-maintainer policy.
It does not replace the broader documentation contract in [`docs/contributing/docs-contract.md`](../contributing/docs-contract.md).

## Policy

1. Repository docs links must use repo-relative paths only.
2. User-specific application files on Linux must use XDG Base Directory paths.
3. In prose and examples:
   - use `<REPO_ROOT>` when referring to repository-root locations
   - use `${XDG_*}` when referring to user-specific directories

## Rationale

- absolute workspace paths such as `/data/projects/zeroclaw/...` are forbidden in committed docs because they are machine-specific
- XDG paths are for application-owned user-space artifacts: config, data, cache, state, history, logs, sockets, locks, and similar runtime files
- XDG paths and repo-relative paths solve different problems and must not be conflated

## Canonical Zeroclaw User Paths

- config: `${XDG_CONFIG_HOME:-$HOME/.config}/zeroclaw`
- data: `${XDG_DATA_HOME:-$HOME/.local/share}/zeroclaw`
- cache: `${XDG_CACHE_HOME:-$HOME/.cache}/zeroclaw`
- state: `${XDG_STATE_HOME:-$HOME/.local/state}/zeroclaw`
- runtime: `${XDG_RUNTIME_DIR}/zeroclaw`

## Rules

- ignore relative values in `XDG_*` environment variables
- do not create ad-hoc directories in `$HOME` without an explicit compatibility reason
- do not confuse XDG paths with repo-relative paths

## Implementation Guidance

### Repository Artifacts

Use repo-relative markdown links for committed repository artifacts such as:

- `../../src/agent/loop_.rs`
- `../../PLAN_TO_FUTURE_PRODUCT.md`

When prose needs to describe a repository-root location without becoming a markdown link, use `<REPO_ROOT>`, for example:

- `<REPO_ROOT>/src/agent/loop_.rs`

### User-Specific Runtime Artifacts

Use XDG notation in prose and examples for user-specific files, for example:

- `${XDG_CONFIG_HOME:-$HOME/.config}/zeroclaw/config.toml`
- `${XDG_STATE_HOME:-$HOME/.local/state}/zeroclaw/history.log`

Do not write machine-local or ad-hoc home paths such as:

- `~/.config/zeroclaw/...`
- `$HOME/.config/zeroclaw/...`
- `/data/projects/zeroclaw/...`

## Gate Model

### Blocking

The blocking docs path/link gate should stay incremental and deterministic.

It should reject only violations introduced on changed lines.

Blocking checks:

- broken added repo-relative markdown links
- added markdown link targets that use absolute filesystem-style paths
- added machine-specific workspace paths such as `/data/projects/zeroclaw/...`
- added non-XDG user-path examples such as `~/.config/zeroclaw/...`

### Optional

External URL availability should remain an optional audit, not a merge blocker.

Why:

- external URLs are inherently less deterministic
- scheduled or manual audit is useful
- per-PR blocking based on remote URL availability is usually process noise

## Current Script Contract

The fork should treat these scripts as the current implementation of this policy:

- `./scripts/ci/docs_quality_gate.sh`
  - blocking changed-line markdown lint
- `./scripts/ci/docs_links_gate.sh`
  - blocking changed-line repo-relative link and path-policy gate
- `./scripts/ci/docs_external_links_audit.sh`
  - optional external URL audit for changed docs links

## Runtime Follow-On

This policy also implies a code-facing follow-on rule:

- runtime/code should provide one canonical helper for XDG path resolution when user-specific zeroclaw paths are needed

That helper should be introduced only where there is a concrete runtime use case.
This document does not define that code change by itself.
