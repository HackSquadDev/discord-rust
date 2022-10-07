use std::env::{self, Vars};

use dotenvy::dotenv;
use serde::Deserialize;

pub fn check() -> Configuration {
    dotenv().ok();

    envy::from_iter::<Vars, Configuration>(env::vars())
        .expect("Please provide environment variables")
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub discord_token: String,
    pub guild_id: String,

    #[serde(default = "default_owner_id")]
    pub owner_id: String,
}

fn default_owner_id() -> String {
    "000000000".to_string()
}
