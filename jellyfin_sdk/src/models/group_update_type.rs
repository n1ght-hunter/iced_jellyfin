/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupUpdateType : Enum GroupUpdateType.

/// Enum GroupUpdateType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupUpdateType {
    #[serde(rename = "UserJoined")]
    UserJoined,
    #[serde(rename = "UserLeft")]
    UserLeft,
    #[serde(rename = "GroupJoined")]
    GroupJoined,
    #[serde(rename = "GroupLeft")]
    GroupLeft,
    #[serde(rename = "StateUpdate")]
    StateUpdate,
    #[serde(rename = "PlayQueue")]
    PlayQueue,
    #[serde(rename = "NotInGroup")]
    NotInGroup,
    #[serde(rename = "GroupDoesNotExist")]
    GroupDoesNotExist,
    #[serde(rename = "CreateGroupDenied")]
    CreateGroupDenied,
    #[serde(rename = "JoinGroupDenied")]
    JoinGroupDenied,
    #[serde(rename = "LibraryAccessDenied")]
    LibraryAccessDenied,

}

impl ToString for GroupUpdateType {
    fn to_string(&self) -> String {
        match self {
            Self::UserJoined => String::from("UserJoined"),
            Self::UserLeft => String::from("UserLeft"),
            Self::GroupJoined => String::from("GroupJoined"),
            Self::GroupLeft => String::from("GroupLeft"),
            Self::StateUpdate => String::from("StateUpdate"),
            Self::PlayQueue => String::from("PlayQueue"),
            Self::NotInGroup => String::from("NotInGroup"),
            Self::GroupDoesNotExist => String::from("GroupDoesNotExist"),
            Self::CreateGroupDenied => String::from("CreateGroupDenied"),
            Self::JoinGroupDenied => String::from("JoinGroupDenied"),
            Self::LibraryAccessDenied => String::from("LibraryAccessDenied"),
        }
    }
}

impl Default for GroupUpdateType {
    fn default() -> GroupUpdateType {
        Self::UserJoined
    }
}




