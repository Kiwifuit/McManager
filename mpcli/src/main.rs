use clap::{Parser, Subcommand};
use log::{error, info, warn};
use mparse::{
    get_modpack_manifest, ForgeModpack, ModpackMetadata, ModpackProvider, ModrinthModpack,
};
use serde_json::from_str;
use std::{ffi::OsString, path::PathBuf};

mod install;
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

#[derive(Debug)]
enum ManifestType {
    Forge(ForgeModpack),
    Modrinth(ModrinthModpack),
}

fn show_modpack_info(meta: ModpackMetadata) {
    let meta = match meta.loader {
        ModpackProvider::Forge => {
            let file = from_str::<ForgeModpack>(&meta.raw);

            if file.is_err() {
                error!(
                    "Unable to parse forge modpack information: {}",
                    file.unwrap_err()
                );
                return;
            }

            ManifestType::Forge(file.unwrap())
        }
        ModpackProvider::Modrinth => {
            let file = from_str::<ModrinthModpack>(&meta.raw);

            if file.is_err() {
                error!(
                    "Unable to parse modrinth modpack information: {}",
                    file.unwrap_err()
                );
                return;
            }

            ManifestType::Modrinth(file.unwrap())
        }
        ModpackProvider::None => {
            panic!("somehow get_modpack_manifest provided a 'None' value, which shouldn't have happened");
        }
    };

    dbg!(meta);
}

fn subcmd_info(args: InfoArgs) {
    info!("Showing info for pack {}", args.file.display());
    match get_modpack_manifest(args.file) {
        Err(err) => error!("Unable to unpack modpack: {}", err),
        Ok(modpack) => show_modpack_info(modpack),
    };
}
fn subcmd_install(args: InstallArgs, install_dir: PathBuf) {
    info!(
        "Installing pack {} to {}",
        args.file.display(),
        install_dir.display()
    );
}
fn subcmd_uninstall(args: UninstallArgs, install_dir: PathBuf) {
    todo!()
}

fn main() {
    let global_args = Args::parse();

    let _ = logger::setup_logger(global_args.verbosity.log_level_filter());
    warn!("This program is partially complete, running in 'dry run' mode");

    match global_args.subcommand {
        Commands::Info(args) => subcmd_info(args),
        Commands::Install(args) => subcmd_install(args, global_args.minecraft_home),
        Commands::Uninstall(args) => subcmd_uninstall(args, global_args.minecraft_home),
    }
}
