use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected String Option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    if let CommandDataOptionValue::String(team_id) = option {
        let response = reqwest::get(format!("https://www.hacksquad.dev/api/team?id={}", team_id))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        format!("team information is {}", response)
    } else {
        "Please provide a valid user".to_string()
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
