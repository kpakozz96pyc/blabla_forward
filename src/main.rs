use crate::bot_impl::discord_bot::DiscordBot;
use crate::bot_impl::telegram_bot::TelegramBot;
use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::send::SendMessage;
use crate::message_handler::{MessageHandler};
use std::sync::Arc;
use tokio::sync::mpsc;

mod bot_impl;
mod bot_traits;
mod message_handler;
mod settings;

#[tokio::main]
async fn main() {
    println!("BlaBLa version 0.1.1");
    let settings = settings::Settings::new();
    let (tx, mut rx) = mpsc::unbounded_channel::<UniMessage>();

    let shared_tx = Arc::new(tx);

    let message_handler = MessageHandler {
        bridges: settings.bridges.clone(),
        sender: Arc::clone(&shared_tx),
    };

    let telegram_bot = create_telegram_bot(&settings.telegram_bot_token)
        .await
        .expect("Failed to create Telegram bot");

    let telegram_bot_clone = Arc::clone(&telegram_bot);

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _ = telegram_bot_clone.send(message).await;
        }
    });

    let discord_bot =
        create_discord_bot(&settings.discord_bot_token, Arc::new(message_handler));

    discord_bot.await.expect("Failed to initialize Discord bot");
}

async fn create_telegram_bot(token: &str) -> Result<Arc<TelegramBot>, &'static str> {
    for attempt in 1..=3 {
        let telegram_bot = TelegramBot::new(token);
        println!("Telegram bot created successfully on attempt {attempt}");
        return Ok(Arc::new(telegram_bot));
    }
    Err("Failed to create Telegram bot after 3 attempts.")
}

async fn create_discord_bot(
    token: &str,
    handler: Arc<MessageHandler>,
) -> Result<DiscordBot, &'static str> {
    for _ in 1..=3 {
        // Attempt to create a Discord bot
        let discord_bot = DiscordBot::new(token, handler).await;
        return Ok(discord_bot);
    }
    Err("Failed to create Discord bot after 3 attempts.")
}
