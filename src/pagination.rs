use serenity::{
    builder::{CreateButton, CreateEmbed},
    model::application::interaction::application_command::ApplicationCommandInteraction,
    model::{
        prelude::{component::ButtonStyle, interaction::Interaction, ReactionType},
        user::User,
    },
    prelude::Context,
};

#[derive(Debug)]
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
            .create_interaction_response(&ctx.http, |response| {
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

    pub fn handle_interaction(&self, _ctx: &Context, interaction: Interaction) {
        if let Interaction::MessageComponent(component) = interaction {
            match component.data.custom_id.as_str() {
                "⏮️" => {
                    println!("FIRST")
                }
                _ => {
                    println!("Unknown interaction: {}", component.data.custom_id);
                }
            }
        }
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
