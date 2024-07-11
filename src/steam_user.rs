#![allow(non_snake_case)]
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;

use crate::query_builder;

pub struct SteamUser {
    key: String,
    url: String,
}

struct Player {
    steamid: u64,
    personaname: String,
    profileurl: String,
    avatar: String,
    avatarmedium: String,
    avatarfull: String,
    personastate: u8,
    communityvisibilitystate: u8,
    profilestate: bool,
    lastlogoff: u64,
    commentpermission: bool,
    realname: Option<String>,
    primaryclanid: Option<String>,
    timecreated: Option<u64>,
    gameid: Option<u64>,
    gameserverip: Option<String>,
    gameextrainfo: Option<String>,
    cityid: Option<u64>,
    loccountrycode: Option<String>,
    locstatecode: Option<String>,
    loccityid: Option<u64>,
}

#[derive(Debug)]
pub struct VanityURLError(());

#[derive(Deserialize, Debug)]
struct VanityURL {
    steamid: Option<String>,
    success: u8,
    message: Option<String>
}

#[derive(Deserialize, Debug)]
struct ApiResponse<T> {
    response: T,
}

impl SteamUser {
    pub fn new(key: String) -> SteamUser {
        SteamUser {
            key,
            url: "http://api.steampowered.com/ISteamUser/".to_string(),
        }
    }
    pub async fn resolve_vanity_URL(&self, vanityurl: &str) -> Result<String, StatusCode> {
        resolve_vanity_URL(&self.key, vanityurl).await
    }
}

pub async fn resolve_vanity_URL(key: &str, vanityurl: &str) -> Result<String, StatusCode> {
    let response = query_builder::build::<ApiResponse<VanityURL>>(format!(
        "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}",
        key, vanityurl
    ))
    .await;

    match response {
        Ok(s) => {
            match s.response.success {
                1 => {
                    match s.response.steamid {
                        Some(sid) => Ok(sid),
                        None => Ok(s.response.message.unwrap())
                    }
                },
                42 => Ok("No match found".to_string()),
                _ => Err(StatusCode::BAD_REQUEST)
            }
        },
        Err(err) => Err(err),
    }
}
