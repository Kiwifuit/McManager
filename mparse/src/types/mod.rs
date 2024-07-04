pub mod forge;
pub mod modrinth;

pub use forge::ForgeModpack;
pub use modrinth::ModrinthModpack;

#[derive(PartialEq)]
pub enum ModLoader {
    Forge,
    Modrinth,
    None,
}
