use serenity::{
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    model::{
        application::{Command, CommandDataOptionValue, CommandOptionType, Interaction},
        gateway::Ready,
        id::{GuildId, UserId},
    },
    prelude::*,
};

pub struct Handler {
}

impl Handler {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "bmi" => {
                    "Implemented!".to_string()
                }
                //"target_user" | "target_regex" => {
                //    let mut builder = Target::builder();
                //    builder = builder.set_guild(command.guild_id.unwrap());
                //    for entry in &command.data.options {
                //        match entry.name.as_ref() {
                //            "emotes" => {
                //                if let CommandDataOptionValue::String(s) = &entry.value {
                //                    builder = builder.set_emotes(s)
                //                }
                //            }
                //            "duration" => {
                //                if let CommandDataOptionValue::Integer(int) = &entry.value {
                //                    builder = builder.set_expiration(*int as u64)
                //                }
                //            }
                //            "user" => {
                //                if let CommandDataOptionValue::User(user) = &entry.value {
                //                    builder = builder.set_user(*user)
                //                }
                //            }
                //            "regex" => {
                //                if let CommandDataOptionValue::String(s) = &entry.value {
                //                    builder = builder.set_regex(s)
                //                }
                //            }
                //            _ => println!("Unexpected entry name: {}", entry.name),
                //        }
                //    }
                //    match builder.build() {
                //        Ok(target) => {
                //            self.target(target);
                //            "Target added".to_string()
                //        }
                //        Err(TargetBuilderError::BadRegex(_)) => {
                //            "Your regex game is weak, bitch. Refer to \
                //            https://docs.rs/regex/latest/regex/index.html#syntax"
                //                .to_string()
                //        }
                //        Err(TargetBuilderError::MissingUserAndRegex) => {
                //            "Need either a user or a regex or both... bitch".to_string()
                //        }
                //        Err(e) => e.to_string(),
                //    }
                //}
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_response(&ctx.http, {
                    let response = CreateInteractionResponseMessage::new().content(content);
                    CreateInteractionResponse::Message(response)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Commands for all servers
        let mut commands = Vec::new();
        let command = CreateCommand::new("bmi")
            .description("Calculate a BMI")
            .add_option({
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "height",
                    "Height of person",
                )
                .required(true)
            })
            .add_option({
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "weight",
                    "Weight of person",
                )
                .required(true)
            });
        commands.push(command);

        Command::set_global_commands(&ctx.http, commands)
            .await
            .unwrap();
        println!("{} is connected!", ready.user.name);
    }
}
