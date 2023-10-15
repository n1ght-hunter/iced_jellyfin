/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.11
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CollectionTypeOptions {
    #[serde(rename = "Movies")]
    Movies,
    #[serde(rename = "TvShows")]
    TvShows,
    #[serde(rename = "Music")]
    Music,
    #[serde(rename = "MusicVideos")]
    MusicVideos,
    #[serde(rename = "HomeVideos")]
    HomeVideos,
    #[serde(rename = "BoxSets")]
    BoxSets,
    #[serde(rename = "Books")]
    Books,
    #[serde(rename = "Mixed")]
    Mixed,

}

impl ToString for CollectionTypeOptions {
    fn to_string(&self) -> String {
        match self {
            Self::Movies => String::from("Movies"),
            Self::TvShows => String::from("TvShows"),
            Self::Music => String::from("Music"),
            Self::MusicVideos => String::from("MusicVideos"),
            Self::HomeVideos => String::from("HomeVideos"),
            Self::BoxSets => String::from("BoxSets"),
            Self::Books => String::from("Books"),
            Self::Mixed => String::from("Mixed"),
        }
    }
}

impl Default for CollectionTypeOptions {
    fn default() -> CollectionTypeOptions {
        Self::Movies
    }
}




