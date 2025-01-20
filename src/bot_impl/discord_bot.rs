use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::listen::Listen;
use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

struct Handler {
    unbounded_sender: UnboundedSender<UniMessage>,
    monitored_channel_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.channel_id == self.monitored_channel_id {
            let u_m = UniMessage {
                id: msg.id.to_string(),
                message: parse_message(&msg.content, &msg),
                author: msg.author.name,
                from_channel_id: msg.channel_id.get() as i64,
                to_channel_id: None,
                attachment_urls: msg
                    .attachments
                    .into_iter()
                    .map(|a| a.url.to_string())
                    .collect(),
            };

            if let Err(err) = self.unbounded_sender.send(u_m) {
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
    pub async fn new(
        bot_token: &str,
        unbounded_sender: UnboundedSender<UniMessage>,
        monitored_channel_id: u64,
    ) -> Self {
        let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler {
            unbounded_sender,
            monitored_channel_id,
        };
        let mut client = Client::builder(bot_token, intents)
            .event_handler(handler)
            .await
            .expect("Error creating client");

        client.start().await.unwrap();

        DiscordBot { client }
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
    parsed_content
}
