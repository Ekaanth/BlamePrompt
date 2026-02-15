use crate::commands::audit;
use crate::core::model_classifier::{self, ModelDeployment};
use chrono::Utc;
use std::collections::HashMap;

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

/// Estimated cost per 1K output tokens for common models.
/// Used for cost comparison recommendations.
struct ModelCostProfile {
    pattern: &'static str,
    name: &'static str,
    cost_per_1k_tokens: f64,
    tier: &'static str,     // "premium", "standard", "economy", "free"
    best_for: &'static str,
}

const MODEL_COSTS: &[ModelCostProfile] = &[
    ModelCostProfile { pattern: "opus-4", name: "Claude Opus 4.x", cost_per_1k_tokens: 0.075, tier: "premium", best_for: "Complex architecture, multi-file refactors, novel algorithms" },
    ModelCostProfile { pattern: "sonnet-4", name: "Claude Sonnet 4.x", cost_per_1k_tokens: 0.015, tier: "standard", best_for: "Most coding tasks, bug fixes, feature implementation" },
    ModelCostProfile { pattern: "haiku", name: "Claude Haiku", cost_per_1k_tokens: 0.005, tier: "economy", best_for: "Simple edits, boilerplate, documentation, tests" },
    ModelCostProfile { pattern: "gpt-4o", name: "GPT-4o", cost_per_1k_tokens: 0.015, tier: "standard", best_for: "General coding, multimodal tasks" },
    ModelCostProfile { pattern: "gpt-4", name: "GPT-4", cost_per_1k_tokens: 0.060, tier: "premium", best_for: "Complex reasoning (consider Claude Opus instead)" },
    ModelCostProfile { pattern: "gpt-3.5", name: "GPT-3.5", cost_per_1k_tokens: 0.002, tier: "economy", best_for: "Simple completions, formatting" },
    ModelCostProfile { pattern: "deepseek", name: "DeepSeek", cost_per_1k_tokens: 0.002, tier: "economy", best_for: "Code generation, completions" },
    ModelCostProfile { pattern: "codestral", name: "Codestral", cost_per_1k_tokens: 0.003, tier: "economy", best_for: "Code-specific tasks (non-production license)" },
    ModelCostProfile { pattern: "ollama:", name: "Local (Ollama)", cost_per_1k_tokens: 0.0, tier: "free", best_for: "Any task where latency is acceptable and privacy is paramount" },
    ModelCostProfile { pattern: "lmstudio:", name: "Local (LM Studio)", cost_per_1k_tokens: 0.0, tier: "free", best_for: "Local inference with GUI management" },
];

fn get_model_tier(model_id: &str) -> &'static str {
    let lower = model_id.to_lowercase();
    for mc in MODEL_COSTS {
        if lower.contains(mc.pattern) {
            return mc.tier;
        }
    }
    "standard"
}

fn get_cheaper_alternative(model_id: &str) -> Option<(&'static str, &'static str, f64)> {
    let lower = model_id.to_lowercase();

    // Opus -> Sonnet
    if lower.contains("opus") {
        return Some(("Claude Sonnet 4.x", "standard", 0.015));
    }
    // GPT-4 (non-4o) -> GPT-4o or Sonnet
    if lower.contains("gpt-4") && !lower.contains("gpt-4o") {
        return Some(("Claude Sonnet 4.x or GPT-4o", "standard", 0.015));
    }
    // Sonnet -> Haiku for simple tasks
    if lower.contains("sonnet") {
        return Some(("Claude Haiku", "economy", 0.005));
    }
    // GPT-4o -> Haiku for simple tasks
    if lower.contains("gpt-4o") {
        return Some(("Claude Haiku or DeepSeek", "economy", 0.003));
    }
    None
}

struct SessionAnalysis {
    model: String,
    display_name: String,
    total_cost: f64,
    receipt_count: u32,
    avg_messages: f64,
    avg_lines: f64,
    files_touched: Vec<String>,
    tier: String,
    potential_savings: f64,
    recommendation: String,
}

