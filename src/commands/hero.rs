use serenity::builder::CreateButton;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;
use serenity::utils::Colour;

use crate::api::get_hero;

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

    if let CommandDataOptionValue::String(hero_github_id) = option {
        let hero = get_hero(hero_github_id).await;

        let data = format!("**Name:** {:?}\n", hero.name);

        if let Err(err) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.components(|c|{
                            c.create_action_row(|r|{
                                r.add_button(link_button("Hero Page", format!("https://github.com/{}", hero.github), "🔗".parse().unwrap()))
                            })
                        });
                        message.embed(|e| e.title("Novu Hero Information").description(data)
                        .color(Colour::BLITZ_BLUE)
                        .thumbnail("https://cdn.discordapp.com/emojis/1026095278941552690.webp?size=128&quality=lossless"))
                    })
            })
            .await {
                println!("Error sending message: {:?}", err);


            }
    }
}
