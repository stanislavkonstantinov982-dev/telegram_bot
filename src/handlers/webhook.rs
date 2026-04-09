use anyhow::Result;
use axum::{extract::Json, http::StatusCode};
use teloxide_core::types::{Update, UpdateKind};

use crate::telegram::client::TelegramClient;
use super::commands::handle_command;

pub async fn webhook_handler(
    axum::extract::State(tg): axum::extract::State<TelegramClient>,
    Json(update): Json<Update>,
) -> StatusCode {
    if let Err(e) = handle_update(&tg, update).await {
        tracing::error!("Error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

async fn handle_update(tg: &TelegramClient, update: Update) -> Result<()> {
    if let UpdateKind::Message(message) = update.kind {
        let text = message.text().unwrap_or_default();
        let chat_id = message.chat.id.0;

        tracing::info!("Message: {}", text);

        handle_command(tg, chat_id, text).await?;
    }

    Ok(())
}