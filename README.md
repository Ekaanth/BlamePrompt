# blameprompt

**Your AI skills deserve a portfolio.**

Track your AI coding activity across 15 agents. Build your developer score. Earn achievement badges. Climb the leaderboard. Get a public profile at `blameprompt.com/username` that proves your AI coding skills — with real data, not self-reported claims.

```
$ blameprompt profile

  Username:  alice
  Email:     alice@dev.com
  Score:     82 (Expert)
  Rank:      #12 on leaderboard
  Badges:    8/12 unlocked
  Prompts:   14,832
  Streak:    47 days
  Profile:   blameprompt.com/alice
```

## Install

```bash
# macOS / Linux
curl -sSL https://blameprompt.com/install.sh | bash

# Windows (PowerShell)
irm https://blameprompt.com/install.ps1 | iex

# Or build from source
cargo install --path .
```

The installer automatically configures hooks for all 15 supported agents, installs Git hooks globally, and sets up the transparent git wrapper — every repo is tracked from that point forward.

## Get started

```bash
# 1. Install (one-time)
curl -sSL https://blameprompt.com/install.sh | bash

# 2. Log in with GitHub
blameprompt login

# 3. Sync your data and build your profile
blameprompt sync

# 4. Open your dashboard
blameprompt dash
```

That's it. Your profile, score, heatmap, and badges start building automatically as you code with AI.

