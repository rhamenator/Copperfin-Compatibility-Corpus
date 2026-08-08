[CmdletBinding()]
param(
    [string]$CopperfinRoot = 'E:\Project-Copperfin'
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$repository = Split-Path -Parent $PSScriptRoot
$runtimeHost = Join-Path $CopperfinRoot 'build\Release\copperfin_runtime_host.exe'

if (-not (Test-Path -LiteralPath $runtimeHost -PathType Leaf)) {
    throw "Copperfin runtime host was not found: $runtimeHost"
}

function Invoke-CorpusContract {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string[]]$Sources,
        [Parameter(Mandatory = $true)][string]$StartupItem
    )

    $scratch = Join-Path ([System.IO.Path]::GetTempPath()) ("copperfin-corpus-$Name-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $scratch | Out-Null
    try {
        foreach ($source in $Sources) {
            Copy-Item -LiteralPath (Join-Path $repository "corpus\vfp\$source") -Destination $scratch
        }

        $startupSource = Join-Path $scratch $StartupItem
        $manifestPath = Join-Path $scratch 'app.cfmanifest'
        @(
            'manifest_version=1'
            "project_title=COPPERFIN_CORPUS_$($Name.ToUpperInvariant())"
            "project_path=$startupSource"
            "package_root=$scratch"
            "content_root=$scratch"
            "working_directory=$scratch"
            "startup_item=$StartupItem"
            "startup_source=$startupSource"
            'configuration=debug'
            'security_enabled=false'
            'security_mode=off'
            'dotnet_enabled=false'
            'dotnet_story='
        ) | Set-Content -LiteralPath $manifestPath -Encoding utf8

        $output = & $runtimeHost --manifest $manifestPath 2>&1
        if ($LASTEXITCODE -ne 0) {
            throw "Copperfin contract '$Name' failed with exit code $LASTEXITCODE.`n$($output -join [Environment]::NewLine)"
        }
        if ($output -notcontains 'runtime.completed: true') {
            throw "Copperfin contract '$Name' did not report completion.`n$($output -join [Environment]::NewLine)"
        }
        if ($output -notcontains 'warning.count: 0') {
            throw "Copperfin contract '$Name' reported warnings.`n$($output -join [Environment]::NewLine)"
        }

        Write-Host "Copperfin contract passed: $Name" -ForegroundColor Green
    }
    finally {
        if (Test-Path -LiteralPath $scratch) {
            Remove-Item -LiteralPath $scratch -Recurse -Force
        }
    }
}

Invoke-CorpusContract -Name 'geodesy' -Sources @(
    'libfunct_updated.prg',
    'geodesy_contract.prg'
) -StartupItem 'geodesy_contract.prg'

Invoke-CorpusContract -Name 'graph' -Sources @(
    'matchprg_updated.prg',
    'graph_contract.prg'
) -StartupItem 'graph_contract.prg'
