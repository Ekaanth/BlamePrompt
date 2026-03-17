use crate::core::auth;

pub fn run() {
    if !auth::is_logged_in() {
        eprintln!("  \x1b[1;31mError:\x1b[0m Not logged in. Run `blameprompt login` first.");
        std::process::exit(1);
    }

    let url = "https://blameprompt.com/dashboard";

    if open::that(url).is_err() {
        eprintln!("  \x1b[1;31mError:\x1b[0m Could not open browser. Visit manually:");
        eprintln!("  {}", url);
        std::process::exit(1);
    }

    println!("  \x1b[1;32m\u{2713}\x1b[0m Opening dashboard in your browser...");
}
