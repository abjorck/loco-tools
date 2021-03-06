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
pub struct TranslationError {
    /// Unique asset ID for current translation
    #[serde(rename = "id")]
    pub id: String,
    /// Status of translation as string compatible with export status parameter
    #[serde(rename = "status")]
    pub status: String,
    /// Source text in default project locale
    #[serde(rename = "source")]
    pub source: String,
    /// Translated text in current target locale (unless locale is source)
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Validation errors as simple message strings
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
}

impl TranslationError {
    pub fn new(id: String, status: String, source: String, errors: Vec<String>) -> TranslationError {
        TranslationError {
            id,
            status,
            source,
            target: None,
            errors,
        }
    }
}


