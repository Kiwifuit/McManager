use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use log::{debug, error, info};
use thiserror::Error;
use zip::ZipArchive;

use crate::ModpackProvider;

const FORGE_META: &str = "manifest.json";
const MODRINTH_META: &str = "modrinth.index.json";

#[derive(Debug, Error)]
pub enum UnzipError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("no manifest from modrinth or forge found!")]
    NoManifest,
}

pub struct ModpackMetadata {
    pub loader: ModpackProvider,
    pub raw: String,
}

pub fn get_modpack_manifest<F: AsRef<Path>>(file: F) -> Result<ModpackMetadata, UnzipError> {
    let zipfile = File::open(file)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let (manifest_file, loader) = if archive.by_name(FORGE_META).is_ok() {
        info!("Modpack manifest found at {}", FORGE_META);
        (FORGE_META, ModpackProvider::Forge)
    } else if archive.by_name(MODRINTH_META).is_ok() {
        info!("Modpack manifest found at {}", MODRINTH_META);
        (MODRINTH_META, ModpackProvider::Modrinth)
    } else {
        error!("No manifest found!");
        ("", ModpackProvider::None)
    };

    if loader == ModpackProvider::None {
        return Err(UnzipError::NoManifest);
    }

    let mut file = archive.by_name(manifest_file).expect("expected that by here, modpack provider should be either forge or modrinth, so this should not appear at all");

    info!("Reading manifest file at {}", manifest_file);
    let mut raw = String::new();
    let len = file.read_to_string(&mut raw)?;
    debug!("Read {} bytes to buffer", len);

    Ok(ModpackMetadata { loader, raw })
}
