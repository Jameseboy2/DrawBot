use serde::Serialize;
use std::cmp::PartialEq;
use tokio::sync::{Mutex, broadcast};

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum BotStatus {
    Offline,
    Available,
    JobPending, // waiting for the bot to poll for the job
    Working,
    AwaitingReset,               // waiting for the board to be erased, etc.
    AwaitingResetWithJobPending, // awaiting reset with a job ready for once the bot is reset
}

pub struct AppState {
    bot_status: Mutex<BotStatus>,
    pub tx_bot_status: broadcast::Sender<BotStatus>,
}

impl AppState {
    #[must_use]
    pub fn new() -> AppState {
        let (tx, _rx) = broadcast::channel(100);
        AppState {
            bot_status: Mutex::from(BotStatus::Offline),
            tx_bot_status: tx,
        }
    }

    pub async fn bot_status(&self) -> BotStatus {
        *self.bot_status.lock().await
    }

    pub async fn set_bot_status(&self, new_status: BotStatus) {
        let mut current_status = self.bot_status.lock().await;
        if new_status == *current_status {
            tracing::info!(
                "Discarding bot status update because it's already the current status: {:?}",
                new_status
            );
            return;
        }

        tracing::info!("Updating bot status: {:?}", new_status);
        *current_status = new_status;
        let _ = self.tx_bot_status.send(new_status);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
