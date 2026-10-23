use crate::app_state::{AppState, BotStatus};
use axum::extract::State;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::Serialize;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::Instrument;

#[derive(Serialize)]
enum OutgoingMessage {
    UpdateBotStatus(BotStatus),
}

pub async fn handle(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown client")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");

    let parent_span = tracing::Span::current();

    ws.on_upgrade(move |socket| {
        async move {
            tracing::info!("Opening websocket for {addr}.");

            if let Err(error) = handle_socket(socket, app_state).await {
                tracing::error!("Aborted websocket with error: {}", error);
            }

            tracing::info!("Closed websocket for {addr}.");
        }
        .instrument(parent_span)
    })
}

async fn handle_socket(socket: WebSocket, app_state: Arc<AppState>) -> Result<(), Box<dyn Error>> {
    let (raw_sender, mut raw_receiver) = socket.split();
    let mut sender = AppSender::new(raw_sender);
    // let mut receiver = AppReceiver::new(raw_receiver);

    let mut rx_bot_status = app_state.tx_bot_status.subscribe();

    sender
        .send(OutgoingMessage::UpdateBotStatus(
            app_state.bot_status().await,
        ))
        .await?;

    loop {
        tokio::select! {
            Ok(bot_status) = rx_bot_status.recv() => {
                tracing::info!("Sending bot status update over socket ({:?})", bot_status);
                sender.send(OutgoingMessage::UpdateBotStatus(bot_status)).await?;
            },

            Some(message) = raw_receiver.next() => {
                if let Message::Close(_) = message? {
                    break;
                }
            }
        }
    }

    Ok(())
}

struct AppSender {
    inner: futures_util::stream::SplitSink<WebSocket, Message>,
}

impl AppSender {
    fn new(inner: futures_util::stream::SplitSink<WebSocket, Message>) -> Self {
        Self { inner }
    }

    async fn send(&mut self, msg: OutgoingMessage) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&msg)?;
        self.inner
            .send(Message::Text(Utf8Bytes::from(json)))
            .await?;

        Ok(())
    }
}

// struct AppReceiver {
//     inner: futures_util::stream::SplitStream<WebSocket>,
// }
//
// impl AppReceiver {
//     fn new(inner: futures_util::stream::SplitStream<WebSocket>) -> Self {
//         Self { inner }
//     }
//
//     pub async fn next(&mut self) -> Option<Result<IncomingMessage, String>> {
//         if let Some(result) = self.inner.next().await {
//             match result {
//                 Ok(Message::Text(text)) => match serde_json::from_str::<IncomingMessage>(&text) {
//                     Ok(cmd) => Some(Ok(cmd)),
//                     Err(err) => Some(Err(format!("Invalid JSON: {err}"))),
//                 },
//                 Err(e) => Some(Err(e.to_string())),
//                 Ok(message) => {
//                     tracing::warn!("Ignoring non-text websocket message: {message:?}");
//                     None
//                 }
//             }
//         } else {
//             None
//         }
//     }
// }
