use mar::types::MavenArtifact;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;

macro_rules! args {
    ($ ( $arg:expr ),+ $(,)?) => {
        vec![$($arg.as_ref(), )+]
    }
}

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
    fn installer_args<'a, I>(&self, installer_dir: &'a I, game_version: &'a str) -> Vec<&'a OsStr>
    where
        I: AsRef<OsStr> + 'a;
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
}
