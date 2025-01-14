use std::sync::Arc;
use serenity::futures::TryFutureExt;
use tokio::sync::mpsc;
use crate::bot_impl::discord_bot::DiscordBot;
use crate::bot_impl::telegram_bot::TelegramBot;
use crate::bot_traits::send::Send;
use dotenv::dotenv;
use std::env;

mod bot_traits;
mod bot_impl;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN is not set in .env");
    let telegram_bot = Arc::new(TelegramBot::new(&telegram_bot_token));


    let telegram_chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID is not set in .env")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid i64");

    let telegram_bot_clone = Arc::clone(&telegram_bot);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            // Send the message to Telegram chat
            let _ = telegram_bot_clone.send(telegram_chat_id, &message).await;
        }
    });

    let discord_bot_token = env::var("DISCORD_BOT_TOKEN")
        .expect("DISCORD_BOT_TOKEN is not set in .env");

    let discord_channel_id: u64 = env::var("DISCORD_CHANNEL_ID")
        .expect("DISCORD_CHANNEL_ID is not set in .env")
        .parse()
        .expect("DISCORD_CHANNEL_ID must be a valid u64");

    // Initialize the Discord bot
    DiscordBot::new(&discord_bot_token, tx, discord_channel_id).await;
}