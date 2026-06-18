use std::path::Path;
use std::process::Command;

pub fn ensure_ready() -> Result<(), String> {
    let cards = Path::new("plugins/wizards/cards");
    if cards.read_dir().ok().and_then(|mut d| d.next()).is_some() {
        return Ok(());
    }
    fetch_data_if_needed()?;
    generate_cards()?;
    Ok(())
}

fn fetch_data_if_needed() -> Result<(), String> {
    if Path::new("data/mtgjson/AtomicCards.json").exists() {
        return Ok(());
    }
    eprintln!("cargo run: first-run setup: downloading card data...");
    let status = Command::new("fish")
        .args(["scripts/fetch_data", "--minimal"])
        .status()
        .map_err(|e| format!("failed to run fetch_data: {e}"))?;
    if !status.success() {
        return Err("fetch_data --minimal failed".into());
    }
    Ok(())
}

fn generate_cards() -> Result<(), String> {
    eprintln!("cargo run: first-run setup: generating card corpus...");
    let status = Command::new("cargo")
        .args(["xtask", "generate", "plugins/wizards", "--minimal"])
        .status()
        .map_err(|e| format!("failed to run cargo xtask generate: {e}"))?;
    if !status.success() {
        return Err("cargo xtask generate --minimal failed".into());
    }
    Ok(())
}
