/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateMediaPathRequestDto : Update library options dto.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMediaPathRequestDto {
    /// Gets or sets the library name.
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PathInfo")]
    pub path_info: Box<crate::models::MediaPathInfo>,
}

impl UpdateMediaPathRequestDto {
    /// Update library options dto.
    pub fn new(name: String, path_info: crate::models::MediaPathInfo) -> UpdateMediaPathRequestDto {
        UpdateMediaPathRequestDto {
            name,
            path_info: Box::new(path_info),
        }
    }
}

