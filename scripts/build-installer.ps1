$ErrorActionPreference = 'Stop'

Write-Host "1/2 Building Weysko Studio release binary..." -ForegroundColor Cyan
cargo build --release -p weysko

Write-Host "2/2 Building Weysko Setup Installer release binary..." -ForegroundColor Cyan
cargo build --release -p weysko-installer

$appExe = Resolve-Path ".\target\release\weysko.exe"
$setupExe = Resolve-Path ".\target\release\weysko-setup.exe"

Write-Host "`nBuilt artifacts:" -ForegroundColor Green
Get-Item $appExe, $setupExe | Select-Object Name, Length, FullName | Format-Table -AutoSize
Get-FileHash -Algorithm SHA256 $appExe, $setupExe | Format-Table -AutoSize
