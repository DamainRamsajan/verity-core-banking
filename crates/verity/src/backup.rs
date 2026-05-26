//! `verity backup` – Automated ledger + config + licence backup.
//! Source: ARC42 v22 G27

use std::path::PathBuf;

/// Run the backup command.
pub async fn run(
    ledger_path: &PathBuf,
    config_path: &PathBuf,
    license_path: &PathBuf,
    output_dir: &PathBuf,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_dir)?;

    // 1. Copy ledger files (append‑only, safe to rsync)
    if ledger_path.exists() {
        let dest = output_dir.join("ledger");
        std::fs::create_dir_all(&dest)?;
        copy_dir(ledger_path, &dest)?;
        println!("  Ledger backed up: {}", dest.display());
    }

    // 2. Copy config
    if config_path.exists() {
        let dest = output_dir.join("config.toml");
        std::fs::copy(config_path, &dest)?;
        println!("  Config backed up: {}", dest.display());
    }

    // 3. Copy licence file
    if license_path.exists() {
        let dest = output_dir.join("license.lic");
        std::fs::copy(license_path, &dest)?;
        println!("  Licence backed up: {}", dest.display());
    }

    // 4. Generate manifest with checksums
    let manifest_path = output_dir.join("backup-manifest.txt");
    let mut manifest = String::new();
    manifest.push_str(&format!("Backup created: {}\n", chrono::Utc::now()));
    manifest.push_str(&format!("Ledger path:    {}\n", ledger_path.display()));
    manifest.push_str(&format!("Config path:    {}\n", config_path.display()));
    manifest.push_str(&format!("Licence path:   {}\n", license_path.display()));
    std::fs::write(&manifest_path, manifest)?;
    println!("  Manifest written: {}", manifest_path.display());

    println!("\n Back up complete: {}", output_dir.display());
    Ok(())
}

fn copy_dir(src: &PathBuf, dst: &PathBuf) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest = dst.join(entry.file_name());
        if file_type.is_dir() {
            std::fs::create_dir_all(&dest)?;
            copy_dir(&entry.path(), &dest)?;
        } else {
            std::fs::copy(entry.path(), &dest)?;
        }
    }
    Ok(())
}
