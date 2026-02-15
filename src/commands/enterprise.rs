use crate::commands::audit;
use crate::core::config;
use crate::core::receipt::Receipt;
use serde::{Deserialize, Serialize};

// ── Payloads ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct SyncPayload {
    org: String,
    team: String,
    repo: String,
    receipts: Vec<SyncReceipt>,
}

#[derive(Debug, Serialize)]
struct SyncReceipt {
    #[serde(flatten)]
    receipt: Receipt,
    commit_sha: String,
    commit_date: String,
    commit_author: String,
}

#[derive(Debug, Deserialize)]
struct SyncResponse {
    accepted: usize,
    duplicates: usize,
    #[serde(default)]
    message: String,
}

#[derive(Debug, Deserialize)]
struct PolicyViolation {
    rule: String,
    severity: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct PolicyCheckResponse {
    compliant: bool,
    violations: Vec<PolicyViolation>,
}

// ── Helpers ─────────────────────────────────────────────────────

fn get_repo_name() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let path = String::from_utf8_lossy(&o.stdout).trim().to_string();
            std::path::Path::new(&path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        }
        _ => "unknown".to_string(),
    }
}

fn resolve_api_key(cfg: &config::EnterpriseConfig) -> Result<String, String> {
    std::env::var(&cfg.api_key_env).map_err(|_| {
        format!(
            "Enterprise API key not found. Set the {} environment variable.",
            cfg.api_key_env
        )
    })
}

fn require_config(cfg: &config::EnterpriseConfig) -> Result<(), String> {
    if !cfg.enabled {
        return Err(
            "Enterprise is not enabled. Add this to your .blamepromptrc:\n\n\
             [enterprise]\n\
             enabled = true\n\
             api_url = \"https://your-server.example.com\"\n"
                .to_string(),
        );
    }
    if cfg.api_url.is_empty() {
        return Err(
            "Enterprise api_url is not configured. Set it in your .blamepromptrc:\n\n\
             [enterprise]\n\
             api_url = \"https://your-server.example.com\"\n"
                .to_string(),
        );
    }
    Ok(())
}

// ── Subcommands ─────────────────────────────────────────────────

/// Sync local receipts to the enterprise server.
pub fn sync(from: Option<&str>, to: Option<&str>) {
    let cfg = config::load_config();
    if let Err(e) = require_config(&cfg.enterprise) {
        eprintln!("Error: {}", e);
        return;
    }

    let api_key = match resolve_api_key(&cfg.enterprise) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Collect all receipts (committed + staged)
    let entries = match audit::collect_all_entries(from, to, None, true) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error collecting receipts: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        println!("No receipts to sync.");
        return;
    }

    let repo = get_repo_name();
    let sync_receipts: Vec<SyncReceipt> = entries
        .iter()
        .flat_map(|entry| {
            entry.receipts.iter().map(move |r| SyncReceipt {
                receipt: r.clone(),
                commit_sha: entry.commit_sha.clone(),
                commit_date: entry.commit_date.clone(),
                commit_author: entry.commit_author.clone(),
            })
        })
        .collect();

    let total = sync_receipts.len();
    let payload = SyncPayload {
        org: cfg.enterprise.org.clone(),
        team: cfg.enterprise.team.clone(),
        repo,
        receipts: sync_receipts,
    };

    println!("Syncing {} receipts to {}...", total, cfg.enterprise.api_url);

    let url = format!("{}/api/v1/receipts", cfg.enterprise.api_url.trim_end_matches('/'));
    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&payload).unwrap_or_default());

    match resp {
        Ok(r) => {
            if let Ok(body) = r.into_json::<SyncResponse>() {
                println!(
                    "[Enterprise] Sync complete: {} accepted, {} duplicates skipped.",
                    body.accepted, body.duplicates
                );
                if !body.message.is_empty() {
                    println!("  Server: {}", body.message);
                }
            } else {
                println!("[Enterprise] Sync complete ({} receipts sent).", total);
            }
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            eprintln!("Error: Server returned {} — {}", code, body);
        }
        Err(e) => {
            eprintln!("Error: Failed to connect to enterprise server: {}", e);
            eprintln!("  Check that api_url is correct: {}", cfg.enterprise.api_url);
        }
    }
}

