#!/usr/bin/env bash

set -euo pipefail

BASE_SHA="${BASE_SHA:-}"
DOCS_FILES_RAW="${DOCS_FILES:-}"

python3 ./scripts/ci/docs_links_gate.py \
    --base "$BASE_SHA" \
    --docs-files "$DOCS_FILES_RAW"
