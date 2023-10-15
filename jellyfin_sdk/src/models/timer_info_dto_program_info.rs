/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TimerInfoDtoProgramInfo : Gets or sets the program information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimerInfoDtoProgramInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "OriginalTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<Option<String>>,
    /// Gets or sets the server identifier.
    #[serde(rename = "ServerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Option<String>>,
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Gets or sets the etag.
    #[serde(rename = "Etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// Gets or sets the type of the source.
    #[serde(rename = "SourceType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<Option<String>>,
    /// Gets or sets the playlist item identifier.
    #[serde(rename = "PlaylistItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<Option<String>>,
    /// Gets or sets the date created.
    #[serde(rename = "DateCreated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    #[serde(rename = "DateLastMediaAdded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_last_media_added: Option<Option<String>>,
    #[serde(rename = "ExtraType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extra_type: Option<Option<String>>,
    #[serde(rename = "AirsBeforeSeasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub airs_before_season_number: Option<Option<i32>>,
    #[serde(rename = "AirsAfterSeasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub airs_after_season_number: Option<Option<i32>>,
    #[serde(rename = "AirsBeforeEpisodeNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub airs_before_episode_number: Option<Option<i32>>,
    #[serde(rename = "CanDelete", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<Option<bool>>,
    #[serde(rename = "CanDownload", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub can_download: Option<Option<bool>>,
    #[serde(rename = "HasSubtitles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_subtitles: Option<Option<bool>>,
    #[serde(rename = "PreferredMetadataLanguage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preferred_metadata_language: Option<Option<String>>,
    #[serde(rename = "PreferredMetadataCountryCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preferred_metadata_country_code: Option<Option<String>>,
    /// Gets or sets a value indicating whether [supports synchronize].
    #[serde(rename = "SupportsSync", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<Option<bool>>,
    #[serde(rename = "Container", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub container: Option<Option<String>>,
    /// Gets or sets the name of the sort.
    #[serde(rename = "SortName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<Option<String>>,
    #[serde(rename = "ForcedSortName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forced_sort_name: Option<Option<String>>,
    #[serde(rename = "Video3DFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video3_d_format: Option<Option<crate::models::Video3DFormat>>,
    /// Gets or sets the premiere date.
    #[serde(rename = "PremiereDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<Option<String>>,
    /// Gets or sets the external urls.
    #[serde(rename = "ExternalUrls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Option<Vec<crate::models::ExternalUrl>>>,
    /// Gets or sets the media versions.
    #[serde(rename = "MediaSources", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media_sources: Option<Option<Vec<crate::models::MediaSourceInfo>>>,
    /// Gets or sets the critic rating.
    #[serde(rename = "CriticRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub critic_rating: Option<Option<f32>>,
    #[serde(rename = "ProductionLocations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub production_locations: Option<Option<Vec<String>>>,
    /// Gets or sets the path.
    #[serde(rename = "Path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "EnableMediaSourceDisplay", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_media_source_display: Option<Option<bool>>,
    /// Gets or sets the official rating.
    #[serde(rename = "OfficialRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub official_rating: Option<Option<String>>,
    /// Gets or sets the custom rating.
    #[serde(rename = "CustomRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_rating: Option<Option<String>>,
    /// Gets or sets the channel identifier.
    #[serde(rename = "ChannelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "ChannelName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<Option<String>>,
    /// Gets or sets the overview.
    #[serde(rename = "Overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    /// Gets or sets the taglines.
    #[serde(rename = "Taglines", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub taglines: Option<Option<Vec<String>>>,
    /// Gets or sets the genres.
    #[serde(rename = "Genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    /// Gets or sets the community rating.
    #[serde(rename = "CommunityRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<Option<f32>>,
    /// Gets or sets the cumulative run time ticks.
    #[serde(rename = "CumulativeRunTimeTicks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cumulative_run_time_ticks: Option<Option<i64>>,
    /// Gets or sets the run time ticks.
    #[serde(rename = "RunTimeTicks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<Option<i64>>,
    #[serde(rename = "PlayAccess", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub play_access: Option<Option<crate::models::PlayAccess>>,
    /// Gets or sets the aspect ratio.
    #[serde(rename = "AspectRatio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<Option<String>>,
    /// Gets or sets the production year.
    #[serde(rename = "ProductionYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub production_year: Option<Option<i32>>,
    /// Gets or sets a value indicating whether this instance is place holder.
    #[serde(rename = "IsPlaceHolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_place_holder: Option<Option<bool>>,
    /// Gets or sets the number.
    #[serde(rename = "Number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number: Option<Option<String>>,
    #[serde(rename = "ChannelNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_number: Option<Option<String>>,
    /// Gets or sets the index number.
    #[serde(rename = "IndexNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<Option<i32>>,
    /// Gets or sets the index number end.
    #[serde(rename = "IndexNumberEnd", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<Option<i32>>,
    /// Gets or sets the parent index number.
    #[serde(rename = "ParentIndexNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<Option<i32>>,
    /// Gets or sets the trailer urls.
    #[serde(rename = "RemoteTrailers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_trailers: Option<Option<Vec<crate::models::MediaUrl>>>,
    /// Gets or sets the provider ids.
    #[serde(rename = "ProviderIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<Option<::std::collections::HashMap<String, String>>>,
    /// Gets or sets a value indicating whether this instance is HD.
    #[serde(rename = "IsHD", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_hd: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is folder.
    #[serde(rename = "IsFolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<Option<bool>>,
    /// Gets or sets the parent id.
    #[serde(rename = "ParentId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::BaseItemKind>,
    /// Gets or sets the people.
    #[serde(rename = "People", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub people: Option<Option<Vec<crate::models::BaseItemPerson>>>,
    /// Gets or sets the studios.
    #[serde(rename = "Studios", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub studios: Option<Option<Vec<crate::models::NameGuidPair>>>,
    #[serde(rename = "GenreItems", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genre_items: Option<Option<Vec<crate::models::NameGuidPair>>>,
    /// Gets or sets wether the item has a logo, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentLogoItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_logo_item_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets wether the item has any backdrops, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentBackdropItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_backdrop_item_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the parent backdrop image tags.
    #[serde(rename = "ParentBackdropImageTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_backdrop_image_tags: Option<Option<Vec<String>>>,
    /// Gets or sets the local trailer count.
    #[serde(rename = "LocalTrailerCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_trailer_count: Option<Option<i32>>,
    #[serde(rename = "UserData", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Option<Box<crate::models::BaseItemDtoUserData>>>,
    /// Gets or sets the recursive item count.
    #[serde(rename = "RecursiveItemCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recursive_item_count: Option<Option<i32>>,
    /// Gets or sets the child count.
    #[serde(rename = "ChildCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub child_count: Option<Option<i32>>,
    /// Gets or sets the name of the series.
    #[serde(rename = "SeriesName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_name: Option<Option<String>>,
    /// Gets or sets the series id.
    #[serde(rename = "SeriesId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the season identifier.
    #[serde(rename = "SeasonId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the special feature count.
    #[serde(rename = "SpecialFeatureCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub special_feature_count: Option<Option<i32>>,
    /// Gets or sets the display preferences id.
    #[serde(rename = "DisplayPreferencesId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_preferences_id: Option<Option<String>>,
    /// Gets or sets the status.
    #[serde(rename = "Status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    /// Gets or sets the air time.
    #[serde(rename = "AirTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub air_time: Option<Option<String>>,
    /// Gets or sets the air days.
    #[serde(rename = "AirDays", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub air_days: Option<Option<Vec<crate::models::DayOfWeek>>>,
    /// Gets or sets the tags.
    #[serde(rename = "Tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
    /// Gets or sets the primary image aspect ratio, after image enhancements.
    #[serde(rename = "PrimaryImageAspectRatio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_image_aspect_ratio: Option<Option<f64>>,
    /// Gets or sets the artists.
    #[serde(rename = "Artists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Option<Vec<String>>>,
    /// Gets or sets the artist items.
    #[serde(rename = "ArtistItems", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub artist_items: Option<Option<Vec<crate::models::NameGuidPair>>>,
    /// Gets or sets the album.
    #[serde(rename = "Album", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album: Option<Option<String>>,
    /// Gets or sets the type of the collection.
    #[serde(rename = "CollectionType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<Option<String>>,
    /// Gets or sets the display order.
    #[serde(rename = "DisplayOrder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_order: Option<Option<String>>,
    /// Gets or sets the album id.
    #[serde(rename = "AlbumId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the album image tag.
    #[serde(rename = "AlbumPrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_primary_image_tag: Option<Option<String>>,
    /// Gets or sets the series primary image tag.
    #[serde(rename = "SeriesPrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_primary_image_tag: Option<Option<String>>,
    /// Gets or sets the album artist.
    #[serde(rename = "AlbumArtist", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<Option<String>>,
    /// Gets or sets the album artists.
    #[serde(rename = "AlbumArtists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_artists: Option<Option<Vec<crate::models::NameGuidPair>>>,
    /// Gets or sets the name of the season.
    #[serde(rename = "SeasonName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_name: Option<Option<String>>,
    /// Gets or sets the media streams.
    #[serde(rename = "MediaStreams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Option<Vec<crate::models::MediaStream>>>,
    #[serde(rename = "VideoType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<Option<crate::models::VideoType>>,
    /// Gets or sets the part count.
    #[serde(rename = "PartCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub part_count: Option<Option<i32>>,
    #[serde(rename = "MediaSourceCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media_source_count: Option<Option<i32>>,
    /// Gets or sets the image tags.
    #[serde(rename = "ImageTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Option<::std::collections::HashMap<String, String>>>,
    /// Gets or sets the backdrop image tags.
    #[serde(rename = "BackdropImageTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub backdrop_image_tags: Option<Option<Vec<String>>>,
    /// Gets or sets the screenshot image tags.
    #[serde(rename = "ScreenshotImageTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub screenshot_image_tags: Option<Option<Vec<String>>>,
    /// Gets or sets the parent logo image tag.
    #[serde(rename = "ParentLogoImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_logo_image_tag: Option<Option<String>>,
    /// Gets or sets wether the item has fan art, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentArtItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_art_item_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the parent art image tag.
    #[serde(rename = "ParentArtImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_art_image_tag: Option<Option<String>>,
    /// Gets or sets the series thumb image tag.
    #[serde(rename = "SeriesThumbImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_thumb_image_tag: Option<Option<String>>,
    #[serde(rename = "ImageBlurHashes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Option<Box<crate::models::BaseItemDtoImageBlurHashes>>>,
    /// Gets or sets the series studio.
    #[serde(rename = "SeriesStudio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_studio: Option<Option<String>>,
    /// Gets or sets the parent thumb item id.
    #[serde(rename = "ParentThumbItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_thumb_item_id: Option<Option<uuid::Uuid>>,
    /// Gets or sets the parent thumb image tag.
    #[serde(rename = "ParentThumbImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_thumb_image_tag: Option<Option<String>>,
    /// Gets or sets the parent primary image item identifier.
    #[serde(rename = "ParentPrimaryImageItemId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_primary_image_item_id: Option<Option<String>>,
    /// Gets or sets the parent primary image tag.
    #[serde(rename = "ParentPrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_primary_image_tag: Option<Option<String>>,
    /// Gets or sets the chapters.
    #[serde(rename = "Chapters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Option<Vec<crate::models::ChapterInfo>>>,
    #[serde(rename = "LocationType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<Option<crate::models::LocationType>>,
    #[serde(rename = "IsoType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_type: Option<Option<crate::models::IsoType>>,
    /// Gets or sets the type of the media.
    #[serde(rename = "MediaType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Option<String>>,
    /// Gets or sets the end date.
    #[serde(rename = "EndDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Gets or sets the locked fields.
    #[serde(rename = "LockedFields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locked_fields: Option<Option<Vec<crate::models::MetadataField>>>,
    /// Gets or sets the trailer count.
    #[serde(rename = "TrailerCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trailer_count: Option<Option<i32>>,
    /// Gets or sets the movie count.
    #[serde(rename = "MovieCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_count: Option<Option<i32>>,
    /// Gets or sets the series count.
    #[serde(rename = "SeriesCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_count: Option<Option<i32>>,
    #[serde(rename = "ProgramCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub program_count: Option<Option<i32>>,
    /// Gets or sets the episode count.
    #[serde(rename = "EpisodeCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<Option<i32>>,
    /// Gets or sets the song count.
    #[serde(rename = "SongCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub song_count: Option<Option<i32>>,
    /// Gets or sets the album count.
    #[serde(rename = "AlbumCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_count: Option<Option<i32>>,
    #[serde(rename = "ArtistCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub artist_count: Option<Option<i32>>,
    /// Gets or sets the music video count.
    #[serde(rename = "MusicVideoCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub music_video_count: Option<Option<i32>>,
    /// Gets or sets a value indicating whether [enable internet providers].
    #[serde(rename = "LockData", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lock_data: Option<Option<bool>>,
    #[serde(rename = "Width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<i32>>,
    #[serde(rename = "Height", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<i32>>,
    #[serde(rename = "CameraMake", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<Option<String>>,
    #[serde(rename = "CameraModel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<Option<String>>,
    #[serde(rename = "Software", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub software: Option<Option<String>>,
    #[serde(rename = "ExposureTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<Option<f64>>,
    #[serde(rename = "FocalLength", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<Option<f64>>,
    #[serde(rename = "ImageOrientation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_orientation: Option<Option<crate::models::ImageOrientation>>,
    #[serde(rename = "Aperture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub aperture: Option<Option<f64>>,
    #[serde(rename = "ShutterSpeed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shutter_speed: Option<Option<f64>>,
    #[serde(rename = "Latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f64>>,
    #[serde(rename = "Longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f64>>,
    #[serde(rename = "Altitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<Option<f64>>,
    #[serde(rename = "IsoSpeedRating", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iso_speed_rating: Option<Option<i32>>,
    /// Gets or sets the series timer identifier.
    #[serde(rename = "SeriesTimerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_timer_id: Option<Option<String>>,
    /// Gets or sets the program identifier.
    #[serde(rename = "ProgramId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<Option<String>>,
    /// Gets or sets the channel primary image tag.
    #[serde(rename = "ChannelPrimaryImageTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_primary_image_tag: Option<Option<String>>,
    /// Gets or sets the start date of the recording, in UTC.
    #[serde(rename = "StartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Gets or sets the completion percentage.
    #[serde(rename = "CompletionPercentage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<Option<f64>>,
    /// Gets or sets a value indicating whether this instance is repeat.
    #[serde(rename = "IsRepeat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<Option<bool>>,
    /// Gets or sets the episode title.
    #[serde(rename = "EpisodeTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_title: Option<Option<String>>,
    #[serde(rename = "ChannelType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<Option<crate::models::ChannelType>>,
    #[serde(rename = "Audio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio: Option<Option<crate::models::ProgramAudio>>,
    /// Gets or sets a value indicating whether this instance is movie.
    #[serde(rename = "IsMovie", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is sports.
    #[serde(rename = "IsSports", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_sports: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is series.
    #[serde(rename = "IsSeries", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_series: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is live.
    #[serde(rename = "IsLive", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_live: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is news.
    #[serde(rename = "IsNews", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_news: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is kids.
    #[serde(rename = "IsKids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_kids: Option<Option<bool>>,
    /// Gets or sets a value indicating whether this instance is premiere.
    #[serde(rename = "IsPremiere", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_premiere: Option<Option<bool>>,
    /// Gets or sets the timer identifier.
    #[serde(rename = "TimerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<Option<String>>,
    #[serde(rename = "CurrentProgram", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_program: Option<Option<Box<crate::models::BaseItemDtoCurrentProgram>>>,
}

