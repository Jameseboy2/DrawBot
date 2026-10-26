export enum BotStatus {
    Loading = 'Loading',
    Offline = 'Offline',
    Available = 'Available',
    JobPending = 'JobPending',
    Working = 'Working',
    AwaitingReset = 'AwaitingReset',
    AwaitingResetWithJobPending = 'AwaitingResetWithJobPending',
}

export enum OutgoingSocketMessage {
    CancelPendingJob = 'CancelPendingJob',
}
