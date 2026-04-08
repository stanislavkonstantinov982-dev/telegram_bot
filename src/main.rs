use anyhow::Result;
use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use teloxide_core::types::{Update, UpdateKind};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    let bot_token = std::env::var("BOT_TOKEN")?;
    
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(bot_token);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    
    tracing::info!("Webhook server listening on 0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn webhook_handler(
    axum::extract::State(bot_token): axum::extract::State<String>,
    Json(update): Json<Update>,
) -> StatusCode {
    if let Err(e) = handle_update(bot_token, update).await {
        tracing::error!("Error handling update: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

async fn handle_update(bot_token: String, update: Update) -> Result<()> {
    if let UpdateKind::Message(message) = update.kind {
        let text = message.text().unwrap_or_default();
        let chat_id = message.chat.id.0;
        
        tracing::info!("Got message from {}: {}", chat_id, text);
        
        let client = reqwest::Client::new();
        let send_url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            bot_token
        );
        
        let payload = serde_json::json!({
            "chat_id": chat_id,
            "text": format!("Ты написал: {}", text)
        });
        
        client.post(&send_url)
            .json(&payload)
            .send()
            .await?;
    }
    
    Ok(())
}