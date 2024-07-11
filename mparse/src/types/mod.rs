pub mod forge;
pub mod modrinth;

pub use forge::ForgeModpack;
pub use modrinth::ModrinthModpack;

#[derive(PartialEq)]
pub enum ModpackProvider {
    Forge,
    Modrinth,
    None,
}

pub trait ModpackProviderMetadata {
    fn overrides_dir(&self) -> &str;
    fn modpack_name(&self) -> String;
}
