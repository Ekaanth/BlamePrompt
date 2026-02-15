# BlamePrompt Report: blameprompt
> Generated: 2026-02-15 | Period: entire history | Commits: 1

## Executive Summary

| Metric | Value |
|--------|-------|
| Total commits analyzed | 1 |
| Commits with AI-generated code | 1 (100.0%) |
| Total AI-assisted sessions | 4 |
| Total files modified by AI | 27 |
| Total AI-generated lines | 66 |
| Estimated total AI cost | $3.84 |
| Unique contributors | 1 |
| AI tools used | claude |

## AI vs Human Code Attribution

### Overall
- **AI-generated**: 66 lines

### By Directory
| Directory | AI Lines |
|-----------|----------|
| src/commands | 45 |
|  | 7 |
| src/core | 7 |
| src | 6 |
| /Users/metaquity/.claude | 1 |

### Top AI-Heavy Files
| File | AI Lines | Model Used |
|------|----------|------------|
| src/commands/report.rs | 8 | Claude Opus 4.6 |
| src/commands/audit.rs | 7 | Claude Opus 4.6 |
| src/commands/prompt_injection.rs | 6 | Claude Opus 4.6 |
| src/main.rs | 6 | Claude Opus 4.6 |
| src/commands/budget.rs | 4 | Claude Opus 4.6 |
| README.md | 4 | Claude Opus 4.6 |
| src/commands/mod.rs | 3 | Claude Opus 4.6 |
| src/core/config.rs | 3 | Claude Opus 4.6 |
| src/commands/checkpoint.rs | 2 | Claude Opus 4.6 |
| src/commands/vuln_scan.rs | 2 | Claude Opus 4.6 |

## Token & Cost Analysis

### Cost Summary
| Metric | Value |
|--------|-------|
| Total estimated cost | $3.84 |
| Avg cost per session | $0.959 |
| Avg cost per AI commit | $3.837 |

### Cost by Model
| Model | Receipts | Est. Cost | % of Total |
|-------|----------|-----------|------------|
| Claude Opus 4.6 | 66 | $3.84 | 100.0% |

## User Contributions

| User | AI Sessions | AI Lines | Est. Cost |
|------|-------------|----------|-----------|
| Ekaanth <akanth1994@gmail.com> | 66 | 66 | $3.84 |

## Time & AI Generation Analysis

### Total Time Invested in AI
| Metric | Value |
|--------|-------|
| Total AI session time | 1h 37m |
| Unique sessions | 4 |
| Avg session duration | 24m 24s |
| Total AI-generated lines | 66 |
| Estimated dev-hours saved | ~0.6h (66 AI lines x 30s/line) |
| Time ROI | 0.3x |

### AI Model Response Speed
| Model | Avg Response | Sessions |
|-------|-------------|----------|
| Claude Opus 4.6 | 4.6s | 66 |

## Security Audit

### Redaction Summary
- **Total prompts scanned**: 66
- **Prompts with secrets detected**: 0 (0.0%)
- **All secrets were auto-redacted**: Yes

## Model Usage: Open-Source vs Closed-Source

### License Breakdown
| License | Sessions | Est. Cost |
|---------|----------|-----------|
| Closed-source | 66 | $3.84 |
| Open-source | 0 | $0.00 |

### Deployment Breakdown
| Deployment | Sessions |
|-----------|----------|
| Cloud | 66 |
| Local | 0 |

### Vendor Comparison
| Vendor | License | Sessions | Est. Cost |
|--------|---------|----------|-----------|
| anthropic | Closed | 66 | $3.84 |

## File-Level AI Heatmap

