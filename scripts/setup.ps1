$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if (-not (Get-Command cargo -ErrorAction SilentlyContinue) -or -not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw 'Rust is required. Install from https://rustup.rs/ and restart PowerShell.'
}

if (Get-Command py -ErrorAction SilentlyContinue) {
    & py -3 -m venv .venv
} elseif (Get-Command python -ErrorAction SilentlyContinue) {
    & python -m venv .venv
} else {
    throw 'Python 3 is required: https://www.python.org/downloads/'
}

& .\.venv\Scripts\python.exe -m pip install --upgrade pip
& .\.venv\Scripts\python.exe -m pip install jupyterlab
& rustup component add rust-src

if (-not (Get-Command evcxr_jupyter -ErrorAction SilentlyContinue)) {
    & cargo install --locked evcxr_jupyter
}

# Register the Rust kernel in this Jupyter virtual environment.
$env:JUPYTER_PATH = (Join-Path (Get-Location) '.venv\share\jupyter')
try {
    & evcxr_jupyter --install
} finally {
    Remove-Item Env:JUPYTER_PATH -ErrorAction SilentlyContinue
}

Write-Host 'Ready! Run: .\.venv\Scripts\jupyter.exe lab'
Write-Host 'Then choose Rust and open notebooks/01_hello_rust.ipynb.'
