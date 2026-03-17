use crate::core::api_client::ApiClient;
use crate::core::auth;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProfileResponse {
    #[serde(default)]
    username: String,
    #[serde(default, rename = "displayName")]
    display_name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    bio: Option<String>,
    #[serde(default, rename = "publicProfile")]
    public_profile: Option<bool>,
    #[serde(default, rename = "createdAt")]
    created_at: Option<String>,
}

pub fn run(edit: bool) {
    if !auth::is_logged_in() {
        eprintln!("  \x1b[1;31mError:\x1b[0m Not logged in. Run `blameprompt login` first.");
        std::process::exit(1);
    }

    if edit {
        let url = "https://blameprompt.com/settings/profile";
        if open::that(url).is_err() {
            eprintln!("  \x1b[1;31mError:\x1b[0m Could not open browser. Visit manually:");
            eprintln!("  {}", url);
            std::process::exit(1);
        }
        println!("  \x1b[1;32m\u{2713}\x1b[0m Opening profile settings in your browser...");
        return;
    }

    let api = match ApiClient::from_credentials() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  \x1b[1;31mError:\x1b[0m {}", e);
            std::process::exit(1);
        }
    };

    let profile: ProfileResponse = match api.get("/api/me") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("  \x1b[1;31mError:\x1b[0m {}", e);
            std::process::exit(1);
        }
    };

    println!();
    println!("  \x1b[1mBlamePrompt Profile\x1b[0m");
    println!("  \x1b[2m─────────────────────────\x1b[0m");
    println!();

    let none_str = "\x1b[2m(not set)\x1b[0m";

    println!(
        "  \x1b[1mUsername:\x1b[0m       @{}",
        profile.username
    );
    println!(
        "  \x1b[1mDisplay Name:\x1b[0m  {}",
        profile.display_name.as_deref().unwrap_or(none_str)
    );
    println!(
        "  \x1b[1mEmail:\x1b[0m         {}",
        profile.email.as_deref().unwrap_or(none_str)
    );
    println!(
        "  \x1b[1mBio:\x1b[0m           {}",
        profile.bio.as_deref().unwrap_or(none_str)
    );
    println!(
        "  \x1b[1mPublic:\x1b[0m        {}",
        match profile.public_profile {
            Some(true) => "Yes",
            Some(false) => "No",
            None => none_str,
        }
    );
    println!(
        "  \x1b[1mMember since:\x1b[0m  {}",
        profile.created_at.as_deref().unwrap_or(none_str)
    );

    println!();
    println!("  \x1b[2mTip: Run `blameprompt profile --edit` to update your profile.\x1b[0m");
    println!();
}
