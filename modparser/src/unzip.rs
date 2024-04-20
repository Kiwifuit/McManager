use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use thiserror::Error;
use zip::ZipArchive;

const FORGE_META: &str = "META-INF/mods.toml";
const FABRIC_META: &str = "fabric.mod.json";

pub struct ModMeta {
    pub(crate) loader: ModLoader,
    pub raw: String,
}

pub enum ModLoader {
    Forge,
    Fabric,
    Guess,
}

#[derive(Error, Debug)]
pub enum UnzipError {
    #[error("unable to open file: {0}")]
    FileOpen(#[from] std::io::Error),

    #[error("error during reading the zip file: {0}")]
    ZipRead(#[from] zip::result::ZipError),

    #[error("/META-INF/mods.toml file not found within mod")]
    MetaFileNotFound,

    #[error("temporary file was not made")]
    TempFileNotMade,

    #[error("write to temporary file was not made")]
    WriteToTempFile,
}

pub fn grab_meta_file<F: AsRef<Path>>(file: F, ml_type: ModLoader) -> Result<ModMeta, UnzipError> {
    let zipfile = File::open(file)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let (config_file, loader) = match ml_type {
        ModLoader::Guess => {
            if archive.by_name(FORGE_META).is_ok() {
                (FORGE_META, ModLoader::Forge)
            } else {
                (FABRIC_META, ModLoader::Fabric)
            }
        }
        ModLoader::Forge => (FORGE_META, ModLoader::Forge),
        ModLoader::Fabric => (FABRIC_META, ModLoader::Fabric),
    };

    let mut file = archive
        .by_name(config_file)
        .or(Err(UnzipError::MetaFileNotFound))?;

    let mut raw = String::new();
    file.read_to_string(&mut raw)
        .or(Err(UnzipError::WriteToTempFile))?;

    Ok(ModMeta { loader, raw })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_get_forge() {
        let file = "samples/forge/tisadvanced-1.19.2-0.3.0.jar";
        let res = grab_meta_file(file, ModLoader::Forge);

        assert!(res.is_ok());
        assert!(!res.unwrap().raw.is_empty());
    }

    #[test]
    fn meta_get_fabric() {
        let file = "samples/fabric/antique-atlas-2.5.0+1.20.jar";
        let res = grab_meta_file(file, ModLoader::Fabric);

        assert!(res.is_ok());
        assert!(!res.unwrap().raw.is_empty());
    }

    #[test]
    fn meta_readable() {
        let forge_mod = grab_meta_file(
            "samples/forge/tisadvanced-1.19.2-0.3.0.jar",
            ModLoader::Guess,
        );
        let fabric_mod = grab_meta_file(
            "samples/fabric/antique-atlas-2.5.0+1.20.jar",
            ModLoader::Guess,
        );

        assert!(forge_mod.is_ok());
        assert!(!forge_mod.unwrap().raw.is_empty());

        assert!(fabric_mod.is_ok());
        assert!(!fabric_mod.unwrap().raw.is_empty());
    }
}
