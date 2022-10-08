use std::collections::HashMap;

use serde::Deserialize;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;

use crate::pagination::Pagination;

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

pub async fn run(
    command: ApplicationCommandInteraction,
    ctx: Context,
    mut paginations: HashMap<UserId, Pagination>,
) {
    let api_response: Response = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let iter = api_response.teams.chunks(10);

    let mut pages = Vec::new();

    for (_index, e) in iter.enumerate() {
        let page = CreateEmbed::default()
            .fields(e.into_iter().map(|f| {
                (
                    format!(
                        "#{} {}",
                        api_response
                            .teams
                            .iter()
                            .position(|r| r.slug == f.slug)
                            .unwrap()
                            + 1,
                        f.name.clone()
                    ),
                    format!("{} points", f.score),
                    false,
                )
            }))
            .clone();

        pages.push(page)
    }

    let mut pagination = Pagination::new(pages);

    pagination.handle_message(ctx, command).await;

    paginations.insert(pagination.author.clone().unwrap().id, pagination);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