### Most AI-Modified Files
| Rank | File | AI Edits | AI Lines | Last AI Edit | Primary Model |
|------|------|----------|----------|-------------|---------------|
| 1 | src/commands/report.rs | 8 | 8 | 2026-02-15 | Claude Opus 4.6 |
| 2 | src/commands/audit.rs | 7 | 7 | 2026-02-15 | Claude Opus 4.6 |
| 3 | src/commands/prompt_injection.rs | 6 | 6 | 2026-02-15 | Claude Opus 4.6 |
| 4 | src/main.rs | 6 | 6 | 2026-02-15 | Claude Opus 4.6 |
| 5 | src/commands/budget.rs | 4 | 4 | 2026-02-15 | Claude Opus 4.6 |
| 6 | README.md | 4 | 4 | 2026-02-15 | Claude Opus 4.6 |
| 7 | src/core/config.rs | 3 | 3 | 2026-02-15 | Claude Opus 4.6 |
| 8 | src/commands/mod.rs | 3 | 3 | 2026-02-15 | Claude Opus 4.6 |
| 9 | src/core/receipt.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 10 | src/commands/compliance.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 11 | src/commands/secret_rotation.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 12 | src/commands/checkpoint.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 13 | src/commands/record.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 14 | src/commands/vuln_scan.rs | 2 | 2 | 2026-02-15 | Claude Opus 4.6 |
| 15 | src/commands/show.rs | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |
| 16 | src/commands/supply_chain.rs | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |
| 17 | src/core/db.rs | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |
| 18 | ENTERPRISE_PLAN.md | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |
| 19 | test_e2e.sh | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |
| 20 | Cargo.toml | 1 | 1 | 2026-02-15 | Claude Opus 4.6 |

## Session Analysis

### Session Statistics
| Metric | Value |
|--------|-------|
| Total unique sessions | 4 |
| Sessions modifying 1 file | 1 |
| Sessions modifying 2+ files | 3 |

### Top Sessions by Message Count
| Session ID | Messages | Files | Est. Cost | Prompt Summary |
|-----------|----------|-------|-----------|---------------|
| 7f2e447b | 544 | 22 | $3.66 | "use @CHAIN_OF_THOUGHT.md and iplementt that into t" |
| 61605b06 | 86 | 7 | $0.08 | "I want to add enterprise side to help enterprise m" |
| 1f66264c | 62 | 1 | $0.03 | "(base) metaquity@Abhisheks-MacBook-Pro-2 blameprom" |
| b400e0d0 | 42 | 2 | $0.07 | " 12. Budget Controls — Set monthly/quarterly budge" |

## Prompt Details

Full prompt context for each AI-assisted change.

### Commit `80a32d48`

- **Date**: 2026-02-15T15:59:57+05:30
- **Author**: Ekaanth <akanth1994@gmail.com>
- **Message**: initial commit

#### src/commands/analytics.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 175 |
| Cost | $0.0129 |
| Lines | 1-1 |
| Prompt Hash | `sha256:b3b85da829eef42a6ef8a23337fd079acc21f7660875303c1193edddf58016b9` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Session duration: 12m 12s
- Avg AI response: 3.0s

#### test_e2e.sh | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 186 |
| Cost | $0.0134 |
| Lines | 1-1 |
| Prompt Hash | `sha256:2a1d42d0e85d0163a676024a2e5fba45733e294295e7bab4ea4439e01c880f63` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.0s

#### /Users/metaquity/.claude/settings.json | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `1f66264c-b310-4de9-a13e-2566574086bb` |
| Messages | 62 |
| Cost | $0.0271 |
| Lines | 1-1 |
| Prompt Hash | `sha256:706e8368fdc9db7e544abfc838bbc01001d910f2828520f293a82cd6d2a758d3` |

**Prompt:**
> (base) metaquity@Abhisheks-MacBook-Pro-2 blameprompt % blameprompt                          
zsh: command not found: blameprompt

- Session duration: 13m 28s
- Avg AI response: 3.9s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 230 |
| Cost | $0.0293 |
| Lines | 1-1 |
| Prompt Hash | `sha256:5e184757207b7071c9474b405d50b0a057a070ad30878352cf0cbe1743edc9f9` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.6s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 237 |
| Cost | $0.0295 |
| Lines | 1-1 |
| Prompt Hash | `sha256:3030cc94b2dd9cfb7a2078d5a65258e372956125677cb0efa5063d8ff4619064` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.5s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 263 |
| Cost | $0.0348 |
| Lines | 1-1 |
| Prompt Hash | `sha256:14c2178681b339abab17d6839805542ab1e408f23ef0879ae5f244b8c9666bea` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.6s

