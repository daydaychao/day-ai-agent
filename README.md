# 🦞 Project: Cloud Lobster (Rust Edition)

[![Latest Release](https://img.shields.io/github/v/release/daydaychao/day-ai-agent?color=green&label=latest)](https://github.com/daydaychao/day-ai-agent/releases/latest)

> **Goal:** Build a 24/7 AI agent using GitHub Actions free compute

## 🚀 Installation & Usage

### Install dayai CLI

#### Option 1: Using install script (Recommended)

```bash
git clone https://github.com/daydaychao/day-ai-agent.git
cd day-ai-agent
./install.sh
```

#### Option 2: Manual Installation

```bash
cargo install --path .
dayai setup
```

#### Option 3: From GitHub (Released versions)

```bash
cargo install --git https://github.com/daydaychao/day-ai-agent --bin dayai
dayai setup
```

### CLI Usage

```bash
dayai setup              # First time setup: API Key and model selection
dayai agent              # Execute agent (call Gemini API)
dayai agent --prompt "..."  # Custom prompt
dayai update            # Update dayai to latest version
dayai update --version v0.0.2 # Update to specific version
```

## 🎯 Learning Roadmap

### Phase 1: Cloud Infrastructure (GitHub Actions)

- [x] **Workflow Triggers**: Understanding `workflow_dispatch` (manual) vs `schedule` (cron).
- [x] **Environment Isolation**: Learning to set up clean execution environment on `ubuntu-latest`.
- [x] **Security Management**: Properly using `GitHub Secrets` for Gemini API Key.

### Phase 2: Agent Brain (Rust & AI)

- [x] **Async Requests**: Using `tokio` + `reqwest` to call Gemini API.
- [x] **Structured Dialogue**: Using `serde_json` to handle AI responses for stable logic.
- [x] **Prompt Engineering**: Designing "command-style" prompts for task execution mode.
- [x] **CLI Tool**: Using `clap` to implement subcommands (agent, update, setup).

### Phase 3: Automation Practice

- [ ] **Information Monitoring**: Writing scrapers to periodically fetch website info (jobs, tech news).
- [ ] **Automated Flow**: AI processes info and sends notifications via Discord Webhook.
- [ ] **Continuous Evolution**: Using GitHub repo as database to store state.

## 🛠 Current Status

- [x] GitHub Public Repo established.
- [x] Gemini API Key configuration (`dayai setup`).
- [x] `dayai` CLI tool complete with `agent`, `update`, `setup` subcommands.
- [ ] Phase 3: Information monitoring + Discord/State automation not yet implemented.

## 📦 Release Process

1. Commit your changes:
   ```bash
   git add . && git commit -m "feat: new feature"
   ```

2. Tag your version:
   ```bash
   git tag v0.0.1
   ```

3. Push code and tags:
   ```bash
   git push && git push --tags
   ```

4. GitHub Actions will automatically:
   - Build `dayai` binary
   - Publish to GitHub Releases

## 💰 Free Tier Optimization Tips

- Leverage GitHub's 2,000 free minutes per month.
- Use Rust compiled binaries to keep execution under 10 seconds.
- Take advantage of Gemini 1.5 Flash free API quota.

## If You Are An AGENT

- Read AGENTS.md
