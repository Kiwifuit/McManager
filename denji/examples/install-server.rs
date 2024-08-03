use anyhow::{Context, Result};
use humantime::format_duration;
use log::info;
use tempdir::TempDir;
use tokio::task::spawn;

use denji::{ServerSoftware, ServerSoftwareOptions};

use std::sync::mpsc::channel;
use std::time::Duration;

const CHANNEL_TIMEOUT: Duration = Duration::from_secs(90);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let root_dir = TempDir::new("test.denji.serverInstall")?.into_path();
    let install_server_opts = ServerSoftwareOptions::with(
        ServerSoftware::Forge,
        "1.20.4-49.1.4",
        "1.20.4",
        root_dir,
        "dockerfs",
    );
    let (tx, rx) = channel();
    let install_task = spawn(async move { install_server_opts.build(tx).await });

    info!(
        "started installer (timeout: {})",
        format_duration(CHANNEL_TIMEOUT)
    );
    loop {
        match rx.recv_timeout(CHANNEL_TIMEOUT) {
            Err(_e) => {
                break;
            }
            Ok(line) => {
                info!("{}", line)
            }
        }
    }

    install_task
        .await // Returns a Result<Result<(), InstallError>, JoinError>
        .context("while tryinig to join install task")? // JoinError Context
        .context("while installing minecraft server")?; // InstallError Context

    println!("you may test the channel and close this program when finished");

    Result::Ok(())
}
