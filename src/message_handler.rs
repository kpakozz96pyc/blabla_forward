use crate::bot_impl::uni_message::UniMessage;
use serde::Deserialize;
use crate::bot_impl::channel_id::ChannelId;
use crate::bot_traits::listen::Listen;

pub enum BridgeTarget {
    Discord,
    Telegram,
    Unknown
}

#[derive(Deserialize, Copy, Clone)]
pub struct DTBridge {
    pub from_channel_id: u64,
    pub to_channel_id: i64,
}

#[derive(Deserialize, Copy, Clone)]
pub struct TDBridge {
    pub from_channel_id: i64,
    pub to_channel_id: u64,
}

#[derive(Clone)]
pub struct UniBridge {
    pub from_channel_id: ChannelId,
    pub to_channel_id: ChannelId,
}
#[derive(Clone)]
pub struct MessageHandler {
    pub discord_channels: Vec<ChannelId>,
    pub telegram_channels: Vec<ChannelId>,
    pub bridges: Vec<UniBridge>
}

impl MessageHandler {
    pub fn handle_message(&self, message: UniMessage)-> Vec<(BridgeTarget, UniMessage)>
    {
        let mut messages = Vec::new();
        for bridge in self.bridges.iter() {
            if bridge.from_channel_id == message.from_channel_id {
                let mut msg = message.clone();
                msg.to_channel_id = Some(bridge.to_channel_id.clone());
                if msg.to_channel_id.is_some() {
                    messages.push((self.get_bridge_target(&bridge.to_channel_id), msg));
                }
            }
        }
        messages
    }

    fn get_bridge_target(&self, channel_id: &ChannelId) -> BridgeTarget {
        if self.discord_channels.contains(channel_id) {
            return BridgeTarget::Discord;
        } else if self.telegram_channels.contains(channel_id) {
            return BridgeTarget::Telegram;
        }
        return BridgeTarget::Unknown;
    }
}