impl TimerInfoDtoProgramInfo {
    /// Gets or sets the program information.
    pub fn new() -> TimerInfoDtoProgramInfo {
        TimerInfoDtoProgramInfo {
            name: None,
            original_title: None,
            server_id: None,
            id: None,
            etag: None,
            source_type: None,
            playlist_item_id: None,
            date_created: None,
            date_last_media_added: None,
            extra_type: None,
            airs_before_season_number: None,
            airs_after_season_number: None,
            airs_before_episode_number: None,
            can_delete: None,
            can_download: None,
            has_subtitles: None,
            preferred_metadata_language: None,
            preferred_metadata_country_code: None,
            supports_sync: None,
            container: None,
            sort_name: None,
            forced_sort_name: None,
            video3_d_format: None,
            premiere_date: None,
            external_urls: None,
            media_sources: None,
            critic_rating: None,
            production_locations: None,
            path: None,
            enable_media_source_display: None,
            official_rating: None,
            custom_rating: None,
            channel_id: None,
            channel_name: None,
            overview: None,
            taglines: None,
            genres: None,
            community_rating: None,
            cumulative_run_time_ticks: None,
            run_time_ticks: None,
            play_access: None,
            aspect_ratio: None,
            production_year: None,
            is_place_holder: None,
            number: None,
            channel_number: None,
            index_number: None,
            index_number_end: None,
            parent_index_number: None,
            remote_trailers: None,
            provider_ids: None,
            is_hd: None,
            is_folder: None,
            parent_id: None,
            r#type: None,
            people: None,
            studios: None,
            genre_items: None,
            parent_logo_item_id: None,
            parent_backdrop_item_id: None,
            parent_backdrop_image_tags: None,
            local_trailer_count: None,
            user_data: None,
            recursive_item_count: None,
            child_count: None,
            series_name: None,
            series_id: None,
            season_id: None,
            special_feature_count: None,
            display_preferences_id: None,
            status: None,
            air_time: None,
            air_days: None,
            tags: None,
            primary_image_aspect_ratio: None,
            artists: None,
            artist_items: None,
            album: None,
            collection_type: None,
            display_order: None,
            album_id: None,
            album_primary_image_tag: None,
            series_primary_image_tag: None,
            album_artist: None,
            album_artists: None,
            season_name: None,
            media_streams: None,
            video_type: None,
            part_count: None,
            media_source_count: None,
            image_tags: None,
            backdrop_image_tags: None,
            screenshot_image_tags: None,
            parent_logo_image_tag: None,
            parent_art_item_id: None,
            parent_art_image_tag: None,
            series_thumb_image_tag: None,
            image_blur_hashes: None,
            series_studio: None,
            parent_thumb_item_id: None,
            parent_thumb_image_tag: None,
            parent_primary_image_item_id: None,
            parent_primary_image_tag: None,
            chapters: None,
            location_type: None,
            iso_type: None,
            media_type: None,
            end_date: None,
            locked_fields: None,
            trailer_count: None,
            movie_count: None,
            series_count: None,
            program_count: None,
            episode_count: None,
            song_count: None,
            album_count: None,
            artist_count: None,
            music_video_count: None,
            lock_data: None,
            width: None,
            height: None,
            camera_make: None,
            camera_model: None,
            software: None,
            exposure_time: None,
            focal_length: None,
            image_orientation: None,
            aperture: None,
            shutter_speed: None,
            latitude: None,
            longitude: None,
            altitude: None,
            iso_speed_rating: None,
            series_timer_id: None,
            program_id: None,
            channel_primary_image_tag: None,
            start_date: None,
            completion_percentage: None,
            is_repeat: None,
            episode_title: None,
            channel_type: None,
            audio: None,
            is_movie: None,
            is_sports: None,
            is_series: None,
            is_live: None,
            is_news: None,
            is_kids: None,
            is_premiere: None,
            timer_id: None,
            current_program: None,
        }
    }
}


