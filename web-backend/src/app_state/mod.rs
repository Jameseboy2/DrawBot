pub mod bot_status;

use crate::app_state::bot_status::BotStatus;
use bot_status::BotStatusManager;
use std::time::Instant;
use tokio::sync::Mutex;

pub struct PendingJob(pub Vec<String>);

pub struct AppState {
    bot_status_manager: Mutex<BotStatusManager>,
    bot_last_checkin: Mutex<Option<Instant>>,
}

impl AppState {
    #[must_use]
    pub fn new() -> AppState {
        AppState {
            bot_status_manager: Mutex::from(BotStatusManager::new()),
            bot_last_checkin: Mutex::new(None),
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
