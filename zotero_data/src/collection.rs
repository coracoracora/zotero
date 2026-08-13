use derive_builder::Builder;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Library {
    r#type: String,
    pub id: usize,
    pub name: String,
    pub links: Links,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: usize,
    pub version: usize,
    pub links: Links,
    pub meta: GroupMeta,
    pub data: GroupData,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupMeta {
    pub created: String,
    pub last_modified: String,
    pub num_items: usize,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupData {
    pub id: usize,
    pub version: usize,
    pub name: String,
    pub owner: usize,
    pub r#type: String,
    pub description: String,
    pub url: String,
    pub has_image: Option<usize>,
    pub library_editing: String,
    pub library_reading: String,
    pub file_editing: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Links {
    #[serde(alias = "self")]
    pub self_link: Option<Link>,
    pub alternate: Link,
    // Only for collections
    pub up: Option<Link>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Link {
    pub href: String,
    pub r#type: String,
}

/// A struct representing a Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Collection {
    pub key: String,
    pub version: usize,
    pub library: Library,
    pub links: Links,
    pub meta: CollectionMeta,
    pub data: CollectionData,
}

/// This struct can be used to create a new Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Builder, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct CollectionData {
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing)]
    pub version: usize,
    pub name: String,
    pub parent_collection: StringOrBool,
    pub relations: HashMap<String, String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CollectionMeta {
    pub num_collections: Option<usize>,
    pub num_items: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntOrBool {
    Bool(bool),
    Int(i64),
}

impl Default for IntOrBool {
    fn default() -> IntOrBool {
        IntOrBool::Bool(false)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrBool {
    Bool(bool),
    String(String),
}

impl Default for StringOrBool {
    fn default() -> StringOrBool {
        StringOrBool::Bool(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::{Group, GroupData, GroupMeta, Link, Links};

    #[test]
    fn test_group_roundtrip() {
        let group_json = r#"
            {
                "id": 6608545,
                "version": 4,
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/groups/6608545",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/groups/crystaldrift",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "created": "2026-07-08T12:02:13Z",
                    "lastModified": "2026-07-08T12:04:46Z",
                    "numItems": 38
                },
                "data": {
                    "id": 6608545,
                    "version": 4,
                    "name": "Crystaldrift",
                    "owner": 16133,
                    "type": "PublicOpen",
                    "description": "Things I've found interesting in the tubes. I might write about them on my site, I might not.",
                    "url": "https://www.crystaldrift.net",
                    "hasImage": 1,
                    "libraryEditing": "admins",
                    "libraryReading": "all",
                    "fileEditing": "none"
                }
            }
        "#;

        let instantiated_group = Group {
            id: 6608545,
            version: 4,
            links: Links {
                self_link: Some(Link {
                    href: "https://api.zotero.org/groups/6608545".to_string(),
                    r#type: "application/json".to_string(),
                }),
                alternate: Link {
                    href: "https://www.zotero.org/groups/crystaldrift".to_string(),
                    r#type: "text/html".to_string(),
                },
                up: None,
            },
            meta: GroupMeta {
                created: "2026-07-08T12:02:13Z".to_string(),
                last_modified: "2026-07-08T12:04:46Z".to_string(),
                num_items: 38,
            },
            data: GroupData {
                id: 6608545,
                version: 4,
                name: "Crystaldrift".to_string(),
                owner: 16133,
                r#type: "PublicOpen".to_string(),
                description: "Things I've found interesting in the tubes. I might write about them on my site, I might not.".to_string(),
                url: "https://www.crystaldrift.net".to_string(),
                has_image: Some(1),
                library_editing: "admins".to_string(),
                library_reading: "all".to_string(),
                file_editing: "none".to_string(),

            }
        };

        // Test that we match the expected structure
        let deserialized_group: Result<Group, _> = serde_json::from_str(group_json);
        assert!(deserialized_group.is_ok());
        assert_eq!(instantiated_group, deserialized_group.unwrap());

        // Test roundtrip
        assert!(serde_json::to_string::<Group>(&instantiated_group).is_ok());
        // assert!(serde_json::deserialize::<Group>(serde_json::to_string::<Group>(instantiated_group).unwrap()))
    }
}
