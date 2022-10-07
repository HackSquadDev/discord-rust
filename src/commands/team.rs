use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

#[derive(Deserialize)]
struct Response {
    team: Team,
}

#[derive(Deserialize)]
struct Team {
    id: String,
    name: String,
    slug: String,
    score: u32,
    ownerId: String,
    prs: String,
}

#[derive(Deserialize)]
struct PR {
    id: String,
    createdAt: String,
    title: String,
    url: String,
    status: Option<String>,
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
        let response: Response = reqwest::get(&format!(
            "https://www.hacksquad.dev/api/team?id={}",
            team_id
        ))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        let all_prs: Vec<PR> = serde_json::from_str(&response.team.prs).unwrap();
        let mut deleted = 0;
        for pr in all_prs {
            if pr.status.is_some() {
                deleted += 1;
            }
        }

        let data = format!(
            "Name: {}\nScore: {}\nTotal PRs: {}\nTotal PRs Deleted: {}",
            response.team.name,
            response.team.score,
            response.team.score + deleted,
            deleted
        );

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| e.title("HackSquad Team Information").description(data))
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
