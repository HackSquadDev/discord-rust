use chrono::Utc;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::utils::calculate_latency;

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let ping_start = Utc::now();

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.set_embed(generate_embed("Pinging...".to_string()))
            })
        })
        .await
        .unwrap();

    let end_ping = Utc::now();

    let latency = calculate_latency(ping_start, end_ping);

    let embed = generate_embed(format!("{} ms", latency));

    command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
        .unwrap();
}

fn generate_embed(latency: String) -> CreateEmbed {
    let embed = CreateEmbed::default()
        .title("Hacksquad Bot")
        .field("Gateway Latency", latency, false)
        .color(Colour::MAGENTA)
        .thumbnail(
            "https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless",
        )
        .to_owned();

    embed
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Get info about the bot and api latency")
}