#### src/core/receipt.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 280 |
| Cost | $0.0402 |
| Lines | 1-1 |
| Prompt Hash | `sha256:a23d3c7138b21cc41796ab407c74eb773a42316439f03e715395078789e14b48` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.6s

#### src/core/transcript.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 285 |
| Cost | $0.0404 |
| Lines | 1-1 |
| Prompt Hash | `sha256:95c9f5a0c9b2afaefb7f40e5e869c8e5a5a346fae4be68f3b4e63684895d4300` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.6s

#### src/commands/checkpoint.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 288 |
| Cost | $0.0407 |
| Lines | 1-1 |
| Prompt Hash | `sha256:c69c819a32cc38584055c40682f5636e164107681384d1f6e5430b0350a19b5f` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/checkpoint.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 291 |
| Cost | $0.0408 |
| Lines | 1-1 |
| Prompt Hash | `sha256:09c4ecd832f1fb5973b593e43ba16c54c632b37df78ea96585d3b01859bda8ca` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/staging.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 294 |
| Cost | $0.0414 |
| Lines | 1-1 |
| Prompt Hash | `sha256:7a4e2e8c68273b2d849731f011ca3dac0396b45129213b34ccdb3f1bf0d11041` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/record.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 296 |
| Cost | $0.0414 |
| Lines | 1-1 |
| Prompt Hash | `sha256:c38d4414a231747b3b7da6c5e7dc635d73585333624fefdc7ed0326430ad0858` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/record.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 299 |
| Cost | $0.0417 |
| Lines | 1-1 |
| Prompt Hash | `sha256:7931a0846a8df108457541e5197de4e7bf64570b2117e4f1d821ca7d86d1fbc0` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/core/receipt.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 304 |
| Cost | $0.0419 |
| Lines | 1-1 |
| Prompt Hash | `sha256:a74fb2c9d0406071fb510a3f2d7faf8a0236506ec9ee3c2fa0573e156ddef8e9` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/show.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 307 |
| Cost | $0.0427 |
| Lines | 1-1 |
| Prompt Hash | `sha256:40297bf7388f0e8cb537f2f77ccd40e6089fa165bde250a63cddc36a6f4f1570` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.7s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 312 |
| Cost | $0.0437 |
| Lines | 1-1 |
| Prompt Hash | `sha256:7fefef44287107a8e34ca038356c39ec53bc623f1dda2371161b5fe1f6383e7c` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 319 |
| Cost | $0.0440 |
| Lines | 1-1 |
| Prompt Hash | `sha256:4ea736478ff40f7d920571eac1946f59a953dad236074225071628b6ef5c0cc4` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 324 |
| Cost | $0.0442 |
| Lines | 1-1 |
| Prompt Hash | `sha256:e55492a59bb32294f856311105c2d699720eb4acea01ff45d6a3d374a36676fe` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 330 |
| Cost | $0.0451 |
| Lines | 1-1 |
| Prompt Hash | `sha256:389950b031435916c0f499f4db903ba58c78cef1de20d62eb29640eaf290f8ae` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 333 |
| Cost | $0.0453 |
| Lines | 1-1 |
| Prompt Hash | `sha256:079f40a17b3f537d08d532ee59fc760880000f092ac45024975ac8b48ecb0636` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/core/db.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 341 |
| Cost | $0.0456 |
| Lines | 1-1 |
| Prompt Hash | `sha256:0747e034e68a6e9e026a747cda4632ab3586542c3bd568954dc3c3d71398f150` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/license_scan.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 367 |
| Cost | $0.0547 |
| Lines | 1-1 |
| Prompt Hash | `sha256:8497c25e8425ec55276363697e2f20b1122bc11781d14348015340f255c44d30` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/compliance.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 367 |
| Cost | $0.0547 |
| Lines | 1-1 |
| Prompt Hash | `sha256:8497c25e8425ec55276363697e2f20b1122bc11781d14348015340f255c44d30` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 3.8s

