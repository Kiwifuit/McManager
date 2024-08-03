use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

use anyhow::Context;
use log::info;
use log::{debug, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("error while running command: {0}")]
    Io(#[from] std::io::Error),

    #[error("general error: {0}. please read context for...well...context")]
    Channel(#[from] anyhow::Error),
}

pub(crate) fn build_docker_image<P: AsRef<Path>>(
    server_name: String,
    server_version: String,
    game_version: String,
    build_dir: P,
    tx: Sender<String>,
) -> Result<String, DockerError> {
    write_dockerfile(build_dir.as_ref())?;

    let (args, image_name) = build_docker_args(
        &server_name,
        &server_version,
        &game_version,
        build_dir.as_ref(),
    );

    info!(
        "building docker image at {} into {:?}",
        build_dir.as_ref().display(),
        image_name
    );

    let mut docker = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdout = BufReader::new(docker.stdout.as_mut().unwrap());

        for line in stdout.lines() {
            let line = line.unwrap_or_default();

            debug!("sending line: {}", line);
            tx.send(line)
                .context("while streaming docker build output")?;
        }
    }

    let exit_stat = docker.wait()?;

    if let Some(code) = exit_stat.code() {
        tx.send(format!("docker build exited with code {}", code))
            .context("while wrapping up `docker build` command")?;
    } else {
        error!(
            "NON FATAL ERROR occurred. The `docker build` command was forcibly killed: ({})",
            exit_stat
        );

        tx.send(String::from("docker build was forcibly killed"))
            .context("while wrapping up `docker build` command")?;
    }
    Ok(image_name)
}

fn write_dockerfile(build_dir: &Path) -> Result<usize, DockerError> {
    let mut dockerfile = File::create(build_dir.join("Dockerfile"))?;
    let dockerfile_content = crate::generate_dockerfile(17);

    Ok(dockerfile.write(dockerfile_content.as_bytes())?)
}

fn build_docker_args(
    server_name: &String,
    server_version: &String,
    game_version: &String,
    build_dir: &Path,
) -> (Vec<String>, String) {
    (
        vec![
            "build".to_string(),
            "-t".to_string(),
            build_image_name(server_name, server_version, game_version),
            "--progress".to_string(),
            "rawjson".to_string(),
            build_dir.to_string_lossy().to_string(),
        ],
        build_image_name(server_name, server_version, game_version),
    )
}

fn build_image_name(
    server_name: &String,
    server_version: &String,
    game_version: &String,
) -> String {
    format!("mcs/{}-{}:{}", server_name, server_version, game_version)
}
