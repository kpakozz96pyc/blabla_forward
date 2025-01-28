use async_trait::async_trait;

// A generic BotListener trait
#[async_trait]
pub trait Listen {
    // Define a method to start listening for events/messages
    async fn listen(&self, ) -> Result<(), String>;
}