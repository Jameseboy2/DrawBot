use serde::Serialize;
use tokio::sync::broadcast;

pub struct BotStatusManager {
    status: BotStatus,
    pub tx_bot_status: broadcast::Sender<BotStatus>,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum BotStatus {
    Offline,
    Available,
    JobPending, // waiting for the bot to poll for the job
    Working,
    AwaitingReset,               // waiting for the board to be erased, etc.
    AwaitingResetWithJobPending, // awaiting reset with a job ready for once the bot is reset
}

impl BotStatusManager {
    #[must_use]
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            status: BotStatus::Offline,
            tx_bot_status: tx,
        }
    }

    #[must_use]
    pub fn status(&self) -> BotStatus {
        self.status
    }

    pub fn set_status(&mut self, new_status: BotStatus) {
        if new_status == self.status {
            tracing::debug!(
                "Discarding bot status update because it's already the current status: {:?}",
                new_status
            );
            return;
        }

        tracing::info!("Updating bot status: {:?}", new_status);
        self.status = new_status;
        let _ = self.tx_bot_status.send(new_status);
    }
}
