use crate::commands::audit;
use crate::core::model_classifier;
use crate::core::redact;
use chrono::Utc;
use std::collections::{HashMap, HashSet};

fn relative_path(path: &str) -> String {
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_str = cwd.to_string_lossy();
        if path.starts_with(cwd_str.as_ref()) {
            let rel = &path[cwd_str.len()..];
            return rel.strip_prefix('/').unwrap_or(rel).to_string();
        }
    }
    path.to_string()
}

/// Generate SOC2 / ISO 27001 compliance evidence package.
pub fn run_soc2(output: &str, from: Option<&str>, to: Option<&str>) {
    let entries = match audit::collect_all_entries(from, to, None, true) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let all_receipts: Vec<_> = entries.iter().flat_map(|e| &e.receipts).collect();

    let mut users: HashSet<String> = HashSet::new();
    let mut providers: HashSet<String> = HashSet::new();
    let mut models: HashSet<String> = HashSet::new();
    let mut sessions: HashSet<String> = HashSet::new();
    let mut total_cost = 0.0f64;
    let mut total_lines = 0u32;

    for r in &all_receipts {
        users.insert(r.user.clone());
        providers.insert(r.provider.clone());
        models.insert(r.model.clone());
        sessions.insert(r.session_id.clone());
        total_cost += r.cost_usd;
        if r.line_range.1 >= r.line_range.0 {
            total_lines += r.line_range.1 - r.line_range.0 + 1;
        }
    }

    let mut md = String::new();

    // Header
    md.push_str("# AI Code Generation — Compliance Evidence Package\n\n");
    md.push_str(&format!("> **Report Type**: SOC 2 Type II / ISO 27001 Evidence\n"));
    md.push_str(&format!("> **Generated**: {}\n", now));
    md.push_str(&format!("> **Tool**: BlamePrompt v0.1.0\n"));
    if let Some(f) = from {
        md.push_str(&format!("> **Period Start**: {}\n", f));
    }
    if let Some(t) = to {
        md.push_str(&format!("> **Period End**: {}\n", t));
    }
    md.push_str("\n---\n\n");

    // 1. Executive Summary
    md.push_str("## 1. Executive Summary\n\n");
    md.push_str("This document provides a complete audit trail of AI-assisted code generation\n");
    md.push_str("activities within this repository. It serves as evidence for SOC 2 and ISO 27001\n");
    md.push_str("compliance requirements related to change management, access control, and\n");
    md.push_str("third-party service usage.\n\n");

    md.push_str("| Metric | Value |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| Total AI-assisted commits | {} |\n", entries.len()));
    md.push_str(&format!("| Total AI receipts | {} |\n", all_receipts.len()));
    md.push_str(&format!("| Unique users | {} |\n", users.len()));
    md.push_str(&format!("| AI providers used | {} |\n", providers.len()));
    md.push_str(&format!("| AI models used | {} |\n", models.len()));
    md.push_str(&format!("| Unique sessions | {} |\n", sessions.len()));
    md.push_str(&format!("| Total AI-generated lines | {} |\n", total_lines));
    md.push_str(&format!("| Estimated cost | ${:.2} |\n\n", total_cost));

    // 2. Access Control — Who Used AI
    md.push_str("## 2. Access Control — Who Used AI\n\n");
    md.push_str("| User | Sessions | Receipts | Lines Generated | Cost |\n");
    md.push_str("|------|----------|----------|-----------------|------|\n");
    let mut user_stats: HashMap<String, (u32, u32, u32, f64)> = HashMap::new();
    for r in &all_receipts {
        let entry = user_stats.entry(r.user.clone()).or_insert((0, 0, 0, 0.0));
        entry.1 += 1;
        let lines = if r.line_range.1 >= r.line_range.0 { r.line_range.1 - r.line_range.0 + 1 } else { 0 };
        entry.2 += lines;
        entry.3 += r.cost_usd;
    }
    // Count sessions per user
    let mut user_sessions: HashMap<String, HashSet<String>> = HashMap::new();
    for r in &all_receipts {
        user_sessions.entry(r.user.clone()).or_default().insert(r.session_id.clone());
    }
    for (user, (_, receipts, lines, cost)) in &user_stats {
        let sess_count = user_sessions.get(user).map(|s| s.len()).unwrap_or(0);
        md.push_str(&format!("| {} | {} | {} | {} | ${:.4} |\n", user, sess_count, receipts, lines, cost));
    }
    md.push_str("\n");

    // 3. Data Sent to AI Providers
    md.push_str("## 3. Data Sent to AI Providers\n\n");
    md.push_str("### Providers Used\n\n");
    md.push_str("| Provider | Model | Sessions | Data Classification |\n");
    md.push_str("|----------|-------|----------|--------------------|\n");
    let mut provider_models: HashMap<(String, String), u32> = HashMap::new();
    for r in &all_receipts {
        *provider_models.entry((r.provider.clone(), r.model.clone())).or_insert(0) += 1;
    }
    for ((provider, model), count) in &provider_models {
        let c = model_classifier::classify(model);
        let data_class = match c.deployment {
            model_classifier::ModelDeployment::Local => "Internal (local processing)",
            model_classifier::ModelDeployment::Cloud => "External (sent to third-party API)",
        };
        md.push_str(&format!("| {} | {} | {} | {} |\n", provider, c.display_name, count, data_class));
    }
    md.push_str("\n");

    md.push_str("### Sensitive Data Controls\n\n");
    md.push_str("All prompts are processed through BlamePrompt's redaction engine before storage.\n");
    md.push_str("The following secret types are automatically detected and redacted:\n\n");
    md.push_str("- API keys (Anthropic, OpenAI, Stripe, AWS)\n");
    md.push_str("- Passwords and connection strings\n");
    md.push_str("- Bearer tokens and authentication credentials\n");
    md.push_str("- High-entropy strings (potential secrets)\n\n");

    // Scan prompts for residual secrets
    let mut secret_count = 0;
    for r in &all_receipts {
        let report = redact::redact_with_report(&r.prompt_summary);
        secret_count += report.detections.len();
    }
    md.push_str(&format!("**Post-redaction secret scan**: {} potential secret(s) detected in stored prompts.\n\n", secret_count));

    // 4. What Code Was Generated
    md.push_str("## 4. Code Generated — Change Log\n\n");
    md.push_str("| Date | User | Provider | Model | File | Lines | Prompt |\n");
    md.push_str("|------|------|----------|-------|------|-------|--------|\n");
    for entry in &entries {
        for r in &entry.receipts {
            let date = if entry.commit_date.len() >= 10 { &entry.commit_date[..10] } else { &entry.commit_date };
            let prompt_short: String = r.prompt_summary.chars().take(50).collect();
            let rel_file = relative_path(&r.file_path);
            md.push_str(&format!("| {} | {} | {} | {} | {} | {}-{} | {} |\n",
                date, r.user, r.provider, r.model, rel_file,
                r.line_range.0, r.line_range.1, prompt_short));
        }
    }
    md.push_str("\n");

    // 5. Timeline — group by session to avoid repeating same session duration
    md.push_str("## 5. Timeline — When AI Was Used\n\n");
    let mut sessions_shown: std::collections::HashSet<String> = std::collections::HashSet::new();
    for entry in &entries {
        for r in &entry.receipts {
            let sha_short = if entry.commit_sha.len() >= 8 { &entry.commit_sha[..8] } else { &entry.commit_sha };
            let rel_file = relative_path(&r.file_path);

            // Only show session duration on first receipt per session
            let duration_str = if sessions_shown.insert(r.session_id.clone()) {
                r.session_duration_secs
                    .map(|d| format!(", session duration: {}", crate::core::session_stats::format_duration(d)))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            md.push_str(&format!("- **{}** `{}` — {} used {} on `{}`{}\n",
                r.timestamp.format("%Y-%m-%d %H:%M"), sha_short, r.user, r.model, rel_file, duration_str));
        }
    }
    md.push_str("\n");

    // 6. Attestation
    md.push_str("## 6. Attestation\n\n");
    md.push_str("This report was generated automatically by BlamePrompt from Git Notes attached\n");
    md.push_str("to repository commits. The data has not been manually modified. Each receipt\n");
    md.push_str("includes a SHA-256 hash of the original conversation for integrity verification.\n\n");
    md.push_str("---\n\n");
    md.push_str("*Generated by [BlamePrompt](https://github.com/anthropics/blameprompt) — AI Code Provenance Tracking*\n");

    match std::fs::write(output, &md) {
        Ok(_) => println!("SOC2/ISO 27001 compliance export written to {}", output),
        Err(e) => eprintln!("Error writing report: {}", e),
    }
}

