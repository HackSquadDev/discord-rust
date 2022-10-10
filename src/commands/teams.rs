use std::collections::HashMap;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::UserId;
use serenity::prelude::Context;

use crate::api::get_teams;
use crate::pagination::Pagination;

pub async fn run(
    command: ApplicationCommandInteraction,
    ctx: Context,
    paginations: &mut HashMap<UserId, Pagination>,
) {
    let teams = get_teams().await;

    let mut pages = Vec::new();
    for team_list in teams.chunks(10) {
        let page = CreateEmbed::default()
            .fields(team_list.iter().map(|f| {
                (
                    format!(
                        "#{} {}",
                        teams.iter().position(|r| r.slug == f.slug).unwrap() + 1,
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
