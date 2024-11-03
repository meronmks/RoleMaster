use std::env;
use std::sync::Arc;
use serenity::all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::EventHandler;
use log::{error, info};
use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        Command::set_global_commands(&ctx.http, vec![
            commands::command_register::register(),
        ]).await.expect("Fatal: Register Global Command Failed");

        info!("bot Ready!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let mut ephemeral = false;
        if let Interaction::Command(command) = interaction {
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
    }
}