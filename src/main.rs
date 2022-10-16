use std::collections::HashMap;
use std::sync::Arc;

use data::{PaginationMap, UptimeData};
use database::Database;
use serenity::prelude::*;
use time::OffsetDateTime;

mod api;
mod commands;
mod data;
mod database;
mod environment;
mod events;
mod fuzzy;
mod pagination;
mod utils;

use crate::environment::Configuration;
use crate::events::Handler;

#[tokio::main]
async fn main() {
    let config = environment::check();
    let mut client = Client::builder(config.discord_token.clone(), GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let mut db = Database::default();
    db.initialize(config.clone()).await;

    {
        let mut data = client.data.write().await;

        data.insert::<Configuration>(config);
        data.insert::<Database>(db);
        data.insert::<UptimeData>(OffsetDateTime::now_utc());
        data.insert::<PaginationMap>(Arc::new(Mutex::new(HashMap::new())))
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
