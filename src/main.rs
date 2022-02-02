
use std::error::Error;

use config::MelConfig;
use database::crate_database;
use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::{
    async_trait,
    prelude::*,
    client::{Client},
    http::Http,
};

use songbird::SerenityInit;
use log::*;

struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

struct LavalinkHandler;

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

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        println!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
        println!("Track finished!\nGuild: {}", event.guild_id);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    println!("[BOT] Starting up...");

    crate_database().await.expect("[DB] Failed to start DB.");

    let http = Http::new_with_token(MelConfig::get_token().as_str());

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    let mut mel = Client::builder(MelConfig::get_token())
        .event_handler(events::Handler)
        .framework(commands::crete_framework())
        .register_songbird()
        .await
        .expect("[BOT] Failed to start.");

    let lava_client = LavalinkClient::builder(bot_id, MelConfig::get_token())
        .set_host("127.0.0.1")
        .set_password("root")
        .build(LavalinkHandler)
        .await?;

    {
        let mut data = mel.data.write().await;
        data.insert::<Lavalink>(lava_client);
    }

    let _ = mel
        .start()
        .await
        .map_err(|why| warn!("Client ended: {:?}", why));

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

    Ok(())
}
