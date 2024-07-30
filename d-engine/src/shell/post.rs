use anyhow::Context;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Error)]
pub(super) enum PostInstallationError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    // #[error("{0}")]
    // Generic(#[from] anyhow::Error),
}

pub(super) fn add_run_sh(root_dir: PathBuf) -> Result<(), PostInstallationError> {
    let filename = root_dir.join("run.sh");
    let mut lines = get_lines(&filename).unwrap_or(
        vec!["java", "-jar", "server.jar", "@user_jvm_args.txt", "\"$@\""]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );

    if let Some(line) = lines.last_mut() {
        *line = get_modded_line(&line);
    }

    let mut write_file = BufWriter::new(File::create(filename)?);

    for line in lines {
        writeln!(write_file, "{}", line)?;
    }

    Ok(())
}

pub fn agree_eula(base_dir: PathBuf) -> usize {
    let mut file = File::create(base_dir.join("eula.txt"))
        .context("while creating file")
        .unwrap();

    file.write(b"eula=true")
        .context("while writing to file")
        .unwrap()
}

pub fn write_user_jvm_args<T: ToString>(base_dir: PathBuf, args: T) {
    let mut file = File::create(base_dir.join("user_jvm_args.txt")).unwrap();

    let _ = file.write_all(args.to_string().as_bytes());
}

fn get_lines(filename: &PathBuf) -> Result<Vec<String>, PostInstallationError> {
    let read_file = BufReader::new(File::open(&filename)?);
    let lines: Vec<String> = read_file.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

fn get_modded_line(line: &String) -> String {
    let mut args = line.split(' ').collect::<Vec<&str>>();
    // index of the second to the last argument, typically a `"$@"`
    let sttl_arg_index = args.len() - 1;

    //
    if !line.contains("user_jvm_args.txt") {
        args.insert(2, "@user_jvm_args.txt");
    } else if !line.contains("\"$@\"") {
        args.push("\"$@\"");
    }

    args.insert(sttl_arg_index, "--nogui");

    args.join(" ").to_string()
}
