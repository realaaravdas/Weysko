$ErrorActionPreference = 'Stop'

Write-Host "1/2 Building Websir Studio release binary..." -ForegroundColor Cyan
cargo build --release -p websir

Write-Host "2/2 Building Websir Setup Installer release binary..." -ForegroundColor Cyan
cargo build --release -p websir-installer

$appExe = Resolve-Path ".\target\release\websir.exe"
$setupExe = Resolve-Path ".\target\release\websir-setup.exe"

Write-Host "`nBuilt artifacts:" -ForegroundColor Green
Get-Item $appExe, $setupExe | Select-Object Name, Length, FullName | Format-Table -AutoSize
Get-FileHash -Algorithm SHA256 $appExe, $setupExe | Format-Table -AutoSize
