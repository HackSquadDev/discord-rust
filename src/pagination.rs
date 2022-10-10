use serenity::{
    builder::{CreateButton, CreateEmbed},
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            component::ButtonStyle,
            interaction::{
                message_component::MessageComponentInteraction, InteractionResponseType,
            },
            ReactionType,
        },
        user::User,
    },
    prelude::Context,
};

#[derive(Debug, Clone)]
pub struct Pagination {
    pages: Vec<CreateEmbed>,
    index: usize,
    pub author: Option<User>,
}

impl Pagination {
    pub fn new(pages: Vec<CreateEmbed>) -> Self {
        Self {
            pages,
            index: 0,
            author: None,
        }
    }

    pub async fn handle_message(&mut self, ctx: Context, command: ApplicationCommandInteraction) {
        let pages_count = self.pages.len();

        self.author = Some(command.user.clone());

        command
            .create_interaction_response(ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message.components(|c| {
                        c.create_action_row(|r| {
                            r.add_button(button("First", ButtonStyle::Primary, "⏮️"));
                            r.add_button(button("Prev", ButtonStyle::Primary, "◀️"));
                            r.add_button(button("Stop", ButtonStyle::Danger, "⏹️"));
                            r.add_button(button("Next", ButtonStyle::Primary, "▶️"));
                            r.add_button(button("Last", ButtonStyle::Primary, "⏭️"))
                        })
                    });
                    message.set_embed(
                        self.pages[self.index]
                            .clone()
                            .footer(|footer| {
                                footer.text(format!("Page {} of {}", self.index + 1, pages_count))
                            })
                            .clone(),
                    )
                })
            })
            .await
            .unwrap();
    }

    pub async fn handle_interaction(
        &mut self,
        ctx: Context,
        component: MessageComponentInteraction,
    ) {
        let page_count = self.pages.len();
        // println!("Component: {:#?}", component);
        match component.data.custom_id.as_str() {
            "⏮️" => {
                self.index = 0;
            }
            "◀️" => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            "⏹️" => {
                self.pages.clear();
                self.author = None;
                self.index = 0;

                component
                    .message
                    .delete(&ctx.http)
                    .await
                    .expect("Failed to delete Message");

                component
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .expect("Failed to create deferred interaction response");

                return;
            }
            "▶️" => {
                println!("Clicked next");
                println!("Index: {}", self.index);
                if self.index < page_count - 1 {
                    self.index += 1;
                }
            }
            "⏭️" => {
                self.index = self.pages.len() - 1;
            }
            _ => {
                println!(
                    "Unknown component interaction: {}. Somone probably spammed from some API's",
                    component.data.custom_id
                );
            }
        }

        component
            .clone()
            .message
            .edit(&ctx.http, |message| {
                message.set_embed(
                    self.pages[self.index]
                        .clone()
                        .footer(|footer| {
                            footer.text(format!("Page {} of {}", self.index + 1, page_count))
                        })
                        .clone(),
                )
            })
            .await
            .expect("Failed to edit message");

        component
            .create_interaction_response(ctx.http, |r| {
                r.kind(InteractionResponseType::DeferredUpdateMessage)
            })
            .await
            .expect("Failed to create deferred update message");
    }
}

fn button(name: &str, style: ButtonStyle, emoji: &str) -> CreateButton {
    CreateButton::default()
        .emoji(ReactionType::Unicode(emoji.to_string()))
        .label(name)
        .style(style)
        .custom_id(emoji)
        .to_owned()
}
