# Claude Code Monitor 安装脚本 (Windows)
# 一键安装所有依赖并构建

param(
    [switch]$Dev,
    [switch]$Build
)

$ErrorActionPreference = "Stop"

Write-Host "=== Claude Code Monitor 安装脚本 ===" -ForegroundColor Cyan
Write-Host ""

# Check Node.js
Write-Host "Checking Node.js..." -NoNewline
if (!(Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host " NOT FOUND" -ForegroundColor Red
    Write-Host "Installing Node.js..."

    $nodeUrl = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-x64.msi"
    $msiPath = "$env:TEMP\nodejs.msi"

    Invoke-WebRequest -Uri $nodeUrl -OutFile $msiPath
    Start-Process -Wait -FilePath "msiexec.exe" -ArgumentList "/i", $msiPath, "/quiet", "/norestart"
    Remove-Item $msiPath

    # Refresh PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")

    Write-Host "Node.js installed!" -ForegroundColor Green
} else {
    $nodeVersion = node --version
    Write-Host " $nodeVersion" -ForegroundColor Green
}

# Check Rust
Write-Host "Checking Rust..." -NoNewline
if (!(Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host " NOT FOUND" -ForegroundColor Red
    Write-Host "Please install Rust from https://rustup.rs/"
    exit 1
} else {
    $rustVersion = rustc --version
    Write-Host " $rustVersion" -ForegroundColor Green
}

# Install npm dependencies
Write-Host ""
Write-Host "Installing npm dependencies..."
npm install

# Install Rust dependencies
Write-Host ""
Write-Host "Installing Rust dependencies..."
cd src-tauri
cargo fetch
cd ..

# Build icons placeholder
Write-Host ""
Write-Host "Creating placeholder icons..."
$iconDir = "src-tauri\icons"
if (!(Test-Path $iconDir)) {
    New-Item -ItemType Directory -Path $iconDir -Force | Out-Null
}

# Generate simple icon files (will be replaced by tauri icon command)
@'
This is a placeholder icon file.
Run `npm run tauri icon path/to/icon.png` to generate real icons.
'@ | Out-File -FilePath "$iconDir\icon.png.placeholder" -Encoding ASCII

Write-Host ""

if ($Dev) {
    Write-Host "Starting development server..." -ForegroundColor Cyan
    npm run tauri:dev
} elseif ($Build) {
    Write-Host "Building release version..." -ForegroundColor Cyan
    npm run tauri:build
    Write-Host ""
    Write-Host "Build complete! Check src-tauri/target/release/bundle/" -ForegroundColor Green
} else {
    Write-Host "=== Installation Complete ===" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:"
    Write-Host "  npm run tauri:dev    # Development mode"
    Write-Host "  npm run tauri:build  # Build release"
}
