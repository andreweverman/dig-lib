use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dig {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub playlist_id: String,
    pub last_run: DateTime,
    pub running: bool,
    pub days_to_keep: u16,
    pub min_songs: u16,
    pub album_sort: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dug {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub playlist_id: String,
    pub last_run: DateTime,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub catalog_playlist_id: String,
    pub discover_weekly_playlist_id: String,
    pub last_run: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSaveTracks {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub last_run: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub display_name: String,
    pub username: String,
    pub photo: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
