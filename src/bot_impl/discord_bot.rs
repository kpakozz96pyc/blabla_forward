use std::sync::Arc;
use crate::bot_impl::uni_message::UniMessage;
use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use crate::bot_impl::channel_id::ChannelId;
use crate::bot_traits::listen::Listen;
use crate::bot_traits::messenger_bot::MessengerBot;
use crate::bot_traits::send::SendMessage;
use crate::message_handler::MessageHandler;

struct Handler {
    sender: UnboundedSender<UniMessage>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
         let u_m = UniMessage {
             id: msg.id.to_string(),
             message: parse_message(&msg.content, &msg),
             author: msg.author.name,
             from_channel_id: ChannelId::U64(u64::from(msg.channel_id)),
             to_channel_id: None,
             attachment_urls: msg
                 .attachments
                 .into_iter()
                 .map(|a| a.url.to_string())
                 .collect(),
         };

        if let Err(err) = self.sender.send(u_m) {
            eprintln!("Discord failed to send message: {:?}", err);
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
    pub async fn new(
        bot_token: String,
        handler: UnboundedSender<UniMessage>
    ) -> Self {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler {
            sender: handler
        };
        let mut client = Client::builder(bot_token, intents)
            .event_handler(handler)
            .await
            .expect("Error creating client");

        client.start().await.unwrap();

        DiscordBot { client }
    }
}

#[async_trait]
impl SendMessage for DiscordBot {
    async fn send(&self, message: UniMessage) {
        todo!()
    }
}

#[async_trait]
impl Listen for DiscordBot {

    async fn listen(&mut self) {
        self.client
            .start()
            .await
            .map_err(|e| format!("Discord bot stopped: {}", e)).expect("TODO: panic message");
    }
}

impl MessengerBot for DiscordBot {

}



fn parse_message(content: &str, msg: &Message) -> String {
    // Clone the content so the string can be mutated
    let mut parsed_content = content.to_string();

    // Iterate over all mentions contained in the message
    for mention in &msg.mentions {
        // Replace the mention format `<@USER_ID>` or `<@!USER_ID>` with the username
        let mention_pattern = format!("<@{}>", mention.id);
        parsed_content = parsed_content.replace(&mention_pattern, &mention.name);

        // Handle `<@!USER_ID>` format (nickname mentions)
        let nickname_pattern = format!("<@!{}>", mention.id);
        parsed_content = parsed_content.replace(&nickname_pattern, &mention.name);
    }

    // Return the parsed content with mentions replaced
    return parsed_content;
}
