use crate::bot_impl::channel_id::ChannelId;

#[derive(Clone)]
pub struct UniMessage
{
    pub id: String,
    pub message: String,
    pub author: String,
    pub from_channel_id: ChannelId,
    pub to_channel_id: Option<ChannelId>,
    pub attachment_urls: Vec<String>,
}