#### src/commands/mod.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 410 |
| Cost | $0.1187 |
| Lines | 1-1 |
| Prompt Hash | `sha256:df548754e3b0815ff5f7c6cd1159407dd49ae9d5d3830ecf673de5e95685f51f` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.0s

#### src/commands/supply_chain.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 411 |
| Cost | $0.1187 |
| Lines | 1-1 |
| Prompt Hash | `sha256:cccfd67a7af90e4e2789afb6be648e6bcc6ffa1c9de36bb8e7dabbb0cfd2afe0` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.0s

#### src/commands/vuln_scan.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 413 |
| Cost | $0.1187 |
| Lines | 1-1 |
| Prompt Hash | `sha256:8d8359e9e01dc7a2138905f92f00899da99ec4ad81975089b6adf0b4a3c88354` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.2s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 415 |
| Cost | $0.1187 |
| Lines | 1-1 |
| Prompt Hash | `sha256:1a349b409a3a2cfc97e149169e35dd1adcec5a56d9095497be0f15463325113b` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.5s

#### src/commands/secret_rotation.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 417 |
| Cost | $0.1187 |
| Lines | 1-1 |
| Prompt Hash | `sha256:3719447111f97e733ec6c2015c14a6a7b5e2ede259b716c570e6cb76b1ebc792` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.7s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 422 |
| Cost | $0.1188 |
| Lines | 1-1 |
| Prompt Hash | `sha256:3e27879205d4f5da5121231b7f0127ee75ae4db76092190043951f733073a437` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 425 |
| Cost | $0.1190 |
| Lines | 1-1 |
| Prompt Hash | `sha256:d888c6ce7c57f8a821ffbdf82cd52bc86c6f5f53dc330edb3449393519b6e2f9` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

#### src/commands/mod.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 428 |
| Cost | $0.1194 |
| Lines | 1-1 |
| Prompt Hash | `sha256:79450af2cf49166fb392c973fe88e0925147ba93107231ffb6c1925910b4bafc` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

#### src/commands/budget.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 430 |
| Cost | $0.1194 |
| Lines | 1-1 |
| Prompt Hash | `sha256:2185d2cd1d0ce7ce275b9f7682f51ee0ae613eaf13c4fbd39c16af9e2cfc30e1` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

#### src/commands/model_recommend.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 432 |
| Cost | $0.1194 |
| Lines | 1-1 |
| Prompt Hash | `sha256:709575c0295326ca1770106a9daa29b76d7630de317d94e760a54a3248e4f0f5` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/compliance.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 452 |
| Cost | $0.1198 |
| Lines | 1-1 |
| Prompt Hash | `sha256:01f1107a44d1b6beeb8a8f62c45512d94b2135e5f2054a08c123a9bd4c7823bc` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/vuln_scan.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 457 |
| Cost | $0.1200 |
| Lines | 1-1 |
| Prompt Hash | `sha256:fec37cef6a0a4c2b1c1e3f8a8011d2a38746b202470a977c1b5343b4f697cb3e` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 468 |
| Cost | $0.1208 |
| Lines | 1-1 |
| Prompt Hash | `sha256:fe6c0e6eb255d8bd755443bf089e2e7ed361ed8f502b016efdd07b1f3696eb62` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 474 |
| Cost | $0.1211 |
| Lines | 1-1 |
| Prompt Hash | `sha256:99ef31978e0fea2517974d8b56305211d8cb0b76d757796db2c3c65bba56d05b` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 476 |
| Cost | $0.1211 |
| Lines | 1-1 |
| Prompt Hash | `sha256:82b5ea7a4124aa438dc937cd86e63e6c5957b5d1c26e7afb896ee24d1428f90e` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.1s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 484 |
| Cost | $0.1234 |
| Lines | 1-1 |
| Prompt Hash | `sha256:2f6143f3d44cb540a05228ba71f4ac3d8df9408eedd3f3f98de1c44104652100` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### src/commands/budget.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 489 |
| Cost | $0.1238 |
| Lines | 1-1 |
| Prompt Hash | `sha256:713bdd7d0f5dcc1ac5c81799d8b077fe8816e4e964291feffe89bc3a273d19b3` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### src/commands/budget.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 491 |
| Cost | $0.1238 |
| Lines | 1-1 |
| Prompt Hash | `sha256:e931897bc0a9e28583f3ee9d6f707fbf909be8fdca6b62ed1e4287bc0f2347c9` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### src/commands/budget.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 493 |
| Cost | $0.1238 |
| Lines | 1-1 |
| Prompt Hash | `sha256:b3165c7dbc4211455b29200daf3fd9e132b31fe04ad5a6bdf51f85be6858ded0` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### src/commands/prompt_injection.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 495 |
| Cost | $0.1238 |
| Lines | 1-1 |
| Prompt Hash | `sha256:168ef0f9507f6eef12a06640478004885cf3ca072e395d376d3acaaa4a06d098` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### src/commands/secret_rotation.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 497 |
| Cost | $0.1238 |
| Lines | 1-1 |
| Prompt Hash | `sha256:4c3074e5b6a6c04b089f80f7acfede40f03c46cfa82e75e96e88de67c3f0eeeb` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 5.0s

