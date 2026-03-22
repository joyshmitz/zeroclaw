#!/usr/bin/env bash

set -euo pipefail

BASE_SHA="${BASE_SHA:-}"
DOCS_FILES_RAW="${DOCS_FILES:-}"

LINKS_FILE="$(mktemp)"
URLS_FILE="$(mktemp)"
trap 'rm -f "$LINKS_FILE" "$URLS_FILE"' EXIT

python3 ./scripts/ci/collect_changed_links.py \
    --base "$BASE_SHA" \
    --docs-files "$DOCS_FILES_RAW" \
    --output "$LINKS_FILE"

grep -E '^https?://' "$LINKS_FILE" >"$URLS_FILE" || true

if [ ! -s "$URLS_FILE" ]; then
    echo "No added external links detected in changed docs lines."
    exit 0
fi

if ! command -v lychee >/dev/null 2>&1; then
    echo "lychee is required to run the external docs links audit locally."
    echo "Install via: cargo install lychee"
    exit 1
fi

echo "Auditing added external docs links with lychee..."
lychee --no-progress --format detailed "$URLS_FILE"
