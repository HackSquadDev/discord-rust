#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use serenity::prelude::*;

mod api;
mod commands;
mod database;
mod environment;
mod events;
mod fuzzy;
mod pagination;

use crate::database::Database as DB;
use crate::events::Handler;

lazy_static! {
    static ref DATABASE: Arc<Mutex<DB>> = Arc::new(Mutex::new(DB::default()));
}

#[tokio::main]
async fn main() {
    let config = environment::check();

    let mut client = Client::builder(&config.discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    DATABASE.lock().await.initialize(&config);

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
