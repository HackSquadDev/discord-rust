#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use data::UptimeData;
use serenity::model::id::UserId;
use serenity::prelude::*;

mod api;
mod commands;
mod data;
mod database;
mod environment;
mod events;
mod fuzzy;
mod pagination;
mod utils;

use crate::database::Database as DB;
use crate::environment::Configuration;
use crate::events::Handler;
use crate::pagination::Pagination;

lazy_static! {
    static ref DATABASE: Arc<Mutex<DB>> = Arc::new(Mutex::new(DB::default()));
    static ref CONFIG: Arc<Mutex<Configuration>> = Arc::new(Mutex::new(environment::check()));
}
lazy_static! {
    static ref PAGINATION: Arc<Mutex<HashMap<UserId, Pagination>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(
        CONFIG.lock().await.discord_token.clone(),
        GatewayIntents::empty(),
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    DATABASE.lock().await.initialize().await;

    {
        let mut data = client.data.write().await;

        data.insert::<UptimeData>(Utc::now())
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
