use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::Context;

use crate::api::get_teams;
use crate::pagination::Pagination;
use crate::PAGINATION;

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
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
    pagination.handle_message(ctx, command.clone()).await;

    let mut paginations = PAGINATION.lock().await;
    paginations.insert(command.user.id, pagination);
}

pub async fn handle_interaction(
    ctx: Context,
    component: MessageComponentInteraction,
    interaction: Interaction,
) {
    let paginations = PAGINATION.lock().await;

    // Ensures only the user who started the command can interact with the pagination
    let pagination = paginations.get(&interaction.clone().message_component().unwrap().user.id);

    match pagination {
        Some(page) => page.clone().handle_interaction(ctx, component).await,
        None => {
            interaction
                .message_component()
                .unwrap()
                .user
                .direct_message(ctx.http, |m| {
                    m.content("You can't interact with other's pagination")
                })
                .await
                .unwrap();
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
