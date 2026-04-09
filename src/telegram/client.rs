use anyhow::Result;
use serde_json::json;

#[derive(Clone)]
pub struct TelegramClient {
    client: reqwest::Client,
    bot_token: String,
}

impl TelegramClient {
    pub fn new(bot_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            bot_token,
        }
    }

    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let payload = json!({
            "chat_id": chat_id,
            "text": text
        });

        self.client.post(&url).json(&payload).send().await?;
        Ok(())
    }
}