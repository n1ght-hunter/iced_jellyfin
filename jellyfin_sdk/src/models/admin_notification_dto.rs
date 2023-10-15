/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AdminNotificationDto : The admin notification dto.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminNotificationDto {
    /// Gets or sets the notification name.
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Gets or sets the notification description.
    #[serde(rename = "Description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "NotificationLevel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<Option<crate::models::NotificationLevel>>,
    /// Gets or sets the notification url.
    #[serde(rename = "Url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
}

impl AdminNotificationDto {
    /// The admin notification dto.
    pub fn new() -> AdminNotificationDto {
        AdminNotificationDto {
            name: None,
            description: None,
            notification_level: None,
            url: None,
        }
    }
}

