/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseItemPerson : This is used by the api to get information about a Person within a BaseItem.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemPerson {
    /// Gets or sets the name.
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Gets or sets the identifier.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Gets or sets the role.
    #[serde(rename = "Role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<String>>,
    /// Gets or sets the type.
    #[serde(rename = "Type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    /// Gets or sets the primary image tag.
    #[serde(rename = "PrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_image_tag: Option<Option<String>>,
    #[serde(rename = "ImageBlurHashes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Option<Box<crate::models::BaseItemPersonImageBlurHashes>>>,
}

impl BaseItemPerson {
    /// This is used by the api to get information about a Person within a BaseItem.
    pub fn new() -> BaseItemPerson {
        BaseItemPerson {
            name: None,
            id: None,
            role: None,
            r#type: None,
            primary_image_tag: None,
            image_blur_hashes: None,
        }
    }
}

