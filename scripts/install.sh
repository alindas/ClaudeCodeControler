#!/bin/bash
# Claude Code Monitor 安装脚本 (macOS/Linux)
# 一键安装所有依赖并构建

set -e

echo "=== Claude Code Monitor 安装脚本 ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check Node.js
echo -n "Checking Node.js..."
if ! command -v node &> /dev/null; then
    echo -e " ${RED}NOT FOUND${NC}"
    echo "Installing Node.js..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install node
        else
            echo -e "${RED}Please install Homebrew first: https://brew.sh${NC}"
            exit 1
        fi
    else
        # Linux
        curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
        sudo apt-get install -y nodejs
    fi

    echo -e "${GREEN}Node.js installed!${NC}"
else
    echo -e " ${GREEN}$(node --version)${NC}"
fi

# Check Rust
echo -n "Checking Rust..."
if ! command -v rustc &> /dev/null; then
    echo -e " ${RED}NOT FOUND${NC}"
    echo "Please install Rust:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
else
    echo -e " ${GREEN}$(rustc --version)${NC}"
fi

# Install npm dependencies
echo ""
echo "Installing npm dependencies..."
npm install

# Install Rust dependencies
echo ""
echo "Installing Rust dependencies..."
cd src-tauri
cargo fetch
cd ..

# Create placeholder icons
echo ""
echo "Creating placeholder icons..."
mkdir -p src-tauri/icons
echo "This is a placeholder. Run 'npm run tauri icon path/to/icon.png' to generate real icons." > src-tauri/icons/README.txt

echo ""

# Parse arguments
if [ "$1" == "dev" ]; then
    echo -e "${CYAN}Starting development server...${NC}"
    npm run tauri:dev
elif [ "$1" == "build" ]; then
    echo -e "${CYAN}Building release version...${NC}"
    npm run tauri:build
    echo ""
    echo -e "${GREEN}Build complete! Check src-tauri/target/release/bundle/${NC}"
else
    echo -e "${GREEN}=== Installation Complete ===${NC}"
    echo ""
    echo "Next steps:"
    echo "  npm run tauri:dev    # Development mode"
    echo "  npm run tauri:build  # Build release"
fi
