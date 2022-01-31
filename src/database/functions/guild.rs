use mysql::{params, prelude::Queryable};
use serenity::{
    framework::standard::{CommandError, CommandResult},
    model::guild::Guild,
};

use crate::database::{
    get_database_connection,
    models::guild::{DbGuild, DbGuildType},
};

pub async fn register_guild(guild: Guild) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"INSERT IGNORE INTO guilds (discord_id, name) VALUES (:discord_id, :name)",
        params! {
            "discord_id" => guild.id.to_string(),
            "name" => guild.name
        },
    )?;
    
    Ok(())
}

pub async fn get_db_guild(guild: Guild) -> Result<DbGuild, CommandError> {
    let mut conn = get_database_connection().await?;

    let mut result: Vec<DbGuild> = conn.exec_map(
        r"
        SELECT * FROM guilds g
        WHERE g.discord_id = :discord_id
        LIMIT 1
    ",
        params! {
            "discord_id" => guild.id.to_string()
        },
        |(discord_id, name, prefix, guild_type)| DbGuild {
            discord_id,
            name,
            prefix,
            guild_type,
        },
    )?;

    if let Some(db_guild) = result.pop() {
        Ok(db_guild)
    } else {
        Err("[DB] Guild not registered.".into())
    }
}

pub async fn set_prefix(guild: Guild, new_prefix: &str) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"
        UPDATE guilds
        SET prefix = :prefix
        WHERE discord_id = :discord_id 
    ",
        params! {
            "discord_id" => guild.id.to_string(),
            "prefix" => new_prefix
        },
    )?;

    let rows = conn.affected_rows();

    if rows == 1 {
        println!("[DB] Guild prefix set ({})", guild.id.to_string());
        Ok(())
    } else {
        Err("[DB] Failed to update prefix in DB.".into())
    }
}

pub async fn set_special(guild: Guild, new_type: DbGuildType) -> CommandResult {
    let mut conn = get_database_connection().await?;

    conn.exec_drop(
        r"
        UPDATE guilds
        SET guild_type = :guild_type
        WHERE discord_id = :discord_id 
    ",
        params! {
            "discord_id" => guild.id.to_string(),
            "guild_type" => u32::from(new_type).to_string()
        },
    )?;

    let rows = conn.affected_rows();

    if rows == 1 {
        println!("[DB] Guild type updated ({})", guild.id.to_string());
        Ok(())
    } else {
        Err("[DB] Failed to update typo in DB.".into())
    }
}