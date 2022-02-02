#[allow(unused_imports)]
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

use crate::utils::constants::colors;

#[group]
#[commands(play)]
#[description("Voice 🎶 - Music/Voice related commands")]
pub struct Voice;

#[command("play")]
#[description("Play a song :>")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {


    Ok(())
}