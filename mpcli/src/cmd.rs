use crate::types::get_default_minecraft_home;
use crate::types::{InfoArgs, InstallArgs, ManifestType, UninstallArgs};
use log::{debug, error, info};
use mparse::{
    get_modpack_manifest, unzip_modpack_to, ForgeModpack, ModpackMetadata, ModpackProvider,
    ModpackProviderMetadata, ModrinthModpack,
};
use serde_json::from_str;
use std::path::PathBuf;
use thiserror::Error;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use dirs::config_dir;

use crate::install::InstallError;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("error while installing: {0}")]
    Install(#[from] InstallError),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unzip error: {0}. file corrupted?")]
    Unzip(#[from] mparse::UnzipError),
    #[error("manifest parse error: {0}. manifest corrupted?")]
    Unparsable(#[from] serde_json::Error),
}

pub fn info(args: InfoArgs) -> Result<(), CommandError> {
    info!("Showing info for pack {}", args.file.display());
    let mp_manifest = get_modpack_manifest(&args.file)?;
    let parsed_manifest = show_modpack_info(mp_manifest)?;

    // println!("Name: {}",);

    Ok(())
}

pub fn install(args: InstallArgs, install_dir: PathBuf) -> Result<(), CommandError> {
    debug!("Grabbing manifest...");
    let manifest_file = get_modpack_manifest(&args.file)?;
    let manifest = show_modpack_info(manifest_file)?;

    // resolve `install_dir` by OS
    let mut install_dir = if install_dir.as_os_str() == get_default_minecraft_home() {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let new_install_dir = config_dir().unwrap().as_path().join(".minecraft");

        #[cfg(unix)]
        let new_install_dir = std::path::absolute(install_dir)?;

        new_install_dir
    } else if !install_dir.ends_with(".minecraft") {
        let mut install_dir = install_dir.to_path_buf();
        install_dir.push(".minecraft");

        std::path::absolute(install_dir)?
    } else {
        std::path::absolute(install_dir)?
    };

    // transform `install_dir` as required
    install_dir.push(format!("versions/{}", manifest.modpack_name()));

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
    println!("Installed modpack at {}", install_dir.display());

    Ok(())
}

pub fn uninstall(args: UninstallArgs, install_dir: PathBuf) -> Result<(), CommandError> {
    todo!()
}

fn show_modpack_info(meta: ModpackMetadata) -> Result<ManifestType, CommandError> {
    let meta = match meta.loader {
        ModpackProvider::Forge => {
            let forge_manifest = from_str::<ForgeModpack>(&meta.raw)?;
            ManifestType::Forge(forge_manifest)
        }
        ModpackProvider::Modrinth => {
            let modrinth_manifest = from_str::<ModrinthModpack>(&meta.raw)?;
            ManifestType::Modrinth(modrinth_manifest)
        }
        ModpackProvider::None => {
            panic!("somehow get_modpack_manifest provided a 'None' value, which shouldn't have happened");
        }
    };

    Ok(meta)
}
