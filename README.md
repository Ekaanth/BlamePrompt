# BlamePrompt

**Track AI-generated code provenance via Git Notes. 100% local. Zero data collection. No API key required.**

BlamePrompt is an open-source CLI tool that automatically tracks which parts of your codebase were written by AI. It hooks into Claude Code (and other AI coding tools) to create tamper-evident receipts stored entirely in your local Git repository using Git Notes.

## Privacy & Data Policy

- **No API key required** -- BlamePrompt works out of the box with zero configuration
- **No data collection** -- We do not collect, transmit, or store any of your data. Ever.
- **No telemetry** -- No usage analytics, no phone-home, no tracking of any kind
- **No external dependencies** -- Everything runs locally on your machine
- **100% offline** -- Works without an internet connection (except `push`/`pull` for team sharing)
- **Your data stays yours** -- All receipts, prompts, and audit trails are stored in your Git repo as Git Notes (`refs/notes/blameprompt`) and optionally in a local SQLite cache at `~/.blameprompt/prompts.db`
- **Built-in redaction** -- Secrets and sensitive data are automatically stripped from prompts before storage using pattern matching and Shannon entropy detection

## Quick Start

```bash
# Install from source
cargo install --path .

# Initialize in your repo
cd your-project
blameprompt init

# That's it. BlamePrompt now automatically captures AI code receipts
# every time you use Claude Code in this repo.
```

## Why BlamePrompt?

As AI coding tools become ubiquitous, organizations need to answer critical questions:

- **Who** used AI to generate code, and **when**?
- **What** prompts were used, and **which model** generated the code?
- **How much** is AI coding costing us?
- Are we **license-compliant** with the AI models we use?
- Did AI-generated code introduce **security vulnerabilities**?
- Was **sensitive data** accidentally sent to AI providers?

BlamePrompt answers all of these automatically, without changing your workflow.

## How It Works

1. **Claude Code hooks** (PreToolUse/PostToolUse) intercept AI coding sessions automatically
2. **Receipts** are generated for each AI code change -- capturing model, prompt, cost, lines, timestamp, and full conversation chain of thought
3. A **redaction engine** strips API keys, passwords, tokens, and high-entropy secrets before anything is stored
4. Receipts are held in a **staging area** until you commit
5. On `git commit`, receipts are attached as **Git Notes** (`refs/notes/blameprompt`) -- living alongside your commits, not polluting your files
6. **30+ analysis commands** let you audit, report, scan, and comply at any time

## All Commands

### Setup & Configuration

| Command | Description |
|---------|-------------|
| `blameprompt init` | Initialize BlamePrompt hooks in the current repo |
| `blameprompt init --global` | Configure git template so all future repos are auto-initialized |
| `blameprompt install-hooks` | Install Claude Code hooks + git post-commit hook |
| `blameprompt uninstall` | Remove all BlamePrompt hooks and data from the repo |
| `blameprompt uninstall --purge` | Remove everything including Git Notes history |

### Code Analysis & Attribution

| Command | Description |
|---------|-------------|
| `blameprompt blame <file>` | Line-by-line AI vs human attribution for any file |
| `blameprompt show <commit>` | Display all AI receipts attached to a specific commit (supports short SHA) |
| `blameprompt search <query>` | Full-text search across all stored prompts, files, and models |
| `blameprompt search <query> --limit 100` | Search with custom result limit (default: 50) |

### Audit & Reporting

| Command | Description |
|---------|-------------|
| `blameprompt audit` | Generate full audit trail as markdown (default output: `blameprompt-audit.md`) |
| `blameprompt audit --format json` | Export audit trail as JSON |
| `blameprompt audit --format csv` | Export audit trail as CSV |
| `blameprompt audit --format table` | Display audit trail as a CLI table |
| `blameprompt audit --from 2026-01-01 --to 2026-02-15` | Filter by date range |
| `blameprompt audit --author "Alice"` | Filter by commit author |
| `blameprompt audit --include-uncommitted` | Include staged/uncommitted receipts |
| `blameprompt analytics` | Show aggregated AI usage statistics (sessions, costs, models, users) |
| `blameprompt analytics --export json` | Export analytics as JSON |
| `blameprompt analytics --export csv` | Export analytics as CSV |
| `blameprompt report` | Generate comprehensive markdown report with all sections |
| `blameprompt report --output ./my-report.md` | Custom output path |

### Compliance & Regulatory

| Command | Description |
|---------|-------------|
| `blameprompt license-scan` | Scan all AI models used against a license restriction database (Llama MAU limits, Codestral non-production, Apache 2.0 attribution, etc.) |
| `blameprompt soc2` | Generate SOC 2 Type II / ISO 27001 compliance evidence package |
| `blameprompt soc2 --from 2026-Q1 --to 2026-Q1` | SOC2 report for a specific audit period |
| `blameprompt gdpr` | Generate GDPR Data Processing Impact Assessment (DPIA) with PII detection and data flow mapping |

### Security Scanning

| Command | Description |
|---------|-------------|
| `blameprompt supply-chain-risk` | Calculate AI supply chain risk score (0-10) based on model diversity, cloud exposure, prompt sensitivity, and critical file access |
| `blameprompt vuln-scan` | Static analysis (SAST) of AI-generated code for vulnerabilities -- command injection, SQL injection, XSS, path traversal, hardcoded creds, insecure deserialization, and more (10 CWE patterns) |
| `blameprompt prompt-injection` | Detect prompt injection and backdoor patterns in AI-generated code -- eval/exec backdoors, base64 payloads, data exfiltration, obfuscated strings, suspicious AI responses |
| `blameprompt secret-rotation` | Identify secrets that may have been sent to cloud AI providers and generate per-secret rotation instructions |
| `blameprompt redact --test <file>` | Dry-run the redaction engine on any file to see what secrets would be caught |

