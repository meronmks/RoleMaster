mod commands;
mod event_handler;
mod job_manager;
mod db_manager;

use std::env;
use std::fs::File;
use std::io::BufReader;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::all::GatewayIntents;
use serenity::Client;
use crate::db_manager::DbManager;
use crate::event_handler::Handler;
use crate::job_manager::JobManager;

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

#[tokio::main]
async fn main() {

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES;

    if cfg!(debug_assertions) {
        env::set_var("RUST_LOG", "info");
        env::set_var("DATABASE_URL", "sqlite:./database.db");
    }

    env_logger::init();
    info!("Starting bot...");
    JobManager::initialize().await.expect("Error: JobManager initialize fail");
    DbManager::initialize().await.expect("Error: db_manager initialize fail");
    let token = get_token("config.json").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    info!("Shutting down bot...");
}