$ErrorActionPreference = 'Stop'

if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    throw 'wasm-pack was not found. Run cargo install wasm-pack in PowerShell, then try again.'
}

$pythonCommand = $null
$pythonArgs = @()
foreach ($candidate in @('py', 'python', 'python3')) {
    $command = Get-Command $candidate -CommandType Application -ErrorAction SilentlyContinue
    if (-not $command) {
        continue
    }

    $candidateArgs = @()
    if ($candidate -eq 'py') {
        $candidateArgs = @('-3')
    }

    & $command.Source @candidateArgs -c 'import sys; sys.exit(0 if sys.version_info.major == 3 else 1)' 2>$null
    if ($LASTEXITCODE -eq 0) {
        $pythonCommand = $command.Source
        $pythonArgs = $candidateArgs
        break
    }
}

if (-not $pythonCommand) {
    throw 'Python 3 was not found. Install Python 3 and make py or python available in PowerShell.'
}

Push-Location (Split-Path -Parent $PSScriptRoot)
try {
    wasm-pack build --dev --target web --out-dir web/pkg --out-name rustbound_idle
    if ($LASTEXITCODE -ne 0) {
        throw "wasm-pack build failed (exit code $LASTEXITCODE)."
    }

    Write-Host "`nServing http://localhost:8080 (Ctrl+C to stop)"
    & $pythonCommand @pythonArgs -m http.server 8080 --bind 127.0.0.1 --directory web
    if ($LASTEXITCODE -ne 0) {
        throw "The development server stopped (exit code $LASTEXITCODE)."
    }
}
finally {
    Pop-Location
}
