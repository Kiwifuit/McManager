use serde::Deserialize;

mod project;
mod version;

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
