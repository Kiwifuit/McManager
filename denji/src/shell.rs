use anyhow::Context;
use futures_util::StreamExt;
use log::{error, info};
use mar::types::MavenArtifact;
use mar::{get_artifact, get_versions};
use reqwest::get;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::Sender;
use thiserror::Error;

macro_rules! args {
    ($ ( $arg:expr ),+ $(,)?) => {
        vec![$($arg.as_ref(), )+]
    }
}

#[derive(Clone, Copy)]
pub enum ServerSoftware {
    Forge,
    Neoforge,
    Fabric,
    Quilt,
    Glowstone,
}

#[derive(Debug, Error)]
pub enum ServerInstallError {
    #[error("error while trying to fetch artifact data: {0}")]
    Artifact(#[from] mar::RepositoryError),
    #[error("version for artifact not found: {0}")]
    Version(String),
    #[error("net error: {0}")]
    Net(#[from] reqwest::Error),
    #[error("{0}")]
    Contextual(#[from] anyhow::Error),
}

pub struct MinecraftServer<S, I> {
    server: S,
    server_version: String,
    game_version: String,
    root_dir: I,
}

impl<I: AsRef<Path>, S: ServerSoftwareMeta> MinecraftServer<S, I> {
    pub async fn build_server(&self, tx: Sender<String>) -> Result<(), ServerInstallError> {
        Ok(())
    }

    async fn download_server(&self) -> Result<(), ServerInstallError> {
        let mut artifact: MavenArtifact = self.server.into();
        let versions = get_versions(&artifact).await?;

        if !versions
            .versioning
            .versions()
            .contains(&self.server_version)
        {
            error!("unable to find version {}", self.server_version);

            return Err(ServerInstallError::Version(
                self.server.artifact_name(&self.server_version),
            ));
        }

        info!("version {} resolved!", self.server_version);
        artifact.set_version(self.server_version.clone());

        let artifact_url =
            get_artifact(&artifact, self.server.artifact_name(&self.server_version))?;

        info!("resolved artifact to {:?}", artifact_url);
        info!("starting download...");
        let mut file = BufWriter::new(
            File::create_new(self.root_dir.as_ref().join("installer.jar"))
                .context("while creating file")?,
        );
        let mut stream = get(artifact_url).await?.bytes_stream();

        let mut total = 0;
        while let Some(chunk) = stream.next().await {
            total += file.write(&chunk?).context("while downloading file")?;
        }

        info!("finished download (downloaded {} bytes)", total);
        Ok(())
    }
}

trait ServerSoftwareMeta: Display + Into<MavenArtifact> + Copy {
    fn artifact_name<V: Display>(&self, version: V) -> String;
    fn installer_args<'a, I>(&self, installer_dir: &'a I, game_version: &'a str) -> Vec<&'a OsStr>
    where
        I: AsRef<OsStr> + ?Sized + 'a;
    fn run_sh_content(&self) -> Vec<String>;
}

impl Display for ServerSoftware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Forge => "forge",
                Self::Neoforge => "neoforge",
                Self::Fabric => "fabric",
                Self::Quilt => "quilt",
                Self::Glowstone => "glowstone",
            }
        )
    }
}

impl From<ServerSoftware> for MavenArtifact {
    fn from(value: ServerSoftware) -> Self {
        match value {
            ServerSoftware::Forge => "maven.minecraftforge.net:net.minecraftforge:forge:",
            ServerSoftware::Neoforge => "maven.neoforged.net/releases:net.neoforged:neoforge:",
            ServerSoftware::Fabric => "maven.fabricmc.net:net.fabricmc:fabric-installer:",
            ServerSoftware::Quilt => {
                "maven.quiltmc.org/repository/release:org.quiltm:quilt-installer:"
            }
            ServerSoftware::Glowstone => {
                "repo.glowstone.net/content/repositories/snapshots:net.glowstone:glowstone:"
            }
        }
        .parse()
        .unwrap()
    }
}

impl ServerSoftwareMeta for ServerSoftware {
    fn artifact_name<V: Display>(&self, version: V) -> String {
        match self {
            Self::Forge => format!("forge-{}-installer.jar", version),
            Self::Neoforge => format!("neoforge-{}-installer.jar", version),
            Self::Quilt => format!("quilt-installer-{}.jar", version),
            Self::Fabric => format!("fabric-installer-{}.jar", version),
            Self::Glowstone => todo!(), // TODO: Fix this
        }
    }

    fn installer_args<'a, I>(&self, install_dir: &'a I, game_version: &'a str) -> Vec<&'a OsStr>
    where
        I: AsRef<OsStr> + 'a,
    {
        match self {
            Self::Forge => args!["--installServer", install_dir],
            Self::Neoforge => args!["--installServer", install_dir],
            Self::Quilt => args![
                "install",
                "server",
                game_version,
                "--install-dir",
                install_dir,
                "--create-scripts",
                "--download-server"
            ],
            Self::Fabric => args![
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

    fn run_sh_content(&self) -> Vec<String> {
        match self {
            ServerSoftware::Fabric | ServerSoftware::Quilt => vec![
                "#!/usr/bin/env sh".to_string(),
                format!(
                    "java -jar {}-server-launch.jar @user_jvm_args.txt \"$@\"",
                    self.to_string()
                ),
            ],
            _ => vec![
                "#!/usr/bin/env sh".to_string(),
                "java -jar server.jar @user_jvm_args.txt \"$@\"".to_string(),
            ],
        }
    }
}
