#![cfg_attr(
    not(any(feature = "modrinth", feature = "forge", feature = "packing")),
    allow(unused_imports)
)]
use crate::types::get_default_minecraft_home;

#[cfg(feature = "packing")]
use crate::types::ExportArgs;

#[cfg(any(feature = "forge", feature = "modrinth"))]
use crate::types::{InfoArgs, InstallArgs, ManifestType, UninstallArgs};
use log::{debug, error, info};
#[cfg(any(feature = "forge", feature = "modrinth"))]
use mparse::{
    get_modpack_manifest, unzip_modpack_to, ForgeModpack, ModpackMetadata, ModpackProvider,
    ModpackProviderMetadata, ModrinthModpack,
};
use owo_colors::OwoColorize;

#[cfg(any(feature = "forge", feature = "modrinth"))]
use serde_json::from_str;
use std::path::PathBuf;
use thiserror::Error;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use dirs::config_dir;

#[cfg(any(feature = "forge", feature = "modrinth"))]
use crate::install::InstallError;
#[cfg(feature = "packing")]
use crate::pack::PackError;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(any(feature = "forge", feature = "modrinth"))]
    #[error("error while installing: {0}")]
    Install(#[from] InstallError),

    #[cfg(feature = "packing")]
    #[error("pack error: {0}")]
    Pack(#[from] PackError),

    #[cfg(any(feature = "forge", feature = "modrinth"))]
    #[error("unzip error: {0}. file corrupted or missing?")]
    Unzip(#[from] mparse::UnzipError),

    #[cfg(any(feature = "forge", feature = "modrinth"))]
    #[error("manifest parse error: {0}. manifest corrupted?")]
    Unparsable(#[from] serde_json::Error),
}

#[cfg(any(feature = "forge", feature = "modrinth"))]
pub fn info(args: InfoArgs) -> Result<(), CommandError> {
    info!("Showing info for pack {}", args.file.display());
    let mp_manifest = get_modpack_manifest(&args.file)?;
    let parsed_manifest = show_modpack_info(mp_manifest)?;
    let structure = crate::tree::generate_tree(&args.file, &parsed_manifest)?;

    println!(
        "{} {} for minecraft {}\nLoader: {} {}\nMods: {} mod(s) listed\nStructure:\n{}",
        parsed_manifest.name().bright_magenta(),
        parsed_manifest.pack_version().bright_magenta(),
        parsed_manifest.game_version().cyan(),
        parsed_manifest.loader().bright_green(),
        parsed_manifest.loader_version().bright_green(),
        parsed_manifest.mod_count().bright_yellow(),
        structure
    );
    Ok(())
}

#[cfg(any(feature = "forge", feature = "modrinth"))]
pub fn install(args: InstallArgs, install_dir: PathBuf) -> Result<(), CommandError> {
    debug!("Grabbing manifest...");
    let manifest_file = get_modpack_manifest(&args.file)?;
    let manifest = show_modpack_info(manifest_file)?;

    // resolve `install_dir` by OS
    let mut install_dir = get_modpack_home_dir(install_dir)?;

    // transform `install_dir` as required
    install_dir.push(manifest.modpack_name());

    // finalize `install_dir`
    let install_dir = std::path::absolute(install_dir)?;

    info!(
        "Installing pack {} to {}",
        args.file.display(),
        install_dir.display()
    );

    println!("Downloading Mods");
    crate::install::get_mods(&manifest, &install_dir)?;
    println!("Unpacking overrides");
    unzip_modpack_to(args.file, &install_dir, &manifest)?;
    println!(
        "Installed modpack at {}",
        install_dir.display().bright_purple()
    );

    Ok(())
}

#[cfg(any(feature = "forge", feature = "modrinth"))]
pub fn uninstall(args: UninstallArgs, install_dir: PathBuf) -> Result<(), CommandError> {
    todo!()
}

#[cfg(feature = "packing")]
pub fn export(args: ExportArgs, base_dir: PathBuf) -> Result<(), CommandError> {
    let home_dir = get_modpack_home_dir(base_dir)?;
    let modpacks = crate::pack::list_modpacks(&home_dir)?;
    let modpack_selected = crate::pack::select_modpack(args, &modpacks)?;

    let modpack = std::path::absolute(&modpacks[modpack_selected])?;
    let outdir = std::env::current_dir()?;

    debug!("Selected modpack at {:?}", &modpack.display().bright_blue());
    crate::pack::write_modpack(&modpack, &outdir)?;

    Ok(())
}

fn get_modpack_home_dir(base_dir: PathBuf) -> Result<PathBuf, CommandError> {
    let home_dir = if base_dir.as_os_str() == get_default_minecraft_home() {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let new_base_dir = config_dir().unwrap().as_path().join(".minecraft/versions");

        #[cfg(unix)]
        let new_base_dir = std::path::absolute(base_dir)?;

        new_base_dir
    } else if !base_dir.ends_with(".minecraft/versions") {
        let mut base_dir = base_dir.to_path_buf();
        base_dir.push(".minecraft/versions");

        std::path::absolute(base_dir)?
    } else {
        std::path::absolute(base_dir)?
    };

    Ok(home_dir)
}

#[cfg(any(feature = "forge", feature = "modrinth"))]
#[cfg_attr(
    any(not(feature = "forge"), not(feature = "modrinth")),
    allow(unreachable_code)
)]
fn show_modpack_info(meta: ModpackMetadata) -> Result<ManifestType, CommandError> {
    let meta = match meta.loader {
        ModpackProvider::Forge => {
            #[cfg(not(feature = "forge"))]
            return panic!("This version of mpcli is not capable of parsing forge modpacks");

            let forge_manifest = from_str::<ForgeModpack>(&meta.raw)?;
            #[cfg(feature = "forge")]
            ManifestType::Forge(forge_manifest)
        }
        ModpackProvider::Modrinth => {
            #[cfg(not(feature = "modrinth"))]
            return panic!("This version of mpcli is not capable of parsing modrinth modpacks");

            let modrinth_manifest = from_str::<ModrinthModpack>(&meta.raw)?;
            #[cfg(feature = "modrinth")]
            ManifestType::Modrinth(modrinth_manifest)
        }
        ModpackProvider::None => {
            panic!("somehow get_modpack_manifest provided a 'None' value, which shouldn't have happened");
        }
    };

    Ok(meta)
}
