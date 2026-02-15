# BlamePrompt Enterprise Plan

## Overview

Enterprise layer for BlamePrompt that gives organizations centralized visibility and control over AI-generated code across all teams and repositories.

**Core value prop:** BlamePrompt already captures receipts per-repo. Enterprise aggregates that data into a single dashboard with team management, policies, and org-wide reporting.

---

## Architecture

```
Individual Repos (existing)         Enterprise Layer (new)
┌──────────────────────┐
│ Dev A: repo-1        │───push──┐
│  blameprompt receipts│         │    ┌─────────────────────────┐
└──────────────────────┘         ├───>│   Enterprise Server     │
┌──────────────────────┐         │    │                         │
│ Dev B: repo-2        │───push──┤    │  - REST API             │
│  blameprompt receipts│         │    │  - PostgreSQL store     │
└──────────────────────┘         │    │  - Auth (API keys)      │
┌──────────────────────┐         │    │  - Policy engine        │
│ Dev C: repo-3        │───push──┘    │  - Dashboard (web)      │
│  blameprompt receipts│              └─────────────────────────┘
└──────────────────────┘
```

---

## Phase 1: Core (MVP)

Keep it minimal. Ship in ~1-2 weeks.

### 1.1 Receipt Sync Endpoint

New CLI command: `blameprompt enterprise sync`

- Reads local Git Notes receipts
- POSTs them to the enterprise server
- Deduplicates by receipt UUID
- Auth via org API key (env var `BLAMEPROMPT_API_KEY`)

```rust
// New command in src/commands/enterprise.rs
pub async fn sync(api_url: &str, api_key: &str) -> Result<()> {
    let receipts = collect_all_receipts()?;
    let client = reqwest::Client::new();
    client.post(format!("{}/api/v1/receipts", api_url))
        .bearer_auth(api_key)
        .json(&receipts)
        .send().await?;
    Ok(())
}
```

### 1.2 Enterprise Server

Lightweight Rust server (axum).

