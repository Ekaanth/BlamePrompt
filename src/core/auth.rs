use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub api_key: String,
    pub username: String,
    pub api_url: String,
}

fn credentials_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".blameprompt")
        .join("credentials")
}

pub fn save(creds: &Credentials) -> Result<(), String> {
    let path = credentials_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    let content = toml::to_string_pretty(creds).map_err(|e| format!("Serialize error: {}", e))?;
    std::fs::write(&path, content).map_err(|e| format!("Failed to write credentials: {}", e))?;

    // Restrict file permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms).ok();
    }

    Ok(())
}

pub fn load() -> Option<Credentials> {
    let path = credentials_path();
    let content = std::fs::read_to_string(&path).ok()?;
    toml::from_str(&content).ok()
}

pub fn clear() -> Result<(), String> {
    let path = credentials_path();
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("Failed to remove credentials: {}", e))?;
    }
    Ok(())
}

pub fn is_logged_in() -> bool {
    load().is_some()
}
