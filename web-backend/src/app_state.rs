use serde::Serialize;
use std::cmp::PartialEq;
use std::time::Instant;
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

pub struct PendingJob(pub String);

pub struct AppState {
    bot_status: Mutex<BotStatus>,
    bot_last_checkin: Mutex<Option<Instant>>,
    pending_job: Mutex<Option<PendingJob>>,
    pub tx_bot_status: broadcast::Sender<BotStatus>,
}

impl AppState {
    #[must_use]
    pub fn new() -> AppState {
        let (tx, _rx) = broadcast::channel(100);
        AppState {
            bot_status: Mutex::from(BotStatus::Offline),
            bot_last_checkin: Mutex::new(None),
            pending_job: Mutex::new(None),
            tx_bot_status: tx,
        }
    }

    pub async fn bot_status(&self) -> BotStatus {
        *self.bot_status.lock().await
    }

    pub async fn has_pending_job(&self) -> bool {
        self.pending_job.lock().await.is_some()
    }

    pub async fn take_pending_job(&self) -> Option<PendingJob> {
        self.pending_job.lock().await.take()
    }

    pub async fn cancel_pending_job(&self) {
        *self.pending_job.lock().await = None;

        match self.bot_status().await {
            BotStatus::JobPending => self.set_bot_status(BotStatus::Available).await,
            BotStatus::AwaitingResetWithJobPending => {
                self.set_bot_status(BotStatus::AwaitingReset).await;
            }
            _ => {}
        }
    }

    pub async fn register_bot_checkin(&self) {
        let now = Instant::now();
        *self.bot_last_checkin.lock().await = Some(now);
    }

    pub async fn mark_bot_offline_if_no_checkin(&self) {
        let now = Instant::now();

        if let Some(last_checkin) = *self.bot_last_checkin.lock().await {
            let secs_since_last_checking = now.duration_since(last_checkin).as_secs();
            if secs_since_last_checking > 10 {
                self.set_bot_status(BotStatus::Offline).await;
            }
        } else {
            self.set_bot_status(BotStatus::Offline).await;
        }
    }

    pub async fn set_bot_status(&self, new_status: BotStatus) {
        let mut current_status = self.bot_status.lock().await;
        if new_status == *current_status {
            tracing::debug!(
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
