use std::collections::VecDeque;

pub struct AppState {
    pending_jobs: VecDeque<i32>, // TODO: Use non-placeholder value
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    #[must_use]
    pub fn new() -> AppState {
        AppState {
            pending_jobs: VecDeque::new(),
        }
    }

    pub fn add_pending_job(&mut self, pending_job: i32) {
        self.pending_jobs.push_back(pending_job);
    }

    pub fn take_pending_job(&mut self) -> Option<i32> {
        self.pending_jobs.pop_front()
    }
}
