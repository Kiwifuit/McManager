use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use reqwest::blocking::get;
use thiserror::Error;

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{absolute, Path},
};

use mparse::ModrinthModpack;

#[derive(Debug, Error)]
pub enum ModrinthInstallError {
    #[error("error while initiating progbar: {0}\n\tNote that under normal circumstances, this message should not be visible")]
    ProgbarTemplate(#[from] indicatif::style::TemplateError),
    #[error("network error: {0}. wifi problems?")]
    Net(#[from] reqwest::Error),
    #[error("i/o error: {0}\n\tNote that under normal circumstances, this message should not be visible")]
    Io(#[from] std::io::Error),
}

pub(super) fn download_mods<F: AsRef<Path>>(
    modpack: &ModrinthModpack,
    install_dir: &F,
) -> Result<(), ModrinthInstallError> {
    info!("Downloading mods");
    let mod_progress = ProgressBar::new(modpack.files.len() as u64)
        .with_message("Progress:")
        .with_style(
            ProgressStyle::with_template("{msg:>15} [{wide_bar}] {percent}%")?
                .progress_chars("=> "),
        );

    for file in &modpack.files {
        let outfilepath = absolute(install_dir.as_ref().join(&file.path)).unwrap();
        let file_url = file.downloads.first().unwrap();
        let mut resp = get(file_url)?;

        if !outfilepath.parent().unwrap().exists() {
            create_dir_all(outfilepath.parent().unwrap())?;
        }

        let filename = String::from(outfilepath.file_stem().unwrap().to_string_lossy());

        debug!("Downloading {}", filename);
        let mut buf = Vec::new();
        let mut outfile = File::create(outfilepath.clone())?;
        resp.read_to_end(&mut buf)?;
        outfile.write_all(&buf)?;

        mod_progress.inc(1);
    }

    Ok(())
}
