use async_trait::async_trait;
use crate::bot_impl::uni_message::UniMessage;

#[async_trait]
pub trait SendMessage {
    async fn send(&self, message: UniMessage);
}