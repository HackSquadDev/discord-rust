use std::collections::HashMap;

use serde::Deserialize;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::Interaction;
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
    paginations: &mut HashMap<UserId, Pagination>,
) {
    let api_response: Response = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut pages = Vec::new();
    for team_list in api_response.teams.chunks(10) {
        let page = CreateEmbed::default()
            .fields(team_list.into_iter().map(|f| {
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
    // println!("Pagination in initial request {:#?}", pagination);
    pagination.handle_message(ctx, command.clone()).await;

    paginations.insert(command.user.id, pagination);
    // println!("Paginations in initial request {:#?}", paginations);
}

pub async fn handle_interaction(
    ctx: &Context,
    interaction: Interaction,
    paginations: &HashMap<UserId, Pagination>,
) {
    let pagination = paginations.get(&interaction.clone().message_component().unwrap().user.id);

    // println!("Paginations {:?}", paginations);
    // println!(
    //     "Interaction cloned user id in message component{:?}",
    //     &interaction.clone().message_component().unwrap().user.id
    // );
    // println!("Pagination in handle interaction {:?}", pagination);

    match pagination {
        Some(page) => page.handle_interaction(ctx, interaction),
        None => println!("No pagination found"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
