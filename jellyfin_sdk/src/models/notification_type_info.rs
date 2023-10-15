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
pub struct NotificationTypeInfo {
    #[serde(rename = "Type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    #[serde(rename = "IsBasedOnUserEvent", skip_serializing_if = "Option::is_none")]
    pub is_based_on_user_event: Option<bool>,
}

impl NotificationTypeInfo {
    pub fn new() -> NotificationTypeInfo {
        NotificationTypeInfo {
            r#type: None,
            name: None,
            enabled: None,
            category: None,
            is_based_on_user_event: None,
        }
    }
}

