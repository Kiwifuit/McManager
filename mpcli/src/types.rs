use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(any(feature = "forge", feature = "modrinth"))]
use mparse::ModpackProviderMetadata;

#[cfg(feature = "forge")]
use mparse::ForgeModpack;
#[cfg(feature = "modrinth")]
use mparse::ModrinthModpack;

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
#[command(version = format!("v{}-{}", env!("CARGO_PKG_VERSION"), env!("GIT_SHA_SHORT")), about, long_about = None)]
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
    #[cfg(any(feature = "forge", feature = "modrinth"))]
    /// Install a modpack
    Install(InstallArgs),
    #[cfg(any(feature = "forge", feature = "modrinth"))]
    /// Uninstall a modpack
    Uninstall(UninstallArgs),
    #[cfg(any(feature = "forge", feature = "modrinth"))]
    /// Show modpack information
    Info(InfoArgs),

    #[cfg(feature = "packing")]
    /// Exports a modpack to an archive
    Export(ExportArgs),
}

#[cfg(feature = "packing")]
#[derive(Debug, Parser)]
pub struct ExportArgs {
    /// Modpack name.
    pub name: Option<String>,
}

#[derive(Debug, Parser)]
#[cfg(any(feature = "forge", feature = "modrinth"))]
pub struct InstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    pub name: Option<String>,

    /// File to install. Must be a ZIP archive
    pub file: PathBuf,
}

#[derive(Debug, Parser)]
#[cfg(any(feature = "forge", feature = "modrinth"))]
pub struct UninstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Debug, Parser)]
#[cfg(any(feature = "forge", feature = "modrinth"))]
pub struct InfoArgs {
    /// File to install. Must be a ZIP archive
    pub file: PathBuf,
}

// TODO: Deprecate this whole thing
#[derive(Debug)]
#[cfg(any(feature = "forge", feature = "modrinth"))]
pub enum ManifestType {
    #[cfg(feature = "forge")]
    Forge(ForgeModpack),
    #[cfg(feature = "modrinth")]
    Modrinth(ModrinthModpack),
}

#[cfg(any(feature = "modrinth", feature = "forge"))]
impl ManifestType {
    pub fn name(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => manifest.name.clone(),
            ManifestType::Modrinth(manifest) => manifest.name.clone(),
        }
    }
    pub fn pack_version(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => manifest.version.clone(),
            ManifestType::Modrinth(manifest) => manifest.version_id.clone(),
        }
    }
    pub fn game_version(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => manifest.minecraft.version.clone(),
            #[cfg(feature = "modrinth")]
            ManifestType::Modrinth(manifest) => {
                manifest.dependencies.first().unwrap().version.clone()
            }
        }
    }
    pub fn loader(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => {
                let raw_id = &manifest
                    .minecraft
                    .mod_loaders
                    .iter()
                    .filter(|loader| loader.primary)
                    .next()
                    .unwrap()
                    .id;

                raw_id.split('-').next().unwrap().to_string()
            }
            #[cfg(feature = "modrinth")]
            ManifestType::Modrinth(manifest) => {
                manifest.dependencies.last().unwrap().dependency.clone()
            }
        }
    }
    pub fn loader_version(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => {
                let raw_id = &manifest
                    .minecraft
                    .mod_loaders
                    .iter()
                    .filter(|loader| loader.primary)
                    .next()
                    .unwrap()
                    .id;

                raw_id.split('-').last().unwrap().to_string()
            }
            #[cfg(feature = "modrinth")]
            ManifestType::Modrinth(manifest) => {
                manifest.dependencies.last().unwrap().version.clone()
            }
        }
    }

    pub fn mod_count(&self) -> usize {
        match self {
            #[cfg(feature = "forge")]
            ManifestType::Forge(manifest) => manifest.files.len(),
            #[cfg(feature = "modrinth")]
            ManifestType::Modrinth(manifest) => manifest.files.len(),
        }
    }
}

#[cfg(any(feature = "forge", feature = "modrinth"))]
impl ModpackProviderMetadata for ManifestType {
    fn overrides_dir(&self) -> &str {
        match self {
            #[cfg(feature = "forge")]
            Self::Forge(content) => content.overrides_dir(),
            #[cfg(feature = "modrinth")]
            Self::Modrinth(content) => content.overrides_dir(),
        }
    }

    fn modpack_name(&self) -> String {
        match self {
            #[cfg(feature = "forge")]
            Self::Forge(content) => content.modpack_name(),
            #[cfg(feature = "modrinth")]
            Self::Modrinth(content) => content.modpack_name(),
        }
    }
}
