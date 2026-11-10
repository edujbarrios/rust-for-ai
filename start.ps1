$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot
if (-not (Test-Path '.venv\.rust-for-ai-ready') -or -not (Test-Path '.venv\Scripts\jupyter.exe')) {
    & (Join-Path $PSScriptRoot 'scripts\setup.ps1')
    if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) { throw 'Setup failed.' }
}
& .\.venv\Scripts\jupyter.exe lab --notebook-dir "$PSScriptRoot"
if ($LASTEXITCODE -ne 0) { throw 'JupyterLab failed to start.' }
