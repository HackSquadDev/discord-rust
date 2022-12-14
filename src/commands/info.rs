use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Colour;
use time::OffsetDateTime;

use crate::data::UptimeData;
use crate::utils::calculate_latency::calculate_latency;
use crate::utils::git::{get_version, VersionInfo};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let data = ctx.data.read().await;
    let version = get_version();
    let uptime = data.get::<UptimeData>();

    // start measuring latency
    let ping_start = OffsetDateTime::now_utc();

    let embed = generate_embed("Pinging...".to_string(), uptime, &version);
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| message.set_embed(embed))
        })
        .await
        .unwrap();

    // end measuring latency
    let end_ping = OffsetDateTime::now_utc();
    let latency = calculate_latency(ping_start, end_ping);

    let embed = generate_embed(format!("{} ms", latency), uptime, &version);

    command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
        .unwrap();
}

fn generate_embed(
    latency: String,
    uptime: Option<&OffsetDateTime>,
    version: &VersionInfo,
) -> CreateEmbed {
    let uptime = match uptime {
        Some(data) => format!("<t:{}:R>", data.unix_timestamp()),
        None => "Not Availible".to_string(),
    };

    let embed = CreateEmbed::default()
        .title("HackSquad Bot")
        .url("https://github.com/HackSquadDev/discord-rust")
        .field("API Ping (Discord)", format!("`{}`", latency), false)
        .field("Bot Started", uptime, false)
        .field("Version", format!("`{}`", version.clone()), false)
        .color(Colour::MAGENTA)
        .thumbnail(
            "https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless",
        )
        .footer(|footer| footer.text("Made with ??? by Midka, Nishant, Sloth816"))
        .to_owned();

    embed
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Get info about the bot and api latency")
}
