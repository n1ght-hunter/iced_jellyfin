/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticateUserByName : The authenticate user by name request body.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateUserByName {
    /// Gets or sets the username.
    #[serde(rename = "Username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
    /// Gets or sets the plain text password.
    #[serde(rename = "Pw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pw: Option<Option<String>>,
    /// Gets or sets the sha1-hashed password.
    #[serde(rename = "Password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
}

impl AuthenticateUserByName {
    /// The authenticate user by name request body.
    pub fn new() -> AuthenticateUserByName {
        AuthenticateUserByName {
            username: None,
            pw: None,
            password: None,
        }
    }
}