**Endpoints:**

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/receipts` | Ingest receipts from CLI |
| GET | `/api/v1/receipts` | Query receipts (filters: repo, user, date, model) |
| GET | `/api/v1/analytics` | Org-wide aggregated stats |
| GET | `/api/v1/teams` | List teams |
| POST | `/api/v1/teams` | Create team |
| GET | `/api/v1/policies` | List active policies |
| POST | `/api/v1/policies` | Create/update policy |

**Database (PostgreSQL):**

```sql
-- Core tables
CREATE TABLE orgs (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    api_key TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE teams (
    id UUID PRIMARY KEY,
    org_id UUID REFERENCES orgs(id),
    name TEXT NOT NULL
);

CREATE TABLE team_members (
    team_id UUID REFERENCES teams(id),
    email TEXT NOT NULL,
    role TEXT DEFAULT 'member', -- admin | member | viewer
    PRIMARY KEY (team_id, email)
);

CREATE TABLE receipts (
    id UUID PRIMARY KEY,          -- receipt UUID from CLI
    org_id UUID REFERENCES orgs(id),
    repo TEXT NOT NULL,
    commit_hash TEXT,
    provider TEXT,
    model TEXT,
    prompt_summary TEXT,
    prompt_hash TEXT,
    cost_usd REAL,
    user_name TEXT,
    user_email TEXT,
    file_path TEXT,
    line_start INT,
    line_end INT,
    timestamp TIMESTAMPTZ,
    session_duration_secs INT,
    synced_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE policies (
    id UUID PRIMARY KEY,
    org_id UUID REFERENCES orgs(id),
    name TEXT NOT NULL,
    rule JSONB NOT NULL,          -- e.g. {"max_cost_per_day": 50.0}
    action TEXT DEFAULT 'warn',   -- warn | block
    enabled BOOLEAN DEFAULT TRUE
);
```

### 1.3 Org-Wide Analytics

`GET /api/v1/analytics` returns:

```json
{
  "total_receipts": 12483,
  "total_cost_usd": 847.32,
  "ai_code_percentage": 34.2,
  "top_models": [
    {"model": "claude-opus-4-6", "count": 5200, "cost": 512.00},
    {"model": "claude-sonnet-4-5", "count": 7283, "cost": 335.32}
  ],
  "top_repos": [
    {"repo": "backend-api", "receipts": 4100, "cost": 310.50},
    {"repo": "frontend", "receipts": 3200, "cost": 198.20}
  ],
  "daily_trend": [
    {"date": "2026-02-14", "receipts": 120, "cost": 18.40},
    {"date": "2026-02-15", "receipts": 95, "cost": 14.10}
  ]
}
```

---

## Phase 2: Policies & Compliance

### 2.1 Policy Engine

Simple rule-based policies evaluated server-side.

**Supported rules:**

| Rule | Example | Action |
|------|---------|--------|
| `max_cost_per_day` | `50.00` | Warn/block when daily spend exceeds limit |
| `allowed_models` | `["claude-sonnet-4-5"]` | Only permit specific models |
| `blocked_models` | `["gpt-4"]` | Block specific models |
| `require_review_above` | `100` lines | Flag large AI changes for human review |
| `max_ai_percentage` | `60` | Alert if AI-generated code exceeds threshold per repo |

**CLI enforcement:**

```toml
# .blameprompt/config.toml (added fields)
[enterprise]
api_url = "https://blameprompt.company.com"
api_key_env = "BLAMEPROMPT_API_KEY"
sync_on_commit = true    # auto-sync after each commit
enforce_policies = true  # check policies before commit
```

When `enforce_policies = true`, the post-commit hook calls the server to validate the receipt against org policies before allowing the commit.

### 2.2 Compliance Reports

Extend existing `soc2`, `gdpr`, and `audit` commands with enterprise-wide scope:

```bash
# Existing (single repo)
blameprompt soc2

# Enterprise (org-wide, pulls from server)
blameprompt enterprise report --type soc2
blameprompt enterprise report --type gdpr
blameprompt enterprise report --type audit --format pdf
```

---

## Phase 3: Dashboard (Web UI)

Minimal web dashboard. Use a simple SPA (React or htmx).

### Pages

1. **Overview** - Org stats, cost trend chart, top repos/users
2. **Receipts** - Searchable table of all receipts with filters
3. **Teams** - Manage teams and members
4. **Policies** - Create/edit policy rules
5. **Reports** - Generate and download compliance reports
6. **Settings** - API keys, org config

### Stack

- Frontend: htmx + Tailwind (keep it simple, no build step)
- Served by the same axum server
- Auth: session cookies, SSO integration later

---

## Implementation Order

| Step | What | Effort |
|------|------|--------|
| 1 | `enterprise.rs` CLI command + sync endpoint | 2-3 days |
| 2 | PostgreSQL schema + receipt ingestion API | 2 days |
| 3 | Analytics query endpoint | 1 day |
| 4 | Policy engine (server-side rules) | 2 days |
| 5 | CLI policy enforcement (pre-commit check) | 1 day |
| 6 | Enterprise report commands | 1-2 days |
| 7 | Web dashboard (htmx) | 3-5 days |

**Total: ~2-3 weeks for full enterprise layer.**

MVP (steps 1-3) can ship in **~1 week**.

---

## File Structure (new code)

```
src/
├── commands/
│   ├── enterprise.rs          # sync, report, policy commands
│   └── mod.rs                 # add enterprise module
├── server/                    # enterprise server (separate binary)
│   ├── main.rs                # axum server entry
│   ├── routes/
│   │   ├── receipts.rs
│   │   ├── analytics.rs
│   │   ├── teams.rs
│   │   └── policies.rs
│   ├── db.rs                  # PostgreSQL queries
│   ├── policy_engine.rs       # rule evaluation
│   └── auth.rs                # API key + session auth
└── enterprise/
    └── types.rs               # shared types between CLI and server
```

---

## Config Addition

```toml
# .blameprompt/config.toml
[enterprise]
enabled = true
api_url = "https://blameprompt.example.com"
api_key_env = "BLAMEPROMPT_API_KEY"
sync_on_commit = true
enforce_policies = true
team = "backend-team"
```

---

## Key Decisions

1. **Self-hosted first** - Enterprise server runs on customer infra. SaaS later.
2. **PostgreSQL** - Familiar, robust, JSONB for flexible policy rules.
3. **No new dependencies for CLI** - `reqwest` is already available; enterprise commands are behind a feature flag.
4. **Feature flag** - `cargo build --features enterprise` to keep the base CLI lightweight.
5. **Git Notes remain source of truth** - Server is a read-only aggregation layer. Receipts are never modified server-side.
