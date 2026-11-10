$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if (-not (Get-Command cargo -ErrorAction SilentlyContinue) -or -not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw 'Rust is missing. Install it from https://rustup.rs/ and restart PowerShell.'
}
$venvPython = Join-Path (Get-Location) '.venv\Scripts\python.exe'
if (-not (Test-Path $venvPython)) {
    if (Get-Command py -ErrorAction SilentlyContinue) {
        & py -3 -m venv .venv
    } elseif (Get-Command python -ErrorAction SilentlyContinue) {
        & python -m venv .venv
    } else {
        throw 'Python 3.9+ is required: https://www.python.org/downloads/'
    }
    if ($LASTEXITCODE -ne 0) { throw 'Failed to create the Python virtual environment.' }
}
& $venvPython -c 'import jupyterlab'
if ($LASTEXITCODE -ne 0) {
    & $venvPython -m pip install jupyterlab
    if ($LASTEXITCODE -ne 0) { throw 'Failed to install JupyterLab.' }
}
& rustup component add rust-src
if ($LASTEXITCODE -ne 0) { throw 'Failed to install rust-src.' }

$evcxr = Get-Command evcxr_jupyter -ErrorAction SilentlyContinue
if ($evcxr) {
    $evcxrExe = $evcxr.Source
} else {
    $evcxrExe = Join-Path $HOME '.cargo\bin\evcxr_jupyter.exe'
    if (-not (Test-Path $evcxrExe)) {
        & cargo install --locked evcxr_jupyter
        if ($LASTEXITCODE -ne 0) { throw 'Failed to install the Rust Jupyter kernel.' }
    }
}
$priorPath = $env:JUPYTER_PATH
$env:JUPYTER_PATH = Join-Path (Get-Location) '.venv\share\jupyter'
try {
    & $evcxrExe --install
    if ($LASTEXITCODE -ne 0) { throw 'Rust kernel registration failed.' }
} finally {
    $env:JUPYTER_PATH = $priorPath
}
& $venvPython -c 'from jupyter_client.kernelspec import KernelSpecManager; assert "rust" in KernelSpecManager().find_kernel_specs(), "Rust kernel registration failed"'
if ($LASTEXITCODE -ne 0) { throw 'Rust kernel was not found.' }
New-Item -ItemType File -Force '.venv\.rust-for-ai-ready' | Out-Null
Write-Host 'Setup complete. Start notebooks anytime with: .\start.ps1'
