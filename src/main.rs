mod core;
mod git;
mod integrations;
mod commands;

use clap::{Parser, Subcommand};

/// BlamePrompt: Track AI-generated code provenance via Git Notes.
/// No API key needed — hooks into Claude Code's native session data.
#[derive(Parser)]
#[command(name = "blameprompt", version = "0.1.0", about = "Track AI-generated code in git")]
struct Cli {
    /// Enable verbose debug output
    #[arg(long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Called by Claude Code hooks to capture AI receipts (internal)
    Checkpoint {
        /// Agent name (e.g., "claude")
        agent: String,
        /// Read hook input from stdin
        #[arg(long, default_value = "stdin")]
        hook_input: String,
    },

    /// Initialize BlamePrompt in the current repo or globally
    Init {
        /// Configure git template for all future repos
        #[arg(long)]
        global: bool,
    },

    /// Install Claude Code + git hooks (legacy, same as 'init')
    InstallHooks,

    /// Remove all BlamePrompt hooks and data
    Uninstall {
        /// Keep Git Notes (receipt history) when uninstalling
        #[arg(long)]
        keep_notes: bool,
        /// Remove everything including Git Notes and binary info
        #[arg(long)]
        purge: bool,
    },

    /// Show line-by-line AI/human attribution for a file
    Blame {
        /// File to analyze
        file: String,
    },

    /// Display all AI receipts attached to a specific commit
    Show {
        /// Commit SHA (full or short)
        commit: String,
    },

    /// Search across stored prompts
    Search {
        /// Search query
        query: String,
        /// Maximum number of results (default: 50)
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Show complete AI audit trail with filters
    Audit {
        /// Start date filter (e.g., 2026-01-01)
        #[arg(long)]
        from: Option<String>,
        /// End date filter (e.g., 2026-02-09)
        #[arg(long)]
        to: Option<String>,
        /// Filter by author name
        #[arg(long)]
        author: Option<String>,
        /// Output format: md, table, json, csv
        #[arg(long, default_value = "md")]
        format: String,
        /// Include uncommitted/staged receipts
        #[arg(long)]
        include_uncommitted: bool,
    },

    /// Show aggregated AI usage statistics
    Analytics {
        /// Export format: json, csv
        #[arg(long)]
        export: Option<String>,
    },

    /// Generate comprehensive markdown report
    Report {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-report.md")]
        output: String,
        /// Start date filter
        #[arg(long)]
        from: Option<String>,
        /// End date filter
        #[arg(long)]
        to: Option<String>,
        /// Filter by author name
        #[arg(long)]
        author: Option<String>,
        /// Include uncommitted/staged receipts
        #[arg(long)]
        include_uncommitted: bool,
    },

    /// Detect Replit Agent changes in unstaged files
    ReplitDetect,

    /// Manually tag code as AI-generated
    Tag {
        /// File path to tag
        file: String,
        /// Start line number
        #[arg(long)]
        start_line: u32,
        /// End line number
        #[arg(long)]
        end_line: u32,
        /// AI provider (e.g., "replit", "chatgpt", "local")
        #[arg(long)]
        provider: String,
        /// Model name (e.g., "replit-agent", "gpt-4o", "ollama:llama3.2")
        #[arg(long)]
        model: String,
        /// Description of the prompt used
        #[arg(long)]
        prompt: String,
    },

    /// Push BlamePrompt notes to origin
    Push,

    /// Fetch BlamePrompt notes from origin
    Pull,

    /// Dry-run the redaction engine on a file
    Redact {
        /// File to test redaction on
        #[arg(long)]
        test: String,
    },

    /// Manually ingest a Claude Code JSONL transcript
    Record {
        /// Path to the JSONL session transcript
        #[arg(long)]
        session: String,
        /// AI provider name (default: "claude")
        #[arg(long)]
        provider: Option<String>,
    },

    /// Manage the local SQLite cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },

    /// Scan AI model licenses for compliance issues
    LicenseScan {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-license-scan.md")]
        output: String,
    },

    /// Generate SOC2/ISO 27001 compliance evidence package
    Soc2 {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-soc2.md")]
        output: String,
        /// Start date filter
        #[arg(long)]
        from: Option<String>,
        /// End date filter
        #[arg(long)]
        to: Option<String>,
    },

    /// Generate GDPR data flow map and DPIA report
    Gdpr {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-gdpr.md")]
        output: String,
    },

    /// Assess AI supply chain risk score
    SupplyChainRisk {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-supply-chain-risk.md")]
        output: String,
    },

    /// Scan AI-generated code for vulnerabilities (SAST)
    VulnScan {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-vuln-scan.md")]
        output: String,
    },

    /// Detect prompt injection patterns in AI-generated code
    PromptInjection {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-prompt-injection.md")]
        output: String,
    },

    /// Alert on secrets that may need rotation after AI exposure
    SecretRotation {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-secret-rotation.md")]
        output: String,
    },

    /// Analyze AI usage budgets and cost efficiency
    Budget {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-budget.md")]
        output: String,
        /// Monthly budget limit in USD
        #[arg(long)]
        monthly_limit: Option<f64>,
        /// Quarterly budget limit in USD
        #[arg(long)]
        quarterly_limit: Option<f64>,
    },

    /// Recommend optimal models based on usage patterns
    ModelRecommend {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-model-recommendations.md")]
        output: String,
    },

    /// Enterprise management commands
    Enterprise {
        #[command(subcommand)]
        action: EnterpriseAction,
    },
}

#[derive(Subcommand)]
enum CacheAction {
    /// Sync Git Notes into the local SQLite cache for fast queries
    Sync,
}

#[derive(Subcommand)]
enum EnterpriseAction {
    /// Sync local receipts to the enterprise server
    Sync {
        /// Start date filter
        #[arg(long)]
        from: Option<String>,
        /// End date filter
        #[arg(long)]
        to: Option<String>,
    },
    /// Show enterprise configuration status
    Status,
    /// Export all receipts as a JSON bundle for enterprise ingestion
    Export {
        /// Output file path
        #[arg(long, default_value = "./blameprompt-enterprise-export.json")]
        output: String,
        /// Start date filter
        #[arg(long)]
        from: Option<String>,
        /// End date filter
        #[arg(long)]
        to: Option<String>,
        /// Filter by author
        #[arg(long)]
        author: Option<String>,
    },
    /// Check staged receipts against enterprise policy rules
    PolicyCheck,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Checkpoint { agent, hook_input } => {
            commands::checkpoint::run(&agent, &hook_input);
        }

