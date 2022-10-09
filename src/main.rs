use std::env;

use serenity::prelude::*;

mod commands;
mod environment;
mod events;
mod fuzzy;
mod pagination;

use crate::events::Handler;

#[tokio::main]
async fn main() {
    environment::check();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
