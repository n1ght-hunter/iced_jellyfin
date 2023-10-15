/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`close_live_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseLiveStreamError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bitrate_test_bytes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBitrateTestBytesError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_playback_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlaybackInfoError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_posted_playback_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPostedPlaybackInfoError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`open_live_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenLiveStreamError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}


pub async fn close_live_stream(configuration: &configuration::Configuration, live_stream_id: &str) -> Result<(), Error<CloseLiveStreamError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/LiveStreams/Close", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("liveStreamId", &live_stream_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CloseLiveStreamError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_bitrate_test_bytes(configuration: &configuration::Configuration, size: Option<i32>) -> Result<std::path::PathBuf, Error<GetBitrateTestBytesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Playback/BitrateTest", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = size {
        local_var_req_builder = local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBitrateTestBytesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_playback_info(configuration: &configuration::Configuration, item_id: &str, user_id: &str) -> Result<crate::models::PlaybackInfoResponse, Error<GetPlaybackInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Items/{itemId}/PlaybackInfo", local_var_configuration.base_path, itemId=crate::apis::urlencode(item_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("userId", &user_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPlaybackInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// For backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence.  Query parameters are obsolete.
pub async fn get_posted_playback_info(configuration: &configuration::Configuration, item_id: &str, user_id: Option<&str>, max_streaming_bitrate: Option<i32>, start_time_ticks: Option<i64>, audio_stream_index: Option<i32>, subtitle_stream_index: Option<i32>, max_audio_channels: Option<i32>, media_source_id: Option<&str>, live_stream_id: Option<&str>, auto_open_live_stream: Option<bool>, enable_direct_play: Option<bool>, enable_direct_stream: Option<bool>, enable_transcoding: Option<bool>, allow_video_stream_copy: Option<bool>, allow_audio_stream_copy: Option<bool>, unknown_base_type: Option<crate::models::UNKNOWN_BASE_TYPE>) -> Result<crate::models::PlaybackInfoResponse, Error<GetPostedPlaybackInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Items/{itemId}/PlaybackInfo", local_var_configuration.base_path, itemId=crate::apis::urlencode(item_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_streaming_bitrate {
        local_var_req_builder = local_var_req_builder.query(&[("maxStreamingBitrate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_ticks {
        local_var_req_builder = local_var_req_builder.query(&[("startTimeTicks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_stream_index {
        local_var_req_builder = local_var_req_builder.query(&[("audioStreamIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = subtitle_stream_index {
        local_var_req_builder = local_var_req_builder.query(&[("subtitleStreamIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_channels {
        local_var_req_builder = local_var_req_builder.query(&[("maxAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = media_source_id {
        local_var_req_builder = local_var_req_builder.query(&[("mediaSourceId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = live_stream_id {
        local_var_req_builder = local_var_req_builder.query(&[("liveStreamId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = auto_open_live_stream {
        local_var_req_builder = local_var_req_builder.query(&[("autoOpenLiveStream", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_direct_play {
        local_var_req_builder = local_var_req_builder.query(&[("enableDirectPlay", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_direct_stream {
        local_var_req_builder = local_var_req_builder.query(&[("enableDirectStream", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_transcoding {
        local_var_req_builder = local_var_req_builder.query(&[("enableTranscoding", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = allow_video_stream_copy {
        local_var_req_builder = local_var_req_builder.query(&[("allowVideoStreamCopy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = allow_audio_stream_copy {
        local_var_req_builder = local_var_req_builder.query(&[("allowAudioStreamCopy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&unknown_base_type);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPostedPlaybackInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn open_live_stream(configuration: &configuration::Configuration, open_token: Option<&str>, user_id: Option<&str>, play_session_id: Option<&str>, max_streaming_bitrate: Option<i32>, start_time_ticks: Option<i64>, audio_stream_index: Option<i32>, subtitle_stream_index: Option<i32>, max_audio_channels: Option<i32>, item_id: Option<&str>, enable_direct_play: Option<bool>, enable_direct_stream: Option<bool>, unknown_base_type: Option<crate::models::UNKNOWN_BASE_TYPE>) -> Result<crate::models::LiveStreamResponse, Error<OpenLiveStreamError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/LiveStreams/Open", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = open_token {
        local_var_req_builder = local_var_req_builder.query(&[("openToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = play_session_id {
        local_var_req_builder = local_var_req_builder.query(&[("playSessionId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_streaming_bitrate {
        local_var_req_builder = local_var_req_builder.query(&[("maxStreamingBitrate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time_ticks {
        local_var_req_builder = local_var_req_builder.query(&[("startTimeTicks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = audio_stream_index {
        local_var_req_builder = local_var_req_builder.query(&[("audioStreamIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = subtitle_stream_index {
        local_var_req_builder = local_var_req_builder.query(&[("subtitleStreamIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_audio_channels {
        local_var_req_builder = local_var_req_builder.query(&[("maxAudioChannels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = item_id {
        local_var_req_builder = local_var_req_builder.query(&[("itemId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_direct_play {
        local_var_req_builder = local_var_req_builder.query(&[("enableDirectPlay", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_direct_stream {
        local_var_req_builder = local_var_req_builder.query(&[("enableDirectStream", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&unknown_base_type);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenLiveStreamError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

