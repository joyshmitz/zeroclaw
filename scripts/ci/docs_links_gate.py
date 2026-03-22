#!/usr/bin/env python3

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path


DOC_PATH_RE = re.compile(r"\.mdx?$")
INLINE_LINK_RE = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
REF_LINK_RE = re.compile(r"^\s*\[[^\]]+\]:\s*(\S+)")
HUNK_RE = re.compile(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@")
TRAILING_PUNCTUATION = ").,;:!?]}'\""

FORBIDDEN_LINE_PATTERNS = [
    (
        re.compile(r"/data/projects/zeroclaw/"),
        "machine-specific workspace paths are forbidden; use repo-relative links or <REPO_ROOT> in prose",
    ),
    (
        re.compile(r"(?<!\w)(?:~|\$HOME)/\.config/zeroclaw(?:/|\b)"),
        "use ${XDG_CONFIG_HOME:-$HOME/.config}/zeroclaw in prose/examples",
    ),
    (
        re.compile(r"(?<!\w)(?:~|\$HOME)/\.local/share/zeroclaw(?:/|\b)"),
        "use ${XDG_DATA_HOME:-$HOME/.local/share}/zeroclaw in prose/examples",
    ),
    (
        re.compile(r"(?<!\w)(?:~|\$HOME)/\.cache/zeroclaw(?:/|\b)"),
        "use ${XDG_CACHE_HOME:-$HOME/.cache}/zeroclaw in prose/examples",
    ),
    (
        re.compile(r"(?<!\w)(?:~|\$HOME)/\.local/state/zeroclaw(?:/|\b)"),
        "use ${XDG_STATE_HOME:-$HOME/.local/state}/zeroclaw in prose/examples",
    ),
]

FORBIDDEN_PATTERN_ALLOWLIST_FILES = {
    "docs/maintainers/docs-path-and-gate-policy.md",
}


def run_git(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["git", *args], check=False, capture_output=True, text=True)


def commit_exists(rev: str) -> bool:
    if not rev:
        return False
    return run_git(["cat-file", "-e", f"{rev}^{{commit}}"]).returncode == 0


def normalize_docs_files(raw: str) -> list[str]:
    if not raw:
        return []
    files: list[str] = []
    for line in raw.splitlines():
        path = line.strip()
        if path:
            files.append(path)
    return files


def infer_base_sha(provided: str) -> str:
    if commit_exists(provided):
        return provided
    if run_git(["rev-parse", "--verify", "origin/master"]).returncode != 0:
        return ""
    proc = run_git(["merge-base", "origin/master", "HEAD"])
    candidate = proc.stdout.strip()
    return candidate if commit_exists(candidate) else ""


def infer_docs_files(base_sha: str, provided: list[str]) -> list[str]:
    if provided:
        return provided
    if not base_sha:
        return []
    diff = run_git(["diff", "--name-only", base_sha, "HEAD"])
    files: list[str] = []
    for line in diff.stdout.splitlines():
        path = line.strip()
        if not path:
            continue
        if DOC_PATH_RE.search(path) or path in {"LICENSE", ".github/pull_request_template.md"}:
            files.append(path)
    return files


def added_lines_for_file(base_sha: str, path: str) -> list[tuple[int, str]]:
    if not base_sha:
        file_path = Path(path)
        if not file_path.is_file():
            return []
        return list(enumerate(file_path.read_text(encoding="utf-8", errors="ignore").splitlines(), start=1))

    diff = run_git(["diff", "--unified=0", base_sha, "HEAD", "--", path])
    added: list[tuple[int, str]] = []
    current_new_line = 0
    for raw_line in diff.stdout.splitlines():
        hunk = HUNK_RE.match(raw_line)
        if hunk:
            current_new_line = int(hunk.group(1))
            continue
        if raw_line.startswith("+++"):
            continue
        if raw_line.startswith("+"):
            added.append((current_new_line, raw_line[1:]))
            current_new_line += 1
            continue
        if raw_line.startswith("-") and not raw_line.startswith("---"):
            continue
        if raw_line.startswith(" "):
            current_new_line += 1
    return added


def extract_link_targets(text: str) -> list[str]:
    targets: list[str] = []
    for match in INLINE_LINK_RE.findall(text):
        targets.append(match)
    ref_match = REF_LINK_RE.match(text)
    if ref_match:
        targets.append(ref_match.group(1))
    return targets


def prepare_target(raw_target: str) -> str:
    target = raw_target.strip()
    if target.startswith("<") and target.endswith(">"):
        target = target[1:-1].strip()
    if " " in target:
        target = target.split()[0].strip()
    return target.rstrip(TRAILING_PUNCTUATION)


def normalize_internal_target(target: str, source_path: str) -> str | None:
    if not target or target.startswith("#"):
        return None
    lower = target.lower()
    if lower.startswith(("http://", "https://", "mailto:", "tel:", "javascript:", "data:")):
        return None
    path_without_fragment = target.split("#", 1)[0].split("?", 1)[0]
    if not path_without_fragment:
        return None
    return os.path.normpath(
        os.path.join(os.path.dirname(source_path) or ".", path_without_fragment)
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate changed-line docs links and docs path policy"
    )
    parser.add_argument("--base", default="", help="Base commit SHA")
    parser.add_argument(
        "--docs-files",
        default="",
        help="Newline-separated docs files list",
    )
    args = parser.parse_args()

    base_sha = infer_base_sha(args.base)
    docs_files = infer_docs_files(base_sha, normalize_docs_files(args.docs_files))
    existing_files = [path for path in docs_files if Path(path).is_file()]

    if not existing_files:
        print("No docs files available for docs links gate.")
        return 0

    violations: list[str] = []

    for path in existing_files:
        for line_no, line in added_lines_for_file(base_sha, path):
            if path not in FORBIDDEN_PATTERN_ALLOWLIST_FILES:
                for pattern, message in FORBIDDEN_LINE_PATTERNS:
                    if pattern.search(line):
                        violations.append(f"{path}:{line_no} {message}")

            for raw_target in extract_link_targets(line):
                target = prepare_target(raw_target)
                if not target:
                    continue
                lower = target.lower()
                if lower.startswith(("http://", "https://", "mailto:", "tel:", "javascript:", "data:")):
                    continue
                if target.startswith("/"):
                    violations.append(
                        f"{path}:{line_no} absolute filesystem-style markdown links are forbidden; use repo-relative paths only"
                    )
                    continue
                resolved = normalize_internal_target(target, path)
                if resolved and not Path(resolved).exists():
                    violations.append(
                        f"{path}:{line_no} broken repo-relative link target: {target}"
                    )

    if violations:
        print("Docs links/path gate violations on changed lines:")
        for violation in violations:
            print(f"  - {violation}")
        print(f"Blocking docs links/path violations: {len(violations)}")
        return 1

    print("No blocking docs links/path violations on changed lines.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
