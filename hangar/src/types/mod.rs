use serde::Deserialize;

pub mod project;
pub mod version;

pub use project::HangarProjects;
pub use version::HangarVersions;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HangarVisibility {
    Public,
    New,
    NeedsChanges,
    NeedsApproval,
    SoftDelete,
}
