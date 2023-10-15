/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WakeOnLanInfo : Provides the MAC address and port for wake-on-LAN functionality.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WakeOnLanInfo {
    /// Gets the MAC address of the device.
    #[serde(rename = "MacAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Option<String>>,
    /// Gets or sets the wake-on-LAN port.
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

impl WakeOnLanInfo {
    /// Provides the MAC address and port for wake-on-LAN functionality.
    pub fn new() -> WakeOnLanInfo {
        WakeOnLanInfo {
            mac_address: None,
            port: None,
        }
    }
}


