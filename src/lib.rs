use std::collections::HashMap;

use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dig {
    pub playlist_id: String,
    pub last_run: DateTime,
    pub running: bool,
    pub days_to_keep: u16,
    pub min_songs: u16,
    pub album_sort: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dug {
    pub playlist_id: String,
    pub last_run: DateTime,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub catalog_playlist_id: String,
    pub discover_weekly_playlist_id: String,
    pub last_run: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSaveTracks {
    pub last_run: DateTime,
}

#[derive(EnumString, Debug, Display, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Services {
    #[strum(serialize = "dig")]
    Dig,
    #[strum(serialize = "dug")]
    Dugs,
    #[strum(serialize = "catalog")]
    Catalogs,
    #[strum(serialize = "albumSaveTracks")]
    AlbumSaveTracks,
}

pub trait Service: Serialize + for<'de> Deserialize<'de> + Clone {}

impl Service for Dig {}
impl Service for Dug {}
impl Service for Catalog {}
impl Service for AlbumSaveTracks {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ServiceEnum {
    Dig(Dig),
    Dug(Dug),
    Catalog(Catalog),
    AlbumSaveTracks(AlbumSaveTracks),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub service_list: Vec<Services>,
    pub services: HashMap<Services, ServiceEnum>,
}

impl User {
    pub fn get_service<T: Service>(&self, service: Services) -> Option<T>
    where
        T: From<Dig> + From<Dug> + From<Catalog> + From<AlbumSaveTracks>,
    {
        self.services.get(&service).map(|s| match s {
            ServiceEnum::Dig(d) => d.clone().into(),
            ServiceEnum::Dug(d) => d.clone().into(),
            ServiceEnum::Catalog(d) => d.clone().into(),
            ServiceEnum::AlbumSaveTracks(d) => d.clone().into(),
        })
    }
}
