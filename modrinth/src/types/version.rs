use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct VersionQuery {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    loaders: Vec<Loader>,
    #[serde(rename = "game_versions", skip_serializing_if = "Vec::is_empty")]
    versions: Vec<String>,
    featured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Loader {
    Bukkit,
    Bungeecord,
    Canvas,
    Datapack,
    Fabric,
    Folia,
    Forge,
    Iris,
    Liteloader,
    Minecraft,
    Modloader,
    Neoforge,
    Optifine,
    Purpur,
    Quilt,
    Rift,
    Spigot,
    Sponge,
    Vanilla,
    Velocity,
    Waterfall,
}
