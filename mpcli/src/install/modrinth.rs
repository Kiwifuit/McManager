use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use reqwest::blocking::get;

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{absolute, Path},
};

use crate::ModrinthModpack;

const DOWNLOAD_BUFFER_LEN: usize = 512;

pub(super) fn download_mods<F: AsRef<Path>>(modpack: &ModrinthModpack, install_dir: &F) {
    info!("Downloading mods");

    for file in &modpack.files {
        let outfilepath = absolute(install_dir.as_ref().join(&file.path)).unwrap();
        let file_url = file.downloads.first().unwrap();
        let mut resp = get(file_url).unwrap();

        if !outfilepath.parent().unwrap().exists() {
            create_dir_all(outfilepath.parent().unwrap());
        }

        let filename = String::from(outfilepath.file_stem().unwrap().to_string_lossy());

        info!("Downloading {}", filename);
        let mut buf = [0u8; DOWNLOAD_BUFFER_LEN];
        let mut remaining = resp.content_length().unwrap();
        let mut outfile = File::create(outfilepath.clone()).unwrap();
        let download_progress = ProgressBar::new(resp.content_length().unwrap())
            .with_message(filename)
            .with_style(
                ProgressStyle::with_template("{wide_msg:<20} [{bar:80}] {percent}%")
                    .unwrap()
                    .progress_chars("#>-"),
            );

        download_progress.tick();
        while remaining != 0 {
            let read = resp.read(&mut buf).unwrap() as u64;
            outfile.write(&buf);

            remaining -= read;
            download_progress.inc(read);
        }

        download_progress.finish();
    }
}
