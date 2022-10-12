use std::env;

use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, GuildId, Ready},
    prelude::{Context, EventHandler},
};

use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(ref command) => match command.data.name.as_str() {
                "team" => commands::team::run(ctx, command.to_owned()).await,
                "leaderboard" => commands::leaderboard::run(ctx, command.to_owned()).await,
                "hero" => commands::hero::hero(ctx, command.to_owned()).await,
                "randomhero" => commands::hero::random_hero(ctx, command.to_owned()).await,
                other_commands => println!("Unknown command {}", other_commands),
            },
            Interaction::MessageComponent(ref component) => match component.message.interaction {
                Some(ref message_interaction) => match message_interaction.name.as_str() {
                    "leaderboard" => {
                        commands::leaderboard::handle_interaction(
                            ctx,
                            component.to_owned(),
                            interaction.to_owned(),
                        )
                        .await
                    }
                    _ => println!("We only handle component interaction in leaderboard command"),
                },
                None => println!("No interaction"),
            },
            Interaction::Autocomplete(ref command) => match command.data.name.as_str() {
                "team" => {
                    commands::team::handle_autocomplete(ctx, command, interaction.to_owned()).await
                }
                other_commands => println!("No autocompletions for {}", other_commands),
            },
            other_interactions => println!("Unhandled interaction {:?}", other_interactions),
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
            commands.create_application_command(|command| commands::leaderboard::register(command));
            commands.create_application_command(|command| {
                commands::hero::register_random_hero(command)
            });
            commands.create_application_command(|command| commands::hero::register_hero(command))
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
