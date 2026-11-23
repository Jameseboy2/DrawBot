use crate::app_state::AppState;
use crate::app_state::bot_status::BotStatus;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub enum StatusArg {
    Available,
    Working,
    AwaitingReset,
}

#[derive(Deserialize)]
pub struct PostBody {
    status: StatusArg,
    need_page: Option<u32>,
}

pub async fn handle(
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<PostBody>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown client")
    };
    tracing::info!("Serving `{user_agent}` at {addr}.");
    tracing::info!("Received status: {:?}", body.status);

    app_state.register_bot_checkin().await;

    match body.status {
        StatusArg::Available => {
            let mut bot_status_manager = app_state.bot_status_manager().lock().await;

            if let Some(pending_job) = bot_status_manager.pending_job() {
                let bot_status = bot_status_manager.status();
                log_available_reported_with_job_pending(bot_status);

                match bot_status {
                    BotStatus::JobPending | BotStatus::AwaitingResetWithJobPending => {
                        let first_page = pending_job
                            .0
                            .first()
                            .unwrap_or(&"S1,1;END".to_owned())
                            .clone();
                        bot_status_manager.set_status(BotStatus::Working);
                        first_page.into_response()
                    }
                    BotStatus::Offline
                    | BotStatus::Working
                    | BotStatus::AwaitingReset
                    | BotStatus::Available => {
                        bot_status_manager.clear_pending_job();
                        bot_status_manager.set_status(BotStatus::Available);
                        StatusCode::NO_CONTENT.into_response()
                    }
                }
            } else {
                bot_status_manager.set_status(BotStatus::Available);
                StatusCode::NO_CONTENT.into_response()
            }
        }
        StatusArg::Working => {
            let mut bot_status_manager = app_state.bot_status_manager().lock().await;
            bot_status_manager.set_status(BotStatus::Working);

            if let Some(need_page) = body.need_page {
                if let Some(pending_job) = bot_status_manager.pending_job() {
                    pending_job
                        .0
                        .get(need_page as usize)
                        .unwrap_or(&"S1,1;END".to_owned())
                        .clone()
                        .into_response()
                } else {
                    tracing::error!(
                        "Page {need_page} was requested while in working state, but no job was pending!"
                    );
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            } else {
                StatusCode::NO_CONTENT.into_response()
            }
        }
        StatusArg::AwaitingReset => {
            let mut bot_status_manager = app_state.bot_status_manager().lock().await;

            bot_status_manager.clear_pending_job();
            bot_status_manager.set_status(BotStatus::AwaitingReset);

            StatusCode::NO_CONTENT.into_response()
        }
    }
}

fn log_available_reported_with_job_pending(bot_status: BotStatus) {
    match bot_status {
        BotStatus::JobPending | BotStatus::AwaitingResetWithJobPending => {}
        BotStatus::Offline => {
            tracing::info!("Bot restarted, clearing pending job.");
        }
        BotStatus::Working => {
            tracing::warn!(
                "Illegal status transition: Working -> Available. Clearing pending job."
            );
        }
        BotStatus::Available | BotStatus::AwaitingReset => {
            tracing::error!(
                "Error: Had pending job with illegal status ({:?}). Clearing pending job.",
                bot_status
            );
        }
    }
}
