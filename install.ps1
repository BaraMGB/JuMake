# JuMake Windows Installer
# Run this in PowerShell:
# iwr https://raw.githubusercontent.com/BaraMGB/JuMake/main/install.ps1 -useb | iex

$ErrorActionPreference = "Stop"
$InstallDir = "$env:LOCALAPPDATA\JuMake"
$BinaryUrl = "https://github.com/BaraMGB/JuMake/releases/latest/download/jumake-windows-x64.zip"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  JuMake Installer for Windows" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Create install directory
if (Test-Path $InstallDir) {
    Write-Host "Removing old installation..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force $InstallDir
}

New-Item -ItemType Directory -Path $InstallDir | Out-Null
Write-Host "[1/3] Created install directory: $InstallDir" -ForegroundColor Green

# Download
Write-Host "[2/3] Downloading JuMake..." -ForegroundColor Green
$ZipFile = "$InstallDir\jumake.zip"
try {
    Invoke-WebRequest -Uri $BinaryUrl -OutFile $ZipFile -UseBasicParsing
} catch {
    Write-Host "ERROR: Download failed." -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
}

# Extract
Write-Host "[3/3] Extracting..." -ForegroundColor Green
Expand-Archive -Path $ZipFile -DestinationPath $InstallDir -Force
Remove-Item $ZipFile

# Add to PATH (user level)
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    Write-Host ""
    Write-Host "Added JuMake to your PATH." -ForegroundColor Green
}

# Get version from binary
$Version = & "$InstallDir\jumake.exe" --version 2>&1 | Select-Object -First 1

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Installation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Installed: $Version" -ForegroundColor Cyan
Write-Host "Location:  $InstallDir\jumake.exe" -ForegroundColor Cyan
Write-Host ""
Write-Host "IMPORTANT: Restart your terminal or open a new" -ForegroundColor Yellow
Write-Host "PowerShell/CMD window, then run:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  jumake --version" -ForegroundColor Cyan
Write-Host ""
