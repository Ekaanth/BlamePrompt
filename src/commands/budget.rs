use crate::commands::audit;
use crate::core::model_classifier;
use chrono::{Datelike, Utc};
use std::collections::HashMap;

pub fn run(output: &str, monthly_limit: Option<f64>, quarterly_limit: Option<f64>) {
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
    let now = Utc::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    // Current month/quarter filtering
    let current_month = now.month();
    let current_year = now.year();
    let current_quarter = (current_month - 1) / 3 + 1;
    let quarter_start_month = (current_quarter - 1) * 3 + 1;

    let mut monthly_cost = 0.0f64;
    let mut quarterly_cost = 0.0f64;
    let mut total_cost = 0.0f64;

    // Per-user costs
    let mut user_monthly: HashMap<String, f64> = HashMap::new();
    let mut user_total: HashMap<String, f64> = HashMap::new();

    // Per-model costs
    let mut model_monthly: HashMap<String, (f64, u32)> = HashMap::new();
    let mut model_total: HashMap<String, (f64, u32)> = HashMap::new();

    // Monthly breakdown
    let mut monthly_breakdown: HashMap<String, f64> = HashMap::new();

    for r in &all_receipts {
        total_cost += r.cost_usd;
        *user_total.entry(r.user.clone()).or_insert(0.0) += r.cost_usd;

        let entry = model_total.entry(r.model.clone()).or_insert((0.0, 0));
        entry.0 += r.cost_usd;
        entry.1 += 1;

        let month_key = r.timestamp.format("%Y-%m").to_string();
        *monthly_breakdown.entry(month_key).or_insert(0.0) += r.cost_usd;

        // Check if in current month
        if r.timestamp.month() == current_month && r.timestamp.year() == current_year {
            monthly_cost += r.cost_usd;
            *user_monthly.entry(r.user.clone()).or_insert(0.0) += r.cost_usd;
            let entry = model_monthly.entry(r.model.clone()).or_insert((0.0, 0));
            entry.0 += r.cost_usd;
            entry.1 += 1;
        }

        // Check if in current quarter
        if r.timestamp.year() == current_year && r.timestamp.month() >= quarter_start_month
            && r.timestamp.month() < quarter_start_month + 3
        {
            quarterly_cost += r.cost_usd;
        }
    }

    let mut md = String::new();
    md.push_str("# BlamePrompt Budget & Cost Analysis\n\n");
    md.push_str(&format!("> Generated: {}\n\n", now_str));

    // Budget status
    md.push_str("## Budget Status\n\n");
    md.push_str("| Period | Spent | Limit | Usage | Status |\n");
    md.push_str("|--------|-------|-------|-------|--------|\n");

    if let Some(limit) = monthly_limit {
        let pct = (monthly_cost / limit) * 100.0;
        let status = if pct >= 100.0 {
            "EXCEEDED"
        } else if pct >= 80.0 {
            "WARNING (>80%)"
        } else {
            "OK"
        };
        md.push_str(&format!("| Monthly ({}-{:02}) | ${:.2} | ${:.2} | {:.1}% | {} |\n",
            current_year, current_month, monthly_cost, limit, pct, status));
    } else {
        md.push_str(&format!("| Monthly ({}-{:02}) | ${:.2} | Not set | — | — |\n",
            current_year, current_month, monthly_cost));
    }

    if let Some(limit) = quarterly_limit {
        let pct = (quarterly_cost / limit) * 100.0;
        let status = if pct >= 100.0 {
            "EXCEEDED"
        } else if pct >= 80.0 {
            "WARNING (>80%)"
        } else {
            "OK"
        };
        md.push_str(&format!("| Q{} {} | ${:.2} | ${:.2} | {:.1}% | {} |\n",
            current_quarter, current_year, quarterly_cost, limit, pct, status));
    } else {
        md.push_str(&format!("| Q{} {} | ${:.2} | Not set | — | — |\n",
            current_quarter, current_year, quarterly_cost));
    }

    md.push_str(&format!("| All time | ${:.2} | — | — | — |\n\n", total_cost));

    // Alerts
    let mut alerts: Vec<String> = Vec::new();
    if let Some(limit) = monthly_limit {
        let pct = (monthly_cost / limit) * 100.0;
        if pct >= 100.0 {
            alerts.push(format!("BUDGET EXCEEDED: Monthly spending ${:.2} exceeds limit ${:.2}", monthly_cost, limit));
        } else if pct >= 80.0 {
            alerts.push(format!("WARNING: Monthly spending ${:.2} is at {:.0}% of ${:.2} limit", monthly_cost, pct, limit));
        }
    }
    if let Some(limit) = quarterly_limit {
        let pct = (quarterly_cost / limit) * 100.0;
        if pct >= 100.0 {
            alerts.push(format!("BUDGET EXCEEDED: Quarterly spending ${:.2} exceeds limit ${:.2}", quarterly_cost, limit));
        } else if pct >= 80.0 {
            alerts.push(format!("WARNING: Quarterly spending ${:.2} is at {:.0}% of ${:.2} limit", quarterly_cost, pct, limit));
        }
    }

    if !alerts.is_empty() {
        md.push_str("## Alerts\n\n");
        for alert in &alerts {
            md.push_str(&format!("- **{}**\n", alert));
        }
        md.push_str("\n");
    }

    // Per-user breakdown
    md.push_str("## Cost by User\n\n");
    md.push_str("| User | This Month | All Time | Receipts |\n");
    md.push_str("|------|-----------|----------|----------|\n");
    let mut user_receipt_count: HashMap<String, u32> = HashMap::new();
    for r in &all_receipts {
        *user_receipt_count.entry(r.user.clone()).or_insert(0) += 1;
    }
    let mut user_sorted: Vec<_> = user_total.iter().collect();
    user_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (user, total) in &user_sorted {
        let monthly = user_monthly.get(*user).unwrap_or(&0.0);
        let count = user_receipt_count.get(*user).unwrap_or(&0);
        md.push_str(&format!("| {} | ${:.4} | ${:.4} | {} |\n", user, monthly, total, count));
    }
    md.push_str("\n");

    // Per-model breakdown
    md.push_str("## Cost by Model\n\n");
    md.push_str("| Model | Vendor | This Month | All Time | Receipts | Avg Cost/Receipt |\n");
    md.push_str("|-------|--------|-----------|----------|----------|------------------|\n");
    let mut model_sorted: Vec<_> = model_total.iter().collect();
    model_sorted.sort_by(|a, b| b.1.0.partial_cmp(&a.1.0).unwrap_or(std::cmp::Ordering::Equal));
    for (model_id, (total, count)) in &model_sorted {
        let c = model_classifier::classify(model_id);
        let (m_cost, _m_count) = model_monthly.get(*model_id).unwrap_or(&(0.0, 0));
        let avg = if *count > 0 { total / *count as f64 } else { 0.0 };
        md.push_str(&format!("| {} | {} | ${:.4} | ${:.4} | {} | ${:.4} |\n",
            c.display_name, c.vendor, m_cost, total, count, avg));
    }
    md.push_str("\n");

    // Monthly trend
    md.push_str("## Monthly Trend\n\n");
    md.push_str("| Month | Cost |\n");
    md.push_str("|-------|------|\n");
    let mut months: Vec<_> = monthly_breakdown.iter().collect();
    months.sort_by_key(|(k, _)| (*k).clone());
    for (month, cost) in &months {
        md.push_str(&format!("| {} | ${:.4} |\n", month, cost));
    }
    md.push_str("\n");

    // Recommendations
    md.push_str("## Recommendations\n\n");
    md.push_str("1. **Set budget limits** — Use `--monthly-limit` and `--quarterly-limit` flags.\n");
    md.push_str("2. **Review high-cost users** — Identify training opportunities or workflow improvements.\n");
    md.push_str("3. **Model optimization** — Run `blameprompt model-recommend` for cost-saving suggestions.\n");
    md.push_str("4. **Use local models** — For simple tasks, local models (Ollama) have zero API cost.\n");
    md.push_str("5. **Monitor trends** — Re-run budget analysis weekly to catch spending spikes early.\n\n");

    md.push_str("---\n\n");
    md.push_str("*Generated by [BlamePrompt](https://github.com/anthropics/blameprompt) — Budget Controls*\n");

    match std::fs::write(output, &md) {
        Ok(_) => println!("Budget analysis written to {}", output),
        Err(e) => eprintln!("Error writing report: {}", e),
    }
}
