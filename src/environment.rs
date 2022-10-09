use std::env::{self, Vars};

use dotenvy::dotenv;
use serde::Deserialize;

pub fn check() -> Configuration {
    dotenv().ok();

    envy::from_iter::<Vars, Configuration>(env::vars())
        .expect("Please provide environment variables")
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub discord_token: String,
    pub guild_id: String,
    pub redis_uri: String,

    #[serde(default = "default_leaderboards_ttl")]
    pub cache_leaderboard_ttl: usize,

    #[serde(default = "default_owner_id")]
    pub owner_id: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            discord_token: "".to_string(),
            guild_id: "".to_string(),
            redis_uri: "".to_string(),
            cache_leaderboard_ttl: default_leaderboards_ttl(),
            owner_id: default_owner_id(),
        }
    }
}

fn default_leaderboards_ttl() -> usize {
    60
}

fn default_owner_id() -> String {
    "000000000".to_string()
}
