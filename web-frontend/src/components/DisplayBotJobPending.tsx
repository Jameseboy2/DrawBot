import { Button, Typography } from '@mui/material';

export default function DisplayBotJobPending({
    connected,
    awaitingReset,
    cancelPendingJob,
}: {
    connected: boolean;
    awaitingReset: boolean;
    cancelPendingJob: () => void;
}) {
    const message = awaitingReset
        ? 'DrawBot is currently waiting to be reset. A job is pending for when DrawBot is ready.'
        : 'DrawBot is currently idle. A job is pending and will be started when DrawBot sees it.';

    return (
        <div>
            <Typography>{message}</Typography>
            <br />
            <div className="center">
                <Button
                    variant="outlined"
                    disabled={!connected}
                    onClick={cancelPendingJob}
                >
                    Cancel pending job
                </Button>
            </div>
        </div>
    );
}
