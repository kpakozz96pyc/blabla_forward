use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::Client;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use crate::bot_traits::listen::Listen;

struct Handler{
    unbounded_sender: UnboundedSender<String>,
    monitored_channel_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.channel_id == self.monitored_channel_id {
            println!("Message received in monitored Discord channel: {}", msg.content);

            if let Err(err) = self.unbounded_sender.send(msg.content.clone()) {
                eprintln!("Error forwarding message: {:?}", err);
            }
        }
    }

    // Called when the bot is ready
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot is connected as {}", ready.user.name);
    }
}

pub struct DiscordBot {
    client: Client,
}

impl DiscordBot {
    pub async fn new(bot_token: &str, unbounded_sender: UnboundedSender<String>, monitored_channel_id: u64) -> Self {

        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler {unbounded_sender, monitored_channel_id};
        let mut client = Client::builder(bot_token, intents)
            .event_handler(handler)
            .await
            .expect("Error creating client");

        client.start().await.unwrap();

        DiscordBot { client }
    }
}