#### README.md | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 515 |
| Cost | $0.1248 |
| Lines | 1-1 |
| Prompt Hash | `sha256:f8699401503aacebd90616a40a463fd79290d053c7752bc29a43a2ff4a45c733` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

#### src/commands/audit.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 29 |
| Cost | $0.0091 |
| Lines | 1-1 |
| Prompt Hash | `sha256:6d23faa6b9027cbdba6392a897cacef473ed148935bee57550d314effe5bbc7e` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Session duration: 16m 48s
- Avg AI response: 4.8s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 31 |
| Cost | $0.0093 |
| Lines | 1-1 |
| Prompt Hash | `sha256:c61cc881d4dd08adf680630dfb306f66bafb6316a74a991bb40904ae404f9b44` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.6s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 34 |
| Cost | $0.0095 |
| Lines | 1-1 |
| Prompt Hash | `sha256:59f57bf057393e9cc534cc52acc7b105c57645d48c70620ff4c9612207a4e574` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.5s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 36 |
| Cost | $0.0095 |
| Lines | 1-1 |
| Prompt Hash | `sha256:91c9a2fba24d07a85ab892dfb4cf644bd33dbbb23d55848b5a96fdd3d9c7109d` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.5s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 38 |
| Cost | $0.0095 |
| Lines | 1-1 |
| Prompt Hash | `sha256:c9b91ea8e713abd9d2fb6cf813d47d70b25551b1470dbbb347deefb71fc7abb8` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.6s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 40 |
| Cost | $0.0095 |
| Lines | 1-1 |
| Prompt Hash | `sha256:3f93511adf22ee7c279781bbfdeb033b26f45ca478fd15b20d7d23459c8358f8` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.7s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `b400e0d0-35f1-4c35-9539-43ec857dded9` |
| Messages | 42 |
| Cost | $0.0095 |
| Lines | 1-1 |
| Prompt Hash | `sha256:aff1437d5c125786294eda0f7c2fec6e6f8b051d34817a726c40d0abe87bf078` |

**Prompt:**
>  12. Budget Controls — Set monthly/quarterly budgets per team, per user, per project. Alert at 80%/100%
  thresholds. Hard cap option (block AI usage after budget exceeded).
    15. Model Recommendati

- Avg AI response: 4.7s

#### ENTERPRISE_PLAN.md | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 9 |
| Cost | $0.0014 |
| Lines | 1-1 |
| Prompt Hash | `sha256:e246a8c30488ecd8166923d58f64e121eb594026d1d7a2703ad857f077b85b8b` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Session duration: 1m 36s
- Avg AI response: 18.1s

#### Cargo.toml | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 46 |
| Cost | $0.0057 |
| Lines | 1-1 |
| Prompt Hash | `sha256:08230f5ac1e31155606cd1b3e7c4952345290caa9c7bd8d0829011ff5c5fd2ad` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.3s

