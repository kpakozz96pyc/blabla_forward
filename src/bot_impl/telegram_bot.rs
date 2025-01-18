use teloxide::Bot;
use crate::bot_traits::connect::Connect;
use crate::bot_traits::listen::Listen;
use crate::bot_traits::send::Send;
use async_trait::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Request, Requester};
use teloxide::types::{ChatId, ParseMode, Recipient};

pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(bot_token: &str) -> Self {
        let bot = Bot::new(bot_token);
        TelegramBot { bot }
    }
}

impl Listen for TelegramBot {
    fn listen(&mut self){}
}

#[async_trait]
impl Send for TelegramBot {
    async fn send(&self, chat_id: i64, message: &str) {
        match self.bot.send_message(Recipient::Id(ChatId(chat_id)), message)
            .parse_mode(ParseMode::Html)
            .send()
            .await {
            Ok(_) => {
                println!("Message sent successfully to chat ID {}.", chat_id);
            }
            Err(err) => {
                eprintln!("Failed to send message: {:?}", err);
            }
        }
    }
}
