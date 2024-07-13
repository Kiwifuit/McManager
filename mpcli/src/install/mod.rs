use std::path::Path;

use crate::types::ManifestType;

mod forge;
mod modrinth;

pub fn get_mods<F: AsRef<Path>>(manifest: &ManifestType, install_dir: &F) {
    match manifest {
        ManifestType::Forge(forge_manifest) => todo!(),
        ManifestType::Modrinth(modrinth_manifest) => {
            modrinth::download_mods(modrinth_manifest, install_dir)
        }
    }
}
