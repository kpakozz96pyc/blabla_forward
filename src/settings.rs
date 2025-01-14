use std::env;
use dotenv::dotenv;

pub struct Settings {
    pub telegram_bot_token: String,
    pub discord_bot_token: String,
    pub telegram_chat_id: i64,
    pub discord_channel_id: u64,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN is not set in .env"),

            discord_bot_token: env::var("DISCORD_BOT_TOKEN")
                .expect("DISCORD_BOT_TOKEN is not set in .env"),

            telegram_chat_id: env::var("TELEGRAM_CHAT_ID")
                .expect("TELEGRAM_CHAT_ID is not set in .env")
                .parse()
                .expect("TELEGRAM_CHAT_ID must be a valid i64"),

            discord_channel_id: env::var("DISCORD_CHANNEL_ID")
                .expect("DISCORD_CHANNEL_ID is not set in .env")
                .parse()
                .expect("DISCORD_CHANNEL_ID must be a valid u64"),
        }
    }
}