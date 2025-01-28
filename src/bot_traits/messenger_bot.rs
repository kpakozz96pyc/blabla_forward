use crate::bot_traits::listen::Listen;
use crate::bot_traits::send::SendMessage;

pub trait MessengerBot: SendMessage + Listen + Send + Sync
{}