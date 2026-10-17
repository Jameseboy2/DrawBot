use axum::extract::Query;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Params {
    client_id: Option<String>,
}

#[derive(Deserialize)]
enum IncomingMessage {}

#[derive(Serialize)]
enum OutgoingMessage {
    SetClientId(String),
}

pub async fn handle(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<Params>,
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

            if let Err(error) = handle_socket(socket, params).await {
                tracing::error!("Aborted websocket with error: {}", error);
            }

            tracing::info!("Closed websocket for {addr}.");
        }
        .instrument(parent_span)
    })
}

async fn handle_socket(socket: WebSocket, params: Params) -> Result<(), Box<dyn Error>> {
    let (raw_sender, raw_receiver) = socket.split();
    let mut sender = AppSender::new(raw_sender);
    let mut receiver = AppReceiver::new(raw_receiver);

    let client_id = if let Some(client_id) = params.client_id {
        client_id
    } else {
        let new_id = Uuid::new_v4().to_string();

        sender
            .send(OutgoingMessage::SetClientId(new_id.clone()))
            .await?;

        new_id
    };

    // TODO: Implement the rest of the websocket logic (ideally once a basic web client is established).
    //  Below is example websocket code for reference.

    // if sender
    //     .send(Message::Ping(Bytes::from("health check")))
    //     .await
    //     .is_ok()
    // {
    //     tracing::info!("Pinged {addr}...");
    // } else {
    //     tracing::info!("Could not ping {addr}, aborting websocket connection.");
    //     return;
    // }

    // // receive single message from a client (we can either receive or send with socket).
    // // this will likely be the Pong for our Ping or a hello message from client.
    // // waiting for message from a client will block this task, but will not block other client's
    // // connections.
    // if let Some(msg) = receiver.next().await {
    //     if let Ok(msg) = msg {
    //         if process_message(msg, addr).is_break() {
    //             return;
    //         }
    //     } else {
    //         tracing::info!("client {addr} abruptly disconnected");
    //         return;
    //     }
    // }
    //
    // // Since each client gets an individual state machine, we can pause handling
    // // when necessary to wait for some external event (in this case illustrated by sleeping).
    // // Waiting for this client to finish getting its greetings does not prevent other clients from
    // // connecting to server and receiving their greetings.
    // for i in 1..5 {
    //     if sender
    //         .send(Message::Text(format!("Hi {i} times!").into()))
    //         .await
    //         .is_err()
    //     {
    //         tracing::info!("client {addr} abruptly disconnected");
    //         return;
    //     }
    //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    // }
    //
    // // Spawn a task that will push several messages to the client (does not matter what client does)
    // let mut send_task = tokio::spawn(async move {
    //     let n_msg = 20;
    //     for i in 0..n_msg {
    //         // In case of any websocket error, we exit.
    //         if sender
    //             .send(Message::Text(format!("Server message {i} ...").into()))
    //             .await
    //             .is_err()
    //         {
    //             return i;
    //         }
    //
    //         tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    //     }
    //
    //     tracing::info!("Sending close to {addr}...");
    //     if let Err(e) = sender
    //         .send(Message::Close(Some(CloseFrame {
    //             code: axum::extract::ws::close_code::NORMAL,
    //             reason: Utf8Bytes::from_static("Goodbye"),
    //         })))
    //         .await
    //     {
    //         tracing::info!("Could not send Close due to {e}, probably it is ok?");
    //     }
    //     n_msg
    // });
    //
    // // This second task will receive messages from client and print them on server console
    // let mut receive_task = tokio::spawn(async move {
    //     let mut count = 0;
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         count += 1;
    //         // print message and break if instructed to do so
    //         if process_message(msg, addr).is_break() {
    //             break;
    //         }
    //     }
    //     count
    // });
    //
    // // If any one of the tasks exit, abort the other.
    // tokio::select! {
    //     send_result = &mut send_task => {
    //         match send_result {
    //             Ok(payload) => tracing::info!("{payload} messages sent to {addr}"),
    //             Err(payload) => tracing::info!("Error sending messages: {payload:?}")
    //         }
    //         receive_task.abort();
    //     },
    //     receive_result = &mut receive_task => {
    //         match receive_result {
    //             Ok(payload) => tracing::info!("Received {payload} messages"),
    //             Err(payload) => tracing::info!("Error receiving messages: {payload:?}")
    //         }
    //         send_task.abort();
    //     }
    // }

    Ok(())
}

struct AppSender {
    inner: futures_util::stream::SplitSink<WebSocket, Message>,
}

struct AppReceiver {
    inner: futures_util::stream::SplitStream<WebSocket>,
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

impl AppReceiver {
    fn new(inner: futures_util::stream::SplitStream<WebSocket>) -> Self {
        Self { inner }
    }

    pub async fn next(&mut self) -> Option<Result<IncomingMessage, String>> {
        if let Some(result) = self.inner.next().await {
            match result {
                Ok(Message::Text(text)) => match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(cmd) => Some(Ok(cmd)),
                    Err(err) => Some(Err(format!("Invalid JSON: {err}"))),
                },
                Err(e) => Some(Err(e.to_string())),
                Ok(message) => {
                    tracing::warn!("Ignoring non-text websocket message: {message:?}");
                    None
                }
            }
        } else {
            None
        }
    }
}
