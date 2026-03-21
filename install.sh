#!/bin/bash

set -e

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}正在安裝 dayai...${NC}"

# 安裝 binary 到 ~/.cargo/bin/
if ! cargo install --path . --bin dayai 2>/dev/null; then
    echo -e "${RED}Error: 安裝失敗，請檢查 Rust 環境${NC}"
    exit 1
fi

echo -e "${GREEN}✓ dayai 安裝成功${NC}"

# 取得 dayai 的實際路徑
DAYAI_PATH="$HOME/.cargo/bin/dayai"

# 檢查是否已加入 PATH
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    # 加入 bashrc
    if ! grep -q "# dayai" ~/.bashrc 2>/dev/null; then
        echo "" >> ~/.bashrc
        echo "# dayai" >> ~/.bashrc
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
        echo -e "${GREEN}✓ 已將 ~/.cargo/bin 加入 PATH${NC}"
    fi
else
    echo -e "${GREEN}✓ PATH 設定正確${NC}"
fi

# 執行 dayai setup
echo ""
echo -e "${YELLOW}請輸入 GEMINI_API_KEY：${NC}"
if ! dayai setup; then
    echo -e "${RED}Error: API Key 設定失敗${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  安裝完成！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "請關閉並重新開啟 terminal，或執行："
echo -e "  ${YELLOW}exec bash${NC}"
echo ""
echo -e "然後就可以使用："
echo -e "  ${YELLOW}dayai main${NC}    # 執行主要功能"
echo -e "  ${YELLOW}dayai models${NC}  # 列出模型"
echo -e "  ${YELLOW}dayai update${NC}  # 更新版本"
echo ""
