#[derive(Debug, Clone)]
pub struct UniMessage
{
    pub id: String,
    pub message: String,
    pub author: String,
    pub from_channel_id: u64,
    pub to_channel_id: Option<i64>,
    pub attachment_urls: Vec<String>,
}