/// Show current enterprise configuration status.
pub fn status() {
    let cfg = config::load_config();
    let ent = &cfg.enterprise;

    println!("Enterprise Configuration");
    println!("========================");
    println!("  Enabled:        {}", ent.enabled);
    println!("  API URL:        {}", if ent.api_url.is_empty() { "(not set)" } else { &ent.api_url });
    println!("  API Key Env:    {}", ent.api_key_env);
    println!("  Sync on Commit: {}", ent.sync_on_commit);
    println!("  Organization:   {}", if ent.org.is_empty() { "(not set)" } else { &ent.org });
    println!("  Team:           {}", if ent.team.is_empty() { "(not set)" } else { &ent.team });

    // Check if API key is set
    match std::env::var(&ent.api_key_env) {
        Ok(_) => println!("  API Key:        (set)"),
        Err(_) => println!("  API Key:        (not set)"),
    }

    if !ent.enabled {
        println!("\nEnterprise is disabled. Enable it in .blamepromptrc:");
        println!("  [enterprise]");
        println!("  enabled = true");
        println!("  api_url = \"https://your-server.example.com\"");
    }
}

/// Export all receipts as a single JSON bundle for enterprise ingestion.
pub fn export(
    output: &str,
    from: Option<&str>,
    to: Option<&str>,
    author: Option<&str>,
) {
    let entries = match audit::collect_all_entries(from, to, author, true) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error collecting receipts: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        println!("No receipts to export.");
        return;
    }

    let repo = get_repo_name();
    let total_receipts: usize = entries.iter().map(|e| e.receipts.len()).sum();
    let total_cost: f64 = entries.iter().map(|e| e.total_cost_usd).sum();

    let export_data = serde_json::json!({
        "blameprompt_version": "0.1.0",
        "export_type": "enterprise",
        "repo": repo,
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "summary": {
            "total_commits": entries.len(),
            "total_receipts": total_receipts,
            "total_cost_usd": total_cost,
        },
        "entries": entries,
    });

    let json = serde_json::to_string_pretty(&export_data).unwrap_or_default();

    match std::fs::write(output, &json) {
        Ok(_) => {
            println!(
                "[Enterprise] Exported {} receipts from {} commits to {}",
                total_receipts,
                entries.len(),
                output,
            );
            println!("  Total cost: ${:.2}", total_cost);
        }
        Err(e) => eprintln!("Error writing export file: {}", e),
    }
}

/// Check local receipts against enterprise policy rules.
pub fn policy_check() {
    let cfg = config::load_config();
    if let Err(e) = require_config(&cfg.enterprise) {
        eprintln!("Error: {}", e);
        return;
    }

    let api_key = match resolve_api_key(&cfg.enterprise) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Collect staged/uncommitted receipts (policy checks are pre-commit)
    let staged = audit::collect_staged_entries();
    if staged.is_empty() {
        println!("[Enterprise] No staged receipts to check.");
        return;
    }

    let receipts: Vec<&Receipt> = staged.iter().flat_map(|e| &e.receipts).collect();
    let total_cost: f64 = receipts.iter().map(|r| r.cost_usd).sum();

    let check_payload = serde_json::json!({
        "repo": get_repo_name(),
        "receipt_count": receipts.len(),
        "total_cost_usd": total_cost,
        "models": receipts.iter().map(|r| r.model.as_str()).collect::<std::collections::HashSet<_>>(),
        "receipts": receipts,
    });

    let url = format!(
        "{}/api/v1/policies/check",
        cfg.enterprise.api_url.trim_end_matches('/')
    );

    println!("Checking {} receipts against enterprise policies...", receipts.len());

    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&check_payload).unwrap_or_default());

    match resp {
        Ok(r) => {
            if let Ok(body) = r.into_json::<PolicyCheckResponse>() {
                if body.compliant {
                    println!("[Enterprise] All policies passed.");
                } else {
                    println!("[Enterprise] Policy violations found:\n");
                    for v in &body.violations {
                        println!("  [{:>8}] {}: {}", v.severity.to_uppercase(), v.rule, v.message);
                    }
                    println!(
                        "\n{} violation(s) detected. Review your enterprise policies.",
                        body.violations.len()
                    );
                }
            } else {
                println!("[Enterprise] Policy check complete (could not parse server response).");
            }
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            eprintln!("Error: Server returned {} — {}", code, body);
        }
        Err(e) => {
            eprintln!("Error: Failed to connect to enterprise server: {}", e);
        }
    }
}
