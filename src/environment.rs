use dotenvy::dotenv;
use std::env::{self, Vars};

use serde::Deserialize;

pub fn check() -> Configuration {
    dotenv().ok();

    let c = envy::from_iter::<Vars, Configuration>(env::vars().into_iter())
        .expect("Please provide environment variables");

    c
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
