use std::path::Path;
use thiserror::Error;

use crate::types::ManifestType;

mod forge;
mod modrinth;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("WIP")]
    Forge,
    #[error("error while installing modrinth modpack: {0}")]
    Modrinth(#[from] modrinth::ModrinthInstallError),
}

pub fn get_mods<F: AsRef<Path>>(
    manifest: &ManifestType,
    install_dir: &F,
) -> Result<(), InstallError> {
    match manifest {
        ManifestType::Forge(forge_manifest) => todo!(),
        ManifestType::Modrinth(modrinth_manifest) => {
            modrinth::download_mods(modrinth_manifest, install_dir)?
        }
    };

    Ok(())
}
