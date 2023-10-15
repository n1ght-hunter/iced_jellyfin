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
pub struct PinRedeemResult {
    /// Gets or sets a value indicating whether this MediaBrowser.Model.Users.PinRedeemResult is success.
    #[serde(rename = "Success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// Gets or sets the users reset.
    #[serde(rename = "UsersReset", skip_serializing_if = "Option::is_none")]
    pub users_reset: Option<Vec<String>>,
}

impl PinRedeemResult {
    pub fn new() -> PinRedeemResult {
        PinRedeemResult {
            success: None,
            users_reset: None,
        }
    }
}


