use anyhow::Result;
use crdts::GSet;
use std::fs;
use std::path::Path;
use tracing::info;

pub fn get_storage_path(port: u16) -> String {
    format!("data/storage_{}.json", port)
}

pub fn ensure_data_dir() -> Result<()> {
    let path = Path::new("data");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn save_log(port: u16, log: &GSet<String>) -> Result<()> {
    ensure_data_dir()?;
    let path = get_storage_path(port);
    let json = serde_json::to_string_pretty(log)?;
    fs::write(&path, json)?;
    // info!("Saved state to {}", path); // Commented out to avoid spamming logs
    Ok(())
}

pub fn load_log(port: u16) -> Result<GSet<String>> {
    let path = get_storage_path(port);
    let path = Path::new(&path);

    if !path.exists() {
        info!("No existing storage found at {:?}. Starting fresh.", path);
        return Ok(GSet::new());
    }

    info!("Loading state from {:?}", path);
    let content = fs::read_to_string(path)?;
    let log: GSet<String> = serde_json::from_str(&content)?;
    Ok(log)
}