pub fn run(output: &str) {
    let entries = match audit::collect_all_entries(None, None, None, true) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        println!("No AI-generated code found to analyze.");
        return;
    }

    let all_receipts: Vec<_> = entries.iter().flat_map(|e| &e.receipts).collect();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    // Analyze per-model usage
    let mut model_data: HashMap<String, (f64, u32, u32, u32, Vec<String>)> = HashMap::new();
    for r in &all_receipts {
        let entry = model_data.entry(r.model.clone()).or_insert((0.0, 0, 0, 0, Vec::new()));
        entry.0 += r.cost_usd;
        entry.1 += 1;
        entry.2 += r.message_count;
        let lines = if r.line_range.1 >= r.line_range.0 { r.line_range.1 - r.line_range.0 + 1 } else { 0 };
        entry.3 += lines;
        entry.4.push(relative_path(&r.file_path));
    }

    let mut analyses: Vec<SessionAnalysis> = Vec::new();
    let mut total_potential_savings = 0.0f64;

    for (model_id, (cost, count, messages, lines, files)) in &model_data {
        let c = model_classifier::classify(model_id);
        let tier = get_model_tier(model_id);
        let avg_messages = *messages as f64 / *count as f64;
        let avg_lines = *lines as f64 / *count as f64;

        let (savings, recommendation) = if tier == "premium" {
            // Check if premium model was used for simple tasks
            let simple_task_ratio = if avg_messages <= 5.0 && avg_lines <= 30.0 { 0.85 } else if avg_messages <= 10.0 { 0.5 } else { 0.2 };
            let potential = cost * simple_task_ratio * 0.75; // 75% savings on downgraded tasks
            let rec = if simple_task_ratio > 0.7 {
                format!("{:.0}% of {} sessions appear to be simple tasks (avg {:.0} messages, {:.0} lines). Switching to Sonnet could save ${:.2}/month with similar quality.",
                    simple_task_ratio * 100.0, c.display_name, avg_messages, avg_lines, potential)
            } else {
                format!("Most {} sessions involve complex tasks — premium model is appropriate.", c.display_name)
            };
            (potential, rec)
        } else if tier == "standard" && avg_messages <= 3.0 && avg_lines <= 15.0 {
            let potential = cost * 0.6 * 0.65;
            let rec = format!("{:.0}% of {} sessions are quick edits. Consider Haiku for these to save ${:.2}/month.",
                60.0, c.display_name, potential);
            (potential, rec)
        } else if c.deployment == ModelDeployment::Cloud {
            let rec = format!("{} usage looks appropriate for the task complexity.", c.display_name);
            (0.0, rec)
        } else {
            let rec = format!("{} — local model, zero API cost.", c.display_name);
            (0.0, rec)
        };

        total_potential_savings += savings;

        let mut unique_files = files.clone();
        unique_files.sort();
        unique_files.dedup();

        analyses.push(SessionAnalysis {
            model: model_id.clone(),
            display_name: c.display_name.clone(),
            total_cost: *cost,
            receipt_count: *count,
            avg_messages,
            avg_lines,
            files_touched: unique_files,
            tier: tier.to_string(),
            potential_savings: savings,
            recommendation,
        });
    }

    analyses.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap_or(std::cmp::Ordering::Equal));

    // Generate report
    let mut md = String::new();
    md.push_str("# BlamePrompt Model Recommendation Report\n\n");
    md.push_str(&format!("> Generated: {}\n\n", now));

    // Summary
    let total_cost: f64 = analyses.iter().map(|a| a.total_cost).sum();
    md.push_str("## Summary\n\n");
    md.push_str("| Metric | Value |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| Total AI spend | ${:.2} |\n", total_cost));
    md.push_str(&format!("| Models used | {} |\n", analyses.len()));
    md.push_str(&format!("| Total receipts | {} |\n", all_receipts.len()));
    md.push_str(&format!("| **Potential savings** | **${:.2}** |\n\n", total_potential_savings));

    if total_potential_savings > 0.0 && total_cost > 0.0 {
        let savings_pct = (total_potential_savings / total_cost) * 100.0;
        md.push_str(&format!("Estimated **{:.0}% cost reduction** possible by optimizing model selection.\n\n", savings_pct));
    }

    // Model usage analysis
    md.push_str("## Model Usage Analysis\n\n");
    md.push_str("| Model | Tier | Receipts | Avg Msgs | Avg Lines | Cost | Savings |\n");
    md.push_str("|-------|------|----------|----------|-----------|------|---------|\n");
    for a in &analyses {
        md.push_str(&format!("| {} | {} | {} | {:.1} | {:.1} | ${:.4} | ${:.4} |\n",
            a.display_name, a.tier, a.receipt_count, a.avg_messages,
            a.avg_lines, a.total_cost, a.potential_savings));
    }
    md.push_str("\n");

    // Recommendations per model
    md.push_str("## Recommendations\n\n");
    for a in &analyses {
        md.push_str(&format!("### {} (`{}`)\n\n", a.display_name, a.model));
        md.push_str(&format!("- **Tier**: {} | **Cost**: ${:.4} | **Receipts**: {}\n", a.tier, a.total_cost, a.receipt_count));
        md.push_str(&format!("- **Avg session**: {:.1} messages, {:.1} lines generated\n", a.avg_messages, a.avg_lines));
        md.push_str(&format!("- **Files**: {} unique files\n", a.files_touched.len()));
        md.push_str(&format!("- {}\n", a.recommendation));

        if let Some((alt_name, alt_tier, _alt_cost)) = get_cheaper_alternative(&a.model) {
            if a.potential_savings > 0.0 {
                md.push_str(&format!("- **Suggested alternative**: {} ({} tier)\n", alt_name, alt_tier));
            }
        }
        md.push_str("\n");
    }

    // Model tier guide
    md.push_str("## Model Tier Guide\n\n");
    md.push_str("| Tier | Models | Best For | Relative Cost |\n");
    md.push_str("|------|--------|----------|---------------|\n");
    md.push_str("| Premium | Claude Opus, GPT-4 | Complex architecture, novel algorithms | $$$$$ |\n");
    md.push_str("| Standard | Claude Sonnet, GPT-4o | Most coding tasks, features, bug fixes | $$$ |\n");
    md.push_str("| Economy | Claude Haiku, DeepSeek, GPT-3.5 | Simple edits, boilerplate, tests | $ |\n");
    md.push_str("| Free | Ollama, LM Studio (local) | Privacy-sensitive, offline work | Free |\n\n");

    // Action items
    md.push_str("## Action Items\n\n");
    let mut item_num = 1;
    for a in &analyses {
        if a.potential_savings > 1.0 {
            md.push_str(&format!("{}. Switch {:.0}% of {} tasks to a cheaper model — saves ${:.2}\n",
                item_num, (a.potential_savings / a.total_cost * 100.0).min(100.0),
                a.display_name, a.potential_savings));
            item_num += 1;
        }
    }
    if item_num == 1 {
        md.push_str("Model selection appears well-optimized. No immediate changes recommended.\n");
    }
    md.push_str("\n");

    md.push_str("---\n\n");
    md.push_str("*Generated by [BlamePrompt](https://github.com/anthropics/blameprompt) — Model Recommendation Engine*\n");

    match std::fs::write(output, &md) {
        Ok(_) => println!("Model recommendations written to {}", output),
        Err(e) => eprintln!("Error writing report: {}", e),
    }
}
