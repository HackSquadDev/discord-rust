use serenity::builder::{CreateApplicationCommand, CreateButton};
use serenity::model::application::interaction::autocomplete::AutocompleteInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::api::team::{get_leaderboard, get_team, PR};
use crate::database::Database;
use crate::fuzzy::search_teams;
use crate::utils::embeds::error_embed;

fn link_button(name: &str, link: String, emoji: ReactionType) -> CreateButton {
    CreateButton::default()
        .url(link)
        .emoji(emoji)
        .label(name)
        .style(ButtonStyle::Link)
        .to_owned()
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let option = command
        .data
        .options
        .get(0)
        .expect("Expected String Option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let ctx_cloned = ctx.clone();
    let data = ctx_cloned.data.read().await;
    let database = data.get::<Database>().unwrap();

    if let CommandDataOptionValue::String(team_id) = option {
        let team = match get_team(database, team_id).await {
            Some(team) => team,
            None => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .set_embed(error_embed(
                                        "Team not found",
                                        [("Query".to_string(), team_id.clone(), false)].to_vec(),
                                    ))
                                    .ephemeral(true)
                            })
                    })
                    .await
                    .expect("Failed to send response");

                return;
            }
        };

        let leaderboard = match get_leaderboard(database).await {
            Some(leaderboard) => leaderboard,
            None => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Failed to fetch leaderboard")
                            })
                    })
                    .await
                    .expect("Failed to send response");

                return;
            }
        };

        let mut pull_req = String::new();
        let mut user_list = String::new();

        match team.users {
            Some(users) => {
                for (index, user) in users.iter().enumerate() {
                    let mut user_list_cloned = user_list.clone();

                    if users.len() - 1 == index {
                        user_list_cloned += format!(
                            "<:reply:1029065416905076808>[{}](https://github.com/{})\n",
                            user.name, user.handle
                        )
                        .as_ref();
                    } else {
                        user_list_cloned += format!(
                            "<:reply_multi:1029067132572549142>[{}](https://github.com/{})\n",
                            user.name, user.handle
                        )
                        .as_ref();
                    }

                    user_list = user_list_cloned
                }
            }
            None => {
                user_list += "Could not get team members";
                todo!("API Returned invalid response")
            }
        }

        if let Some(prs) = team.prs {
            let all_prs: Vec<PR> = serde_json::from_str(&prs).unwrap();
            for (index, pr) in all_prs.iter().take(3).enumerate() {
                let mut pull_req_cloned = pull_req.clone();

                if all_prs[0..2].len() == index {
                    pull_req_cloned +=
                        &format!("<:reply:1029065416905076808>[{}]({})\n", pr.title, pr.html_url);
                } else {
                    pull_req_cloned += &format!(
                        "<:reply_multi:1029067132572549142>[{}]({})\n",
                        pr.title, pr.html_url
                    );
                }

                pull_req = pull_req_cloned
            }
            let mut deleted = 0;
            for pr in all_prs {
                if pr.status.is_some() {
                    deleted += 1;
                }
            }

            let data = format!(
            "`ℹ️` **Information**\n<:reply_multi:1029067132572549142>**Name:** {}\n<:reply_multi:1029067132572549142>**Rank:**`{}`\n<:reply_multi:1029067132572549142>**Score:** `{}`\n<:reply_multi:1029067132572549142>**Total PRs:** `{}`\n<:reply:1029065416905076808>**Total PRs Deleted:** `{}`\n\n`🏆` **Team Members**\n{}\n`🔗` **Last 3 PRs**\n{}",
            team.name,
            leaderboard.iter().position(|r| r.slug == team.slug).unwrap() + 1,
            team.score,
            team.score + deleted,
            deleted,
            user_list,
            pull_req
        );

            if let Err(err) = command
            .create_interaction_response(ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.components(|c|{
                            c.create_action_row(|r|{
                                r.add_button(link_button("Team Page", format!("https://hacksquad.dev/team/{}", team.slug), "🔗".parse().unwrap()))
                            })
                        });
                        message.embed(|e| e.title("HackSquad Team Information").description(data)
                        .color(Colour::BLITZ_BLUE)
                        .thumbnail("https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless"))
                    })
            })
            .await {
                println!("Error sending message: {:?}", err);


            }
        }
    } else if let Err(err) = command
        .create_interaction_response(ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Please provide a valid team"))
        })
        .await
    {
        println!("Error sending message: {:?}", err);
    }
}

pub async fn handle_autocomplete(
    ctx: Context,
    command: &AutocompleteInteraction,
    _interaction: Interaction,
) {
    let ctx_cloned = ctx.clone();
    let data = ctx_cloned.data.read().await;
    let database = data.get::<Database>().unwrap();

    let search = search_teams(database, command.data.options[0].value.clone()).await;

    command
        .create_autocomplete_response(ctx.http, |response| response.set_choices(search))
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("team")
        .description("Get a teams information")
        .create_option(|option| {
            option
                .name("id")
                .description("The id of the team to look for")
                .kind(CommandOptionType::String)
                .set_autocomplete(true)
                .required(true)
        })
}
