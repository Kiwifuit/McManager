use crate::types::get_default_minecraft_home;
use crate::types::{InfoArgs, InstallArgs, ManifestType, UninstallArgs};
use log::{debug, error, info};
use mparse::{
    get_modpack_manifest, unzip_modpack_to, ForgeModpack, ModpackMetadata, ModpackProvider,
    ModpackProviderMetadata, ModrinthModpack,
};
use serde_json::from_str;
use std::path::PathBuf;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use dirs::config_dir;

pub fn info(args: InfoArgs) {
    info!("Showing info for pack {}", args.file.display());
    match get_modpack_manifest(&args.file) {
        Err(err) => error!("Unable to unpack modpack: {}", err),
        Ok(modpack) => show_modpack_info(modpack),
    };
}

pub fn install(args: InstallArgs, install_dir: PathBuf) {
    debug!("Grabbing manifest...");
    let manifest_file = get_modpack_manifest(&args.file).unwrap();
    let manifest = match manifest_file.loader {
        ModpackProvider::Forge => {
            let forge_manifest = from_str::<ForgeModpack>(&manifest_file.raw);

            if forge_manifest.is_err() {
                error!(
                    "Unable to parse forge modpack information: {}",
                    forge_manifest.unwrap_err()
                );
                return;
            }

            ManifestType::Forge(forge_manifest.unwrap())
        }
        ModpackProvider::Modrinth => {
            let modrinth_manifest = from_str::<ModrinthModpack>(&manifest_file.raw);

            if modrinth_manifest.is_err() {
                error!(
                    "Unable to parse modrinth modpack information: {}",
                    modrinth_manifest.unwrap_err()
                );
                return;
            }

            ManifestType::Modrinth(modrinth_manifest.unwrap())
        }
        ModpackProvider::None => {
            panic!("somehow get_modpack_manifest provided a 'None' value, which shouldn't have happened");
        }
    };

    // resolve `install_dir` by OS
    let mut install_dir = if install_dir.as_os_str() == get_default_minecraft_home() {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let new_install_dir = config_dir().unwrap().as_path().join(".minecraft");

        #[cfg(unix)]
        let new_install_dir = std::path::absolute(install_dir).unwrap();

        new_install_dir
    } else if !install_dir.ends_with(".minecraft") {
        let mut install_dir = install_dir.to_path_buf();
        install_dir.push(".minecraft");

        std::path::absolute(install_dir).unwrap()
    } else {
        std::path::absolute(install_dir).unwrap()
    };

    // transform `install_dir` as required
    install_dir.push(format!("versions/{}", manifest.modpack_name()));

    // finalize `install_dir`
    let install_dir = std::path::absolute(install_dir).unwrap();
    info!(
        "Installing pack {} to {}",
        args.file.display(),
        install_dir.display()
    );

    println!("Downloading Mods");
    crate::install::get_mods(&manifest, &install_dir);
    println!("Unpacking overrides");
    unzip_modpack_to(args.file, &install_dir, &manifest);
    println!("Installed modpack at {}", install_dir.display());
}

pub fn uninstall(args: UninstallArgs, install_dir: PathBuf) {
    todo!()
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
