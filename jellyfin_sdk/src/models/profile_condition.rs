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
pub struct ProfileCondition {
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<crate::models::ProfileConditionType>,
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<crate::models::ProfileConditionValue>,
    #[serde(rename = "Value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<String>>,
    #[serde(rename = "IsRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}

impl ProfileCondition {
    pub fn new() -> ProfileCondition {
        ProfileCondition {
            condition: None,
            property: None,
            value: None,
            is_required: None,
        }
    }
}


