#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v cargo >/dev/null 2>&1 || ! command -v rustup >/dev/null 2>&1; then
  echo "Rust is missing. Install it from https://rustup.rs/ and reopen your terminal." >&2
  exit 1
fi

if command -v python3 >/dev/null 2>&1; then
  PYTHON=python3
elif command -v python >/dev/null 2>&1 && python -c 'import sys; sys.exit(sys.version_info < (3, 9))'; then
  PYTHON=python
else
  echo "Python 3.9+ is required: https://www.python.org/downloads/" >&2
  exit 1
fi

if [ ! -x .venv/bin/python ]; then
  "$PYTHON" -m venv .venv
fi
if ! .venv/bin/python -c 'import jupyterlab' >/dev/null 2>&1; then
  .venv/bin/python -m pip install jupyterlab
fi
rustup component add rust-src
EVCXR="$(command -v evcxr_jupyter || true)"
if [ -z "$EVCXR" ] && [ -x "$HOME/.cargo/bin/evcxr_jupyter" ]; then
  EVCXR="$HOME/.cargo/bin/evcxr_jupyter"
fi
if [ -z "$EVCXR" ]; then
  cargo install --locked evcxr_jupyter
  EVCXR="$HOME/.cargo/bin/evcxr_jupyter"
fi
# Evcxr's supported virtual-environment registration method.
JUPYTER_PATH="$PWD/.venv/share/jupyter" "$EVCXR" --install
.venv/bin/python -c 'from jupyter_client.kernelspec import KernelSpecManager; assert "rust" in KernelSpecManager().find_kernel_specs(), "Rust kernel registration failed"'
touch .venv/.rust-for-ai-ready

echo "Setup complete. Start notebooks anytime with: bash start.sh"
