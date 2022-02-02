use std::env::var;

use serenity::model::id::UserId;

pub struct MelConfig;

// DB_NAME="mel"
// DB_HOST="localhost"
// DB_PASS="root"
// DB_USER="root"


impl MelConfig {
    pub fn get_token() -> String {
        var("BOT_TOKEN").expect("[CFG] Failed to get bot token.")
    }

    pub fn get_database_name() -> String {
        var("DB_NAME").expect("[CFG] Failed to get DB Name")
    }

    pub fn get_database_host() -> String {
        var("DB_HOST").expect("[CFG] Failed to get DB Host")
    }

    pub fn get_database_pass() -> String {
        var("DB_PASS").expect("[CFG] Failed to get DB Password")
    }

    pub fn get_database_user() -> String {
        var("DB_USER").expect("[CFG] Failed to get DB Username")
    }

    pub fn get_default_prefix() -> String {
        var("BOT_PREFIX").unwrap_or_else(|_| "f!".to_string())
    }

    pub fn get_id_mention() -> UserId {
        UserId(
            var("BOT_ID")
                .expect("[CFG] Failed to get ID")
                .parse::<u64>()
                .expect("[CFG] Failed to convert ID to u64"),
        )
    }
}