use std::sync::Arc;
use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::send::SendMessage;
use async_trait::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Request, Requester};
use teloxide::types::{ChatId, ParseMode, Recipient};
use teloxide::Bot;
use tokio::sync::mpsc::UnboundedSender;
use crate::bot_impl::channel_id::ChannelId;
use crate::bot_traits::listen::Listen;
use crate::bot_traits::messenger_bot::MessengerBot;

pub struct TelegramBot {
    bot: Bot,
    sender: UnboundedSender<UniMessage>
}

impl TelegramBot {
    pub fn new(bot_token: String, sender: UnboundedSender<UniMessage>) -> Self {
        let bot = Bot::new(bot_token);
        TelegramBot { bot,  sender}
    }

    async fn inner_send(&self,channel_id: i64,  message_content: String, parse_mode: ParseMode) {
        match self
            .bot
            .send_message(
                Recipient::Id(ChatId(channel_id)),
                message_content,
            )
            .parse_mode(parse_mode)
            .send()
            .await
        {
            Ok(_) => {
                println!(
                    "Message sent successfully to chat ID {}.",
                    channel_id
                );
            }
            Err(err) => {
                eprintln!("Failed to send message: {:?}", err);
            }
        }
    }
}

#[async_trait]
impl SendMessage for TelegramBot {
    async fn send(&self, message: UniMessage) {
        if message.to_channel_id.is_none() {
            eprintln!("Target channel not specified!");
            return;
        }

        let messages = get_messages(&message);
        for m in messages {
            if let Some(ChannelId::I64(channel_id)) = message.to_channel_id.clone() {
                self.inner_send(channel_id, m.0, m.1).await;
            } else {
                eprintln!("Error: Expected ChannelId::I64 but got something else");
            }
        }
    }
}

#[async_trait]
impl Listen for TelegramBot {
    async fn listen(&mut self) {
        let sender = self.sender.clone(); // Clone the sender so it can be moved into the closure

        teloxide::repl(self.bot.clone(), move |_bot: Bot, msg: Message| {
            let sender = sender.clone(); // Clone the sender again for each message
            async move {
                sender
                    .send(create_uni_message(msg))
                    .expect("Failed to send message"); // Handle the message
                Ok(()) // Return the required result type
            }
        })
            .await;
    }
}



fn create_uni_message(msg: Message) -> UniMessage {
    UniMessage{
        message: msg.text().clone().unwrap().to_string(),
        to_channel_id: None,
        attachment_urls: Vec::new(),
        from_channel_id: ChannelId::I64(msg.chat.id.0),
        id: msg.id.0.to_string(),
        author: msg.from.unwrap().username.unwrap().to_string()
    }
}


impl MessengerBot for TelegramBot {

}

fn get_messages(message: &UniMessage) -> Vec<(String, ParseMode)> {
    let mut formatted_message = vec![];
    formatted_message.push((escape_markdown_v2(&format_message(&message.message, &message.author)), ParseMode::MarkdownV2));
    for a_u in message.attachment_urls.iter() {
        formatted_message.push((escape_markdown_v2(&format_message(a_u, &message.author)), ParseMode::MarkdownV2));
    }

    return formatted_message;
}

fn format_message(message: &str, author: &str) -> String {
    format!("*{}*:\n{}", author, message)
}

fn escape_markdown_v2(text: &str) -> String {
    let special_chars = r#"_[]()~`>#+-=|{}.!\"#;
    let mut escaped = String::new();

    for character in text.chars() {
        if special_chars.contains(character) {
            escaped.push('\\');
        }
        escaped.push(character);
    }

    return escaped;
}
