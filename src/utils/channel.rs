use serenity::{client::Context, model::channel::Channel};

use crate::MelResult;

pub async fn get_channel_from_id(ctx: &Context, id: u64) -> MelResult<Channel> {
    match ctx.cache.channel(id).await {
        Some(ch) => Ok(ch),
        None => Ok(ctx.http.get_channel(id).await?),
    }
}