/// Generate GDPR Data Flow Map and DPIA report.
pub fn run_gdpr(output: &str) {
    let entries = match audit::collect_all_entries(None, None, None, true) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let all_receipts: Vec<_> = entries.iter().flat_map(|e| &e.receipts).collect();

    // PII detection patterns
    let pii_patterns: Vec<(&str, &str)> = vec![
        (r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "Email Address"),
        (r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b", "Phone Number"),
        (r"\b\d{3}-\d{2}-\d{4}\b", "SSN"),
        (r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b", "IP Address"),
        (r#"(?i)\b(name|customer|user|employee)\s*[:=]\s*["']?\w+"#, "Personal Name Reference"),
        (r"(?i)\b(address|street|city|zip)\s*[:=]", "Physical Address Reference"),
    ];

    let mut pii_findings: Vec<(String, String, String, String)> = Vec::new();
    let mut providers_data_flow: HashMap<String, Vec<String>> = HashMap::new();

    for r in &all_receipts {
        // Check prompt for PII
        for (pattern, pii_type) in &pii_patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if re.is_match(&r.prompt_summary) {
                    pii_findings.push((
                        r.user.clone(),
                        pii_type.to_string(),
                        r.provider.clone(),
                        relative_path(&r.file_path),
                    ));
                }
            }
        }

        // Check conversation turns for PII
        if let Some(ref turns) = r.conversation {
            for t in turns {
                if t.role == "user" {
                    for (pattern, pii_type) in &pii_patterns {
                        if let Ok(re) = regex::Regex::new(pattern) {
                            if re.is_match(&t.content) {
                                pii_findings.push((
                                    r.user.clone(),
                                    pii_type.to_string(),
                                    r.provider.clone(),
                                    relative_path(&r.file_path),
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Map data flows
        let c = model_classifier::classify(&r.model);
        let destination = match c.deployment {
            model_classifier::ModelDeployment::Cloud => format!("{} API ({})", c.vendor, c.display_name),
            model_classifier::ModelDeployment::Local => format!("Local ({}) — no external transfer", c.display_name),
        };
        providers_data_flow.entry(destination).or_default().push(r.user.clone());
    }

    let mut md = String::new();

    md.push_str("# GDPR Data Flow Map — AI Code Generation\n\n");
    md.push_str(&format!("> **Report Type**: Data Processing Impact Assessment (DPIA)\n"));
    md.push_str(&format!("> **Generated**: {}\n", now));
    md.push_str("> **Regulation**: GDPR (EU 2016/679)\n\n");
    md.push_str("---\n\n");

    // 1. Data Flow Overview
    md.push_str("## 1. Data Flow Overview\n\n");
    md.push_str("```\n");
    md.push_str("Developer Prompt  ──►  BlamePrompt Redaction Engine  ──►  AI Provider API\n");
    md.push_str("     │                        │                              │\n");
    md.push_str("     │                   Secrets removed              Code generated\n");
    md.push_str("     │                   PII flagged                       │\n");
    md.push_str("     ▼                        ▼                            ▼\n");
    md.push_str("  Git Notes             Audit Trail                  Source Code\n");
    md.push_str("(local storage)      (local storage)            (local repository)\n");
    md.push_str("```\n\n");

    // 2. Data Processors (AI Providers)
    md.push_str("## 2. Data Processors (AI Providers)\n\n");
    md.push_str("| Processor | Data Transferred | Users | Processing Location |\n");
    md.push_str("|-----------|-----------------|-------|--------------------|\n");
    for (destination, users_list) in &providers_data_flow {
        let unique_users: HashSet<_> = users_list.iter().collect();
        let location = if destination.contains("Local") { "On-premise" } else { "Cloud (third-party)" };
        md.push_str(&format!("| {} | Code prompts, context | {} | {} |\n",
            destination, unique_users.len(), location));
    }
    md.push_str("\n");

    // 3. PII Detection
    md.push_str("## 3. Personal Data in Prompts (PII Detection)\n\n");
    if pii_findings.is_empty() {
        md.push_str("**No PII detected** in stored prompts. The redaction engine appears to be\n");
        md.push_str("effectively filtering personal data before storage.\n\n");
    } else {
        md.push_str(&format!("**{} potential PII instance(s)** detected in stored prompts:\n\n", pii_findings.len()));
        md.push_str("| User | PII Type | Sent To | Context File |\n");
        md.push_str("|------|----------|---------|-------------|\n");
        for (user, pii_type, provider, file) in &pii_findings {
            md.push_str(&format!("| {} | {} | {} | {} |\n", user, pii_type, provider, file));
        }
        md.push_str("\n");
        md.push_str("**Action Required**: Review these prompts and ensure PII was necessary for the\n");
        md.push_str("AI task. Consider adding custom redaction patterns to `.blamepromptrc` to\n");
        md.push_str("automatically redact this type of data.\n\n");
    }

    // 4. Data Retention
    md.push_str("## 4. Data Retention\n\n");
    md.push_str("| Data Type | Storage Location | Retention | Encryption |\n");
    md.push_str("|-----------|-----------------|-----------|------------|\n");
    md.push_str("| Redacted prompts | Git Notes (local) | Matches git history | At-rest via filesystem |\n");
    md.push_str("| Full prompts | AI provider logs | Per provider policy | Provider-managed |\n");
    md.push_str("| Staging receipts | .blameprompt/staging.json | Until commit | None (gitignored) |\n");
    md.push_str("| SQLite cache | ~/.blameprompt/prompts.db | Manual cleanup | None |\n\n");

    // 5. Risk Assessment
    md.push_str("## 5. Risk Assessment\n\n");
    let cloud_count = providers_data_flow.keys().filter(|k| !k.contains("Local")).count();
    let risk_level = if !pii_findings.is_empty() && cloud_count > 0 {
        "HIGH"
    } else if cloud_count > 0 {
        "MEDIUM"
    } else {
        "LOW"
    };
    md.push_str(&format!("**Overall Risk Level**: {}\n\n", risk_level));
    md.push_str("| Risk | Likelihood | Impact | Mitigation |\n");
    md.push_str("|------|-----------|--------|------------|\n");
    md.push_str("| PII sent to AI provider | Medium | High | Redaction engine + custom patterns |\n");
    md.push_str("| Prompt data breach at provider | Low | High | Use local models for sensitive data |\n");
    md.push_str("| Unauthorized AI usage | Low | Medium | BlamePrompt audit trail |\n");
    md.push_str("| Non-compliant data retention | Medium | Medium | Configure provider retention policies |\n\n");

    // 6. Recommendations
    md.push_str("## 6. Recommendations\n\n");
    md.push_str("1. **Enable custom PII patterns** — Add organization-specific PII patterns to `.blamepromptrc`.\n");
    md.push_str("2. **Use local models** — For sensitive projects, route through local models (Ollama/LM Studio).\n");
    md.push_str("3. **Review provider DPAs** — Ensure Data Processing Agreements are in place with all AI providers.\n");
    md.push_str("4. **Regular DPIA updates** — Re-run this scan monthly or after onboarding new AI tools.\n");
    md.push_str("5. **Data subject rights** — Ensure ability to delete AI prompt history upon request.\n\n");

    md.push_str("---\n\n");
    md.push_str("*Generated by [BlamePrompt](https://github.com/anthropics/blameprompt) — GDPR Data Flow Mapper*\n");

    match std::fs::write(output, &md) {
        Ok(_) => println!("GDPR data flow map written to {}", output),
        Err(e) => eprintln!("Error writing report: {}", e),
    }
}
