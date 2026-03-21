#!/bin/bash

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing dayai...${NC}"

# Install binary to ~/.cargo/bin/
if ! cargo install --path . --bin dayai 2>/dev/null; then
    echo -e "${RED}Error: Installation failed. Please check Rust environment.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ dayai installed successfully${NC}"

# Get dayai path
DAYAI_PATH="$HOME/.cargo/bin/dayai"

# Check if PATH already includes ~/.cargo/bin
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    # Add to bashrc
    if ! grep -q "# dayai" ~/.bashrc 2>/dev/null; then
        echo "" >> ~/.bashrc
        echo "# dayai" >> ~/.bashrc
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
        echo -e "${GREEN}✓ Added ~/.cargo/bin to PATH${NC}"
    fi
else
    echo -e "${GREEN}✓ PATH is correct${NC}"
fi

# Run dayai setup
echo ""
echo -e "${YELLOW}Enter GEMINI_API_KEY:${NC}"
if ! dayai setup; then
    echo -e "${RED}Error: Failed to configure API Key${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Installation complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Please close and reopen your terminal, or run:"
echo -e "  ${YELLOW}exec bash${NC}"
echo ""
echo -e "Then you can use:"
echo -e "  ${YELLOW}dayai main${NC}    # Run main function"
echo -e "  ${YELLOW}dayai update${NC}  # Update version"
echo ""
