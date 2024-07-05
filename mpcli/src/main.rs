use clap::{Parser, Subcommand};
use log::{info, warn};
use mparse::{get_modpack_manifest, ForgeModpack, ModrinthModpack};
use serde_json::from_reader;
use std::{ffi::OsString, path::PathBuf};

mod logger;

#[cfg(unix)]
const DEFAULT_MINECRAFT_HOME: &str = "~/.minecraft";
#[cfg(target_os = "windows")]
const DEFAULT_MINECRAFT_HOME: &str = "%APPDATA%\\.minecraft";
#[cfg(target_os = "macos")]
const DEFAULT_MINECRAFT_HOME: &str = "~/Library/Application Support/minecraft";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    /// Path to .minecraft dir where the game files are stored
    #[arg(short, long, default_value=get_default_minecraft_home())]
    minecraft_home: PathBuf,

    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a modpack
    Install(InstallArgs),
    /// Uninstall a modpack
    Uninstall(UninstallArgs),
    /// Show modpack information
    Info(InfoArgs),
}

#[derive(Debug, Parser)]
struct InstallArgs {
    /// File to install. Must be a ZIP archive
    file: PathBuf,
}

#[derive(Debug, Parser)]
struct UninstallArgs {}

#[derive(Debug, Parser)]
struct InfoArgs {
    /// File to install. Must be a ZIP archive
    file: PathBuf,
}

fn get_default_minecraft_home() -> OsString {
    OsString::from(DEFAULT_MINECRAFT_HOME)
}

fn main() {
    let args = Args::parse();

    let _ = logger::setup_logger(args.verbosity.log_level_filter());
    warn!("This program is partially complete, running in 'dry run' mode");

    match args.subcommand {
        Commands::Info(args) => todo!(),
        Commands::Install(args) => {
            info!("Installing pack {}", args.file.display());
        }
        Commands::Uninstall(args) => todo!(),
    }
}
