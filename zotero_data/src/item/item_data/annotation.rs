use crate::shared_fields::{ItemCommon, Tag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use derive_builder::Builder;
use zotero_derive::ItemCommon;

/// A standalone note. Notes can be used for organizing and annotating in Zotero. If you cite a standalone note, Zotero will use the first 120 characters as the item title (and will treat the note as an author-less and date-less item). Citing notes is not a reliable way to add standalone commentary to a bibliography or reference list.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, ItemCommon)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct AnnotationData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[builder(setter(skip))]
    pub version: usize,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub parent_item: String,
    #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    /// NOTE: This is included only for compatibility with ItemCommon.
    /// Annotations themselves don't have titles, so this will always
    /// be empty.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_text: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_comment: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_color: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_page_label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_sort_index: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub annotation_position: String,
    pub tags: Vec<Tag>,
    pub relations: HashMap<String, String>,
    #[serde(skip_serializing)]
    pub date_added: String,
    #[serde(skip_serializing)]
    pub date_modified: String,
}

fn default_document_type() -> String {
    "note".to_string()
}

use crate::ToJson;
impl ToJson for AnnotationData {}
