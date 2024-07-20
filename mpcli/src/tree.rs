use mparse::unzip_modpack_to;
use owo_colors::OwoColorize;
use std::fs::read_dir;
use std::path::Path;
use tempdir::TempDir;
use termtree::Tree;

use crate::types::ManifestType;

pub fn generate_tree<P: AsRef<Path>>(
    archive: P,
    manifest: &ManifestType,
) -> std::io::Result<Tree<String>> {
    let outdir = TempDir::new("mpcli")?;
    unzip_modpack_to(archive, &outdir, manifest);

    index_tree(outdir.path(), false)
}

fn index_tree<P: AsRef<Path>>(path: P, is_recursive: bool) -> std::io::Result<Tree<String>> {
    let tree = read_dir(&path)?.filter_map(|e| e.ok()).fold(
        Tree::new(if is_recursive {
            label(path.as_ref().canonicalize()?)
                .bright_blue()
                .bold()
                .to_string()
        } else {
            "<modpack root>".bright_purple().bold().to_string()
        }),
        |mut root, entry| {
            if entry.metadata().unwrap().is_dir() {
                root.push(index_tree(entry.path(), true).unwrap());
            } else {
                root.push(Tree::new(
                    label(entry.path()).bright_green().bold().to_string(),
                ));
            }

            root
        },
    );

    Ok(tree)
}

fn label<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
