use crate::bot_impl::uni_message::UniMessage;
use crate::bot_traits::connect::Connect;
use crate::bot_traits::listen::Listen;
use crate::bot_traits::send::Send;
use async_trait::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Request, Requester};
use teloxide::types::{ChatId, ParseMode, Recipient};
use teloxide::Bot;

pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(bot_token: &str) -> Self {
        let bot = Bot::new(bot_token);
        TelegramBot { bot }
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
impl Send for TelegramBot {
    async fn send(&self, message: UniMessage) {
        if message.to_channel_id.is_none() {
            eprintln!("Target channel not specified!");
            return;
        }

        let messages = get_messages(&message);
        for m in messages {
            self.inner_send(message.to_channel_id.unwrap(), m.0, m.1).await;
        }
    }
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
    let special_chars = r#"_*[]()~`>#+-=|{}.!\"#;
    let mut escaped = String::new();

    for character in text.chars() {
        if special_chars.contains(character) {
            escaped.push('\\');
        }
        escaped.push(character);
    }

    return escaped;
}
