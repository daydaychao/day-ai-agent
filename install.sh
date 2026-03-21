#!/bin/bash

set -e

echo "正在安裝 dayai..."

# 安裝 binary 到 ~/.cargo/bin/
cargo install --path . --bin dayai

# 取得 dayai 的實際路徑
DAYAI_PATH="$HOME/.cargo/bin/dayai"

# 檢查是否已加入 PATH
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    # 加入 bashrc
    if ! grep -q "# dayai" ~/.bashrc 2>/dev/null; then
        echo "" >> ~/.bashrc
        echo "# dayai" >> ~/.bashrc
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
        echo ""
        echo "已將 ~/.cargo/bin 加入 PATH"
        echo "請執行以下指令生效："
        echo "  source ~/.bashrc"
    fi
else
    echo "PATH 設定正確"
fi

echo ""
echo "安裝完成！執行以下指令開始使用："
echo "  source ~/.bashrc"
echo "  dayai setkey"
