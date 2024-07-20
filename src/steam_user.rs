#![allow(non_snake_case)]
#![allow(dead_code)]
use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt::Debug;

use crate::query_builder;

pub struct Interface {
    key: String,
}

struct Player {
    pub steamid: String,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub personastate: u8,
    pub communityvisibilitystate: u8,
    pub profilestate: bool,
    pub lastlogoff: u64,
    pub commentpermission: bool,
    pub realname: Option<String>,
    pub primaryclanid: Option<String>,
    pub timecreated: Option<u64>,
    pub gameid: Option<u64>,
    pub gameserverip: Option<String>,
    pub gameextrainfo: Option<String>,
    pub cityid: Option<u64>,
    pub loccountrycode: Option<String>,
    pub locstatecode: Option<String>,
    pub loccityid: Option<u64>,
}

#[derive(Debug)]
pub struct VanityURLError(());

#[derive(Deserialize, Debug)]
struct VanityURL {
    steamid: Option<String>,
    success: u8,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ApiResponse<T> {
    response: T,
}

#[derive(Deserialize, Debug)]
struct Friend {
    pub steamid: String,
    pub relationship: String,
    pub friend_since: u64,
}

#[derive(Deserialize, Debug)]
pub struct BanObject {
    #[serde(alias = "SteamId")]
    pub steamid: String,
    #[serde(alias = "CommunityBanned")]
    pub communitybanned: bool,
    #[serde(alias = "DaysSinceLastBan")]
    pub dayssincelastban: u32,
    #[serde(alias = "VACBanned")]
    pub vacbanned: bool,
    #[serde(alias = "NumberOfGameBans")]
    pub numberofgamebans: u16,
    #[serde(alias = "EconomyBan")]
    pub economyban: String,
}

#[derive(Deserialize, Debug)]
struct BanList {
    players: Vec<BanObject>,
}

impl Interface {
    pub fn new(key: String) -> Interface {
        Interface { key }
    }
    pub async fn resolve_vanity_URL(&self, vanityurl: &str) -> Result<String, StatusCode> {
        resolve_vanity_URL(&self.key, vanityurl).await
    }
    pub async fn get_friend_list(&self, steamid: &str) {
        let response = query_builder::build::<ApiResponse<Option<Vec<Friend>>>>(format!(
                "http://api.steampowered.com/ISteamUser/GetFriendList/v1?steamid={}&relationship=all&key={}",
                steamid,
                self.key
        )).await;
        println!("{:#?}", response);
    }
    // TODO: comma delimited list of steamids (ban)
    pub async fn get_player_bans(&self, steamid: &str) -> Result<Vec<BanObject>, StatusCode> {
        let response = query_builder::build::<BanList>(format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerBans/v1/?key={}&steamids={}",
            self.key, steamid
        ))
        .await;

        match response {
            Ok(s) => Ok(s.players),
            Err(err) => Err(err),
        }
    }
}

pub async fn resolve_vanity_URL(key: &str, vanityurl: &str) -> Result<String, StatusCode> {
    let response = query_builder::build::<ApiResponse<VanityURL>>(format!(
        "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={}",
        key, vanityurl
    ))
    .await;

    match response {
        Ok(s) => match s.response.success {
            1 => match s.response.steamid {
                Some(sid) => Ok(sid),
                None => Ok(s.response.message.unwrap()),
            },
            42 => Ok("No match found".to_string()),
            _ => Err(StatusCode::BAD_REQUEST),
        },
        Err(err) => Err(err),
    }
}
