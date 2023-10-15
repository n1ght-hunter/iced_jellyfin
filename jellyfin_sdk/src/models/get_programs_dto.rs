/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetProgramsDto : Get programs dto.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProgramsDto {
    /// Gets or sets the channels to return guide information for.
    #[serde(rename = "ChannelIds", skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<uuid::Uuid>>,
    /// Gets or sets optional. Filter by user id.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    /// Gets or sets the minimum premiere start date.  Optional.
    #[serde(rename = "MinStartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_start_date: Option<Option<String>>,
    /// Gets or sets filter by programs that have completed airing, or not.  Optional.
    #[serde(rename = "HasAired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_aired: Option<Option<bool>>,
    /// Gets or sets filter by programs that are currently airing, or not.  Optional.
    #[serde(rename = "IsAiring", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_airing: Option<Option<bool>>,
    /// Gets or sets the maximum premiere start date.  Optional.
    #[serde(rename = "MaxStartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_start_date: Option<Option<String>>,
    /// Gets or sets the minimum premiere end date.  Optional.
    #[serde(rename = "MinEndDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_end_date: Option<Option<String>>,
    /// Gets or sets the maximum premiere end date.  Optional.
    #[serde(rename = "MaxEndDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_end_date: Option<Option<String>>,
    /// Gets or sets filter for movies.  Optional.
    #[serde(rename = "IsMovie", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<Option<bool>>,
    /// Gets or sets filter for series.  Optional.
    #[serde(rename = "IsSeries", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_series: Option<Option<bool>>,
    /// Gets or sets filter for news.  Optional.
    #[serde(rename = "IsNews", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_news: Option<Option<bool>>,
    /// Gets or sets filter for kids.  Optional.
    #[serde(rename = "IsKids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_kids: Option<Option<bool>>,
    /// Gets or sets filter for sports.  Optional.
    #[serde(rename = "IsSports", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_sports: Option<Option<bool>>,
    /// Gets or sets the record index to start at. All items with a lower index will be dropped from the results.  Optional.
    #[serde(rename = "StartIndex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<Option<i32>>,
    /// Gets or sets the maximum number of records to return.  Optional.
    #[serde(rename = "Limit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limit: Option<Option<i32>>,
    /// Gets or sets specify one or more sort orders, comma delimited. Options: Name, StartDate.  Optional.
    #[serde(rename = "SortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<String>>,
    /// Gets or sets sort Order - Ascending,Descending.
    #[serde(rename = "SortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<Vec<crate::models::SortOrder>>,
    /// Gets or sets the genres to return guide information for.
    #[serde(rename = "Genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// Gets or sets the genre ids to return guide information for.
    #[serde(rename = "GenreIds", skip_serializing_if = "Option::is_none")]
    pub genre_ids: Option<Vec<uuid::Uuid>>,
    /// Gets or sets include image information in output.  Optional.
    #[serde(rename = "EnableImages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_images: Option<Option<bool>>,
    /// Gets or sets a value indicating whether retrieve total record count.
    #[serde(rename = "EnableTotalRecordCount", skip_serializing_if = "Option::is_none")]
    pub enable_total_record_count: Option<bool>,
    /// Gets or sets the max number of images to return, per image type.  Optional.
    #[serde(rename = "ImageTypeLimit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_type_limit: Option<Option<i32>>,
    /// Gets or sets the image types to include in the output.  Optional.
    #[serde(rename = "EnableImageTypes", skip_serializing_if = "Option::is_none")]
    pub enable_image_types: Option<Vec<crate::models::ImageType>>,
    /// Gets or sets include user data.  Optional.
    #[serde(rename = "EnableUserData", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub enable_user_data: Option<Option<bool>>,
    /// Gets or sets filter by series timer id.  Optional.
    #[serde(rename = "SeriesTimerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_timer_id: Option<Option<String>>,
    /// Gets or sets filter by library series id.  Optional.
    #[serde(rename = "LibrarySeriesId", skip_serializing_if = "Option::is_none")]
    pub library_series_id: Option<uuid::Uuid>,
    /// Gets or sets specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.  Optional.
    #[serde(rename = "Fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::ItemFields>>,
}

impl GetProgramsDto {
    /// Get programs dto.
    pub fn new() -> GetProgramsDto {
        GetProgramsDto {
            channel_ids: None,
            user_id: None,
            min_start_date: None,
            has_aired: None,
            is_airing: None,
            max_start_date: None,
            min_end_date: None,
            max_end_date: None,
            is_movie: None,
            is_series: None,
            is_news: None,
            is_kids: None,
            is_sports: None,
            start_index: None,
            limit: None,
            sort_by: None,
            sort_order: None,
            genres: None,
            genre_ids: None,
            enable_images: None,
            enable_total_record_count: None,
            image_type_limit: None,
            enable_image_types: None,
            enable_user_data: None,
            series_timer_id: None,
            library_series_id: None,
            fields: None,
        }
    }
}