#### src/core/config.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 48 |
| Cost | $0.0059 |
| Lines | 1-1 |
| Prompt Hash | `sha256:ef906499be652c6415e8067cb4accabee2f5c71640fc369da5aafa982756cdd7` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.2s

#### src/core/config.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 50 |
| Cost | $0.0059 |
| Lines | 1-1 |
| Prompt Hash | `sha256:18be3bdcab7da0a3105336178000ee75b42bc419c06bd22c500b346c9117b092` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.2s

#### src/core/config.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 52 |
| Cost | $0.0059 |
| Lines | 1-1 |
| Prompt Hash | `sha256:79bc22a4583f326db1cc5943cc6daf113e91f02648e0ded708847ccc84fddca0` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.2s

#### src/commands/enterprise.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 55 |
| Cost | $0.0060 |
| Lines | 1-1 |
| Prompt Hash | `sha256:0cd5f1b389d91a7383d1a9bcb702d8ca497220182628c99d102e91956cc8cd1b` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.1s

#### src/commands/mod.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 58 |
| Cost | $0.0062 |
| Lines | 1-1 |
| Prompt Hash | `sha256:12ad9932a8012c395d6fe87575175c05352a47d796b80d7d6eb48c2ccf19c666` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.1s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 61 |
| Cost | $0.0064 |
| Lines | 1-1 |
| Prompt Hash | `sha256:41b3c217e478eb5cd721975721799757b3f341d08a773d628cc0d80899bae3d2` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.0s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 64 |
| Cost | $0.0066 |
| Lines | 1-1 |
| Prompt Hash | `sha256:55ed46b137cd60454a8ec2fb634bdc3b3bfa001efd50facb91076c16d26d5cef` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.0s

#### src/main.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 67 |
| Cost | $0.0067 |
| Lines | 1-1 |
| Prompt Hash | `sha256:132ee3ddbfd8c486f2cbfb1839e89960a12c283d7fadf792e324c96463ec7456` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 4.9s

#### README.md | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 79 |
| Cost | $0.0076 |
| Lines | 1-1 |
| Prompt Hash | `sha256:a51d195813958d682cb41f8507fbbad82a878b537ae9937fbac14dbaea607ae2` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.0s

#### README.md | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 83 |
| Cost | $0.0079 |
| Lines | 1-1 |
| Prompt Hash | `sha256:151391912ee4dc5170841591414dc42a5e2c87bf101f1a4135ddf8498c6bf8a4` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.0s

#### README.md | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `61605b06-7f2b-4528-b096-c0c4a2efb3e9` |
| Messages | 86 |
| Cost | $0.0081 |
| Lines | 1-1 |
| Prompt Hash | `sha256:927c1eb064e2f3552a85d0dfa9cb072a7a20f3d4955ba9090192cb182208a915` |

**Prompt:**
> I want to add enterprise side to help enterprise manage the da
ta better can you create a .md file to show the plan and implemetntion for the enterprise, we can keep it simple since I have less time 

- Avg AI response: 5.0s

#### src/commands/report.rs | `claude-opus-4-6` via claude

| Field | Value |
|-------|-------|
| Session ID | `7f2e447b-2c93-497e-bb43-87dc91a66e41` |
| Messages | 544 |
| Cost | $0.1366 |
| Lines | 1-1 |
| Prompt Hash | `sha256:9b61f55c862f3340ab8d4cdfc8ee59c4e5b5610aa4c7d050c47539efc3ceb592` |

**Prompt:**
> use @CHAIN_OF_THOUGHT.md and iplementt that into the exisiting code 

- Avg AI response: 4.9s

---

## Recommendations

Based on the analysis:

1. **Cost Optimization**: 100% of sessions used Opus models (higher cost).
   Consider using Sonnet for simpler tasks to reduce costs.

2. **Open-Source Opportunity**: 0.0% of sessions use open-source models.
   Consider local models (Ollama + DeepSeek/Llama) for non-sensitive code to reduce costs.


---
*Generated by [BlamePrompt](https://github.com/blameprompt/blameprompt) v0.1.0*
