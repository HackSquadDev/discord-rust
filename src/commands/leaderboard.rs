use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::Context;
use serenity::utils::Color;

use crate::api::team::get_leaderboard;
use crate::data::PaginationMap;
use crate::database::Database;
use crate::pagination::Pagination;

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let ctx_cloned = ctx.clone();
    let data = ctx_cloned.data.read().await;
    let database = data.get::<Database>().unwrap();

    let leaderboard = match get_leaderboard(database).await {
        Some(leaderboard) => leaderboard,
        None => {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content("Failed to fetch leaderboard")
                                .ephemeral(true)
                        })
                })
                .await
                .expect("Failed to send response");

            return;
        }
    };

    let mut pages = Vec::new();
    for team_list in leaderboard.chunks(8) {
        let mut description = String::new();
        for team in team_list {
            description += &format!(
                "**[{}](https://hacksquad.dev/team/{})**\n<:reply_multi:1029067132572549142>`🥇`Rank: `{}`\n<:reply:1029065416905076808>Points: `{}`\n",
                team.name.clone(),
                team.slug,
                leaderboard.iter().position(|r| r.slug == team.slug).unwrap() + 1,
                team.score
            )
        }
        let page = CreateEmbed::default()
            .title("HackSquad Leaderboard")
            .url("https://hacksquad.dev/leaderboard")
            .description(description)
            .color(Color::BLITZ_BLUE)
            .thumbnail("https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless")
            .to_owned();
        pages.push(page)
    }

    let mut pagination = Pagination::new(pages);
    pagination.handle_message(ctx, command.clone()).await;

    let mut paginations = data.get::<PaginationMap>().unwrap().lock().await;
    paginations.insert(command.user.id, pagination);
}

pub async fn handle_interaction(
    ctx: Context,
    component: MessageComponentInteraction,
    interaction: Interaction,
) {
    if interaction.clone().message_component().unwrap().user.id
        != component.message.interaction.clone().unwrap().user.id
    {
        component
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message
                        .add_embed(
                            CreateEmbed::default()
                                .description(format!(
                                    "This belongs to <@{}>",
                                    component.message.interaction.clone().unwrap().user.id
                                ))
                                .color(Color::RED)
                                .to_owned(),
                        )
                        .ephemeral(true)
                })
            })
            .await
            .expect("Failed to send response");

        return;
    }

    let ctx_cloned = ctx.clone();
    let data = ctx_cloned.data.read().await;
    let mut paginations = data.get::<PaginationMap>().unwrap().lock().await;

    let mut should_clear = false;
    // println!("1 {:#?}", component);
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
                            .add_embed(
                                CreateEmbed::default()
                                    .description(format!(
                                        "This belongs to <@{}>",
                                        component.clone().message.interaction.unwrap().user.id
                                    ))
                                    .color(Color::RED)
                                    .to_owned(),
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
    command
        .name("leaderboard")
        .description("Get the HackSquad Leaderboard")
}
