use serde::Deserialize;
use std::fs;
use crate::message_handler::Bridge;

#[derive(Deserialize)]
pub struct Settings {
    pub telegram_bot_token: String,
    pub discord_bot_token: String,
    pub bridges: Vec<Bridge>,
}

impl Settings {
    pub fn new() -> Self {
        // Read `settings.json`
        let config_content =
            fs::read_to_string("settings.json").expect("Failed to read settings.json");

        // Parse JSON into the `Settings` struct
        serde_json::from_str(&config_content).expect("Failed to parse settings.json")
    }
}