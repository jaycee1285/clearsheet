use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    let config_dir = PathBuf::from(home).join(".config").join("clearsheet");
    Ok(config_dir)
}

pub fn get_sheet_files() -> Result<Vec<PathBuf>> {
    let config_dir = get_config_dir()?;

    if !config_dir.exists() {
        anyhow::bail!(
            "Config directory not found: {}\nCreate it and add markdown cheatsheets.",
            config_dir.display()
        );
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(&config_dir)
        .context("Failed to read config directory")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect();

    files.sort();
    Ok(files)
}
