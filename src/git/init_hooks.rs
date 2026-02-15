use crate::{integrations::claude_hooks, git::hooks};
use std::path::Path;

pub fn install_git_template() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let template_dir = home.join(".blameprompt").join("git-template");
    let hooks_dir = template_dir.join("hooks");

    std::fs::create_dir_all(&hooks_dir)
        .map_err(|e| format!("Cannot create template dir: {}", e))?;

    // Write post-commit hook template
    let post_commit = hooks_dir.join("post-commit");
    let post_commit_content = format!("#!/bin/sh\n\n{}", hooks::post_commit_hook_content());
    std::fs::write(&post_commit, &post_commit_content)
        .map_err(|e| format!("Cannot write post-commit: {}", e))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&post_commit, std::fs::Permissions::from_mode(0o755));
    }

    // Write pre-commit hook template
    let pre_commit = hooks_dir.join("pre-commit");
    let pre_commit_content = format!("#!/bin/sh\n\n{}", hooks::pre_commit_hook_content());
    std::fs::write(&pre_commit, &pre_commit_content)
        .map_err(|e| format!("Cannot write pre-commit: {}", e))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&pre_commit, std::fs::Permissions::from_mode(0o755));
    }

    // Check if init.templateDir is already set to something else
    let existing = std::process::Command::new("git")
        .args(["config", "--global", "--get", "init.templateDir"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if !existing.is_empty() && !existing.contains(".blameprompt") {
        println!("  [warn] init.templateDir already set to: {}", existing);
        println!("         Overriding with BlamePrompt template.");
    }

    // Configure git to use this template directory
    let status = std::process::Command::new("git")
        .args([
            "config", "--global", "init.templateDir",
            &template_dir.to_string_lossy(),
        ])
        .status()
        .map_err(|e| format!("Cannot set git config: {}", e))?;

    if !status.success() {
        return Err("Failed to set init.templateDir".to_string());
    }

    println!("  [done] Git template directory configured at {}", template_dir.display());
    println!("         All future 'git init' will auto-include BlamePrompt hooks");
    Ok(())
}

pub fn auto_init_blameprompt(repo_root: &str) -> Result<(), String> {
    let bp_dir = Path::new(repo_root).join(".blameprompt");

    if !bp_dir.exists() {
        std::fs::create_dir_all(&bp_dir)
            .map_err(|e| format!("Cannot create .blameprompt/: {}", e))?;
    }

    let staging = bp_dir.join("staging.json");
    if !staging.exists() {
        std::fs::write(&staging, "{\"receipts\":[]}")
            .map_err(|e| format!("Cannot create staging.json: {}", e))?;
    }

    // Add to .gitignore if not present
    let gitignore = Path::new(repo_root).join(".gitignore");
    let needs_entry = if gitignore.exists() {
        let content = std::fs::read_to_string(&gitignore).unwrap_or_default();
        !content.lines().any(|l| l.trim() == ".blameprompt/" || l.trim() == ".blameprompt")
    } else {
        true
    };
    if needs_entry {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore)
            .map_err(|e| format!("Cannot write .gitignore: {}", e))?;
        use std::io::Write;
        writeln!(file, "\n# BlamePrompt staging (auto-generated)\n.blameprompt/")
            .map_err(|e| format!("Cannot append to .gitignore: {}", e))?;
    }

    Ok(())
}

pub fn run_init(global: bool) -> Result<(), String> {
    if global {
        install_git_template()?;
        claude_hooks::install()?;
        println!("\n[BlamePrompt] Global init complete.");
        println!("  All future 'git init' repos will auto-track AI prompts.");
        println!("  Claude Code hooks installed globally.");
    } else {
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Cannot get cwd: {}", e))?;

        git2::Repository::discover(&cwd)
            .map_err(|_| "Not inside a git repository. Run 'git init' first.".to_string())?;

        auto_init_blameprompt(cwd.to_str().unwrap())?;
        hooks::install_hooks()?;

        println!("\n[BlamePrompt] Initialized in {}.", cwd.display());
        println!("  .blameprompt/ directory created");
        println!("  Git hooks installed");
        println!("  Prompt tracking active from now — every AI file edit will be captured.");
    }

    Ok(())
}
