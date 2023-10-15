/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncPlayUserAccessType : Enum SyncPlayUserAccessType.

/// Enum SyncPlayUserAccessType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyncPlayUserAccessType {
    #[serde(rename = "CreateAndJoinGroups")]
    CreateAndJoinGroups,
    #[serde(rename = "JoinGroups")]
    JoinGroups,
    #[serde(rename = "None")]
    None,

}

impl ToString for SyncPlayUserAccessType {
    fn to_string(&self) -> String {
        match self {
            Self::CreateAndJoinGroups => String::from("CreateAndJoinGroups"),
            Self::JoinGroups => String::from("JoinGroups"),
            Self::None => String::from("None"),
        }
    }
}

impl Default for SyncPlayUserAccessType {
    fn default() -> SyncPlayUserAccessType {
        Self::CreateAndJoinGroups
    }
}



