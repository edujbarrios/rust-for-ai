#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v cargo >/dev/null 2>&1 || ! command -v rustup >/dev/null 2>&1; then
  echo "Rust is required. Install it from https://rustup.rs/ and reopen the terminal." >&2
  exit 1
fi

if command -v python3 >/dev/null 2>&1; then
  PYTHON=python3
elif command -v python >/dev/null 2>&1; then
  PYTHON=python
else
  echo "Python 3 is required: https://www.python.org/downloads/" >&2
  exit 1
fi

"$PYTHON" -m venv .venv
./.venv/bin/python -m pip install --upgrade pip
./.venv/bin/python -m pip install jupyterlab
rustup component add rust-src
if ! command -v evcxr_jupyter >/dev/null 2>&1; then
  cargo install --locked evcxr_jupyter
fi
# Install kernel into the virtual environment so it is visible to this JupyterLab.
JUPYTER_PATH="$PWD/.venv/share/jupyter" evcxr_jupyter --install

echo
echo "Ready! Run: ./.venv/bin/jupyter lab"
echo "Then choose the Rust kernel and open notebooks/01_hello_rust.ipynb."
