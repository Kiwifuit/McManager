use std::borrow::Cow;
use std::io::prelude::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use log::{debug, info};
use serde::{Deserialize, Serialize};
use sha1_smol::{Digest, Sha1};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericModpackManifest<'a> {
    pub version_commit: String,
    pub name: Cow<'a, str>,
    pub files: Vec<GenericModpackFile>,

    #[serde(skip)]
    pub(crate) base_dir: PathBuf,
}

impl<'a> Default for GenericModpackManifest<'a> {
    fn default() -> Self {
        Self {
            version_commit: env!("GIT_SHA_LONG").to_string(),
            name: Cow::Owned(String::new()),
            files: vec![],
            base_dir: PathBuf::new(),
        }
    }
}

impl<'a> GenericModpackManifest<'a> {
    fn hash_file<P: AsRef<Path>>(file: &P) -> io::Result<Digest> {
        info!("Hashing file {}", file.as_ref().display());

        let mut hash = Sha1::new();
        let mut hash_file = File::open(file).unwrap();
        let mut buf = vec![];

        hash_file.read_to_end(&mut buf)?;
        hash.update(&buf);

        let digest = hash.digest();
        debug!(
            "SHA1 Hash of {:?}: {:?}",
            file.as_ref().display(),
            digest.to_string()
        );

        Ok(digest)
    }

    pub fn register_file<P: AsRef<Path>, T: Into<GenericModpackFileType>>(
        &mut self,
        file: P,
        ftype: T,
    ) -> io::Result<()> {
        let ftype = ftype.into();

        info!(
            "Registering file {} type {:?}",
            file.as_ref().display(),
            &ftype
        );
        let hash = Self::hash_file(&file.as_ref())?;

        self.files.push(GenericModpackFile {
            file_type: ftype,
            path: file.as_ref().to_path_buf(),
            hash,
        });

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericModpackFile {
    pub file_type: GenericModpackFileType,
    pub path: PathBuf,
    pub hash: Digest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenericModpackFileType {
    Mod,
    Resourcepack,
    Config,
}

impl ToString for GenericModpackFileType {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Mod => "mod",
            Self::Resourcepack => "resourcepack",
            Self::Config => "config",
        })
    }
}

impl From<String> for GenericModpackFileType {
    fn from(value: String) -> Self {
        value.to_lowercase().as_str().into()
    }
}

impl From<&str> for GenericModpackFileType {
    fn from(value: &str) -> Self {
        match value {
            // This configuration supports both the folder name
            // and its type-as-a-str
            "mod" | "mods" => Self::Mod,
            "resourcepack" | "resourcepacks" => Self::Resourcepack,
            "config" => Self::Config,
            // I'm gonna leave it like this
            "" | _ => panic!(":("),
        }
    }
}
