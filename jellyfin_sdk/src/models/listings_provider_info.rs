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
pub struct ListingsProviderInfo {
    #[serde(rename = "Id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "Type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "Username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
    #[serde(rename = "Password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    #[serde(rename = "ListingsId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub listings_id: Option<Option<String>>,
    #[serde(rename = "ZipCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    #[serde(rename = "Country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<String>>,
    #[serde(rename = "Path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "EnabledTuners", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled_tuners: Option<Option<Vec<String>>>,
    #[serde(rename = "EnableAllTuners", skip_serializing_if = "Option::is_none")]
    pub enable_all_tuners: Option<bool>,
    #[serde(rename = "NewsCategories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub news_categories: Option<Option<Vec<String>>>,
    #[serde(rename = "SportsCategories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sports_categories: Option<Option<Vec<String>>>,
    #[serde(rename = "KidsCategories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub kids_categories: Option<Option<Vec<String>>>,
    #[serde(rename = "MovieCategories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_categories: Option<Option<Vec<String>>>,
    #[serde(rename = "ChannelMappings", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_mappings: Option<Option<Vec<crate::models::NameValuePair>>>,
    #[serde(rename = "MoviePrefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_prefix: Option<Option<String>>,
    #[serde(rename = "PreferredLanguage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Option<String>>,
    #[serde(rename = "UserAgent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Option<String>>,
}

impl ListingsProviderInfo {
    pub fn new() -> ListingsProviderInfo {
        ListingsProviderInfo {
            id: None,
            r#type: None,
            username: None,
            password: None,
            listings_id: None,
            zip_code: None,
            country: None,
            path: None,
            enabled_tuners: None,
            enable_all_tuners: None,
            news_categories: None,
            sports_categories: None,
            kids_categories: None,
            movie_categories: None,
            channel_mappings: None,
            movie_prefix: None,
            preferred_language: None,
            user_agent: None,
        }
    }
}


