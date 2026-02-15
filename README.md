# BlamePrompt

**Track AI-generated code provenance via Git Notes.**

BlamePrompt is a CLI tool that automatically tracks which parts of your codebase were written by AI. It hooks into Claude Code (and other AI coding tools) to create tamper-evident receipts stored in Git Notes -- no API key needed.

## Quick Start

```bash
# Install
cargo install --path .

# Initialize in your repo
cd your-project
blameprompt init

# That's it. BlamePrompt now automatically captures AI code receipts.
```

## How It Works

1. **Claude Code hooks** intercept AI coding sessions (PreToolUse/PostToolUse)
2. **Receipts** are generated for each AI code change (model, prompt, cost, lines, timestamp)
3. Receipts are stored in **Git Notes** (`refs/notes/blameprompt`) -- attached to commits, not files
4. A **redaction engine** strips secrets before storing prompts
5. Full **chain of thought** (conversation turns) is captured for audit trails

## Commands

### Core

| Command | Description |
|---------|-------------|
| `blameprompt init` | Initialize BlamePrompt hooks in the current repo |
| `blameprompt init --global` | Configure git template for all future repos |
| `blameprompt install-hooks` | Install Claude Code + git hooks (legacy) |
| `blameprompt uninstall` | Remove all hooks and data |
| `blameprompt checkpoint <agent>` | Internal: called by Claude Code hooks to capture receipts |

### Analysis

| Command | Description |
|---------|-------------|
| `blameprompt blame <file>` | Line-by-line AI/human attribution for a file |
| `blameprompt show <commit>` | Display all AI receipts attached to a specific commit |
| `blameprompt search <query>` | Full-text search across stored prompts |
| `blameprompt audit` | Full audit trail (outputs markdown by default) |
| `blameprompt analytics` | Aggregated AI usage statistics |
| `blameprompt report` | Comprehensive markdown report |

### Compliance & Security

| Command | Description |
|---------|-------------|
| `blameprompt license-scan` | Scan AI model licenses for compliance issues |
| `blameprompt soc2` | Generate SOC2/ISO 27001 compliance evidence package |
| `blameprompt gdpr` | Generate GDPR data flow map and DPIA report |
| `blameprompt supply-chain-risk` | Assess AI supply chain risk score (0-10) |
| `blameprompt vuln-scan` | SAST scan of AI-generated code for vulnerabilities |
| `blameprompt prompt-injection` | Detect prompt injection patterns in AI-generated code |
| `blameprompt secret-rotation` | Alert on secrets that need rotation after AI exposure |

### Cost Management

| Command | Description |
|---------|-------------|
| `blameprompt budget` | Analyze AI usage costs with budget tracking |
| `blameprompt model-recommend` | Get model optimization recommendations to reduce costs |

### Enterprise

| Command | Description |
|---------|-------------|
| `blameprompt enterprise status` | Show enterprise configuration status |
| `blameprompt enterprise sync` | Sync local receipts to enterprise server |
| `blameprompt enterprise export` | Export all receipts as JSON bundle |
| `blameprompt enterprise policy-check` | Check staged receipts against enterprise policies |

### Data Management

| Command | Description |
|---------|-------------|
| `blameprompt push` | Push BlamePrompt notes to origin |
| `blameprompt pull` | Fetch BlamePrompt notes from origin |
| `blameprompt cache sync` | Sync Git Notes into local SQLite cache |
| `blameprompt tag <file>` | Manually tag code as AI-generated |
| `blameprompt record --session <path>` | Ingest a Claude Code JSONL transcript |
| `blameprompt redact --test <file>` | Dry-run the redaction engine on a file |
| `blameprompt replit-detect` | Detect Replit Agent changes in unstaged files |

## Command Details

### `blameprompt audit`

```bash
# Default: generates blameprompt-audit.md
blameprompt audit

# With filters
blameprompt audit --from 2026-01-01 --to 2026-02-15 --author "Alice"

# Different formats
blameprompt audit --format json
blameprompt audit --format csv
blameprompt audit --format table

# Include uncommitted/staged changes
blameprompt audit --include-uncommitted
```

### `blameprompt license-scan`

Scans all AI models used in your codebase against a license restriction database:
- Llama Community License (700M MAU limit)
- Codestral Non-Production License
- Apache 2.0 attribution requirements
- DeepSeek, Qwen, Gemma terms

```bash
blameprompt license-scan
blameprompt license-scan --output custom-report.md
```

### `blameprompt soc2`

Generates a SOC 2 Type II / ISO 27001 evidence package with:
- Executive summary of AI usage
- Access control (who used AI)
- Data sent to AI providers
- Code generation change log
- Timeline of AI usage
- Attestation statement

```bash
blameprompt soc2
blameprompt soc2 --from 2026-01-01 --to 2026-03-31
```

### `blameprompt gdpr`

Generates a GDPR Data Processing Impact Assessment (DPIA) with:
- Data flow diagrams
- AI provider data processor mapping
- PII detection in stored prompts
- Data retention analysis
- Risk assessment

