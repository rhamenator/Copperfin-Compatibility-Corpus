[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$repository = Split-Path -Parent $PSScriptRoot

Push-Location $repository
try {
    cargo fmt --check
    cargo test --all-targets
    cargo clippy --all-targets -- -D warnings
    cargo run --quiet --bin corpus-runner -- demo-route
}
finally {
    Pop-Location
}
