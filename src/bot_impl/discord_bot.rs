use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::bot_impl::uni_message::UniMessage;
use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::builder::CreateMessage; // Necessary for message building.
use tokio::sync::mpsc::UnboundedSender;
use crate::bot_impl::channel_id::ChannelId;
use crate::bot_traits::listen::Listen;
use crate::bot_traits::messenger_bot::MessengerBot;
use crate::bot_traits::send::SendMessage;

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
    bot_token: String,
    client: Option<Client>,
    sender: UnboundedSender<UniMessage>
}

impl DiscordBot {
    pub fn new(
        bot_token: String,
        sender: UnboundedSender<UniMessage>
    ) -> Self {

        DiscordBot { sender, client:None, bot_token }
    }

    pub async fn start(&mut self){

        let handler = Handler {
            sender: self.sender.clone()
        };
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(self.bot_token.clone(), intents)
            .event_handler(handler)
            .await
            .expect("Error creating client");
        client.start().await.unwrap();

        self.client = Some(client);
    }
}

#[async_trait]
impl SendMessage for DiscordBot {
    async fn send(&self, message: UniMessage) {
        if let Some(client) = &self.client {
            let http = client.http.clone();

            // Match the ChannelId enum from UniMessage
            if let ChannelId::U64(channel_id) = message.to_channel_id {
                // Convert the u64 to a Serenity ChannelId
                let channel_id = channel_id;

                // Use http.create_message to send the message
                let _ = http
                    .send_message(ChannelId::from(channel_id), &serenity::builder::CreateMessage::new().content(content))
                    .await
                    .expect("Failed to send message");

            }
        }
    }
}




#[async_trait]
impl Listen for DiscordBot {

    async fn listen(&mut self) {
        if self.client.is_none() {
            self.start().await;
        }

        if let Some(client) = self.client.as_mut() {
            client
                .start()
                .await
                .unwrap_or_else(|e| panic!("Discord bot stopped: {}", e));
        } else {
            panic!("Client was not initialized.");
        }
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
