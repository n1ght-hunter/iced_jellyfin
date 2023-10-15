/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageByNameInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Gets or sets the theme.
    #[serde(rename = "Theme", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Option<String>>,
    /// Gets or sets the context.
    #[serde(rename = "Context", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub context: Option<Option<String>>,
    /// Gets or sets the length of the file.
    #[serde(rename = "FileLength", skip_serializing_if = "Option::is_none")]
    pub file_length: Option<i64>,
    /// Gets or sets the format.
    #[serde(rename = "Format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub format: Option<Option<String>>,
}

impl ImageByNameInfo {
    pub fn new() -> ImageByNameInfo {
        ImageByNameInfo {
            name: None,
            theme: None,
            context: None,
            file_length: None,
            format: None,
        }
    }
}