### Cost Management

| Command | Description |
|---------|-------------|
| `blameprompt budget` | Analyze AI usage costs with per-user, per-model, and monthly trend breakdowns |
| `blameprompt budget --monthly-limit 500` | Set monthly budget with alerts at 80% and 100% thresholds |
| `blameprompt budget --quarterly-limit 1200` | Set quarterly budget limit |
| `blameprompt model-recommend` | Analyze usage patterns and recommend cheaper models where appropriate -- shows potential savings and model tier guide |

### Enterprise

| Command | Description |
|---------|-------------|
| `blameprompt enterprise status` | Show enterprise configuration status |
| `blameprompt enterprise sync` | Sync local receipts to enterprise server |
| `blameprompt enterprise sync --from 2026-01-01` | Sync with date filter |
| `blameprompt enterprise export` | Export all receipts as a JSON bundle for ingestion into external systems |
| `blameprompt enterprise export --author "Alice"` | Export filtered by author |
| `blameprompt enterprise policy-check` | Check staged receipts against organization-wide policies |

### Team Collaboration

| Command | Description |
|---------|-------------|
| `blameprompt push` | Push all BlamePrompt notes to your Git remote |
| `blameprompt pull` | Fetch BlamePrompt notes from your Git remote |
| `blameprompt cache sync` | Populate local SQLite cache from Git Notes for fast queries on large repos |

### Manual Tagging & Import

| Command | Description |
|---------|-------------|
| `blameprompt tag <file> --start-line 10 --end-line 50 --provider chatgpt --model gpt-4o --prompt "..."` | Manually tag code as AI-generated (for non-Claude tools) |
| `blameprompt record --session <path.jsonl>` | Import a Claude Code JSONL transcript file and generate receipts |
| `blameprompt record --session <path> --provider openai` | Import with custom provider name |
| `blameprompt replit-detect` | Auto-detect Replit Agent changes in unstaged files |

### Internal

| Command | Description |
|---------|-------------|
| `blameprompt checkpoint <agent>` | Called automatically by Claude Code hooks -- not for manual use |

## Configuration

Create a `.blamepromptrc` file in your repo root or `~/.blamepromptrc` for global settings:

```toml
# Redaction settings
[redaction]
mode = "replace"  # "replace" (default) or "hash" (SHA-256 prefix)

# Add custom secret patterns
[[redaction.custom_patterns]]
pattern = "INTERNAL-\\d{6}"
replacement = "[REDACTED_INTERNAL_ID]"

[[redaction.custom_patterns]]
pattern = "(?i)employee_id\\s*=\\s*\\d+"
replacement = "[REDACTED_EMPLOYEE_ID]"

# Disable built-in patterns you don't need
[redaction]
disable_patterns = ["BEARER_TOKEN"]

# Capture settings
[capture]
max_prompt_length = 2000        # Max characters per prompt summary
store_full_conversation = true  # Capture full chain of thought

# Enterprise settings (optional)
[enterprise]
enabled = true
api_url = "https://blameprompt.your-company.com"
api_key_env = "BLAMEPROMPT_API_KEY"
sync_on_commit = true
org = "your-org"
team = "backend-team"
```

## Supported AI Providers

| Provider | Models | Detection Method |
|----------|--------|-----------------|
| Anthropic | Claude Opus 4.x, Sonnet 4.x, Haiku 4.x, Haiku 3.5 | Automatic via Claude Code hooks |
| OpenAI | GPT-4o, GPT-4, GPT-3.5, o1, o3 | Manual (`tag` or `record`) |
| Meta | Llama 3.x, Code Llama | Manual (`tag` or `record`) |
| Mistral AI | Mistral, Mixtral, Codestral | Manual (`tag` or `record`) |
| DeepSeek | DeepSeek Coder v2 | Manual (`tag` or `record`) |
| Google | Gemini, Gemma | Manual (`tag` or `record`) |
| Microsoft | Phi-3, Phi-4 | Manual (`tag` or `record`) |
| Alibaba | Qwen, CodeQwen | Manual (`tag` or `record`) |
| Cohere | Command R | Manual (`tag` or `record`) |
| Replit | Replit Agent | Auto-detect (`replit-detect`) |
| Local models | Ollama, LM Studio, any local inference | Manual (`tag` or `record`) |

## Architecture

```
Claude Code Session
        |
        v
  Hook Interceptor (PreToolUse / PostToolUse)
        |
        v
  JSONL Transcript Parser
        |
        v
  Redaction Engine
  (API keys, passwords, tokens, high-entropy strings, custom patterns)
        |
        v
  Receipt Generator
  (UUID, model, cost estimate, line ranges, conversation turns, prompt hash)
        |
        v
  Staging Area (.blameprompt/staging.json)
        |
   git commit
        |
        v
  Git Notes (refs/notes/blameprompt)
        |
        v
  30+ Analysis Commands
  (blame, audit, report, compliance, security, cost, enterprise)
```

### Where Data Is Stored

| Data | Location | Notes |
|------|----------|-------|
| AI receipts | `refs/notes/blameprompt` (Git Notes) | Travels with your repo, shareable via `push`/`pull` |
| Staging area | `.blameprompt/staging.json` | Gitignored, cleared on commit |
| SQLite cache | `~/.blameprompt/prompts.db` | Optional read cache for fast queries on large repos |
| Configuration | `.blamepromptrc` or `~/.blamepromptrc` | TOML format |

**Nothing is ever sent externally.** All processing happens on your machine.

## Building from Source

```bash
# Prerequisites: Rust toolchain (rustup.rs)
git clone https://github.com/anthropics/blameprompt.git
cd blameprompt
cargo build --release
cargo install --path .
```

## License

MIT
