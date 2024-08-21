#[cfg(feature = "types")]
use serde::Deserialize;

use std::rc::Rc;

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersions {
    pub group_id: Rc<str>,
    pub artifact_id: Rc<str>,
    pub versioning: MavenArtifactVersionVersioning,
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
    release: Rc<str>,
    latest: Rc<str>,
    last_updated: u64,
    versions: MAVersioningVersions,
}

#[cfg(feature = "types")]
impl MavenArtifactVersionVersioning {
    pub fn release(&self) -> &str {
        &self.release
    }

    pub fn latest(&self) -> &str {
        &self.latest
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn versions(&self) -> Rc<Vec<Rc<str>>> {
        self.versions.version.clone()
    }
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
pub struct MAVersioningVersions {
    version: Rc<Vec<Rc<str>>>,
}
