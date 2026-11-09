pub mod vectorize;

use crate::app_state::bot_status::BotStatus;
use crate::app_state::{AppState, PendingJob};
use axum::extract::multipart::MultipartError;
use axum::extract::{ConnectInfo, Multipart, State};
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn handle(
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    multipart: Multipart, // uses the default max file size of 2MB
) -> (StatusCode, String) {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown client")
    };
    tracing::info!("Serving `{user_agent}` at {addr}.");

    // Get the lock on the bot status manager until we've finished setting up the pending job.
    // NOTE: If it ends up taking a long time to process the image data, we can add a new
    // JobProcessing (and AwaitingResetWithJobProcessing) status to avoid having to keep the lock
    // for so long.
    let mut bot_status_manager = app_state.bot_status_manager().lock().await;
    let initial_bot_status = bot_status_manager.status();
    if initial_bot_status != BotStatus::Available && initial_bot_status != BotStatus::AwaitingReset
    {
        return (StatusCode::BAD_REQUEST, "Bot unavailable".to_owned());
    }

    let image_data = match extract_image_data(multipart).await {
        Ok(data) => data,
        Err(ExtractImageDataError::MultipartError(err)) => {
            tracing::error!("Multipart Error: {err}");
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
        }
        Err(ExtractImageDataError::NoImage) => {
            return (
                StatusCode::BAD_REQUEST,
                "Request contained no image or image in invalid format".to_owned(),
            );
        }
    };

    match vectorize::vectorize_image(&&*image_data) {
        Err(error) => {
            let error_message = format!("Failed to vectorize image: {}", error.message);
            tracing::error!(error_message);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message)
        }
        Ok(command_string) => {
            app_state.add_pending_job(PendingJob(command_string)).await;

            match initial_bot_status {
                BotStatus::Available => bot_status_manager.set_status(BotStatus::JobPending),
                BotStatus::AwaitingReset => {
                    bot_status_manager.set_status(BotStatus::AwaitingResetWithJobPending);
                }
                _ => {
                    tracing::error!(
                        "Unexpected bot status after adding pending job: {:?}",
                        bot_status_manager.status()
                    );
                }
            }
            drop(bot_status_manager);

            (StatusCode::CREATED, "Pending job created".to_owned())
        }
    }
}

enum ExtractImageDataError {
    MultipartError(MultipartError),
    NoImage,
}

async fn extract_image_data(mut multipart: Multipart) -> Result<Bytes, ExtractImageDataError> {
    match multipart.next_field().await {
        Err(err) => Err(ExtractImageDataError::MultipartError(err)),
        Ok(None) => Err(ExtractImageDataError::NoImage),
        Ok(Some(field)) => {
            if field.name().unwrap_or("") == "image" {
                match field.bytes().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err(ExtractImageDataError::MultipartError(err)),
                }
            } else {
                Err(ExtractImageDataError::NoImage)
            }
        }
    }
}
