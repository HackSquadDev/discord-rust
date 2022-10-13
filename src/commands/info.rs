use chrono::{DateTime, Utc};
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::data::UptimeData;
use crate::utils::calculate_latency::calculate_latency;
use crate::utils::git::{get_version, VersionInfo};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let data = ctx.data.read().await;
    let version = get_version();
    let uptime = data.get::<UptimeData>();

    // start measuring latency
    let ping_start = Utc::now();

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.set_embed(generate_embed("Pinging...".to_string(), uptime, &version))
            })
        })
        .await
        .unwrap();

    // end measuring latency
    let end_ping = Utc::now();
    let latency = calculate_latency(ping_start, end_ping);

    let embed = generate_embed(format!("{} ms", latency), uptime, &version);

    command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
        .unwrap();
}

fn generate_embed(
    latency: String,
    uptime: Option<&DateTime<Utc>>,
    version: &VersionInfo,
) -> CreateEmbed {
    let uptime = match uptime {
        Some(data) => format!("<t:{}:R>", data.timestamp()),
        None => "Not Availible".to_string(),
    };

    let embed = CreateEmbed::default()
        .title("Hacksquad Bot")
        .url("https://github.com/HackSquadDev/discord-rust")
        .field("API Ping (Discord)", format!("`{}`", latency), false)
        .field("Uptime", uptime, false)
        .field("Version", format!("`{}`", version.clone()), false)
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
