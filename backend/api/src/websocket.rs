use axum::{
    extract::{ws::WebSocketUpgrade, Extension, TypedHeader},
    response::IntoResponse,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::Utc;
use crate::state::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    #[serde(rename = "wallpaper_changed")]
    WallpaperChanged(WallpaperChangeEvent),

    #[serde(rename = "renderer_status")]
    RendererStatus(RendererStatusEvent),

    #[serde(rename = "download_progress")]
    DownloadProgress(DownloadProgressEvent),

    #[serde(rename = "error")]
    Error(ErrorEvent),

    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "pong")]
    Pong,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WallpaperChangeEvent {
    pub wallpaper_id: String,
    pub monitor: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RendererStatusEvent {
    pub renderer_type: String,
    pub status: String, // "started", "stopped", "error"
    pub path: Option<String>,
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadProgressEvent {
    pub image_id: String,
    pub progress: f32, // 0.0 to 1.0
    pub status: String, // "downloading", "complete", "error"
    pub speed: Option<String>, // "1.2 MB/s"
    pub eta: Option<String>, // "2m 30s"
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorEvent {
    pub message: String,
    pub code: Option<String>,
    pub timestamp: String,
}

#[derive(Clone, Debug)]
pub struct WsChannel {
    tx: broadcast::Sender<WsMessage>,
}

impl WsChannel {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.tx.subscribe()
    }

    pub fn send(&self, message: WsMessage) {
        let _ = self.tx.send(message);
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<AppState>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    if let Err(e) = _handle_socket(socket, state).await {
        tracing::warn!("WebSocket error: {:?}", e);
    }
}

async fn _handle_socket(mut socket: WebSocket, state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    // Send welcome message
    let welcome = WsMessage::Pong;
    socket.send(Message::Text(serde_json::to_string(&welcome)?)).await?;

    let mut rx = state.ws_channel.subscribe();

    loop {
        tokio::select! {
            // Handle incoming messages from client
            Some(msg) = socket.recv() => {
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(text) => {
                            if let Ok(client_msg) = serde_json::from_str::<WsMessage>(&text) {
                                match client_msg {
                                    WsMessage::Ping => {
                                        let pong = WsMessage::Pong;
                                        socket.send(Message::Text(serde_json::to_string(&pong)?)).await?;
                                    },
                                    _ => {
                                        // Handle other client messages if needed
                                        tracing::debug!("Received client message: {:?}", client_msg);
                                    }
                                }
                            } else {
                                tracing::warn!("Invalid WebSocket message: {}", text);
                            }
                        },
                        Message::Close(_) => {
                            return Ok(());
                        },
                        _ => {}
                    }
                } else {
                    // Client disconnected
                    return Ok(());
                }
            }

            // Forward broadcast messages to client
            Ok(msg) = rx.recv() => {
                let text = serde_json::to_string(&msg)?;
                socket.send(Message::Text(text)).await?;
            }
        }
    }
}
