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
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or("info".into())
        )
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

fn install(key: &str, config_dir: &PathBuf) -> anyhow::Result<()> {
    let vendor_pubkey = std::env!("VERITY_VENDOR_PUBKEY");
    let config = licenz_core::SecurityConfig::default()
        .with_public_key(vendor_pubkey.as_bytes())
        .with_hardware_binding(true)
        .with_environment_check(true);

    let witness = licenz_core::SecurityWitness::new(config)?;
    let license_path = config_dir.join("license.lic");

    let attestation = witness.attest(key, &license_path)?;

    if !attestation.signature_valid {
        anyhow::bail!("Invalid licence signature. Contact Intellectica AI LLC.");
    }
    if attestation.expired {
        anyhow::bail!("Licence has expired.");
    }
    if attestation.hardware_mismatch {
        anyhow::bail!(
            "Licence is bound to different hardware (match: {}%). \
             Contact Intellectica AI LLC for a new licence.",
            attestation.hardware_match_percent
        );
    }
    if attestation.environment_suspicious {
        eprintln!("⚠️  Warning: virtualised/container environment detected.");
    }
    if attestation.clock_rollback_detected {
        anyhow::bail!("System clock appears to have been rolled back.");
    }

    std::fs::create_dir_all(config_dir)?;
    let config_path = config_dir.join("config.toml");
    std::fs::write(&config_path, format!(
        "[platform]\norg = \"{}\"\n\n[ledger]\npath = \"/var/verity/ledger\"\n\n[api]\nbind = \"0.0.0.0:8080\"\n",
        attestation.license_data.get("org").and_then(|v| v.as_str()).unwrap_or("Unknown")
    ))?;

    let ledger_path = config_dir.join("ledger");
    std::fs::create_dir_all(&ledger_path)?;

    println!("✅ Verity installed successfully.");
    println!("   Organisation: {}", attestation.license_data.get("org").and_then(|v| v.as_str()).unwrap_or("Unknown"));
    println!("   Licence expires: {}", attestation.expiry_date.unwrap_or_default());
    println!("\nStart the platform with: verity serve");
    Ok(())
}

fn license_status() -> anyhow::Result<()> {
    let vendor_pubkey = std::env!("VERITY_VENDOR_PUBKEY");
    let config = licenz_core::SecurityConfig::default()
        .with_public_key(vendor_pubkey.as_bytes())
        .with_hardware_binding(true);
    let witness = licenz_core::SecurityWitness::new(config)?;
    let license_path = PathBuf::from("/etc/verity/license.lic");
    let attestation = witness.attest("", &license_path)?;

    println!("Organisation: {}", attestation.license_data.get("org").and_then(|v| v.as_str()).unwrap_or("Unknown"));
    println!("Expiry:       {}", attestation.expiry_date.unwrap_or_default());
    println!("Hardware match: {}%", attestation.hardware_match_percent);
    println!("Signature:    {}", if attestation.signature_valid { "✅ valid" } else { "❌ invalid" });
    Ok(())
}
