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
pub struct LiveTvInfo {
    /// Gets or sets the services.
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::LiveTvServiceInfo>>,
    /// Gets or sets a value indicating whether this instance is enabled.
    #[serde(rename = "IsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Gets or sets the enabled users.
    #[serde(rename = "EnabledUsers", skip_serializing_if = "Option::is_none")]
    pub enabled_users: Option<Vec<String>>,
}

impl LiveTvInfo {
    pub fn new() -> LiveTvInfo {
        LiveTvInfo {
            services: None,
            is_enabled: None,
            enabled_users: None,
        }
    }
}


