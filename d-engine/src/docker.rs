use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use anyhow::Context;
use log::{debug, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("error while running command: {0}")]
    Io(#[from] std::io::Error),

    #[error("general error: {0}. please read context for...well...context")]
    Channel(#[from] anyhow::Error),
}

// enum Status {
//     Exited,
//     Running,
// }

pub fn build_docker_image(
    server_name: String,
    server_version: String,
    game_version: String,
    tx: Sender<String>,
) -> Result<String, DockerError> {
    let (args, image_name) = build_docker_args(&server_name, &server_version, &game_version);
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

fn build_docker_args(
    server_name: &String,
    server_version: &String,
    game_version: &String,
) -> (Vec<String>, String) {
    (
        vec![
            "build".to_string(),
            "-t".to_string(),
            build_image_name(server_name, server_version, game_version),
            ".".to_string(),
        ],
        build_image_name(server_name, server_version, game_version),
    )
}

fn build_image_name(
    server_name: &String,
    server_version: &String,
    game_version: &String,
) -> String {
    format!("mcs/{}:{}-{}", server_name, game_version, server_version)
}
