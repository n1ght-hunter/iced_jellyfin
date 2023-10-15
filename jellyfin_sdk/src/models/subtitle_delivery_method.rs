/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SubtitleDeliveryMethod : Delivery method to use during playback of a specific subtitle format.

/// Delivery method to use during playback of a specific subtitle format.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubtitleDeliveryMethod {
    #[serde(rename = "Encode")]
    Encode,
    #[serde(rename = "Embed")]
    Embed,
    #[serde(rename = "External")]
    External,
    #[serde(rename = "Hls")]
    Hls,
    #[serde(rename = "Drop")]
    Drop,

}

impl ToString for SubtitleDeliveryMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Encode => String::from("Encode"),
            Self::Embed => String::from("Embed"),
            Self::External => String::from("External"),
            Self::Hls => String::from("Hls"),
            Self::Drop => String::from("Drop"),
        }
    }
}

impl Default for SubtitleDeliveryMethod {
    fn default() -> SubtitleDeliveryMethod {
        Self::Encode
    }
}




