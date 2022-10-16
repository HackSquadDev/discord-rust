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
    pub redis_uri: String,

    #[serde(default = "default_leaderboards_ttl")]
    pub cache_leaderboard_ttl: usize,

    #[serde(default = "default_team_ttl")]
    pub cache_team_ttl: usize,

    #[serde(default = "default_heros_ttl")]
    pub cache_heros_ttl: usize,

    #[serde(default = "default_hero_ttl")]
    pub cache_hero_ttl: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            discord_token: "".to_string(),
            redis_uri: "".to_string(),
            cache_leaderboard_ttl: default_leaderboards_ttl(),
            cache_team_ttl: default_team_ttl(),
            cache_heros_ttl: default_heros_ttl(),
            cache_hero_ttl: default_hero_ttl(),
        }
    }
}

fn default_leaderboards_ttl() -> usize {
    60 * 30 // 30 minutes
}

fn default_team_ttl() -> usize {
    60 * 10 // 10 minutes
}

fn default_heros_ttl() -> usize {
    60 * 60 * 12 // 12 hours
}

fn default_hero_ttl() -> usize {
    60 * 60 * 12 // 12 hours
}
