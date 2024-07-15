use std::fs::{copy, create_dir_all, read_dir, File};
use std::io::prelude::*;

use model::GenericModpackManifest;
mod model;

use log::{debug, error, info};
use serde_json::to_writer;
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use thiserror::Error;
use zip::{
    write::{FileOptions, ZipWriter},
    CompressionMethod,
};

pub const MANIFEST_NAME: &str = "mpack-mod.json";

#[derive(Debug, Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("zip error: {0}")]
    Compression(#[from] zip::result::ZipError),

    #[error("required folder does not exist: {0}")]
    DoesNotExist(String),

    #[error("{0} (code 1)")] // TODO: Devise a scheme for these dev codes
    StripPrefix(#[from] std::path::StripPrefixError),
}

pub fn write_modpack<P>(modpack_path: &P, archive_dir: &P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    debug!(
        "Packing modpack at {}",
        modpack_path.as_ref().file_name().unwrap().to_string_lossy()
    );

    let modpack_name = modpack_path.as_ref().file_name().unwrap();
    let modpack_file = File::create_new(
        archive_dir
            .as_ref()
            .join(modpack_name)
            .with_extension("zip"),
    )?;
    let mut archive = ZipWriter::new(modpack_file);
    let mut manifest = model::GenericModpackManifest::default();

    info!(
        "Creating modpack archive {:?} in {:?}",
        archive_dir
            .as_ref()
            .join(modpack_name)
            .with_extension("zip"),
        archive_dir.as_ref().to_string_lossy()
    );
    manifest.name = modpack_name.to_string_lossy();
    manifest.base_dir = modpack_path.as_ref().to_path_buf();

    if !modpack_path.as_ref().join("mods").exists() {
        return Err(Error::DoesNotExist("mods".to_string()));
    }

    for folder in vec!["mods", "config", "resourcepack"] {
        let base_dir = modpack_path.as_ref().join(&folder);
        let mut file_pool = vec![];
        if let Err(e) = find_files(&base_dir, &mut file_pool) {
            error!("Failed to find files within {:?}: {}", folder, e);
        }

        info!("Found {} file(s) in {} folder...", file_pool.len(), folder);
        file_pool
            .iter()
            .map(|file| {
                manifest.register_file(file, folder)?;

                Ok(())
            })
            .collect::<Result<Vec<()>, std::io::Error>>()?;
    }

    let tempdir = TempDir::new("modpack")?;
    let zipfs = make_zipfs_structure(&tempdir, &manifest);
    if zipfs.is_err() {
        archive.finish()?;
        tempdir.close()?;
        return Err(Error::Io(zipfs.unwrap_err()));
    }

    let zip_res = zip_dir(&mut archive, &zipfs.unwrap(), &tempdir.path().to_path_buf());

    if zip_res.is_err() {
        archive.finish()?;
        tempdir.close()?;
        return Err(zip_res.unwrap_err());
    }
    Ok(())
}

fn zip_dir<F, P>(archive: &mut ZipWriter<F>, path: &P, base_dir: &PathBuf) -> Result<(), Error>
where
    F: Write + Seek,
    P: AsRef<Path>,
{
    let options = FileOptions::default()
        .unix_permissions(0o644)
        .compression_method(CompressionMethod::Bzip2);
    // .compression_level(Some(9));
    let mut buf = vec![];

    info!("Zipping dir {} to modpack", path.as_ref().display());
    for entry in read_dir(path)? {
        let path = entry?.path();
        let arc_path = path
            .strip_prefix(&base_dir)?
            .to_str()
            .map(str::to_owned)
            .unwrap()
            .replace("\\", "/");

        dbg!(&arc_path);

        if path.is_dir() {
            archive.add_directory(&arc_path, options)?;
            zip_dir(archive, &path, base_dir)?;
            debug!("Created dir {}", arc_path);
        } else {
            archive.start_file(&arc_path, options)?;
            debug!("Created file {}", arc_path);

            let mut file = File::open(path)?;

            let written = file.read_to_end(&mut buf)?;
            archive.write_all(&buf)?;
            buf.clear();

            debug!("Written {} bytes to archive", written);
        }
    }

    Ok(())
}

fn make_zipfs_structure(
    dir: &TempDir,
    manifest: &GenericModpackManifest,
) -> std::io::Result<PathBuf> {
    let base_dir = &manifest.base_dir;

    info!(
        "Moving files to temporary directory: {}",
        dir.path().display()
    );
    for file in &manifest.files {
        let real_path = base_dir.join(&file.path);
        let dest_path = dir.path().join(&file.path);

        debug!("Copying {} -> {}", real_path.display(), dest_path.display());

        if !dest_path.parent().unwrap().exists() {
            debug!(
                "Created parent dir {}",
                dest_path.parent().unwrap().display()
            );
            create_dir_all(dest_path.parent().unwrap())?;
        }

        // Create file
        File::create_new(&dest_path)?;
        copy(&real_path, &dest_path)?;
        debug!("Copied {} -> {}", real_path.display(), dest_path.display());
    }

    info!("Dumping manifest");
    let mut manifest_file = File::create_new(dir.path().join(MANIFEST_NAME))?;
    to_writer(&mut manifest_file, manifest);

    info!("Copied all files, ready for zipping");
    Ok(dir.path().to_path_buf())
}

fn find_files<P: AsRef<Path>>(path: &P, pool: &mut Vec<PathBuf>) -> Result<(), Error> {
    debug!("Searching files in folder: {}", path.as_ref().display());

    for entry in read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            find_files(&path, pool)?;
        } else {
            debug!("Found file: {}", path.display());
            pool.push(path);
        }
    }

    Ok(())
}
