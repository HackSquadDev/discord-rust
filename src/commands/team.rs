use serde::Deserialize;
use serenity::builder::{CreateApplicationCommand, CreateButton};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;
use serenity::utils::Colour;

#[derive(Deserialize)]
struct Response {
    team: Team,
}

#[derive(Deserialize)]
struct Team {
    name: String,
    score: u32,
    prs: String,
    slug: String,
}

#[derive(Deserialize)]
struct PR {
    status: Option<String>,
}

fn link_button(name: &str, link: String, emoji: ReactionType) -> CreateButton {
    let mut b = CreateButton::default();
    b.url(link);
    b.emoji(emoji);
    b.label(name);
    b.style(ButtonStyle::Link);

    b
}

pub async fn run(command: ApplicationCommandInteraction, ctx: Context) {
    let option = command
        .data
        .options
        .get(0)
        .expect("Expected String Option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    if let CommandDataOptionValue::String(team_id) = option {
        let api_response: Response = reqwest::get(&format!(
            "https://www.hacksquad.dev/api/team?id={}",
            team_id
        ))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        let all_prs: Vec<PR> = serde_json::from_str(&api_response.team.prs).unwrap();
        let mut deleted = 0;
        for pr in all_prs {
            if pr.status.is_some() {
                deleted += 1;
            }
        }

        let data = format!(
            "**Name:** {}\n**Score:** {}\n**Total PRs:** {}\n**Total PRs Deleted:** {}",
            api_response.team.name,
            api_response.team.score,
            api_response.team.score + deleted,
            deleted
        );

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.components(|c|{
                            c.create_action_row(|r|{
                                r.add_button(link_button("Team Page", format!("https://hacksquad.dev/team/{}", api_response.team.slug), "🔗".parse().unwrap()))
                            })
                        });
                        message.embed(|e| e.title("HackSquad Team Information").description(data)
                        .color(Colour::BLITZ_BLUE)
                        .thumbnail("https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless"))
                    })
            })
            .await
            .unwrap();
    } else {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Please provide a valid teams")
                    })
            })
            .await
            .unwrap();
    }
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
                .required(true)
        })
}
