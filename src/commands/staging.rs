use crate::core::receipt::Receipt;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct StagingData {
    pub receipts: Vec<Receipt>,
}

impl StagingData {
    pub fn empty() -> Self {
        StagingData {
            receipts: Vec::new(),
        }
    }
}

fn staging_dir() -> &'static str {
    ".blameprompt"
}

fn staging_path() -> String {
    format!("{}/staging.json", staging_dir())
}

fn ensure_staging_dir() {
    let dir = Path::new(staging_dir());
    if !dir.exists() {
        let _ = std::fs::create_dir_all(dir);
    }
    // Add to .gitignore if not present
    let gitignore = Path::new(".gitignore");
    let needs_entry = if gitignore.exists() {
        let content = std::fs::read_to_string(gitignore).unwrap_or_default();
        !content.lines().any(|l| l.trim() == ".blameprompt/" || l.trim() == ".blameprompt")
    } else {
        true
    };
    if needs_entry {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(gitignore)
            .ok();
        if let Some(ref mut f) = file {
            use std::io::Write;
            let _ = writeln!(f, "\n# BlamePrompt staging (auto-generated)\n.blameprompt/");
        }
    }
}

pub fn add_receipt(receipt: &Receipt) {
    ensure_staging_dir();
    let path = staging_path();
    let tmp_path = format!("{}/staging.json.tmp", staging_dir());

    let mut data = read_staging();
    data.receipts.push(receipt.clone());

    if let Ok(json) = serde_json::to_string_pretty(&data) {
        let _ = std::fs::write(&tmp_path, &json);
        let _ = std::fs::rename(&tmp_path, &path);
    }
}

pub fn read_staging() -> StagingData {
    let path = staging_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| StagingData::empty()),
        Err(_) => StagingData::empty(),
    }
}

#[allow(dead_code)]
pub fn clear_staging() {
    ensure_staging_dir();
    let path = staging_path();
    let data = StagingData::empty();
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        let _ = std::fs::write(&path, json);
    }
}

pub fn manual_tag(
    file: &str,
    start_line: u32,
    end_line: u32,
    provider: &str,
    model: &str,
    prompt: &str,
) {
    let user = get_git_user();
    let receipt = Receipt {
        id: Receipt::new_id(),
        provider: provider.to_string(),
        model: model.to_string(),
        session_id: "manual".to_string(),
        prompt_summary: crate::core::redact::redact_secrets(prompt),
        prompt_hash: format!("sha256:{}", compute_hash(prompt)),
        message_count: 1,
        cost_usd: 0.0,
        timestamp: Utc::now(),
        session_start: None,
        session_end: None,
        session_duration_secs: None,
        ai_response_time_secs: None,
        user,
        file_path: file.to_string(),
        line_range: (start_line, end_line),
        parent_receipt_id: None,
        conversation: None,
    };

    add_receipt(&receipt);
    println!(
        "Tagged {}:{}-{} as {} ({})",
        file, start_line, end_line, provider, model
    );
}

fn get_git_user() -> String {
    let name = std::process::Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let email = std::process::Command::new("git")
        .args(["config", "user.email"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown@unknown".to_string());

    format!("{} <{}>", name, email)
}

fn compute_hash(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staging_roundtrip() {
        let data = StagingData::empty();
        let json = serde_json::to_string(&data).unwrap();
        let parsed: StagingData = serde_json::from_str(&json).unwrap();
        assert!(parsed.receipts.is_empty());
    }
}
