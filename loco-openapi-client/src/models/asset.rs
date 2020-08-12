/*
 * Loco REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.24
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Unique asset identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Content type for translations
    #[serde(rename = "type")]
    pub _type: Type,
    /// Optional context descriptor
    #[serde(rename = "context")]
    pub context: String,
    /// Optional notes for translators
    #[serde(rename = "notes")]
    pub notes: String,
    /// Time last modified in UTC
    #[serde(rename = "modified")]
    pub modified: String,
    /// Number of associated plural forms
    #[serde(rename = "plurals")]
    pub plurals: i32,
    /// Array of terms asset is tagged with
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<serde_json::Value>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<crate::models::AssetProgress>,
}

impl Asset {
    pub fn new(id: String, _type: Type, context: String, notes: String, modified: String, plurals: i32, tags: Vec<String>) -> Asset {
        Asset {
            id,
            _type,
            context,
            notes,
            modified,
            plurals,
            tags,
            aliases: None,
            progress: None,
        }
    }
}

/// Content type for translations
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "xml")]
    Xml,
}
