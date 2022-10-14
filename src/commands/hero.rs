use serenity::builder::{CreateApplicationCommand, CreateButton};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::ReactionType;
use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::api::hero::{get_hero, get_random_hero, Hero, Pulls};
use crate::database::Database;
use crate::utils::embeds::error_embed;

fn link_button(name: &str, link: String, emoji: ReactionType) -> CreateButton {
    CreateButton::default()
        .url(link)
        .emoji(emoji)
        .label(name)
        .style(ButtonStyle::Link)
        .to_owned()
}

pub async fn run(
    ctx: Context,
    command: ApplicationCommandInteraction,
    hero: Option<Hero>,
    github_id: String,
) {
    let mut hero = match hero {
        Some(hero) => hero,
        None => {
            if let Err(err) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .set_embed(error_embed(
                                    "Hero not found :(",
                                    [("Github ID/Username".to_string(), github_id, false)].to_vec(),
                                ))
                                .ephemeral(true)
                        })
                })
                .await
            {
                println!("Error sending message: {:?}", err);
            }
            return;
        }
    };
    let mut pull_req = String::new();

    if hero.pulls.len() < 3 {
        for _ in 0..3 {
            hero.pulls.push(Pulls {
                title: "".to_string(),
                url: "".to_string(),
            })
        }
    }

    for (index, pr) in hero.pulls.iter().take(3).enumerate() {
        let mut pull_req_cloned = pull_req.clone();

        if hero.pulls[0..2].len() == index {
            if !hero.pulls[index].title.is_empty() {
                pull_req_cloned +=
                    &format!("<:reply:1029065416905076808>[{}]({})\n", pr.title, pr.url);
            } else {
                pull_req_cloned += "<:reply:1029065416905076808>Not Available\n";
            }
        } else if hero.pulls[index].title.is_empty() {
            pull_req_cloned += "<:reply_multi:1029067132572549142>Not Available\n";
        } else {
            pull_req_cloned += &format!(
                "<:reply_multi:1029067132572549142>[{}]({})\n",
                pr.title, pr.url
            );
        }

        pull_req = pull_req_cloned
    }

    let data = format!(
        "{} `ℹ️` **Information**\n<:reply_multi:1029067132572549142>**Name:** `{}`\n<:reply_multi:1029067132572549142>**Location:** `{}`\n<:reply_multi:1029067132572549142>**Total PRs:** `{}`\n<:reply:1029065416905076808>**Last Activity:** {}\n\n`📙` **Socials**\n<:gh:1029368861776167004> **GitHub:** https://github.com/{}\n<:lkdn:1029410421641326755> **LinkedIn:** {}\n<:twitter:1029410910432935936> **Twitter:** {}\n<:discord:1029412089170767922> **Discord:** {}\n\n`🔗` **Last 3 PRs**\n{}",
        format_args!("_{}_ \n\n", hero.bio.unwrap_or_else(|| "".to_string())),
        hero.name.unwrap_or_else(|| "Unknown".to_string()),
        hero.location.unwrap_or_else(|| "Unknown".to_string()),
        if let Some(pulls) = hero.total_pulls {
            format!("{}", pulls)
        } else {
            "Not Available".to_string()
        },
        if let Some(last_activity_occurred_at) = hero.last_activity_occurred_at {
            format!("<t:{}:F>",
                Timestamp::from(last_activity_occurred_at).unix_timestamp()
            )
        } else {
            "Not Available".to_string()
        },
        hero.github,
        hero.linkedin.unwrap_or_else(|| "Not Linked".to_string()),
        match hero.twitter {
            Some(handle) => format!("https://twitter.com/{}", handle),
            None => "Not Linked".to_string()
        },
        hero.discord.unwrap_or_else(|| "Not Linked".to_string()),
        pull_req,
    );

    if let Err(err) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.components(|c| {
                        c.create_action_row(|r| {
                            r.add_button(link_button(
                                "Hero Page",
                                format!("https://github.com/{}", hero.github),
                                "🔗".parse().unwrap(),
                            ))
                        })
                    });
                    message.embed(|e| {
                        e.title("Novu Hero Information")
                            .description(data)
                            .color(Colour::BLITZ_BLUE)
                            .thumbnail(hero.avatar_url)
                            .image(format!(
                                "https://contributors.novu.co/profiles/{}-small.jpg",
                                hero.github
                            ))
                            .footer(|footer| {
                                footer
                                    .text("Novu")
                                    .icon_url("https://novu.co/favicon-32x32.png")
                            })
                            .timestamp(Timestamp::now())
                    })
                })
        })
        .await
    {
        println!("Error sending message: {:?}", err);
    }
}

pub async fn hero(ctx: Context, command: ApplicationCommandInteraction) {
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

    if let CommandDataOptionValue::String(hero_github_id) = option {
        let hero = get_hero(database, hero_github_id).await;

        run(ctx, command.clone(), hero, hero_github_id.to_string()).await
    }
}

pub async fn random_hero(ctx: Context, command: ApplicationCommandInteraction) {
    let ctx_cloned = ctx.clone();
    let data = ctx_cloned.data.read().await;
    let database = data.get::<Database>().unwrap();

    let hero = get_random_hero(database).await;

    run(ctx, command, hero, "Random".to_string()).await
}

pub fn register_hero(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("hero")
        .description("Get info on a novu community hero")
        .create_option(|option| {
            option
                .name("github_username")
                .description("The github username of the contributor to look for")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn register_random_hero(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("randomhero")
        .description("Get a random contributor to novu's repository")
}
