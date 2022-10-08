use serde::Deserialize;
use serenity::builder::{CreateApplicationCommand, CreateButton, CreateEmbed};
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;

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

fn button(name: &str, style: ButtonStyle, emoji: ReactionType) -> CreateButton {
    let mut b = CreateButton::default();
    b.emoji(emoji.clone());
    b.label(name);
    b.style(style);
    b.custom_id(emoji);

    b
}

pub async fn run(command: ApplicationCommandInteraction, ctx: Context) {
    let api_response: Response = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let index = 0;

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

    let pages_count = pages.len();

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.components(|c| {
                    c.create_action_row(|r| {
                        r.add_button(button(
                            "First",
                            ButtonStyle::Primary,
                            ReactionType::Unicode("⏮️".to_string()),
                        ));
                        r.add_button(button(
                            "Prev",
                            ButtonStyle::Primary,
                            ReactionType::Unicode("◀️".to_string()),
                        ));
                        r.add_button(button(
                            "Stop",
                            ButtonStyle::Danger,
                            ReactionType::Unicode("⏹️".to_string()),
                        ));
                        r.add_button(button(
                            "Next",
                            ButtonStyle::Primary,
                            ReactionType::Unicode("▶️".to_string()),
                        ));
                        r.add_button(button(
                            "Last",
                            ButtonStyle::Primary,
                            ReactionType::Unicode("⏭️".to_string()),
                        ))
                    })
                });
                message.set_embed(
                    pages[index]
                        .footer(|footer| {
                            footer.text(format!("Page {} of {}", index + 1, pages_count))
                        })
                        .clone(),
                )
            })
        })
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("teams").description("Leaderboard")
}
