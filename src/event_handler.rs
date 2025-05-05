use serenity::all::{Command, CommandType, Context, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::EventHandler;
use log::{error, info};
use serenity::builder::CreateCommand;
use serenity::model::Permissions;
use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        Command::set_global_commands(&ctx.http, vec![
            commands::command_register::register(),
            CreateCommand::new("command_register").description("[bot管理用] コマンドを登録します")
                .default_member_permissions(Permissions::ADMINISTRATOR)
                .kind(CommandType::Message),
        ]).await.expect("Fatal: Register Global Command Failed");

        info!("bot Ready!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let mut ephemeral = false;
        if let Interaction::Command(command) = interaction {
            match command.data.kind {
                CommandType::ChatInput => {
                    let content = match command.data.name.as_str() {
                        "ping" => {
                            ephemeral = true;
                            Some(commands::ping::run(&command.data.options()))
                        },
                        "command_register" => {
                            commands::command_register::run(&ctx, &command).await.unwrap();
                            None
                        },
                        "dice" => Some(commands::dice::run(&command.data.options())),
                        "entry" => {
                            commands::entry::run(ctx.clone(), &command).await.unwrap();
                            None
                        },
                        "list_entry" => {
                            ephemeral = true;
                            Some(commands::list_entry::run(&command).await)
                        },
                        "cancel_entry" => {
                            ephemeral = true;
                            Some(commands::cancel_entry::run(&command.data.options()).await)
                        },
                        "reset_roles" => {
                            ephemeral = true;
                            Some(commands::reset_roles::run(ctx.clone(), &command).await)
                        },
                        _ => {
                            ephemeral = true;
                            Some("not implemented :(".to_string())
                        },
                    };

                    if let Some(content) = content {
                        let data = CreateInteractionResponseMessage::new().content(content).ephemeral(ephemeral);
                        let builder = CreateInteractionResponse::Message(data);
                        if let Err(why) = command.create_response(&ctx.http, builder).await {
                            error!("Error sending message: {:?}", why);
                        }
                    }
                }
                CommandType::Message => {

                }
                _ => {}
            }
        }
    }
}