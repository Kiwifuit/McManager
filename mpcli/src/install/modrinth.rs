use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use reqwest::blocking::get;

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{absolute, Path},
};

use crate::ModrinthModpack;

pub(super) fn download_mods<F: AsRef<Path>>(modpack: &ModrinthModpack, install_dir: &F) {
    info!("Downloading mods");
    let mod_progress = ProgressBar::new(modpack.files.len() as u64)
        .with_message("Progress:")
        .with_style(ProgressStyle::with_template("{msg} {wide_bar} {percent_precise}%").unwrap());

    for file in &modpack.files {
        let outfilepath = absolute(install_dir.as_ref().join(&file.path)).unwrap();
        let file_url = file.downloads.first().unwrap();
        let mut resp = get(file_url).unwrap();

        if !outfilepath.parent().unwrap().exists() {
            create_dir_all(outfilepath.parent().unwrap()).expect("expected");
        }

        let filename = String::from(outfilepath.file_stem().unwrap().to_string_lossy());

        info!("Downloading {}", filename);
        let mut buf = Vec::new();
        let mut outfile = File::create(outfilepath.clone()).unwrap();
        resp.read_to_end(&mut buf)
            .expect("expected read/write process to suceed :(");
        outfile
            .write_all(&buf)
            .expect("expected read/write process to suceed :(");
    }
}
