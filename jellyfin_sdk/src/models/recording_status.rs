/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecordingStatus {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "ConflictedOk")]
    ConflictedOk,
    #[serde(rename = "ConflictedNotOk")]
    ConflictedNotOk,
    #[serde(rename = "Error")]
    Error,

}

impl ToString for RecordingStatus {
    fn to_string(&self) -> String {
        match self {
            Self::New => String::from("New"),
            Self::InProgress => String::from("InProgress"),
            Self::Completed => String::from("Completed"),
            Self::Cancelled => String::from("Cancelled"),
            Self::ConflictedOk => String::from("ConflictedOk"),
            Self::ConflictedNotOk => String::from("ConflictedNotOk"),
            Self::Error => String::from("Error"),
        }
    }
}

impl Default for RecordingStatus {
    fn default() -> RecordingStatus {
        Self::New
    }
}




