use serde::{Deserialize, Serialize, Serializer};

pub mod project;
pub mod query;
pub mod result;
pub mod version;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum License {
    Single(String),
    Detailed {
        id: String,
        name: String,
        url: Option<String>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModRequirement {
    Optional,
    Required,
    Unsupported,
    Unknown,
}

#[derive(Debug, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IndexBy {
    #[default]
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

impl ToString for Loader {
    fn to_string(&self) -> String {
        match self {
            Self::Bukkit => "bukkit",
            Self::Bungeecord => "bungeecord",
            Self::Canvas => "canvas",
            Self::Datapack => "datapack",
            Self::Fabric => "fabric",
            Self::Folia => "folia",
            Self::Forge => "forge",
            Self::Iris => "iris",
            Self::Liteloader => "liteloader",
            Self::Minecraft => "minecraft",
            Self::Modloader => "modloader",
            Self::Neoforge => "neoforge",
            Self::Optifine => "optifine",
            Self::Purpur => "purpur",
            Self::Quilt => "quilt",
            Self::Rift => "rift",
            Self::Spigot => "spigot",
            Self::Sponge => "sponge",
            Self::Vanilla => "vanilla",
            Self::Velocity => "velocity",
            Self::Waterfall => "waterfall",
        }
        .to_string()
    }
}

impl Serialize for Loader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

impl ToString for ProjectType {
    fn to_string(&self) -> String {
        match self {
            Self::Mod => "mod",
            Self::Modpack => "modpack",
            Self::Resourcepack => "resourcepack",
            Self::Shader => "shader",
        }
        .to_string()
    }
}

impl Serialize for ProjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Gallery {
    Single(String),
    Multiple(Vec<String>),
}

// This is the jankiest piece of shit
// I have yet to write. It is an amalgamation
// between me and ChatGPT's stupidity and
// creativity.
//
// If anyone knows serde hacks and can improve
// this, I AM BEGGING YOU PLEASE BROTHER
// FIX MY CODE
pub(crate) fn serialize_vec_urlencoded<S, T>(vec: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    todo!()
    // let mut serialized_str = String::new();
    // serialized_str.push('[');

    // for (i, item) in vec.iter().enumerate() {
    //     if i != 0 {
    //         serialized_str.push(',');
    //     }
    //     // Wrap the item in a temporary map
    //     let map = vec![("item", item)];

    //     let item_str = serde_urlencoded::to_string(&map).map_err(S::Error::custom)?;
    //     let item_str = item_str.trim_start_matches("item=");

    //     serialized_str.push_str(&format!("{:?}", item_str));
    // }

    // serialized_str.push(']');

    // serializer.serialize_str(&serialized_str)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(in crate::types) fn is_zero(num: &u8) -> bool {
    *num == 0
}
