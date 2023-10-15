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
pub struct BookInfoRemoteSearchQuery {
    #[serde(rename = "SearchInfo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_info: Option<Option<Box<crate::models::BookInfo>>>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    /// Gets or sets the provider name to search within if set.
    #[serde(rename = "SearchProviderName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_provider_name: Option<Option<String>>,
    /// Gets or sets a value indicating whether disabled providers should be included.
    #[serde(rename = "IncludeDisabledProviders", skip_serializing_if = "Option::is_none")]
    pub include_disabled_providers: Option<bool>,
}

impl BookInfoRemoteSearchQuery {
    pub fn new() -> BookInfoRemoteSearchQuery {
        BookInfoRemoteSearchQuery {
            search_info: None,
            item_id: None,
            search_provider_name: None,
            include_disabled_providers: None,
        }
    }
}


