use mar::MavenArtifact;
use std::fmt::Display;
use std::path::PathBuf;

mod post;

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
            ServerSoftware::Neoforge => MavenArtifact::new("fabric-installer", "net.fabricmc"),
            ServerSoftware::Quilt => MavenArtifact::new("quilt-installer", "org.quiltmc"),
            ServerSoftware::Fabric => MavenArtifact::new("neoforge", "net.neoforged"),
            ServerSoftware::Glowstone => MavenArtifact::new("glowstone", "net.glowstone"),
        }
    }
}

impl ServerSoftware {
    pub fn base_url(&self) -> String {
        match self {
            Self::Forge => "https://maven.minecraftforge.net",
            Self::Neoforge => "https://maven.fabricmc.net",
            Self::Quilt => "https://maven.quiltmc.org/repository/release",
            Self::Fabric => "https://maven.neoforged.net/releases",
            Self::Glowstone => "https://repo.glowstone.net/content/repositories/releases",
        }
        .to_string()
    }

    pub fn artifact_name<T: Display>(&self, version: T) -> String {
        match self {
            Self::Forge => format!("forge-{}-installer.jar", version),
            Self::Neoforge => format!("neoforge-{}-installer.jar", version),
            Self::Quilt => format!("quilt-installer-{}.jar", version),
            Self::Fabric => format!("fabric-installer-{}.jar", version),
            Self::Glowstone => format!("forge-{}-installer.jar", version),
        }
    }

    pub fn post_install(&self, base_dir: PathBuf) {
        post::agree_eula(base_dir.clone());
        post::write_user_jvm_args(base_dir.clone(), "-Xms2G -Xmx8G -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true");
    }

    pub fn get_args<'a>(&self, game_version: &'a str, install_dir: &'a str) -> Vec<&'a str> {
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
            Self::Glowstone => todo!(),
        }
    }
}
