#!/usr/bin/env bash
# Build both language versions of the slide deck for GitHub Pages (served from /docs).
# --no-stdin is required or marp waits forever on stdin.
set -euo pipefail
cd "$(dirname "$0")"
npx --yes @marp-team/marp-cli --no-stdin docs/garden_guilds.marp.md -o docs/index.html
npx --yes @marp-team/marp-cli --no-stdin docs/garden_guilds.nl.marp.md -o docs/index.nl.html
