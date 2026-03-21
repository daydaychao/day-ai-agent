# 🦞 Project: Cloud Lobster (Rust Edition)

> **目標：** 利用 GitHub Actions 免費算力打造 24h AI 代理人

## 🚀 安裝與使用

### 安裝 dayai CLI

#### 方式一：使用安裝腳本（推薦）

```bash
git clone https://github.com/daydaychao/day-ai-agent.git
cd day-ai-agent
./install.sh
```

#### 方式二：手動安裝

```bash
cargo install --path .
dayai setup
```

#### 方式三：從 GitHub 安裝（已發布版本）

```bash
cargo install --git https://github.com/daydaychao/day-ai-agent --bin dayai
dayai setup
```

### CLI 使用方式

```bash
dayai setup              # 首次設定：API Key 和選擇模型
dayai main               # 執行主要邏輯（呼叫 Gemini API）
dayai main --prompt "..."  # 自定義 prompt
dayai update             # 更新 dayai 到最新版本
dayai update --version v0.0.2 # 更新到指定版本
```

## 🎯 核心學習路徑 (Learning Roadmap)

### Phase 1: 雲端基礎設施 (GitHub Actions)

- [x] **Workflow 觸發機制**：理解 `workflow_dispatch` (手動) 與 `schedule` (定時，Cron Job) 的差異。
- [x] **環境隔離**：學習如何在 `ubuntu-latest` 虛擬機中建立乾淨的執行環境。
- [x] **資安管理**：正確使用 `GitHub Secrets` 存放 Gemini API Key。

### Phase 2: 代理人大腦 (Rust & AI)

- [x] **非同步請求**：使用 `tokio` + `reqwest` 呼叫 Gemini API。
- [x] **結構化對話**：利用 `serde_json` 處理 AI 回傳的 JSON，確保邏輯穩定。
- [x] **提示詞工程 (Prompting)**：設計「指令式」Prompt，讓 AI 從聊天模式轉向「任務執行」模式。
- [x] **CLI 工具**：使用 `clap` 實作 subcommands（main、update、setup）。

### Phase 3: 自動化任務實戰

- [ ] **資訊監控**：撰寫爬蟲腳本，定時抓取特定網站資訊 (如職缺、技術新聞)。
- [ ] **自動化流程**：AI 處理完資訊後，透過 Webhook 自動發送通知至 Discord。
- [ ] **持續進化**：利用 GitHub 倉庫當作資料庫，儲存上次執行的狀態 (State)。

## 🛠 目前開發狀態 (Current Status)

- [x] GitHub Public Repo 建立完畢。
- [x] Gemini API Key 設定功能（`dayai setup`）。
- [x] `dayai` CLI 工具完成，支援 `main`、`update`、`setup` subcommands。
- [ ] Phase 3：資訊監控 + Discord/State 自動化流程尚未實作。

## 📦 版本發布流程

1. 確認程式碼改好後 commit：
   ```bash
   git add . && git commit -m "feat: 新功能"
   ```

2. 打上 version tag：
   ```bash
   git tag v0.0.1
   ```

3. 推送 code + tag：
   ```bash
   git push && git push --tags
   ```

4. GitHub Actions 會自動：
   - 編譯 `dayai` binary
   - 發布到 GitHub Releases

## 💰 資深免費仔省錢秘笈

- 利用 GitHub 每月 2,000 分鐘算力。
- 使用 Rust 編譯二進制檔，將單次執行時間壓低在 10 秒內。
- 善用 Gemini 1.5 Flash 的免費 API 配額。

## 如果你是AGENT

- 讀取AGENTS.md