```bash
blameprompt gdpr
```

### `blameprompt supply-chain-risk`

Calculates a 0-10 risk score based on:
- Model diversity (more models = more supply chain surface)
- Cloud vs local deployment ratio
- Prompt sensitivity (secrets in prompts)
- Critical file exposure (AI touching auth/crypto/config files)
- Human review coverage

```bash
blameprompt supply-chain-risk
```

### `blameprompt vuln-scan`

Static analysis of AI-generated code for:
- Command injection (CWE-78)
- SQL injection (CWE-89)
- XSS (CWE-79)
- Path traversal (CWE-22)
- Hardcoded credentials (CWE-798)
- Insecure deserialization (CWE-502)
- Dynamic code execution (CWE-95)
- And more

```bash
blameprompt vuln-scan
```

### `blameprompt prompt-injection`

Detects potential prompt injection and backdoor patterns:
- Eval/exec backdoors
- Base64 decode + execute chains
- Outbound network calls (data exfiltration)
- Environment variable exfiltration
- Timer-based triggers
- Obfuscated strings
- Suspicious AI response phrases

```bash
blameprompt prompt-injection
```

### `blameprompt secret-rotation`

Identifies secrets that may have been sent to AI providers and need rotation:
- Separates cloud vs local exposure
- Per-secret-type rotation instructions
- Prevention recommendations

```bash
blameprompt secret-rotation
```

### `blameprompt budget`

Track AI spending with configurable budget limits:

```bash
# Basic budget analysis
blameprompt budget

# With monthly limit (alerts at 80%, warns at 100%)
blameprompt budget --monthly-limit 500.00

# With quarterly limit
blameprompt budget --monthly-limit 500.00 --quarterly-limit 1200.00
```

### `blameprompt model-recommend`

Analyzes usage patterns and recommends optimal models:
- Identifies premium models used for simple tasks
- Calculates potential cost savings
- Suggests cheaper alternatives
- Model tier guide (Premium/Standard/Economy/Free)

```bash
blameprompt model-recommend
```

### `blameprompt enterprise`

Centralized receipt management for organizations. Sync receipts to an enterprise server, export data for ingestion, and enforce org-wide policies.

```bash
# Check enterprise config
blameprompt enterprise status

# Sync all receipts to enterprise server
blameprompt enterprise sync
blameprompt enterprise sync --from 2026-01-01 --to 2026-02-15

# Export receipts as JSON bundle (works without a server)
blameprompt enterprise export
blameprompt enterprise export --output team-export.json --author "Alice"

# Check staged receipts against enterprise policies
blameprompt enterprise policy-check
```

## Configuration

Create a `.blamepromptrc` file in your repo root (or `~/.blamepromptrc` globally):

```toml
[redaction]
mode = "replace"  # or "hash" for SHA-256 prefix mode

[[redaction.custom_patterns]]
pattern = "INTERNAL-\\d{6}"
replacement = "[REDACTED_INTERNAL_ID]"

[redaction]
disable_patterns = ["BEARER_TOKEN"]

[capture]
max_prompt_length = 2000
store_full_conversation = true

[enterprise]
enabled = true
api_url = "https://blameprompt.your-company.com"
api_key_env = "BLAMEPROMPT_API_KEY"
sync_on_commit = true
org = "your-org"
team = "backend-team"
```

## Sharing Notes with Your Team

```bash
# Push receipts to remote
blameprompt push

# Pull receipts from remote
blameprompt pull

# Or manually:
git push origin refs/notes/blameprompt
git fetch origin refs/notes/blameprompt:refs/notes/blameprompt
```

## Architecture

```
Claude Code Hooks (PreToolUse/PostToolUse)
        |
        v
  JSONL Transcript Parser
        |
        v
  Redaction Engine (secrets, PII, entropy)
        |
        v
  Receipt Generation (UUID, cost, lines, chain-of-thought)
        |
        v
  Staging Area (.blameprompt/staging.json)
        |
        v
  Git Notes (refs/notes/blameprompt) -- attached on commit
        |
        v
  Analysis Commands (blame, audit, report, compliance, security)
```

## Supported AI Providers

| Provider | Models | Detection |
|----------|--------|-----------|
| Anthropic | Claude Opus 4.x, Sonnet 4.x, Haiku | Automatic (hooks) |
| OpenAI | GPT-4o, GPT-4, GPT-3.5 | Manual (tag/record) |
| Meta | Llama, Code Llama | Manual (tag/record) |
| Mistral AI | Mistral, Mixtral, Codestral | Manual (tag/record) |
| DeepSeek | DeepSeek Coder | Manual (tag/record) |
| Google | Gemini, Gemma | Manual (tag/record) |
| Microsoft | Phi | Manual (tag/record) |
| Replit | Replit Agent | Auto-detect (replit-detect) |
| Local | Ollama, LM Studio | Manual (tag/record) |

## License

MIT
