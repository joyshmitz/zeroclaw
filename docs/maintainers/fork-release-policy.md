# Fork Release Policy

Timestamp: 2026-03-23T06:51:06+02:00

## Status

This document defines how the fork should treat release, publishing, tag, and CI semantics while remaining in the current repository topology.

## Purpose

The fork now needs explicit release ownership.
Upstream release automation is useful substrate, but it is not self-justifying fork policy.

## Policy Goal

Release and CI behavior should reflect fork intent, not inherited defaults.

The strategic question is:

- what should the fork publish
- when should it publish
- under which governance and validation rules

The branch name itself is only an implementation detail inside that larger policy.

## Current Default Stance

Until maintainers explicitly promote the fork to release-capable status, operate as a development-first fork:

- trunk and CI should protect `origin/main`
- stable tags should be treated as high-consequence actions
- no publish workflow should be assumed safe just because it came from upstream
- release secrets and publishing credentials should be considered disabled-by-policy unless explicitly approved

## Workflow Classes

| Surface | Upstream behavior at `v0.5.9` | Fork implication | Current fork stance |
|---|---|---|---|
| `.github/workflows/release-stable-manual.yml` | manual dispatch plus push on stable tags | stable tags now trigger full stable release pipeline | treat stable tag push as a guarded action |
| `.github/workflows/release-beta-on-push.yml` | push automation still keyed to `master` | inherited branch semantics do not match fork trunk | adapt or disable by fork policy |
| `.github/workflows/publish-crates-auto.yml` | auto publish logic still keyed to `master` and `Cargo.toml` | package publishing semantics are inherited unless reviewed | adapt or disable by fork policy |
| `.github/workflows/pub-homebrew-core.yml` | reusable downstream release job | Homebrew publication became part of stable release surface | do not enable casually |
| `.github/workflows/ci-run.yml` | CI still keyed to `master` | trunk protection can drift if left unchanged | adapt to fork trunk |
| `.github/workflows/checks-on-pr.yml` | PR checks still keyed to `master` | review protections can miss fork trunk | adapt to fork trunk |
| `scripts/release/cut_release_tag.sh` | still assumes `origin/master` | release tooling encodes upstream branch model | fork-safe rewrite required before release use |

## Release Modes

The fork should choose one explicit mode and document it.

### Mode A: Development-Only Fork

Use this mode when the fork is still integrating direction, seams, and release identity.

Requirements:

- CI protects trunk
- publish workflows are disabled or clearly guarded
- stable tags are not part of normal maintainer flow

### Mode B: Release-Capable Fork

Use this mode only after release identity, package naming, publish targets, and secret handling are all fork-owned.

Requirements:

- CI and PR gates protect fork trunk
- stable and beta release triggers are intentionally wired for the fork
- `cut_release_tag.sh` or its replacement matches the fork branch model
- package publishing and downstream automation are explicitly approved

## Required Review For Release-Related Changes

Treat these changes as high-risk fork-control work:

- workflow triggers
- publish jobs
- tag creation or tag-triggered release behavior
- package manifests and distribution metadata
- release scripts
- secrets or token assumptions

Required checks:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- workflow semantic review
- shell syntax checks for touched scripts

## Immediate Follow-Up Policy

Before treating the fork as release-capable, complete the following:

1. choose the fork release mode explicitly
2. make CI and PR protections track fork trunk
3. adapt or disable inherited publish workflows
4. make release scripts fork-safe
5. document who may push stable tags and under what checklist

## Update Triggers

Update this document when:

- upstream changes release or publishing automation
- the fork changes branch or tag policy
- package identity changes
- the fork moves from development-only to release-capable operation
