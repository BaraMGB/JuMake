# JuMake Windows Installer
# Run this in PowerShell:
# iwr https://raw.githubusercontent.com/BaraMGB/JuMake/main/install.ps1 -useb | iex

$ErrorActionPreference = "Stop"
$Version = "0.1.5"
$InstallDir = "$env:LOCALAPPDATA\JuMake"
$BinaryUrl = "https://github.com/BaraMGB/JuMake/releases/download/v$Version/jumake-windows-x64.zip"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  JuMake Installer for Windows" -ForegroundColor Cyan
Write-Host "  Version: $Version" -ForegroundColor Cyan
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
    Write-Host "ERROR: Download failed. Please check your internet connection." -ForegroundColor Red
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

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Installation complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "JuMake is now installed at: $InstallDir\jumake.exe" -ForegroundColor Cyan
Write-Host ""
Write-Host "IMPORTANT: Restart your terminal or run:" -ForegroundColor Yellow
Write-Host "  refreshenv" -ForegroundColor Yellow
Write-Host ""
Write-Host "Or open a new PowerShell/CMD window." -ForegroundColor Yellow
Write-Host ""
Write-Host "Then try: jumake --version" -ForegroundColor Cyan
Write-Host ""
