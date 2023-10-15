/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClientLogDocumentResponseDto : Client log document response dto.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientLogDocumentResponseDto {
    /// Gets the resulting filename.
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl ClientLogDocumentResponseDto {
    /// Client log document response dto.
    pub fn new() -> ClientLogDocumentResponseDto {
        ClientLogDocumentResponseDto {
            file_name: None,
        }
    }
}


