use anyhow::Context;
use futures_util::StreamExt;
use log::{error, info};
use mar::{get_artifact, get_versions, MavenArtifact};
use reqwest::get;
use thiserror::Error;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

mod post;
pub use post::*;

#[derive(Debug, Clone)]
pub enum ServerSoftware {
    Forge,
    Neoforge,
    Quilt,
    Fabric,
    Glowstone,
}

impl Display for ServerSoftware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Forge => "Forge",
                Self::Neoforge => "Neoforge",
                Self::Quilt => "Quilt",
                Self::Fabric => "Fabric",
                Self::Glowstone => "Glowstone",
            }
        )
    }
}

impl TryFrom<usize> for ServerSoftware {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Forge),
            1 => Ok(Self::Neoforge),
            2 => Ok(Self::Quilt),
            3 => Ok(Self::Fabric),
            4 => Ok(Self::Glowstone),
            _ => Err(String::from("No such server type")),
        }
    }
}

impl From<ServerSoftware> for MavenArtifact {
    fn from(value: ServerSoftware) -> Self {
        match value {
            ServerSoftware::Forge => MavenArtifact::new("forge", "net.minecraftforge"),
            ServerSoftware::Fabric => MavenArtifact::new("fabric-installer", "net.fabricmc"),
            ServerSoftware::Quilt => MavenArtifact::new("quilt-installer", "org.quiltmc"),
            ServerSoftware::Neoforge => MavenArtifact::new("neoforge", "net.neoforged"),
            ServerSoftware::Glowstone => MavenArtifact::new("glowstone", "net.glowstone"),
        }
    }
}

impl ServerSoftware {
    fn base_url(&self) -> String {
        match self {
            Self::Forge => "https://maven.minecraftforge.net",
            Self::Fabric => "https://maven.fabricmc.net",
            Self::Quilt => "https://maven.quiltmc.org/repository/release",
            Self::Neoforge => "https://maven.neoforged.net/releases",
            Self::Glowstone => "https://repo.glowstone.net/content/repositories/releases",
        }
        .to_string()
    }

    fn artifact_name<T: Display>(&self, version: T) -> String {
        match self {
            Self::Forge => format!("forge-{}-installer.jar", version),
            Self::Neoforge => format!("neoforge-{}-installer.jar", version),
            Self::Quilt => format!("quilt-installer-{}.jar", version),
            Self::Fabric => format!("fabric-installer-{}.jar", version),
            Self::Glowstone => format!("forge-{}-installer.jar", version), // TODO: Fix this
        }
    }

    fn get_args<'a>(&self, game_version: &'a str, install_dir: &'a str) -> Vec<&'a str> {
        match self {
            Self::Forge => vec!["--installServer", install_dir],
            Self::Neoforge => vec!["--installServer", install_dir],
            Self::Quilt => vec![
                "install",
                "server",
                game_version,
                "--install-dir",
                install_dir,
                "--create-scripts",
                "--download-server",
            ],
            Self::Fabric => vec![
                "server",
                "-dir",
                install_dir,
                "-mcversion",
                game_version,
                "-downloadMinecraft",
            ],
            Self::Glowstone => todo!(), // TODO: Also this
        }
    }
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("error while sending data to main thread: {0}")]
    MainThreadSender(#[from] std::sync::mpsc::SendError<String>),
    #[error("error while resolving maven artifact: {0}")]
    MavenResolve(#[from] mar::RepositoryError),
    #[error("error while resolving maven artifact: {0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Contextual(#[from] anyhow::Error),
}

pub struct ServerSoftwareOptions {
    server_type: ServerSoftware,
    software_version: String,
    game_version: String,
    install_dir: PathBuf,
}

impl ServerSoftwareOptions {
    pub fn with<T: ToString>(
        server_type: ServerSoftware,
        software_version: T,
        game_version: T,
        install_dir: PathBuf,
    ) -> Self {
        Self {
            server_type,
            software_version: software_version.to_string(),
            game_version: game_version.to_string(),
            install_dir,
        }
    }

    pub async fn build(&self, tx: Sender<String>) -> Result<(), InstallError> {
        info!(
            "installing {}v{} for mc{} to {}",
            self.server_type.clone(),
            self.software_version,
            self.game_version,
            self.install_dir.display()
        );

        self.download_artifact().await?;
        info!("spawning installer");
        let mut install_command = Command::new("java");
        let install_command = install_command
            .args(vec![
                "-jar",
                self.install_dir.join("installer.jar").to_str().unwrap(),
            ])
            .args(
                self.server_type
                    .get_args(&self.game_version, self.install_dir.to_str().unwrap()),
            )
            .stdout(Stdio::piped());

        info!(
            "installing {} with args: {:?}",
            install_command.get_program().to_string_lossy(),
            install_command.get_args()
        );

        let mut install_command = install_command.spawn()?;
        {
            let stdout = BufReader::new(install_command.stdout.as_mut().unwrap());

            for line in stdout.lines() {
                tx.send(line.unwrap_or_else(|e| format!("internal error: {}", e)))?;
            }
        }

        let status_code = install_command.wait()?;
        if status_code.success() {
            info!("install exited with code {}", status_code);

            // post install
            info!("post-install");
            post::add_run_sh(&self.install_dir).context("while preparing run script")?;
            post::write_user_jvm_args(&self.install_dir, "-Xms2G -Xmx8G -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true").context("while preparing User JVM args")?;
        } else {
            error!("install exited with code {}", status_code);
        }

        Ok(())
    }

    async fn download_artifact(&self) -> Result<(), InstallError> {
        let mut artifact = self.server_type.clone().into();
        let versions = get_versions(&artifact, self.server_type.base_url()).await?;

        if versions
            .versioning
            .versions()
            .contains(&self.software_version)
        {
            artifact.set_version(self.software_version.clone());
        }

        let artifact_url = get_artifact(
            &artifact,
            self.server_type.base_url(),
            self.server_type.artifact_name(&self.software_version),
        )?;

        info!("resolved artifact to: {:?}", artifact_url);
        let mut outfile = BufWriter::new(
            File::create_new(self.install_dir.join("installer.jar"))
                .context("while initializing download")?,
        );
        let mut resp = get(artifact_url).await?.bytes_stream();

        while let Some(chunk) = resp.next().await {
            outfile
                .write_all(&chunk?)
                .context("while downloading file")?;
        }

        outfile.flush().context("while finishing file")?;
        info!("artifact download OK");
        Ok(())
    }
}
