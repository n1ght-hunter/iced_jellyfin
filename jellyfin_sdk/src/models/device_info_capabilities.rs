/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceInfoCapabilities : Gets or sets the capabilities.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfoCapabilities {
    #[serde(rename = "PlayableMediaTypes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub playable_media_types: Option<Option<Vec<String>>>,
    #[serde(rename = "SupportedCommands", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub supported_commands: Option<Option<Vec<crate::models::GeneralCommandType>>>,
    #[serde(rename = "SupportsMediaControl", skip_serializing_if = "Option::is_none")]
    pub supports_media_control: Option<bool>,
    #[serde(rename = "SupportsContentUploading", skip_serializing_if = "Option::is_none")]
    pub supports_content_uploading: Option<bool>,
    #[serde(rename = "MessageCallbackUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_callback_url: Option<Option<String>>,
    #[serde(rename = "SupportsPersistentIdentifier", skip_serializing_if = "Option::is_none")]
    pub supports_persistent_identifier: Option<bool>,
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "DeviceProfile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_profile: Option<Option<Box<crate::models::ClientCapabilitiesDeviceProfile>>>,
    #[serde(rename = "AppStoreUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<Option<String>>,
    #[serde(rename = "IconUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
}

impl DeviceInfoCapabilities {
    /// Gets or sets the capabilities.
    pub fn new() -> DeviceInfoCapabilities {
        DeviceInfoCapabilities {
            playable_media_types: None,
            supported_commands: None,
            supports_media_control: None,
            supports_content_uploading: None,
            message_callback_url: None,
            supports_persistent_identifier: None,
            supports_sync: None,
            device_profile: None,
            app_store_url: None,
            icon_url: None,
        }
    }
}