        Commands::Init { global } => {
            if let Err(e) = git::init_hooks::run_init(global) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::InstallHooks => {
            if let Err(e) = integrations::claude_hooks::install() {
                eprintln!("Error installing Claude Code hooks: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = git::hooks::install_hooks() {
                eprintln!("Error installing git hooks: {}", e);
                std::process::exit(1);
            }
            println!("\nTip: To share receipts with your team:");
            println!("  git push origin refs/notes/blameprompt");
            println!("  git config --add remote.origin.fetch '+refs/notes/blameprompt:refs/notes/blameprompt'");
        }

        Commands::Uninstall { keep_notes, purge } => {
            if let Err(e) = commands::uninstall::run(keep_notes, purge) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::Blame { file } => {
            commands::blame::run(&file);
        }

        Commands::Show { commit } => {
            commands::show::run(&commit);
        }

        Commands::Search { query, limit } => {
            commands::search::run(&query, limit);
        }

        Commands::Audit { from, to, author, format, include_uncommitted } => {
            commands::audit::run(
                from.as_deref(),
                to.as_deref(),
                author.as_deref(),
                &format,
                include_uncommitted,
            );
        }

        Commands::Analytics { export } => {
            commands::analytics::run(export.as_deref());
        }

        Commands::Report { output, from, to, author, include_uncommitted } => {
            if let Err(e) = commands::report::generate_report(
                &output,
                from.as_deref(),
                to.as_deref(),
                author.as_deref(),
                include_uncommitted,
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::ReplitDetect => {
            integrations::replit::detect();
        }

        Commands::Tag { file, start_line, end_line, provider, model, prompt } => {
            commands::staging::manual_tag(&file, start_line, end_line, &provider, &model, &prompt);
        }

        Commands::Push => {
            commands::sync::push();
        }

        Commands::Pull => {
            commands::sync::pull();
        }

        Commands::Redact { test } => {
            commands::redact_test::run(&test);
        }

        Commands::Record { session, provider } => {
            commands::record::run(&session, provider.as_deref());
        }

        Commands::Cache { action } => {
            match action {
                CacheAction::Sync => {
                    if let Err(e) = core::db::sync_from_notes() {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }

        Commands::LicenseScan { output } => {
            commands::license_scan::run(&output);
        }

        Commands::Soc2 { output, from, to } => {
            commands::compliance::run_soc2(&output, from.as_deref(), to.as_deref());
        }

        Commands::Gdpr { output } => {
            commands::compliance::run_gdpr(&output);
        }

        Commands::SupplyChainRisk { output } => {
            commands::supply_chain::run(&output);
        }

        Commands::VulnScan { output } => {
            commands::vuln_scan::run(&output);
        }

        Commands::PromptInjection { output } => {
            commands::prompt_injection::run(&output);
        }

        Commands::SecretRotation { output } => {
            commands::secret_rotation::run(&output);
        }

        Commands::Budget { output, monthly_limit, quarterly_limit } => {
            commands::budget::run(&output, monthly_limit, quarterly_limit);
        }

        Commands::ModelRecommend { output } => {
            commands::model_recommend::run(&output);
        }

        Commands::Enterprise { action } => {
            match action {
                EnterpriseAction::Sync { from, to } => {
                    commands::enterprise::sync(from.as_deref(), to.as_deref());
                }
                EnterpriseAction::Status => {
                    commands::enterprise::status();
                }
                EnterpriseAction::Export { output, from, to, author } => {
                    commands::enterprise::export(
                        &output,
                        from.as_deref(),
                        to.as_deref(),
                        author.as_deref(),
                    );
                }
                EnterpriseAction::PolicyCheck => {
                    commands::enterprise::policy_check();
                }
            }
        }
    }
}
