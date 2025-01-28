use crate::bot_impl::discord_bot::DiscordBot;
use crate::bot_impl::telegram_bot::TelegramBot;
use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::send::SendMessage;
use crate::message_handler::{BridgeTarget, MessageHandler, UniBridge};
use std::sync::Arc;
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


    let (tg_tx, mut tg_rx) = mpsc::unbounded_channel::<UniMessage>();
    let (ds_tx, mut ds_rx) = mpsc::unbounded_channel::<UniMessage>();

    let mut discord_bot =
        DiscordBot::new(settings.discord_bot_token.clone(), ds_tx).await;
    let mut telegram_bot =
        TelegramBot::new(settings.telegram_bot_token.clone(), tg_tx);

    discord_bot.listen().await;


    // Share the bots and message handler across spawned tasks
    let message_handler = Arc::new(Mutex::new(MessageHandler {
        discord_channels: settings
            .td_bridges
            .iter()
            .map(|bridge| ChannelId::U64(bridge.to_channel_id.clone())) // Extract the `discord_channel`
            .collect(),
        telegram_channels: settings
            .dt_bridges
            .iter()
            .map(|bridge| ChannelId::I64(bridge.to_channel_id.clone())) // Extract the `discord_channel`
            .collect(),
        bridges: get_uni_bridges(&settings),
    }));


    spawn(async move {
        while let Some(message) = ds_rx.recv().await {
            println!("{}", message.message)
        }
    });


    spawn(async move {
        while let Some(message) = tg_rx.recv().await {
            println!("{}", message.message)
        }
    });

}

async fn try_send(
    discord_bot: &Arc<Mutex<DiscordBot>>,
    telegram_bot: &Arc<Mutex<TelegramBot>>,
    messages: Vec<(BridgeTarget, UniMessage)>,
) {
    for m in messages {
        match m.0 {
            BridgeTarget::Discord => {
                discord_bot.lock().await.send(m.1).await;
            }
            BridgeTarget::Telegram => {
                telegram_bot.lock().await.send(m.1).await;
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
