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
pub struct TimerInfoDto {
    /// Gets or sets the Id of the recording.
    #[serde(rename = "Id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "Type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    /// Gets or sets the server identifier.
    #[serde(rename = "ServerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Option<String>>,
    /// Gets or sets the external identifier.
    #[serde(rename = "ExternalId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Option<String>>,
    /// Gets or sets the channel id of the recording.
    #[serde(rename = "ChannelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<uuid::Uuid>,
    /// Gets or sets the external channel identifier.
    #[serde(rename = "ExternalChannelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_channel_id: Option<Option<String>>,
    /// Gets or sets the channel name of the recording.
    #[serde(rename = "ChannelName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<Option<String>>,
    #[serde(rename = "ChannelPrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_primary_image_tag: Option<Option<String>>,
    /// Gets or sets the program identifier.
    #[serde(rename = "ProgramId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Option<String>>,
    /// Gets or sets the external program identifier.
    #[serde(rename = "ExternalProgramId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_program_id: Option<Option<String>>,
    /// Gets or sets the name of the recording.
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Gets or sets the description of the recording.
    #[serde(rename = "Overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    /// Gets or sets the start date of the recording, in UTC.
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Gets or sets the end date of the recording, in UTC.
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Gets or sets the name of the service.
    #[serde(rename = "ServiceName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<Option<String>>,
    /// Gets or sets the priority.
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Gets or sets the pre padding seconds.
    #[serde(rename = "PrePaddingSeconds", skip_serializing_if = "Option::is_none")]
    pub pre_padding_seconds: Option<i32>,
    /// Gets or sets the post padding seconds.
    #[serde(rename = "PostPaddingSeconds", skip_serializing_if = "Option::is_none")]
    pub post_padding_seconds: Option<i32>,
    /// Gets or sets a value indicating whether this instance is pre padding required.
    #[serde(rename = "IsPrePaddingRequired", skip_serializing_if = "Option::is_none")]
    pub is_pre_padding_required: Option<bool>,
    /// Gets or sets the Id of the Parent that has a backdrop if the item does not have one.
    #[serde(rename = "ParentBackdropItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_backdrop_item_id: Option<Option<String>>,
    /// Gets or sets the parent backdrop image tags.
    #[serde(rename = "ParentBackdropImageTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_backdrop_image_tags: Option<Option<Vec<String>>>,
    /// Gets or sets a value indicating whether this instance is post padding required.
    #[serde(rename = "IsPostPaddingRequired", skip_serializing_if = "Option::is_none")]
    pub is_post_padding_required: Option<bool>,
    #[serde(rename = "KeepUntil", skip_serializing_if = "Option::is_none")]
    pub keep_until: Option<crate::models::KeepUntil>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RecordingStatus>,
    /// Gets or sets the series timer identifier.
    #[serde(rename = "SeriesTimerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_timer_id: Option<Option<String>>,
    /// Gets or sets the external series timer identifier.
    #[serde(rename = "ExternalSeriesTimerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_series_timer_id: Option<Option<String>>,
    /// Gets or sets the run time ticks.
    #[serde(rename = "RunTimeTicks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<Option<i64>>,
    #[serde(rename = "ProgramInfo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub program_info: Option<Option<Box<crate::models::TimerInfoDtoProgramInfo>>>,
}

impl TimerInfoDto {
    pub fn new() -> TimerInfoDto {
        TimerInfoDto {
            id: None,
            r#type: None,
            server_id: None,
            external_id: None,
            channel_id: None,
            external_channel_id: None,
            channel_name: None,
            channel_primary_image_tag: None,
            program_id: None,
            external_program_id: None,
            name: None,
            overview: None,
            start_date: None,
            end_date: None,
            service_name: None,
            priority: None,
            pre_padding_seconds: None,
            post_padding_seconds: None,
            is_pre_padding_required: None,
            parent_backdrop_item_id: None,
            parent_backdrop_image_tags: None,
            is_post_padding_required: None,
            keep_until: None,
            status: None,
            series_timer_id: None,
            external_series_timer_id: None,
            run_time_ticks: None,
            program_info: None,
        }
    }
}

