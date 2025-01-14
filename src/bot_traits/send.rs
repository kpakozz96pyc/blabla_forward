use async_trait::async_trait;

#[async_trait]
pub trait Send {
    async fn send(&self, chat_id: i64, message: &str);
}