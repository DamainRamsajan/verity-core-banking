use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod api;
mod server;
mod backup;
mod benchmark;
mod config_audit;
mod archive;
mod vault;
mod hsm;
mod shutdown;

#[derive(Parser)]
#[command(name = "verity", about = "Verity Core Banking Platform")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Verity and bind licence to this hardware
    Install {
        #[arg(long)]
        license_key: String,
        #[arg(long, default_value = "/etc/verity")]
        config_dir: PathBuf,
    },
    /// Start the platform (REST API + dashboard)
    Serve {
        #[arg(long, default_value = "0.0.0.0:8080")]
        bind: String,
    },
    /// Licence status
    License {
        #[command(subcommand)]
        cmd: LicenseCmd,
    },
    /// Backup ledger, config, and licence
    Backup {
        #[arg(long, default_value = "/var/verity/ledger")]
        ledger_path: PathBuf,
        #[arg(long, default_value = "/etc/verity/config.toml")]
        config_path: PathBuf,
        #[arg(long, default_value = "/etc/verity/license.lic")]
        license_path: PathBuf,
        #[arg(long, default_value = "/var/verity/backup")]
        output_dir: PathBuf,
    },
    /// Run performance benchmark
    Benchmark {
        #[arg(long, default_value = "30")]
        duration_secs: u64,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
    /// Verify an archived ledger partition
    Archive {
        #[command(subcommand)]
        cmd: ArchiveCmd,
    },
    /// Print version
    Version,
}

#[derive(Subcommand)]
enum LicenseCmd {
    Status,
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Set a configuration value
    Set {
        key: String,
        value: String,
        #[arg(long, default_value = "operator")]
        operator: String,
        #[arg(long, default_value = "/etc/verity/config.toml")]
        config_path: PathBuf,
    },
    /// Show current configuration
    Diff {
        #[arg(long, default_value = "/etc/verity/config.toml")]
        config_path: PathBuf,
    },
}

#[derive(Subcommand)]
enum ArchiveCmd {
    /// Verify an archived ledger partition
    Verify {
        archive_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { license_key, config_dir } => install(&license_key, &config_dir),
        Commands::Serve { bind } => {
            tracing::info!("Starting Verity Core Banking Platform…");
            server::run(&bind).await
        }
        Commands::License { cmd } => match cmd {
            LicenseCmd::Status => license_status(),
        },
        Commands::Backup { ledger_path, config_path, license_path, output_dir } => {
            backup::run(&ledger_path, &config_path, &license_path, &output_dir).await
        }
        Commands::Benchmark { duration_secs } => {
            benchmark::run(duration_secs).await
        }
        Commands::Config { cmd } => match cmd {
            ConfigCmd::Set { key, value, operator, config_path } => {
                config_audit::config_set(&config_path, &key, &value, &operator).await
            }
            ConfigCmd::Diff { config_path } => {
                config_audit::config_diff(&config_path).await
            }
        },
        Commands::Archive { cmd } => match cmd {
            ArchiveCmd::Verify { archive_path } => {
                archive::verify(&archive_path).await
            }
        },
        Commands::Version => {
            println!("verity {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

// --------------------------------------------------------------------
// Helper – decode the licence key and extract the organisation name
// --------------------------------------------------------------------
fn decode_license_org(key: &str) -> anyhow::Result<String> {
    // Key format: VERITY-<base64_payload>-<base64_sig>
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() < 3 {
        anyhow::bail!("Invalid licence key format");
    }
    let payload_b64 = parts[1];
    let payload_bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        payload_b64,
    )?;
    let payload: serde_json::Value = serde_json::from_slice(&payload_bytes)?;
    Ok(payload["org"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string())
}

// --------------------------------------------------------------------
// Install – matches licenz-core 0.2.0 API
// --------------------------------------------------------------------
fn install(key: &str, config_dir: &PathBuf) -> anyhow::Result<()> {
    let vendor_pubkey = std::env!("VERITY_VENDOR_PUBKEY");

    let witness = licenz_core::SecurityWitness::new(vendor_pubkey)?;
    let config = licenz_core::WitnessConfig::default();

    let attestation = witness.attest(key, &config)?;

    // Signature check
    if !attestation.signature_valid {
        anyhow::bail!("Invalid licence signature.");
    }

    // Expiration – field is a bare `DateTime<Utc>`, not an Option
    let exp = attestation.expiration.valid_until;
    if chrono::Utc::now() > exp {
        anyhow::bail!("Licence has expired.");
    }

    // Hardware match – field is `Option<f32>`
    let match_pct = attestation.hardware.match_percentage.unwrap_or(100.0);
    if match_pct < 100.0 {
        anyhow::bail!(
            "Licence bound to different hardware (match: {:.0}%). Contact Intellectica AI LLC.",
            match_pct
        );
    }

    // Environment – field is `is_virtualized`
    if attestation.environment.is_virtualized {
        eprintln!("⚠️  Warning: virtualised/container environment detected.");
    }

    // Clock drift – rollback is inferred from excessive drift
    let drift = attestation.clock.drift_seconds.unwrap_or(0);
    if drift > 10 {
        anyhow::bail!(
            "System clock appears to have drifted significantly ({drift}s)."
        );
    }

    // Decode the organisation from the licence key itself
    let org = decode_license_org(key)?;

    // Write config and ledger
    std::fs::create_dir_all(config_dir)?;
    let config_path = config_dir.join("config.toml");
    std::fs::write(
        &config_path,
        format!(
            "[platform]\norg = \"{}\"\n\n[ledger]\npath = \"/var/verity/ledger\"\n\n[api]\nbind = \"0.0.0.0:8080\"\n",
            org
        ),
    )?;

    let ledger_path = config_dir.join("ledger");
    std::fs::create_dir_all(&ledger_path)?;

    println!("✅ Verity installed successfully.");
    println!("   Organisation: {}", org);
    println!("   Licence expires: {}", exp);
    println!("\nStart the platform with: verity serve");
    Ok(())
}

// --------------------------------------------------------------------
// Licence Status
// --------------------------------------------------------------------
fn license_status() -> anyhow::Result<()> {
    let vendor_pubkey = std::env!("VERITY_VENDOR_PUBKEY");
    let witness = licenz_core::SecurityWitness::new(vendor_pubkey)?;
    let config = licenz_core::WitnessConfig::default();

    // Read the stored licence file (the key was saved during install)
    let license_path = PathBuf::from("/etc/verity/license.lic");
    let stored_key = std::fs::read_to_string(&license_path)
        .unwrap_or_default();

    let attestation = witness.attest(&stored_key, &config)?;

    let org = decode_license_org(&stored_key).unwrap_or_else(|_| "Unknown".into());
    let match_pct = attestation.hardware.match_percentage.unwrap_or(100.0);

    println!("Organisation: {}", org);
    println!("Expiry:       {}", attestation.expiration.valid_until);
    println!("Hardware match: {:.0}%", match_pct);
    println!(
        "Signature:    {}",
        if attestation.signature_valid {
            "✅ valid"
        } else {
            "❌ invalid"
        }
    );
    Ok(())
}