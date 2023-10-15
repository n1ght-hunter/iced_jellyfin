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
pub struct UserPolicy {
    /// Gets or sets a value indicating whether this instance is administrator.
    #[serde(rename = "IsAdministrator", skip_serializing_if = "Option::is_none")]
    pub is_administrator: Option<bool>,
    /// Gets or sets a value indicating whether this instance is hidden.
    #[serde(rename = "IsHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    /// Gets or sets a value indicating whether this instance is disabled.
    #[serde(rename = "IsDisabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// Gets or sets the max parental rating.
    #[serde(rename = "MaxParentalRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_parental_rating: Option<Option<i32>>,
    #[serde(rename = "BlockedTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blocked_tags: Option<Option<Vec<String>>>,
    #[serde(rename = "EnableUserPreferenceAccess", skip_serializing_if = "Option::is_none")]
    pub enable_user_preference_access: Option<bool>,
    #[serde(rename = "AccessSchedules", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_schedules: Option<Option<Vec<crate::models::AccessSchedule>>>,
    #[serde(rename = "BlockUnratedItems", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub block_unrated_items: Option<Option<Vec<crate::models::UnratedItem>>>,
    #[serde(rename = "EnableRemoteControlOfOtherUsers", skip_serializing_if = "Option::is_none")]
    pub enable_remote_control_of_other_users: Option<bool>,
    #[serde(rename = "EnableSharedDeviceControl", skip_serializing_if = "Option::is_none")]
    pub enable_shared_device_control: Option<bool>,
    #[serde(rename = "EnableRemoteAccess", skip_serializing_if = "Option::is_none")]
    pub enable_remote_access: Option<bool>,
    #[serde(rename = "EnableLiveTvManagement", skip_serializing_if = "Option::is_none")]
    pub enable_live_tv_management: Option<bool>,
    #[serde(rename = "EnableLiveTvAccess", skip_serializing_if = "Option::is_none")]
    pub enable_live_tv_access: Option<bool>,
    #[serde(rename = "EnableMediaPlayback", skip_serializing_if = "Option::is_none")]
    pub enable_media_playback: Option<bool>,
    #[serde(rename = "EnableAudioPlaybackTranscoding", skip_serializing_if = "Option::is_none")]
    pub enable_audio_playback_transcoding: Option<bool>,
    #[serde(rename = "EnableVideoPlaybackTranscoding", skip_serializing_if = "Option::is_none")]
    pub enable_video_playback_transcoding: Option<bool>,
    #[serde(rename = "EnablePlaybackRemuxing", skip_serializing_if = "Option::is_none")]
    pub enable_playback_remuxing: Option<bool>,
    #[serde(rename = "ForceRemoteSourceTranscoding", skip_serializing_if = "Option::is_none")]
    pub force_remote_source_transcoding: Option<bool>,
    #[serde(rename = "EnableContentDeletion", skip_serializing_if = "Option::is_none")]
    pub enable_content_deletion: Option<bool>,
    #[serde(rename = "EnableContentDeletionFromFolders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_content_deletion_from_folders: Option<Option<Vec<String>>>,
    #[serde(rename = "EnableContentDownloading", skip_serializing_if = "Option::is_none")]
    pub enable_content_downloading: Option<bool>,
    /// Gets or sets a value indicating whether [enable synchronize].
    #[serde(rename = "EnableSyncTranscoding", skip_serializing_if = "Option::is_none")]
    pub enable_sync_transcoding: Option<bool>,
    #[serde(rename = "EnableMediaConversion", skip_serializing_if = "Option::is_none")]
    pub enable_media_conversion: Option<bool>,
    #[serde(rename = "EnabledDevices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled_devices: Option<Option<Vec<String>>>,
    #[serde(rename = "EnableAllDevices", skip_serializing_if = "Option::is_none")]
    pub enable_all_devices: Option<bool>,
    #[serde(rename = "EnabledChannels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled_channels: Option<Option<Vec<uuid::Uuid>>>,
    #[serde(rename = "EnableAllChannels", skip_serializing_if = "Option::is_none")]
    pub enable_all_channels: Option<bool>,
    #[serde(rename = "EnabledFolders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enabled_folders: Option<Option<Vec<uuid::Uuid>>>,
    #[serde(rename = "EnableAllFolders", skip_serializing_if = "Option::is_none")]
    pub enable_all_folders: Option<bool>,
    #[serde(rename = "InvalidLoginAttemptCount", skip_serializing_if = "Option::is_none")]
    pub invalid_login_attempt_count: Option<i32>,
    #[serde(rename = "LoginAttemptsBeforeLockout", skip_serializing_if = "Option::is_none")]
    pub login_attempts_before_lockout: Option<i32>,
    #[serde(rename = "MaxActiveSessions", skip_serializing_if = "Option::is_none")]
    pub max_active_sessions: Option<i32>,
    #[serde(rename = "EnablePublicSharing", skip_serializing_if = "Option::is_none")]
    pub enable_public_sharing: Option<bool>,
    #[serde(rename = "BlockedMediaFolders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blocked_media_folders: Option<Option<Vec<uuid::Uuid>>>,
    #[serde(rename = "BlockedChannels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blocked_channels: Option<Option<Vec<uuid::Uuid>>>,
    #[serde(rename = "RemoteClientBitrateLimit", skip_serializing_if = "Option::is_none")]
    pub remote_client_bitrate_limit: Option<i32>,
    #[serde(rename = "AuthenticationProviderId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authentication_provider_id: Option<Option<String>>,
    #[serde(rename = "PasswordResetProviderId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password_reset_provider_id: Option<Option<String>>,
    #[serde(rename = "SyncPlayAccess", skip_serializing_if = "Option::is_none")]
    pub sync_play_access: Option<crate::models::SyncPlayUserAccessType>,
}

impl UserPolicy {
    pub fn new() -> UserPolicy {
        UserPolicy {
            is_administrator: None,
            is_hidden: None,
            is_disabled: None,
            max_parental_rating: None,
            blocked_tags: None,
            enable_user_preference_access: None,
            access_schedules: None,
            block_unrated_items: None,
            enable_remote_control_of_other_users: None,
            enable_shared_device_control: None,
            enable_remote_access: None,
            enable_live_tv_management: None,
            enable_live_tv_access: None,
            enable_media_playback: None,
            enable_audio_playback_transcoding: None,
            enable_video_playback_transcoding: None,
            enable_playback_remuxing: None,
            force_remote_source_transcoding: None,
            enable_content_deletion: None,
            enable_content_deletion_from_folders: None,
            enable_content_downloading: None,
            enable_sync_transcoding: None,
            enable_media_conversion: None,
            enabled_devices: None,
            enable_all_devices: None,
            enabled_channels: None,
            enable_all_channels: None,
            enabled_folders: None,
            enable_all_folders: None,
            invalid_login_attempt_count: None,
            login_attempts_before_lockout: None,
            max_active_sessions: None,
            enable_public_sharing: None,
            blocked_media_folders: None,
            blocked_channels: None,
            remote_client_bitrate_limit: None,
            authentication_provider_id: None,
            password_reset_provider_id: None,
            sync_play_access: None,
        }
    }
}


