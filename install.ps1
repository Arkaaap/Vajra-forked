# -----------------------------
# Rust Project Installer Script (Windows)
# -----------------------------

$BINARY_NAME = "vajra.exe"
$INSTALL_DIR = "C:\Program Files\Vajra"

Write-Host "Building project in release mode..." -ForegroundColor Green
cargo build --release

$SOURCE_PATH = "target\release\$BINARY_NAME"

if (!(Test-Path $SOURCE_PATH)) {
    Write-Host "Error: Binary not found!" -ForegroundColor Red
    Write-Host "Make sure BINARY_NAME is correct." -ForegroundColor Red
    exit 1
}

# Create install directory if it doesn't exist
if (!(Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force
}

Write-Host "Installing binary to $INSTALL_DIR..." -ForegroundColor Green
Copy-Item $SOURCE_PATH $INSTALL_DIR -Force

# Add to PATH (if not already added)
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")

if ($currentPath -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        $currentPath + ";$INSTALL_DIR",
        "Machine"
    )
    Write-Host "Added $INSTALL_DIR to system PATH." -ForegroundColor Yellow
}

Write-Host "************0************" -ForegroundColor Green
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "You can now run the program from anywhere using:"
Write-Host "vajra"
Write-Host "Basic syntax: vajra scan -t domain.com" -ForegroundColor Yellow

Write-Host "For more extensive scan please visit:"
Write-Host "https://git.vulntech.com/mayur/Vajra/src/branch/master/COMMANDS.md"