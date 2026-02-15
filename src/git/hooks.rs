use std::path::Path;

const PRE_COMMIT_HOOK: &str = r#"# BlamePrompt pre-commit hook (do not edit between markers)
STAGING=".blameprompt/staging.json"
if [ -f "$STAGING" ]; then
    COUNT=$(python3 -c "import json; d=json.load(open('$STAGING')); print(len(d.get('receipts',[])))" 2>/dev/null || echo "0")
    if [ "$COUNT" != "0" ]; then
        echo "[BlamePrompt] $COUNT receipt(s) will be attached to this commit"
    fi
fi
# /BlamePrompt
"#;

const POST_COMMIT_HOOK: &str = r#"# BlamePrompt post-commit hook (do not edit between markers)
STAGING=".blameprompt/staging.json"
[ -f "$STAGING" ] || exit 0
python3 -c "
import json, sys
d = json.load(open('$STAGING'))
r = d.get('receipts', [])
if not r: sys.exit(0)
p = {'blameprompt_version': '0.1.0', 'receipts': r}
sys.stdout.write(json.dumps(p))
" | git notes --ref refs/notes/blameprompt add -f -F - HEAD 2>/dev/null && \
echo '{"receipts":[]}' > "$STAGING" && \
echo "[BlamePrompt] Receipts attached to $(git rev-parse --short HEAD)"
# /BlamePrompt
"#;

fn git_hooks_dir() -> Result<std::path::PathBuf, String> {
    let repo = git2::Repository::discover(".").map_err(|_| "Not in a git repository. Run this from inside a git repository.".to_string())?;
    Ok(repo.path().join("hooks"))
}

pub fn install_hooks() -> Result<(), String> {
    let hooks_dir = git_hooks_dir()?;
    std::fs::create_dir_all(&hooks_dir)
        .map_err(|e| format!("Cannot create hooks dir: {}", e))?;

    install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK)?;
    install_hook(&hooks_dir, "post-commit", POST_COMMIT_HOOK)?;

    println!("Installed git hooks in {}", hooks_dir.display());
    Ok(())
}

fn install_hook(hooks_dir: &Path, name: &str, content: &str) -> Result<(), String> {
    let hook_path = hooks_dir.join(name);

    if hook_path.exists() {
        let existing = std::fs::read_to_string(&hook_path)
            .map_err(|e| format!("Cannot read {}: {}", name, e))?;

        if existing.contains("BlamePrompt") {
            println!("  [skip] {} already has BlamePrompt hook", name);
            return Ok(());
        }

        // Append to existing hook
        let mut new_content = existing;
        new_content.push_str("\n\n");
        new_content.push_str(content);
        std::fs::write(&hook_path, new_content)
            .map_err(|e| format!("Cannot write {}: {}", name, e))?;
    } else {
        // Create new hook
        let full = format!("#!/bin/sh\n\n{}", content);
        std::fs::write(&hook_path, full)
            .map_err(|e| format!("Cannot write {}: {}", name, e))?;
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755));
    }

    println!("  [done] Installed {} hook", name);
    Ok(())
}

pub fn uninstall_hooks() -> Result<(), String> {
    let hooks_dir = match git_hooks_dir() {
        Ok(d) => d,
        Err(_) => {
            println!("  [skip] Not in a git repository");
            return Ok(());
        }
    };

    for hook_name in &["pre-commit", "post-commit"] {
        let hook_path = hooks_dir.join(hook_name);
        if !hook_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&hook_path)
            .map_err(|e| format!("Cannot read hook: {}", e))?;

        if !content.contains("BlamePrompt") {
            println!("  [skip] {}: no BlamePrompt section found", hook_name);
            continue;
        }

        let cleaned = remove_between_markers(&content, "# BlamePrompt", "# /BlamePrompt");

        if cleaned.trim().is_empty() || cleaned.trim() == "#!/bin/sh" {
            std::fs::remove_file(&hook_path)
                .map_err(|e| format!("Cannot delete hook: {}", e))?;
            println!("  [done] Removed .git/hooks/{} (was BlamePrompt-only)", hook_name);
        } else {
            std::fs::write(&hook_path, &cleaned)
                .map_err(|e| format!("Cannot write hook: {}", e))?;
            println!("  [done] Removed BlamePrompt section from .git/hooks/{}", hook_name);
        }
    }
    Ok(())
}

fn remove_between_markers(content: &str, start_marker: &str, end_marker: &str) -> String {
    let mut result = String::new();
    let mut skipping = false;
    for line in content.lines() {
        if line.contains(start_marker) && !line.contains(end_marker) {
            skipping = true;
            continue;
        }
        if line.contains(end_marker) {
            skipping = false;
            continue;
        }
        if !skipping {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

pub fn pre_commit_hook_content() -> &'static str {
    PRE_COMMIT_HOOK
}

pub fn post_commit_hook_content() -> &'static str {
    POST_COMMIT_HOOK
}
