use std::sync::Arc;
use crate::bot_impl::uni_message::UniMessage;
use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::bot_traits::listen::Listen;
use crate::message_handler::MessageHandler;

struct Handler {
    handler: Arc<MessageHandler>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
         let u_m = UniMessage {
             id: msg.id.to_string(),
             message: parse_message(&msg.content, &msg),
             author: msg.author.name,
             from_channel_id: msg.channel_id.get(),
             to_channel_id: None,
             attachment_urls: msg
                 .attachments
                 .into_iter()
                 .map(|a| a.url.to_string())
                 .collect(),
         };

        self.handler.handle_message(u_m);
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
        bot_token: &str,
        handler: Arc<MessageHandler>
    ) -> Self {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler {
            handler
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
impl Listen for DiscordBot {
    async fn listen(&mut self) -> Result<(), String> {
        self.client
            .start()
            .await
            .map_err(|e| format!("Discord bot stopped: {}", e))
    }
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
