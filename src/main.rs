use crate::bot_impl::discord_bot::DiscordBot;
use crate::bot_impl::telegram_bot::TelegramBot;
use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::send::SendMessage;
use crate::message_handler::{BridgeTarget, MessageHandler, UniBridge};
use std::sync::Arc;
use serenity::futures::SinkExt;
use tokio::spawn;
use crate::bot_impl::channel_id::ChannelId;
use tokio::sync::{mpsc, Mutex};
use crate::bot_traits::listen::Listen;
use crate::bot_traits::messenger_bot::MessengerBot;

mod bot_impl;
mod bot_traits;
mod message_handler;
mod settings;

#[tokio::main]
async fn main() {
    println!("BlaBLa version 0.1.1");

    let settings = settings::Settings::new();

    let (sender, mut receiver) = mpsc::unbounded_channel::<UniMessage>();

    // Wrap the bots with Arc<Mutex<>> for shared ownership and thread-safe mutability
    let discord_bot = Arc::new(Mutex::new(
        DiscordBot::new(settings.discord_bot_token.clone(), sender.clone()),
    ));
    let telegram_bot = Arc::new(Mutex::new(
        TelegramBot::new(settings.telegram_bot_token.clone(), sender.clone()),
    ));

    // Share the bots and message handler across spawned tasks
    let message_handler = MessageHandler {
        discord_channels: settings
            .td_bridges
            .iter()
            .map(|bridge| ChannelId::U64(bridge.to_channel_id.clone())) // Extract the `discord_channel`
            .collect(),
        telegram_channels: settings
            .dt_bridges
            .iter()
            .map(|bridge| ChannelId::I64(bridge.to_channel_id.clone())) // Extract the `telegram_channel`
            .collect(),
        bridges: get_uni_bridges(&settings),
    };

    // Spawn a task for the Telegram bot's `listen` method
    {
        let telegram_bot = Arc::clone(&telegram_bot);
        spawn(async move {
            // Lock the Mutex for mutable access
            let mut telegram_bot = telegram_bot.lock().await;
            telegram_bot.listen().await;
        });
    }

    // Spawn a task for the Discord bot's `listen` method
    {
        let discord_bot = Arc::clone(&discord_bot);
        spawn(async move {
            // Lock the Mutex for mutable access
            let mut discord_bot = discord_bot.lock().await;
            discord_bot.start().await;
            discord_bot.listen().await;
        });
    }
    {
        spawn(async move {
            while let Some(message) = receiver.recv().await {
                let messages = message_handler.handle_message(message);

                // Call try_send with cloned Arc references
                try_send(discord_bot.clone(), telegram_bot.clone(), messages).await;
            }
        });
    }

    // Main loop to keep the program alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn try_send(
    discord_bot: Arc<Mutex<DiscordBot>>,
    telegram_bot: Arc<Mutex<TelegramBot>>,
    messages: Vec<(BridgeTarget, UniMessage)>,
) {
    for m in messages {
        match m.0 {
            BridgeTarget::Discord => {
                // Lock the Discord bot before sending
                let mut discord_bot = discord_bot.lock().await;
                discord_bot.send(m.1).await;
            }
            BridgeTarget::Telegram => {
                // Lock the Telegram bot before sending
                let mut telegram_bot = telegram_bot.lock().await;
                telegram_bot.send(m.1).await;
            }
            BridgeTarget::Unknown => {
                println!("Unknown bridge target");
            }
        }
    }
}

fn get_uni_bridges(settings: &settings::Settings) -> Vec<UniBridge> {
    let mut unibridges = Vec::new();
    settings.dt_bridges.iter().for_each(|dt_bridge| {
        unibridges.push(UniBridge {
            to_channel_id: ChannelId::I64(dt_bridge.to_channel_id),
            from_channel_id: ChannelId::U64(dt_bridge.from_channel_id)
        });
    });

    settings.td_bridges.iter().for_each(|td_bridge| {
        unibridges.push(UniBridge {
            to_channel_id: ChannelId::U64(td_bridge.to_channel_id),
            from_channel_id: ChannelId::I64(td_bridge.from_channel_id)
        });
    });

    return unibridges;
}