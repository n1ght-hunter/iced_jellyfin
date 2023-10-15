/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MediaAttachment : Class MediaAttachment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAttachment {
    /// Gets or sets the codec.
    #[serde(rename = "Codec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub codec: Option<Option<String>>,
    /// Gets or sets the codec tag.
    #[serde(rename = "CodecTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<Option<String>>,
    /// Gets or sets the comment.
    #[serde(rename = "Comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    /// Gets or sets the index.
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Gets or sets the filename.
    #[serde(rename = "FileName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Option<String>>,
    /// Gets or sets the MIME type.
    #[serde(rename = "MimeType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Option<String>>,
    /// Gets or sets the delivery URL.
    #[serde(rename = "DeliveryUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delivery_url: Option<Option<String>>,
}

impl MediaAttachment {
    /// Class MediaAttachment.
    pub fn new() -> MediaAttachment {
        MediaAttachment {
            codec: None,
            codec_tag: None,
            comment: None,
            index: None,
            file_name: None,
            mime_type: None,
            delivery_url: None,
        }
    }
}


