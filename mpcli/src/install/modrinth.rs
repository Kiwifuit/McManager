use std::path::{absolute, Path};

use crate::ModrinthModpack;

pub(super) fn download_mods<F: AsRef<Path>>(modpack: ModrinthModpack, install_dir: &F) {
    for files in modpack.files {
        let outfile = absolute(install_dir.as_ref().join(files.path)).unwrap();
    }
}
