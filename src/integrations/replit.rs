pub fn detect() {
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only"])
        .output();

    let files = match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.lines().map(String::from).collect::<Vec<_>>()
        }
        _ => {
            eprintln!("Error: git diff failed. Are you in a git repository?");
            return;
        }
    };

    if files.is_empty() {
        println!("No unstaged changes detected.");
        return;
    }

    println!("Detected changes (unstaged):\n");

    let mut suggestions = Vec::new();

    for file in &files {
        let diff_output = std::process::Command::new("git")
            .args(["diff", "--unified=0", file])
            .output();

        let hunks = match diff_output {
            Ok(o) if o.status.success() => parse_hunks(&String::from_utf8_lossy(&o.stdout)),
            _ => continue,
        };

        if hunks.is_empty() {
            continue;
        }

        println!("  {}:", file);
        for (start, end) in &hunks {
            let count = end - start + 1;
            println!("    Lines {}-{} ({} lines added)", start, end, count);
            suggestions.push(format!(
                "blameprompt tag {} --start-line {} --end-line {} --provider replit --model replit-agent --prompt \"describe what you asked\"",
                file, start, end
            ));
        }
        println!();
    }

    if !suggestions.is_empty() {
        println!("Suggested commands (copy & paste):");
        for s in &suggestions {
            println!("  {}", s);
        }
    }
}

fn parse_hunks(diff: &str) -> Vec<(u32, u32)> {
    let mut hunks = Vec::new();
    for line in diff.lines() {
        if line.starts_with("@@") {
            if let Some(plus_part) = line.split('+').nth(1) {
                let nums: &str = plus_part.split(' ').next().unwrap_or("0");
                let parts: Vec<&str> = nums.split(',').collect();
                let start: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
                let count: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                if start > 0 && count > 0 {
                    hunks.push((start, start + count - 1));
                }
            }
        }
    }
    hunks
}
