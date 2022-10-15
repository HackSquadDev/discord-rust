use serenity::{
    async_trait,
    http::CacheHttp,
    model::prelude::{command::Command, interaction::Interaction, Ready},
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
                "info" => commands::info::run(ctx, command.to_owned()).await,
                other_commands => {
                    if let Err(err) = command
                        .create_interaction_response(ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message.content("Unknown Command").ephemeral(true)
                            })
                        })
                        .await
                    {
                        println!("Error sending unknown command response: {:?}", err);
                    }

                    println!("Unknown command {}", other_commands)
                }
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
                    other_command => {
                        if let Err(err) = component
                            .create_interaction_response(ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Unknown Command Interaction")
                                        .ephemeral(true)
                                })
                            })
                            .await
                        {
                            println!(
                                "Error sending unknown command interaction response: {:?}",
                                err
                            );
                        };
                        println!("No interaction handler for {}", other_command)
                    }
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

        let commands = Command::create_global_application_command(&ctx.http(), |command| {
            commands::team::register(command);
            commands::leaderboard::register(command);
            commands::hero::register_random_hero(command);
            commands::hero::register_hero(command);
            commands::info::register(command)
        })
        .await;

        println!("Registering global commands: {:?}", commands);
    }
}
