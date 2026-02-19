# Session Context Template (Required) — 2026-02-19

Use this as mandatory context for every new coding session.

## 1) Copy/Paste Block For New Session

```text
Контекст сесії:
repo_path: <absolute path>
branch: <working branch>
base_branch: <target merge base>
head_commit: <short sha>
objective: <one concrete outcome>
done_definition:
- <measurable condition 1>
- <measurable condition 2>
in_scope_paths:
- <path/glob>
out_of_scope_paths:
- <path/glob>
authoritative_docs:
- <doc path>
constraints:
- do not touch unrelated dirty files
- no edits outside in_scope without explicit ask
verification:
- <command 1>
- <command 2>
open_risks:
- <risk + mitigation>
```

## 2) Minimum Required Fields

Without these fields, the session is considered under-specified:

1. `repo_path`
2. `branch`
3. `objective`
4. `done_definition`
5. `in_scope_paths`
6. `authoritative_docs`
7. `verification`

## 3) One-Command Capture (Operator)

Run before opening a new session:

```bash
cd <repo_path>
printf "repo_path: %s\n" "$(pwd)"
printf "branch: %s\n" "$(git branch --show-current)"
printf "head_commit: %s\n" "$(git rev-parse --short HEAD)"
git status -sb
```

## 4) Session Discipline Rules

- Keep one objective per session.
- Keep one repo as primary execution target.
- Keep scope explicit (`in_scope_paths`) and bounded.
- If there are dirty unrelated files, declare them at start and freeze them.
