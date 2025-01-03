use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dig {
    pub id: String, 
    pub playlist_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_run: DateTime<Utc>,
    pub running: bool,
    pub days_to_keep: u16,
    pub min_songs: u16,
    pub album_sort: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dug {
    pub id: String, 
    pub playlist_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_run: DateTime<Utc>,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub id: String, 
    pub catalog_playlist_id: String,
    pub discover_weekly_playlist_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_run: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSaveTracks {
    pub id: String, 
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_run: DateTime<Utc>,
}

#[derive(EnumString, Debug, Display, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Services {
    #[strum(serialize = "dig")]
    Dig,
    #[strum(serialize = "dug")]
    Dug,
    #[strum(serialize = "catalog")]
    Catalog,
    #[strum(serialize = "albumSaveTracks")]
    AlbumSaveTracks,
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
    pub services: Option<Vec<serde_json::Value>>,
}

impl User {
    pub fn get_service<T>(&self, service: Services) -> Result<T, Box<dyn Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let service = self
            .services
            .as_ref()
            .ok_or("No services found")?
            .iter()
            .find(|s| s["type"] == service.to_string())
            .ok_or("Service not found")?;
        let service: T = serde_json::from_value(service.clone())?;
        Ok(service)
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RedisUser {
    pub id: String,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
}

// make a from method for User to RedisUser
impl From<User> for RedisUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap_or_default(),
            user_id: user.user_id.unwrap_or_default(),
            access_token: user.access_token.unwrap_or_default(),
            refresh_token: user.refresh_token.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisMessage {
    pub id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub user: RedisUser,
    pub service: serde_json::Value,
}

impl RedisMessage {
    pub fn new(id: String, timestamp: DateTime<Utc>, user: RedisUser, service: serde_json::Value) -> Self {
        Self {
            id,
            timestamp,
            user,
            service,
        }
    }
}

