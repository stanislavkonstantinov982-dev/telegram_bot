use anyhow::Result;
use crate::telegram::client::TelegramClient;

pub async fn handle_command(
    tg: &TelegramClient,
    chat_id: i64,
    text: &str,
) -> Result<()> {
    if text.starts_with("/start") {
        tg.send_message(chat_id, "Привет! Я бот для игры Кто хочет стать миллионером?.").await?;
    }

    if text.starts_with("/newgame") {
        tg.send_message(chat_id, "Игра создана!").await?;
    }

    Ok(())
}