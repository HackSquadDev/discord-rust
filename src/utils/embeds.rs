use serenity::{builder::CreateEmbed, utils::Colour};

pub fn error_embed(error: &str, fields: Vec<(String, String, bool)>) -> CreateEmbed {
    let mut default = CreateEmbed::default();
    let embed = default
        .color(Colour::RED)
        .title("Oh no! An error occurred...")
        .description(format!("```{}```", error))
        .fields(fields)
        .footer(|footer| footer.text("Any Questions? Well we dont have a support chat :P"));

    embed.to_owned()
}
