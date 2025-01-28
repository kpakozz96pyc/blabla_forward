use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::bot_impl::uni_message::UniMessage;
use std::sync::Arc;
use serde::Deserialize;
use crate::bot_impl::channel_id::ChannelId;
use crate::bot_traits::send::SendMessage;

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

pub struct MessageHandler {
    pub publisher: Arc<UnboundedSender<UniMessage>>,
    pub bridges: Vec<UniBridge>,
    pub subscriber: Arc<UnboundedReceiver<UniMessage>>,
    pub bots : Vec<Arc<(ChannelId, dyn SendMessage)>>
}

impl MessageHandler {
    pub fn handle_message(&self, message: UniMessage)
    {
        for bridge in self.bridges.iter() {
            if bridge.from_channel_id == message.from_channel_id {
                let mut m = message.clone();
                m.to_channel_id = Some(bridge.to_channel_id.clone());
                if let Err(err) = self.publisher.send(m) {
                    eprintln!("Failed to send message: {:?}", err);
                }
            }
        }
    }

    pub fn handle_message_from_bot(&self, message: UniMessage){
        self.bots.
    }
}