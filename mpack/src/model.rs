use std::path::PathBuf;

use serde::Serialize;
use sha1_smol::Digest;

#[derive(Debug, Serialize)]
pub struct GenericModpackManifest {
    version_commit: String,
    files: Vec<GenericModpackFile>,
}

#[derive(Debug, Serialize)]
pub struct GenericModpackFile {
    file_type: GenericModpackFileType,
    path: PathBuf,
    hash: Digest,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum GenericModpackFileType {
    Mod,
    Resourcepack,
    Config,
}
