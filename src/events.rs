use std::{collections::HashMap, env};

use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, GuildId, Ready, UserId},
    prelude::{Context, EventHandler},
};

use crate::{commands, pagination::Pagination};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let mut paginations: HashMap<UserId, Pagination> = HashMap::new();

        match interaction {
            Interaction::ApplicationCommand(ref command) => match command.data.name.as_str() {
                "team" => commands::team::run(command.clone(), ctx.clone()).await,
                "teams" => {
                    commands::teams::run(command.clone(), ctx.clone(), &mut paginations).await
                }
                _ => todo!(),
            },
            Interaction::MessageComponent(ref b) => {
                match b.data.custom_id.as_str() {
                    _ => {
                        commands::teams::handle_interaction(&ctx, interaction.clone(), &paginations)
                    }
                }
                .await
            }
            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::team::register(command));
            commands.create_application_command(|command| commands::teams::register(command))
        })
        .await;

        let mut slash_command_names: Vec<String> = Vec::new();

        match commands {
            Ok(commands) => {
                for command in commands {
                    slash_command_names.push(command.name)
                }
            }
            Err(_) => todo!(),
        }

        println!(
            "I now have the following guild slash commands: {:#?}",
            slash_command_names
        );
    }
}
