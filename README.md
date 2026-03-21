# 🦞 Project: Cloud Lobster (Rust Edition)

> **目標：** 復刻「小龍蝦」核心邏輯，利用 GitHub Actions 免費算力打造 24h 雲端代理人。

## 🎯 核心學習路徑 (Learning Roadmap)

### Phase 1: 雲端基礎設施 (GitHub Actions)

- [ ] **Workflow 觸發機制**：理解 `workflow_dispatch` (手動) 與 `schedule` (定時，Cron Job) 的差異。
- [ ] **環境隔離**：學習如何在 `ubuntu-latest` 虛擬機中建立乾淨的執行環境。
- [ ] **資安管理**：正確使用 `GitHub Secrets` 存放 Gemini API Key 與 Discord Webhook。

### Phase 2: 代理人大腦 (Rust & AI)

- [ ] **非同步請求**：使用 `tokio` + `reqwest` 呼叫 Google Gemini API。
- [ ] **結構化對話**：利用 `serde_json` 處理 AI 回傳的 JSON，確保邏輯穩定。
- [ ] **提示詞工程 (Prompting)**：設計「指令式」Prompt，讓 AI 從聊天模式轉向「任務執行」模式。

### Phase 3: 自動化任務實戰

- [ ] **資訊監控**：撰寫爬蟲腳本，定時抓取特定網站資訊 (如職缺、技術新聞)。
- [ ] **自動化流程**：AI 處理完資訊後，透過 Webhook 自動發送通知至 Discord。
- [ ] **持續進化**：利用 GitHub 倉庫當作資料庫，儲存上次執行的狀態 (State)。

## 🛠 目前開發狀態 (Current Status)

- [x] GitHub Private Repo 建立完畢。
- [x] Gemini API Key (Google AI Studio) 配置完成。
- [x] Python 版本 PoC 驗證成功。
- [x] 使用 Rust 重構 `lobster-brain` 並導入 `rust-cache` 優化執行速度。
- [ ] **Next Step**: 實作 `tokio` + `reqwest` 來串接 Gemini API (Phase 2)。

## 💰 資深免費仔省錢秘笈

- 利用 GitHub 每月 2,000 分鐘算力。
- 使用 Rust 編譯二進制檔，將單次執行時間壓低在 10 秒內。
- 善用 Gemini 1.5 Flash 的免費 API 配額。
