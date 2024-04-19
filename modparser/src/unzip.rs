use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use thiserror::Error;
use zip::ZipArchive;

const MOD_META_FILE: &str = "META-INF/mods.toml";

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

pub fn grab_meta_file<F: AsRef<Path>>(file: F) -> Result<String, UnzipError> {
    let zipfile = File::open(file)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let mut file = archive
        .by_name(MOD_META_FILE)
        .or(Err(UnzipError::MetaFileNotFound))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .or(Err(UnzipError::WriteToTempFile))?;

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::grab_meta_file;

    #[test]
    fn meta_get() {
        let file = "samples/tisadvanced-1.19.2-0.3.0.jar";
        let res = grab_meta_file(file);

        assert!(res.is_ok());
    }

    #[test]
    fn meta_readable() {
        let file = "samples/tisadvanced-1.19.2-0.3.0.jar";
        let res = grab_meta_file(file);

        assert!(res.is_ok());
    }
}