Or sign in directly at [blameprompt.com/login](https://blameprompt.com/login).

## What you get

### Developer score (0-100)

A composite rating based on real data:

- **Prompt quality** — clarity, actionability, context, and efficiency scoring
- **Acceptance rate** — how much AI-generated code you keep
- **Cost efficiency** — results without burning tokens
- **Model diversity** — knowing when to use Opus vs Sonnet vs GPT-4o
- **Consistency** — daily streaks and sustained usage

Ratings: Novice → Intermediate → Advanced → Expert → Elite

### Public profile

Your profile at `blameprompt.com/username` showcases:

- Developer score and rating
- GitHub-style prompt heatmap
- Achievement badges (12 types)
- Model proficiency breakdown
- Language, tool, and editor usage
- AI vs human code ratio

Put it on your resume. Embed a badge in your GitHub README. Share on LinkedIn.

### Achievement badges

12 badges that track real milestones:

- **Century** — 100 prompts
- **Thousand Club** — 1,000 prompts
- **Week Warrior** — 7-day streak
- **Month Master** — 30-day streak
- **Polyglot** — 5+ programming languages
- **Multi-Agent** — 3+ AI agents used
- **Cost Pro** — below-average cost per prompt
- **Elite** — developer score 90+
- And 4 more to discover

### Global leaderboard

Rank against developers worldwide by prompts, score, or active days. Browse the leaderboard at [blameprompt.com/leaderboard](https://blameprompt.com/leaderboard).

## How it works

```
AI Agent ──> hooks/import ──> staging.json ──> git commit ──> Git Notes ──> sync ──> profile
              (files, lines,     (local,        (post-commit    (refs/notes/    (score, badges,
               prompt, model,     gitignored)     hook)          blameprompt)    heatmap, rank)
               tools, cost)
```

1. **You code with AI** — hooks fire automatically or you import sessions from other agents
2. **One receipt per prompt** — files, lines, model, cost, tokens, quality score — all captured
3. **Receipts attach on commit** — `post-commit` hook writes everything as a Git Note
4. **Sync to your profile** — `blameprompt sync` uploads aggregated metrics to your public profile
5. **Score and badges update** — your developer score, heatmap, and achievement badges build automatically

Everything is local-first. Nothing leaves your machine unless you choose to sync.

## Supported agents (15)

All detected agents are auto-configured by `blameprompt init --global`. If an agent isn't installed, it's silently skipped.

| Agent | Hook config | Import historical sessions |
|-------|------------|---------------------------|
| **Claude Code** | `~/.claude/settings.json` | Automatic |
| **GitHub Copilot** | `~/.github/hooks/blameprompt.json` | `blameprompt record-copilot` |
| **OpenAI Codex CLI** | `~/.codex/config.toml` | `blameprompt record-codex` |
| **Google Gemini CLI** | `~/.gemini/settings.json` | `blameprompt record-gemini` |
| **Cursor** | `~/.cursor/hooks.json` | `blameprompt record-cursor` |
| **Windsurf (Codeium)** | `~/.windsurf/hooks.json` | `blameprompt record-windsurf` |
| **Antigravity IDE** | `~/.antigravity/settings.json` | `blameprompt record-antigravity` |
| **Continue** | `~/.continue/hooks.json` | `blameprompt record-continue` |
| **Droid** | `~/.droid/hooks.json` | `blameprompt record-droid` |
| **JetBrains Junie** | `~/.junie/hooks.json` | `blameprompt record-junie` |
| **Atlassian Rovo Dev** | `~/.rovo-dev/hooks.json` | `blameprompt record-rovo-dev` |
| **Sourcegraph Amp** | `~/.amp/hooks.json` | `blameprompt record-amp` |
| **OpenCode** | `~/.opencode/hooks.json` | `blameprompt record-opencode` |
| **Any provider** | — | `blameprompt record --session <file> --provider <name>` |

## VS Code extension

Install from [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=Blameprompt.blameprompt).

- **Prompt Receipts** — tree view of commits > prompts > files/tools/agents
- **Prompt History** — git-log-style timeline with model-colored labels
- **File History** — all prompts that modified the currently open file

## Commands

### Account & profile

```bash
blameprompt login                   # sign in via GitHub (opens browser)
blameprompt login --token <key>     # authenticate with API token (CI/headless)
blameprompt logout                  # clear stored credentials
blameprompt profile                 # show your score, rank, and badges
blameprompt profile --edit          # edit profile in browser
blameprompt sync                    # sync receipts to your public profile
blameprompt dash                    # open dashboard in browser
```

### Attribution

```bash
blameprompt blame src/auth.rs       # line-by-line AI vs human
blameprompt show a1b2c3d            # all receipts for a commit
blameprompt search "JWT"            # full-text search across prompts
blameprompt diff                    # annotated working-tree diff
blameprompt diff a1b2c3d            # annotated commit diff
blameprompt check-provenance src/auth.rs          # AI vs human lines
blameprompt check-provenance src/auth.rs --line 5 # specific line
```

### Analytics & reporting

```bash
blameprompt analytics                       # AI score, cost breakdown, model usage
blameprompt audit                           # full audit trail (md, table, json, csv)
blameprompt audit --from 2026-01-01 --author "Jane" --format json
blameprompt report --output report.md       # comprehensive markdown report
blameprompt report --quality                # prompt quality scoring report
```

### Security

```bash
blameprompt vuln-scan               # CWE pattern scanning on AI-generated code
blameprompt prompt-injection        # detect backdoors and hidden instructions
blameprompt secret-rotation         # flag secrets exposed to AI
blameprompt supply-chain-risk       # risk score 0-10
blameprompt license-scan            # model license compliance
```

### Hackathon fairness

```bash
blameprompt hackathon-report                    # last 24h, all participants
blameprompt hackathon-report --start "2026-02-26T09:00:00Z" --end "2026-02-26T21:00:00Z"
```

Generates an integrity report with timeline, code attribution, and anomaly detection.

### Sharing & interop

```bash
blameprompt push                    # push notes to remote
blameprompt pull                    # fetch notes from remote
blameprompt cache sync              # build local SQLite cache
blameprompt export-agent-trace      # export to Agent Trace v0.1.0 format
blameprompt import-agent-trace      # display Agent Trace record
blameprompt github-comment          # post AI attribution as PR comment
```

### Setup & diagnostics

```bash
blameprompt init --global           # global setup (hooks, git template, 15 agents)
blameprompt init                    # setup in current repo only
blameprompt install-git-wrap        # transparent git wrapper (auto-attach on commit)
blameprompt doctor                  # diagnose installation issues
blameprompt update                  # self-update
blameprompt uninstall               # remove hooks, keep receipt history
blameprompt uninstall --purge       # remove everything including Git Notes
```

## What gets captured

Every AI receipt includes: provider, model, user, timestamp, session ID, prompt & response summaries, files changed (with line ranges), token usage (input, output, cache read, cache creation), real token-based cost, tools used, MCP servers called, agents spawned, conversation chain of thought, prompt quality score (4-dimension: clarity, actionability, context, efficiency), prompt category, acceptance rate, and parent receipt links.

Cost tracking uses actual API token data — cache reads at 90% discount, cache creation at 25% surcharge. Pricing for Claude, GPT-4o/4.1/o1/o3, Gemini 2.5, Codex, and more.

## Privacy & data

Everything is local-first. Nothing leaves your machine unless you `push` or `sync`.

| What | Where |
|------|-------|
| AI receipts | `refs/notes/blameprompt` (inside `.git`) |
| Staging | `.blameprompt/staging.json` (gitignored) |
| Credentials | `~/.blameprompt/credentials` |
| Cache | `~/.blameprompt/prompts.db` |
| Config | `.blamepromptrc` or `~/.blamepromptrc` |

Zero telemetry. Zero tracking. Built-in redaction engine strips secrets before storage. You choose what to sync to your public profile.

## Enterprise

BlamePrompt Enterprise provides team-level AI code observability, compliance reporting, and adoption benchmarking. Visit [blameprompt.com/enterprise](https://blameprompt.com/enterprise).

## License

[MIT](LICENSE)
