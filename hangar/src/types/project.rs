use super::{DateTime, HangarVisibility};
use bitflags::bitflags;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProject {
    pub created_at: DateTime,
    pub name: String,
    pub namespace: HangarProjectNamespace,
    pub last_updated: DateTime,
    pub avatar_url: String,
    pub description: String,
    pub category: HangarProjectCategory,
    pub visibility: HangarVisibility,
    pub settings: HangarProjectSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectSettings {
    pub links: Option<Vec<HangarProjectLinks>>,
    pub tags: HangarProjectTags,
    pub license: HangarProjectLicense,
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectNamespace {
    owner: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HangarProjectLinks {
    links: Vec<HangarProjectLink>,
}

#[derive(Debug, Deserialize)]
struct HangarProjectLink {
    pub id: u8,
    pub name: String,
    pub url: String,
}

bitflags! {
    #[derive(Debug)]
    pub struct HangarProjectTags: u8 {
        const ADDON          = 1;
        const LIBRARY        = 2;
        const SUPPORTS_FOLIA = 3;
    }
}

impl<'de> Deserialize<'de> for HangarProjectTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tags: Vec<String> = Vec::deserialize(deserializer)?;
        let mut flags = Self::empty();

        for tag in tags {
            match tag.as_str() {
                "ADDON" => flags |= Self::ADDON,
                "LIBRARY" => flags |= Self::LIBRARY,
                "SUPPORTS_FOLIA" => flags |= Self::SUPPORTS_FOLIA,
                other => {
                    return Err(serde::de::Error::unknown_variant(
                        other,
                        &["ADDON", "LIBRARY", "SUPPORTS_FOLIA"],
                    ))
                }
            }
        }

        Ok(flags)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HangarProjectCategory {
    AdminTools,
    Chat,
    DevTools,
    Economy,
    Gameplay,
    Games,
    Protection,
    RolePlaying,
    WorldManagement,
    Misc,
    Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectLicense {
    name: String,
    url: String,

    #[serde(rename = "type")]
    license_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn one_project() {
        let raw = r#"
{
	"createdAt": "2022-12-22T14:04:48.773082Z",
	"name": "Maintenance",
	"namespace": {
		"owner": "kennytv",
		"slug": "Maintenance"
	},
	"stats": {
		"views": 49672,
		"downloads": 9660,
		"recentViews": 5414,
		"recentDownloads": 1395,
		"stars": 106,
		"watchers": 61
	},
	"category": "admin_tools",
	"lastUpdated": "2024-05-17T13:48:41.703391Z",
	"visibility": "public",
	"avatarUrl": "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1",
	"description": "The most customizable maintenance plugin for your Minecraft server you can find.",
	"userActions": {
		"starred": false,
		"watching": false,
		"flagged": false
	},
	"settings": {
		"links": [
			{
				"id": 0,
				"type": "top",
				"title": "top",
				"links": [
					{
						"id": 5,
						"name": "Donate",
						"url": "https://github.com/sponsors/kennytv"
					},
					{
						"id": 1,
						"name": "Issues",
						"url": "https://github.com/kennytv/Maintenance/issues"
					},
					{
						"id": 2,
						"name": "Source",
						"url": "https://github.com/kennytv/Maintenance"
					},
					{
						"id": 3,
						"name": "Support",
						"url": "https://discord.gg/vGCUzHq"
					},
					{
						"id": 4,
						"name": "Wiki",
						"url": "https://github.com/kennytv/Maintenance/wiki"
					}
				]
			}
		],
		"tags": [],
		"license": {
			"name": "GPL",
			"url": "https://github.com/kennytv/Maintenance/blob/main/LICENSE.txt",
			"type": "GPL"
		},
		"keywords": [
			"maintenance",
			"maintenancemode"
		],
		"sponsors": "",
		"donation": {
			"enable": false,
			"subject": ""
		}
	}
}"#;
        let project = from_str(&raw);

        dbg!(&project);
        assert!(project.is_ok());

        let project: HangarProject = project.unwrap();
        unreachable!();
    }
}
