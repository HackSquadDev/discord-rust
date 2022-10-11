use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::Context;

use crate::api::team::get_teams;
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
    let mut paginations = PAGINATION.lock().await;

    let mut should_clear = false;
    // Ensures only the user who started the command can interact with the pagination
    let page = paginations
        // safe unwrap
        .entry(interaction.clone().message_component().unwrap().user.id)
        .or_insert_with(|| Pagination::new(Vec::new()));

    match page.clone().author {
        Some(_) => {
            should_clear = page
                .handle_interaction_and_delete_pagination(ctx, component)
                .await
        }
        None => {
            if let Err(err) = component
                .create_interaction_response(ctx.http, |response| {
                    response.interaction_response_data(|message| {
                        message
                            .content(
                                "This embed has expired or ".to_owned()
                                    + " you don't have permission to interact with it.",
                            )
                            .ephemeral(true)
                    })
                })
                .await
            {
                println!("Error sending interaction response: {:?}", err);
            }
        }
    }

    if should_clear {
        // safe unwrap, never paniks
        paginations.remove(&interaction.clone().message_component().unwrap().user.id);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
