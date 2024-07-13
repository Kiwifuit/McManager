use clap::{Parser, Subcommand};
use mparse::{ForgeModpack, ModpackProviderMetadata, ModrinthModpack};
use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(unix)]
const DEFAULT_MINECRAFT_HOME: &str = "~/.minecraft";
#[cfg(target_os = "windows")]
const DEFAULT_MINECRAFT_HOME: &str = "%APPDATA%\\.minecraft";
#[cfg(target_os = "macos")]
const DEFAULT_MINECRAFT_HOME: &str = "~/Library/Application Support/minecraft";

pub fn get_default_minecraft_home() -> OsString {
    OsString::from(DEFAULT_MINECRAFT_HOME)
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
    /// Path to .minecraft dir where the game files are stored
    #[arg(short, long, default_value=get_default_minecraft_home())]
    pub minecraft_home: PathBuf,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a modpack
    Install(InstallArgs),
    /// Uninstall a modpack
    Uninstall(UninstallArgs),
    /// Show modpack information
    Info(InfoArgs),
}

#[derive(Debug, Parser)]
pub struct InstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    pub name: Option<String>,

    /// File to install. Must be a ZIP archive
    pub file: PathBuf,
}

#[derive(Debug, Parser)]
pub struct UninstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Debug, Parser)]
pub struct InfoArgs {
    /// File to install. Must be a ZIP archive
    pub file: PathBuf,
}

// TODO: Deprecate this whole thing
#[derive(Debug)]
pub enum ManifestType {
    Forge(ForgeModpack),
    Modrinth(ModrinthModpack),
}

impl ModpackProviderMetadata for ManifestType {
    fn overrides_dir(&self) -> &str {
        match self {
            Self::Forge(content) => content.overrides_dir(),
            Self::Modrinth(content) => content.overrides_dir(),
        }
    }

    fn modpack_name(&self) -> String {
        match self {
            Self::Forge(content) => content.modpack_name(),
            Self::Modrinth(content) => content.modpack_name(),
        }
    }
}
