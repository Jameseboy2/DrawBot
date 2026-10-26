import { Typography } from '@mui/material';
import { useServerWebsocket } from '../hooks/useServerWebsocket.ts';
import { BotStatus, OutgoingSocketMessage } from '../types/types.ts';
import * as React from 'react';
import { useCallback } from 'react';
import DisplayBotUploadAvailable from './DisplayBotUploadAvailable.tsx';
import DisplayBotJobPending from './DisplayBotJobPending.tsx';

export default function App() {
    const { connected, botStatus, sendMessage } = useServerWebsocket();

    const cancelPendingJob = useCallback(() => {
        sendMessage(OutgoingSocketMessage.CancelPendingJob);
    }, [sendMessage]);

    return (
        <div>
            <Typography variant="h3" className="center">
                DrawBot
            </Typography>
            <br />
            <br />
            <div className="center">
                {(() => {
                    switch (botStatus) {
                        case BotStatus.Loading:
                            return <Typography>Connecting...</Typography>;
                        case BotStatus.Offline:
                            return (
                                <Typography>
                                    DrawBot is currently offline.
                                </Typography>
                            );
                        case BotStatus.Available:
                            return (
                                <DisplayBotUploadAvailable
                                    connected={connected}
                                    awaitingReset={false}
                                />
                            );
                        case BotStatus.JobPending:
                            return (
                                <DisplayBotJobPending
                                    connected={connected}
                                    awaitingReset={false}
                                    cancelPendingJob={cancelPendingJob}
                                />
                            );
                        case BotStatus.Working:
                            return (
                                <Typography>
                                    DrawBot is currently working.
                                </Typography>
                            );
                        case BotStatus.AwaitingReset:
                            return (
                                <DisplayBotUploadAvailable
                                    connected={connected}
                                    awaitingReset={true}
                                />
                            );
                        case BotStatus.AwaitingResetWithJobPending:
                            return (
                                <DisplayBotJobPending
                                    connected={connected}
                                    awaitingReset={true}
                                    cancelPendingJob={cancelPendingJob}
                                />
                            );
                        default:
                            return (
                                <Typography>
                                    Error: Received unrecognized bot status "
                                    {botStatus}" from server.
                                </Typography>
                            );
                    }
                })()}
            </div>
            <div id="connect-status-text">
                {connected ? (
                    <Typography style={{ color: 'green' }}>
                        Connected to server.
                    </Typography>
                ) : (
                    <Typography style={{ color: 'var(--error-red)' }}>
                        Not connected to server. Refresh the page to reconnect.
                    </Typography>
                )}
            </div>
        </div>
    );
}
