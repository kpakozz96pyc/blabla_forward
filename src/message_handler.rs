use tokio::sync::mpsc::{UnboundedSender};
use crate::bot_traits::send::SendMessage;
use crate::bot_impl::uni_message::UniMessage;
use std::sync::Arc;

pub struct Bridge {
    pub from_channel_id: u64,
    pub to_channel_id: i64,
}

pub struct MessageHandler {
    pub bus: Arc<UnboundedSender<UniMessage>>,
    pub bridges: Vec<Bridge>
}

impl MessageHandler {

    fn new (bus: Arc<UnboundedSender<UniMessage>>, bridges: Vec<Bridge>) -> Self {
        Self {
            bus,
            bridges
        }
    }

    pub fn handle_message(&self, message: UniMessage)
    {
        for bridge in self.bridges.iter() {
            if (bridge.from_channel_id == message.from_channel_id) {
                let mut m = message.clone();
                m.to_channel_id = Some(bridge.to_channel_id);
                if let Err(err) = self.bus.send(m) {
                    eprintln!("Failed to send message: {:?}", err);
                }
            }
        }
    }
}