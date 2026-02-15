use serde_json::json;
use std::path::PathBuf;

fn settings_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    Ok(home.join(".claude").join("settings.json"))
}

fn blameprompt_binary_path() -> String {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "blameprompt".to_string())
}

pub fn install() -> Result<(), String> {
    let path = settings_path()?;

    // Create ~/.claude/ if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create ~/.claude/: {}", e))?;
    }

    // Read existing settings or create empty object
    let mut settings: serde_json::Value = if path.exists() {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Cannot read settings: {}", e))?;
        serde_json::from_str(&content).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    // Check if BlamePrompt hooks already installed
    let settings_str = serde_json::to_string(&settings).unwrap_or_default();
    if settings_str.contains("blameprompt") {
        println!("BlamePrompt hooks already installed in ~/.claude/settings.json");
        return Ok(());
    }

    let binary = blameprompt_binary_path();
    let command = format!("{} checkpoint claude --hook-input stdin", binary);

    let hook_entry = json!({
        "matcher": "Write|Edit|MultiEdit",
        "hooks": [{
            "type": "command",
            "command": command
        }]
    });

    // Ensure hooks object exists
    if settings.get("hooks").is_none() {
        settings["hooks"] = json!({});
    }

    let hooks = settings.get_mut("hooks").unwrap();

    // Add PreToolUse
    if hooks.get("PreToolUse").is_none() {
        hooks["PreToolUse"] = json!([]);
    }
    if let Some(arr) = hooks.get_mut("PreToolUse").and_then(|v| v.as_array_mut()) {
        arr.push(hook_entry.clone());
    }

    // Add PostToolUse
    if hooks.get("PostToolUse").is_none() {
        hooks["PostToolUse"] = json!([]);
    }
    if let Some(arr) = hooks.get_mut("PostToolUse").and_then(|v| v.as_array_mut()) {
        arr.push(hook_entry);
    }

    // Write back
    let json_str = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    std::fs::write(&path, json_str)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    println!("Installed Claude Code hooks in {}", path.display());
    Ok(())
}

pub fn uninstall() -> Result<(), String> {
    let path = settings_path()?;

    if !path.exists() {
        println!("  [skip] No ~/.claude/settings.json found");
        return Ok(());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read settings: {}", e))?;
    let mut settings: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    if let Some(hooks) = settings.get_mut("hooks") {
        for event in &["PreToolUse", "PostToolUse"] {
            if let Some(arr) = hooks.get_mut(*event).and_then(|v| v.as_array_mut()) {
                arr.retain(|entry| {
                    let json_str = serde_json::to_string(entry).unwrap_or_default();
                    !json_str.contains("blameprompt")
                });
            }
        }

        // Clean up empty arrays
        if let Some(hooks_obj) = hooks.as_object_mut() {
            hooks_obj.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
        }
        if hooks.as_object().map_or(false, |o| o.is_empty()) {
            settings.as_object_mut().unwrap().remove("hooks");
        }
    }

    let json_str = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    std::fs::write(&path, json_str)
        .map_err(|e| format!("Failed to write: {}", e))?;

    println!("  [done] Removed Claude Code hooks from ~/.claude/settings.json");
    Ok(())
}
