use crate::app_state::{AppState, BotStatus};
use axum::extract::{ConnectInfo, Query, State};
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
pub struct Args {
    status: StatusArg,
}

pub async fn handle(
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Query(args): Query<Args>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown client")
    };
    tracing::info!("Serving `{user_agent}` at {addr}.");
    tracing::info!("Received status: {:?}", args.status);

    app_state.register_bot_checkin().await;

    match args.status {
        StatusArg::Available => {
            let pending_job = app_state.take_pending_job().await;
            (if let Some(pending_job) = pending_job {
                app_state.set_bot_status(BotStatus::Working).await;
                pending_job.0
            } else {
                app_state.set_bot_status(BotStatus::Available).await;
                "NO_PENDING_JOB".to_string()
            })
            .into_response()
        }
        StatusArg::Working => {
            app_state.set_bot_status(BotStatus::Working).await;
            StatusCode::NO_CONTENT.into_response()
        }
        StatusArg::AwaitingReset => {
            // In theory, this should never be called when there's a pending job, but let's handle
            // it smoothly just in case it does happen.
            if app_state.has_pending_job().await {
                app_state
                    .set_bot_status(BotStatus::AwaitingResetWithJobPending)
                    .await;
            } else {
                app_state.set_bot_status(BotStatus::AwaitingReset).await;
            }
            StatusCode::OK.into_response()
        }
    }
}
