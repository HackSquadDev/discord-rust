use std::sync::Arc;

use serde::Deserialize;
use serenity::builder::{CreateApplicationCommand, CreateEmbed, CreateMessage};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::{EmbedField, Message, Reaction, ReactionType};
use serenity::prelude::Context;
use serenity::utils::Colour;

#[derive(Deserialize, Debug)]
struct Response {
    teams: Vec<Team>,
}

#[derive(Deserialize, Debug)]
struct Team {
    name: String,
    score: u32,
    slug: String,
}

pub async fn run(command: ApplicationCommandInteraction, ctx: Context) {
    let api_response: Response = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let index = 0;

    let iter = api_response.teams.chunks(10);

    let mut pages = Vec::new();

    for (index, e) in iter.enumerate() {
        let page = CreateEmbed::default()
            .description("test")
            .field(e[0].name.clone(), e[0].score, true)
            .fields(
                e.into_iter()
                    .map(|f| (format!("#place {}", f.name.clone()), f.score, true)),
            )
            .clone();

        pages.push(page)
    }

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| message.set_embed(pages[index].clone()))
        })
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
