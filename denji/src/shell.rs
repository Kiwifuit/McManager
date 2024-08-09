use mar::types::MavenArtifact;
use std::{borrow::Cow, fmt::Display, path::Path};

pub enum ServerSoftware {
    Forge,
    Neoforge,
    Fabric,
    Quilt,
    Glowstone,
}

pub struct MinecraftServer<S, I> {
    server: S,
    server_version: String,
    game_version: String,
    root_dir: I,
}

impl<I: AsRef<Path>, S: ServerSoftwareMeta> MinecraftServer<S, I> {}

trait ServerSoftwareMeta: Display + Into<MavenArtifact> {
    fn artifact_name<V: Display>(&self, version: V) -> String;
    fn installer_args<'a, I>(&self, installer_dir: I, game_version: String) -> Vec<Cow<'a, str>>
    where
        I: AsRef<Path> + 'a;
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

    fn installer_args<'a, I>(&self, install_dir: I, game_version: String) -> Vec<Cow<'a, str>>
    where
        I: AsRef<Path> + 'a,
    {
        // Massive hacky solution. I don't
        // like it, I really don't, but
        // Rust lowkey just had to be a
        // bitch, huh
        match self {
            Self::Forge => vec![
                Cow::from("--installServer"),
                Cow::from(install_dir.as_ref().to_string_lossy().to_string()),
            ],
            Self::Neoforge => vec![
                Cow::from("--installServer"),
                Cow::from(install_dir.as_ref().to_string_lossy().to_string()),
            ],
            Self::Quilt => vec![
                Cow::from("install"),
                Cow::from("server"),
                Cow::from(game_version),
                Cow::from(format!(
                    "--install-dir={}",
                    install_dir.as_ref().to_string_lossy()
                )),
                Cow::from("--create-scripts"),
                Cow::from("--download-server"),
            ],
            Self::Fabric => vec![
                Cow::from("server"),
                Cow::from("-dir"),
                Cow::from(install_dir.as_ref().to_string_lossy().to_string()),
                Cow::from("-mcversion"),
                Cow::from(game_version),
                Cow::from("-downloadMinecraft"),
            ],
            Self::Glowstone => todo!(), // TODO: Also this
        }
    }
}
