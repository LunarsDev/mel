use std::error::Error;

use config::MelConfig;
use database::crate_database;
use serenity::Client;

#[macro_use]
extern crate lazy_static;

pub mod apis;
mod commands;
pub mod components;
pub mod config;
pub mod database;
pub mod errors;
mod events;
pub mod utils;

type MelError = Box<dyn Error + Send + Sync + 'static>;
type MelResult<T> = Result<T, MelError>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("[BOT] Starting up...");

    crate_database().await.expect("[DB] Failed to start DB.");

    let mut mel = Client::builder(MelConfig::get_token())
        .event_handler(events::Handler)
        .framework(commands::crete_framework())
        .await
        .expect("[BOT] Failed to start.");

    let shard_manager = mel.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("[SHTDWN] Could not register ctrl+c handler");
        println!("[SHTDWN] Shutdown signal received (Ctrl-C) | Shutting down...");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(err) = mel.start().await {
        println!("[BOT] Error: {:?}", err);
    }
}
