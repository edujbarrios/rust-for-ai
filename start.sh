#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [ ! -f .venv/.rust-for-ai-ready ] || [ ! -x .venv/bin/jupyter ]; then
  bash scripts/setup.sh
fi
exec .venv/bin/jupyter lab --notebook-dir="$PWD"
