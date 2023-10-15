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
pub enum RepeatMode {
    #[serde(rename = "RepeatNone")]
    RepeatNone,
    #[serde(rename = "RepeatAll")]
    RepeatAll,
    #[serde(rename = "RepeatOne")]
    RepeatOne,

}

impl ToString for RepeatMode {
    fn to_string(&self) -> String {
        match self {
            Self::RepeatNone => String::from("RepeatNone"),
            Self::RepeatAll => String::from("RepeatAll"),
            Self::RepeatOne => String::from("RepeatOne"),
        }
    }
}

impl Default for RepeatMode {
    fn default() -> RepeatMode {
        Self::RepeatNone
    }
}




