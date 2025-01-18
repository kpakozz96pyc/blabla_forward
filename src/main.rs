use std::sync::Arc;
use serenity::futures::{SinkExt, TryFutureExt};
use tokio::sync::mpsc;
use crate::bot_impl::discord_bot::DiscordBot;
use crate::bot_impl::telegram_bot::TelegramBot;
use crate::bot_traits::send::Send;

mod bot_traits;
mod bot_impl;
mod settings;

#[tokio::main]
async fn main() {
    println!("BlaBLa version 0.1.0");
    let settings = settings::Settings::new();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let telegram_bot = create_telegram_bot(&settings.telegram_bot_token)
        .await.expect("Failed to create Telegram bot");

    let telegram_bot_clone = Arc::clone(&telegram_bot);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _ = telegram_bot_clone.send(settings.telegram_chat_id, &message).await;
        }
    });

    let mut discord_bot = create_discord_bot(&settings.discord_bot_token, tx, settings.discord_channel_id);
    // Initialize the Discord bot
    discord_bot.await.expect("Failed to initialize Discord bot");
}

async fn create_telegram_bot(token: &str) -> Result<Arc<TelegramBot>, &'static str> {
    for attempt in 1..=3 {
        // Attempt to create the Telegram bot
        let telegram_bot = TelegramBot::new(token);
        println!("Telegram bot created successfully on attempt {attempt}");
        return Ok(Arc::new(telegram_bot));
    }
    Err("Failed to create Telegram bot after 3 attempts.")
}

async fn create_discord_bot(
    token: &str,
    tx: mpsc::UnboundedSender<String>,
    channel_id: u64,
) -> Result<DiscordBot, &'static str> {
    for attempt in 1..=3 {
        // Attempt to create a Discord bot
        let discord_bot = DiscordBot::new(token, tx.clone(), channel_id).await;
        println!("Discord bot created successfully on attempt {attempt}");
        return Ok(discord_bot);
    }
    Err("Failed to create Discord bot after 3 attempts.")
}