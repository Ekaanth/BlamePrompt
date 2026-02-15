use std::io::Write;
use std::path::Path;

pub fn run(keep_notes: bool, purge: bool) -> Result<(), String> {
    println!("Uninstalling BlamePrompt...\n");

    // Purge confirmation
    if purge {
        let note_count = count_git_notes();
        println!("WARNING: This will permanently delete:");
        println!("  - All {} Git Note(s) (receipt history)", note_count);
        println!("  - SQLite database (~/.blameprompt/prompts.db)");
        println!("  - All hooks and local data");
        print!("Continue? [y/N] ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap_or(0);
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    // Step 1: Remove Claude Code hooks
    crate::integrations::claude_hooks::uninstall()?;

    // Step 2: Remove git hooks
    crate::git::hooks::uninstall_hooks()?;

    // Step 3: Remove staging directory
    remove_staging_dir()?;

    // Step 4: Remove .gitignore entry
    remove_gitignore_entry()?;

    // Step 5: Remove global data
    remove_global_data()?;

    // Step 6: Remove git template directory
    remove_git_template()?;

    // Step 7: Remove Git Notes (only with --purge)
    if purge && !keep_notes {
        remove_git_notes()?;
    } else {
        println!("  [kept] Git Notes (refs/notes/blameprompt)");
        println!("         To remove manually: git notes --ref=blameprompt prune");
    }

    // Step 8: Remove binary (only with --purge)
    if purge {
        remove_binary()?;
    }

    println!("\nBlamePrompt uninstalled successfully.");
    if !purge {
        println!("Note: Git Notes preserved. Your receipt history is still in the repo.");
        println!("      Run 'blameprompt uninstall --purge' to remove everything.");
    }

    Ok(())
}

fn remove_staging_dir() -> Result<(), String> {
    let staging = Path::new(".blameprompt");
    if staging.exists() {
        std::fs::remove_dir_all(staging)
            .map_err(|e| format!("Cannot remove .blameprompt/: {}", e))?;
        println!("  [done] Removed .blameprompt/ directory");
    } else {
        println!("  [skip] No .blameprompt/ directory found");
    }
    Ok(())
}

fn remove_gitignore_entry() -> Result<(), String> {
    let gitignore = Path::new(".gitignore");
    if !gitignore.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(gitignore)
        .map_err(|e| format!("Cannot read .gitignore: {}", e))?;

    let cleaned: Vec<&str> = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed != ".blameprompt/" && trimmed != ".blameprompt"
                && !trimmed.contains("# BlamePrompt staging")
        })
        .collect();

    if cleaned.len() < content.lines().count() {
        let mut result = cleaned.join("\n");
        // Remove trailing blank lines from cleanup
        while result.ends_with("\n\n") {
            result.pop();
        }
        if !result.ends_with('\n') {
            result.push('\n');
        }
        std::fs::write(gitignore, result)
            .map_err(|e| format!("Cannot write .gitignore: {}", e))?;
        println!("  [done] Removed .blameprompt/ from .gitignore");
    }
    Ok(())
}

fn remove_global_data() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let global_dir = home.join(".blameprompt");
    if global_dir.exists() {
        std::fs::remove_dir_all(&global_dir)
            .map_err(|e| format!("Cannot remove ~/.blameprompt/: {}", e))?;
        println!("  [done] Removed ~/.blameprompt/ (SQLite database + config)");
    } else {
        println!("  [skip] No ~/.blameprompt/ directory found");
    }
    Ok(())
}

fn remove_git_template() -> Result<(), String> {
    // Check if init.templateDir points to our template
    let output = std::process::Command::new("git")
        .args(["config", "--global", "--get", "init.templateDir"])
        .output();

    if let Ok(out) = output {
        let current = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if current.contains(".blameprompt") {
            let _ = std::process::Command::new("git")
                .args(["config", "--global", "--unset", "init.templateDir"])
                .status();
            println!("  [done] Reset git init.templateDir");
        }
    }
    Ok(())
}

fn remove_git_notes() -> Result<(), String> {
    let list = std::process::Command::new("git")
        .args(["notes", "--ref", "refs/notes/blameprompt", "list"])
        .output();

    if let Ok(output) = list {
        if output.status.success() {
            let notes = String::from_utf8_lossy(&output.stdout);
            let count = notes.lines().count();

            let _ = std::process::Command::new("git")
                .args(["update-ref", "-d", "refs/notes/blameprompt"])
                .output();

            println!("  [done] Removed {} Git Note(s) from refs/notes/blameprompt", count);

            let _ = std::process::Command::new("git")
                .args([
                    "config", "--unset", "remote.origin.fetch",
                    "+refs/notes/blameprompt:refs/notes/blameprompt",
                ])
                .output();
        } else {
            println!("  [skip] No Git Notes found");
        }
    } else {
        println!("  [skip] No Git Notes found");
    }
    Ok(())
}

fn remove_binary() -> Result<(), String> {
    if let Ok(exe_path) = std::env::current_exe() {
        println!("  [info] Binary location: {}", exe_path.display());
        println!("  [info] To remove the binary, run:");
        println!("         cargo uninstall blameprompt");
        println!("         # or: rm {}", exe_path.display());
    }
    Ok(())
}

fn count_git_notes() -> usize {
    let output = std::process::Command::new("git")
        .args(["notes", "--ref", "refs/notes/blameprompt", "list"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).lines().count()
        }
        _ => 0,
    }
}
