use dotenv::dotenv;
use std::env::{self, Vars};

use serde::Deserialize;

pub fn check() {
    dotenv().ok();

    let c = envy::from_iter::<Vars, Configuration>(env::vars().into_iter())
        .expect("Please provide DISCORD_TOKEN");

    println!("{:#?}", c)
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub discord_token: String,

    #[serde(default = "default_owner_id")]
    pub owner_id: String,
}

fn default_owner_id() -> String {
    "000000000".to_string()
}
