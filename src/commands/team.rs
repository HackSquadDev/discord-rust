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

use crate::api::{get_team, get_teams, PR};
use crate::fuzzy::search_teams;

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

    if let CommandDataOptionValue::String(team_id) = option {
        let team = get_team(team_id).await;
        let teams = get_teams().await;

        let mut pull_req = String::new();
        let mut user_list = String::new();

        for (index, user) in team.users.iter().enumerate() {
            let mut user_list_cloned = user_list.clone();

            if team.users.len() - 1 == index {
                user_list_cloned += &format!(
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

        if let Some(prs) = team.prs {
            let all_prs: Vec<PR> = serde_json::from_str(&prs).unwrap();
            for (index, pr) in all_prs.iter().take(3).enumerate() {
                let mut pull_req_cloned = pull_req.clone();

                if all_prs[0..2].len() == index {
                    pull_req_cloned +=
                        &format!("<:reply:1029065416905076808>[{}]({})\n", pr.title, pr.url);
                } else {
                    pull_req_cloned += &format!(
                        "<:reply_multi:1029067132572549142>[{}]({})\n",
                        pr.title, pr.url
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
            teams.iter().position(|r| r.slug == team.slug).unwrap() + 1,
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
                .interaction_response_data(|message| {
                    message.content("Please provide a valid teams")
                })
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
    let search = search_teams(command.data.options[0].value.clone()).await;

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
