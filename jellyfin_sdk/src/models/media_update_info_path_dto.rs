/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MediaUpdateInfoPathDto : The media update info path.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaUpdateInfoPathDto {
    /// Gets or sets media path.
    #[serde(rename = "Path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    /// Gets or sets media update type.  Created, Modified, Deleted.
    #[serde(rename = "UpdateType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub update_type: Option<Option<String>>,
}

impl MediaUpdateInfoPathDto {
    /// The media update info path.
    pub fn new() -> MediaUpdateInfoPathDto {
        MediaUpdateInfoPathDto {
            path: None,
            update_type: None,
        }
    }
}


