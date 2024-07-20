use chrono::{DateTime, Utc};
use details::*;
use serde::Deserialize;

mod details;
mod traits;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProject {
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub visibility: HangarProjectVisibility,
    pub description: String,
    pub author: String,
    #[serde(deserialize_with = "traits::deserialize_kv")]
    pub downloads: Vec<HangarProjectDownload>,
    #[serde(deserialize_with = "traits::deserialize_kv")]
    pub plugin_dependencies: Vec<HangarProjectPluginDependencies>,
    #[serde(deserialize_with = "traits::deserialize_kv")]
    pub platform_dependencies: Vec<HangarProjectPlatformDependencies>,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HangarProjectPlatform {
    Paper,
    Waterfall,
    Velocity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HangarProjectVisibility {
    Public,
    New,
    NeedsChanges,
    NeedsApproval,
    SoftDelete,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectDownload {
    pub platform: HangarProjectPlatform,

    #[serde(flatten)]
    pub details: HPDownloadDetails,
}

impl traits::KeyValueType for HangarProjectDownload {
    type Key = HangarProjectPlatform;
    type Value = HPDownloadDetails;

    fn init(key: Self::Key, value: Self::Value) -> Self {
        Self {
            platform: key,
            details: value,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectPluginDependencies {
    pub name: String,

    #[serde(flatten)]
    pub details: Vec<HPPluginDependencyDetails>,
}

impl traits::KeyValueType for HangarProjectPluginDependencies {
    type Key = String;
    type Value = Vec<HPPluginDependencyDetails>;

    fn init(key: Self::Key, value: Self::Value) -> Self {
        Self {
            name: key,
            details: value,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectPlatformDependencies {
    pub platform: HangarProjectPlatform,
    pub version: Vec<String>,
}

impl traits::KeyValueType for HangarProjectPlatformDependencies {
    type Key = HangarProjectPlatform;
    type Value = Vec<String>;

    fn init(key: Self::Key, value: Self::Value) -> Self {
        Self {
            platform: key,
            version: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn one_version() {
        let raw = r#"
        {
            "createdAt": "2024-05-17T13:48:41.703391Z",
            "name": "4.2.1",
            "visibility": "public",
            "description": "Happy birthday Minecraft once again :)\n* The proxied maintenance status is now also shown for forced hosts (thanks to alec-jensen)\n* Offline player fetching improvements (thanks to EpicPlayerA10)\n  * Added the setting `fallback-to-offline-uuid` to use offline player uuids in maintenance whitelist commands (defaults to false)\n* Maintenance now skips the plugin remapping process on 1.20.5+ Paper servers\n* Small optimizations\n* Updated language files from [Crowdin](https://crowdin.com/translate/maintenance)\n  * Massive thank you to all the contributors! These include: vortix22, xdalul, CodeZhangBorui, jesusskye, Doc94,\n    Texider_, marvin2k0, leonard.bausenwein, pro.timeo.kerjean, meviper, troev5, Murka124, jhqwqmc, Pryzinho,\n    raysetratyboy, Kolja07, and rikunightcore",
            "stats": {
                "totalDownloads": 2608,
                "platformDownloads": {
                    "PAPER": 1764,
                    "WATERFALL": 102,
                    "VELOCITY": 742
                }
            },
            "author": "kennytv",
            "reviewState": "reviewed",
            "channel": {
                "createdAt": "2022-12-22T14:04:48.875876Z",
                "name": "Release",
                "description": null,
                "color": "00E1E1",
                "flags": [
                    "SENDS_NOTIFICATIONS",
                    "PINNED",
                    "FROZEN"
                ]
            },
            "pinnedStatus": "CHANNEL",
            "downloads": {
                "PAPER": {
                    "fileInfo": {
                        "name": "Maintenance-4.2.1.jar",
                        "sizeBytes": 1461873,
                        "sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
                },
                "WATERFALL": {
                    "fileInfo": {
                        "name": "Maintenance-4.2.1.jar",
                        "sizeBytes": 1461873,
                        "sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
                },
                "VELOCITY": {
                    "fileInfo": {
                        "name": "Maintenance-Velocity-4.2.1.jar",
                        "sizeBytes": 5375841,
                        "sha256Hash": "6a7c0ecc80ea30e7423ebf5754be7afb30e0b00d821423f6aff15d9f2287349e"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/VELOCITY/Maintenance-Velocity-4.2.1.jar"
                }
            },
            "pluginDependencies": {
                "PAPER": [
                    {
                        "name": "ProtocolLib",
                        "required": false,
                        "externalUrl": null,
                        "platform": "PAPER"
                    }
                ]
            },
            "platformDependencies": {
                "PAPER": [
                    "1.10",
                    "1.10.1",
                    "1.10.2",
                    "1.11",
                    "1.11.1",
                    "1.11.2",
                    "1.12",
                    "1.12.1",
                    "1.12.2",
                    "1.13",
                    "1.13.1",
                    "1.13.2",
                    "1.14",
                    "1.14.1",
                    "1.14.2",
                    "1.14.3",
                    "1.14.4",
                    "1.15",
                    "1.15.1",
                    "1.15.2",
                    "1.16",
                    "1.16.1",
                    "1.16.2",
                    "1.16.3",
                    "1.16.4",
                    "1.16.5",
                    "1.17",
                    "1.17.1",
                    "1.18",
                    "1.18.1",
                    "1.18.2",
                    "1.19",
                    "1.19.1",
                    "1.19.2",
                    "1.19.3",
                    "1.19.4",
                    "1.20",
                    "1.20.1",
                    "1.20.2",
                    "1.20.3",
                    "1.20.4",
                    "1.20.5",
                    "1.20.6",
                    "1.21",
                    "1.8",
                    "1.9",
                    "1.9.1",
                    "1.9.2",
                    "1.9.3",
                    "1.9.4"
                ],
                "WATERFALL": [
                    "1.20"
                ],
                "VELOCITY": [
                    "3.0",
                    "3.1",
                    "3.2",
                    "3.3"
                ]
            },
            "platformDependenciesFormatted": {
                "PAPER": "1.8-1.21",
                "WATERFALL": "1.20",
                "VELOCITY": "3.0-3.3"
            }
        }
        "#;
        let project = from_str(raw);

        dbg!(&project);
        assert!(project.is_ok());
        let _project: HangarProject = project.unwrap();
    }
}
