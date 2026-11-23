use crate::app_state::PendingJob;
use serde::Serialize;
use tokio::sync::broadcast;

pub struct BotStatusManager {
    status: BotStatus,
    pub tx_bot_status: broadcast::Sender<BotStatus>,
    /// Should only be set when `status` is one of the following:
    /// `Offline` (to allow resuming), `JobPending`, `Working`, `AwaitingResetWithJobPending`
    /// Will be automatically cleared if `status` is set to something else.
    pending_job: Option<PendingJob>,
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
            pending_job: None,
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

        if self.pending_job.is_some()
            && new_status != BotStatus::Offline
            && new_status != BotStatus::JobPending
            && new_status != BotStatus::Working
            && new_status != BotStatus::AwaitingResetWithJobPending
        {
            tracing::debug!("Clearing pending job because of status update.");
            self.pending_job = None;
        }

        self.status = new_status;
        let _ = self.tx_bot_status.send(new_status);
    }

    #[must_use]
    pub fn pending_job(&self) -> &Option<PendingJob> {
        &self.pending_job
    }

    /// Adds a new pending job if there's not already one pending.
    ///
    /// This function should not be called if there's already a pending job, but if it is, the new
    /// job will be discarded silently and an error will be logged.
    pub fn add_pending_job(&mut self, new_pending_job: PendingJob) {
        if self.pending_job.is_some() {
            tracing::error!("Tried to add a pending job, but one already exists!");
        } else {
            self.pending_job = Some(new_pending_job);
            tracing::debug!("Added a new pending job.");
        }
    }

    pub fn clear_pending_job(&mut self) {
        self.pending_job = None;
    }
}
