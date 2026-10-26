pub mod bot_status;

use crate::app_state::bot_status::BotStatus;
use bot_status::BotStatusManager;
use std::time::Instant;
use tokio::sync::Mutex;

pub struct PendingJob(pub String);

pub struct AppState {
    bot_status_manager: Mutex<BotStatusManager>,
    bot_last_checkin: Mutex<Option<Instant>>,
    pending_job: Mutex<Option<PendingJob>>,
}

impl AppState {
    #[must_use]
    pub fn new() -> AppState {
        AppState {
            bot_status_manager: Mutex::from(BotStatusManager::new()),
            bot_last_checkin: Mutex::new(None),
            pending_job: Mutex::new(None),
        }
    }

    #[must_use]
    pub async fn bot_status(&self) -> BotStatus {
        self.bot_status_manager.lock().await.status()
    }

    pub async fn set_bot_status(&self, bot_status: BotStatus) {
        self.bot_status_manager.lock().await.set_status(bot_status);
    }

    #[must_use]
    pub fn bot_status_manager(&self) -> &Mutex<BotStatusManager> {
        &self.bot_status_manager
    }

    #[must_use]
    pub async fn has_pending_job(&self) -> bool {
        self.pending_job.lock().await.is_some()
    }

    pub async fn take_pending_job(&self) -> Option<PendingJob> {
        self.pending_job.lock().await.take()
    }

    /// Adds a new pending job if there's not already one pending.
    ///
    /// This function should not be called if there's already a pending job, but if it is, the new
    /// job will be discarded silently and an error will be logged.
    pub async fn add_pending_job(&self, new_pending_job: PendingJob) {
        let mut pending_job = self.pending_job.lock().await;
        if pending_job.is_some() {
            tracing::error!("Tried to add a pending job, but one already exists!");
        } else {
            *pending_job = Some(new_pending_job);
            tracing::debug!("Added a new pending job.");
        }
    }

    pub async fn cancel_pending_job(&self) {
        *self.pending_job.lock().await = None;

        let mut bot_status_manager = self.bot_status_manager.lock().await;
        match bot_status_manager.status() {
            BotStatus::JobPending => bot_status_manager.set_status(BotStatus::Available),
            BotStatus::AwaitingResetWithJobPending => {
                bot_status_manager.set_status(BotStatus::AwaitingReset);
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
            if secs_since_last_checking > 60 {
                self.set_bot_status(BotStatus::Offline).await;
            }
        } else {
            self.set_bot_status(BotStatus::Offline).await;
        }
    }
}
