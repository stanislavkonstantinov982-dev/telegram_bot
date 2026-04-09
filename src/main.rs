mod telegram;
mod handlers;

use anyhow::Result;
use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;

use telegram::client::TelegramClient;
use handlers::webhook::webhook_handler;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let bot_token = std::env::var("BOT_TOKEN")?;
    let tg = TelegramClient::new(bot_token);

    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(tg);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server started");

    axum::serve(listener, app).await?;

    Ok(())
}