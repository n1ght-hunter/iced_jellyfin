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
pub enum TranscodeSeekInfo {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "Bytes")]
    Bytes,

}

impl ToString for TranscodeSeekInfo {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => String::from("Auto"),
            Self::Bytes => String::from("Bytes"),
        }
    }
}

impl Default for TranscodeSeekInfo {
    fn default() -> TranscodeSeekInfo {
        Self::Auto
    }
}




