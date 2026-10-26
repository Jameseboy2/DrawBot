import { useCallback, useEffect, useRef, useState } from 'react';
import { BotStatus, OutgoingSocketMessage } from '../types/types.ts';

interface SocketState {
    connected: boolean;
    botStatus: BotStatus;
    sendMessage: (message: OutgoingSocketMessage) => void;
}

export function useServerWebsocket(): SocketState {
    const websocket = useRef<WebSocket | null>(null);
    const [connected, setConnected] = useState<boolean>(false);
    const [botStatus, setBotStatus] = useState<BotStatus>(BotStatus.Loading);

    const sendMessage = useCallback((message: OutgoingSocketMessage): void => {
        if (websocket.current == null) {
            console.error(
                'Tried to send message with uninitialized websocket',
                message
            );
            return;
        }

        websocket.current.send(`"${message}"`);
    }, []);

    useEffect(() => {
        if (websocket.current == null) {
            const localSocket = new WebSocket(import.meta.env.VITE_WS_URL);

            localSocket.onmessage = (event: MessageEvent) => {
                setConnected(true);

                const decodedMessage = JSON.parse(event.data);
                if ('UpdateBotStatus' in decodedMessage) {
                    setBotStatus(decodedMessage['UpdateBotStatus']);
                } else {
                    console.log('Received unrecognized message', event.data);
                }
            };

            localSocket.onclose = () => {
                setConnected(false);
                websocket.current = null;
            };

            websocket.current = localSocket;

            return () => {
                localSocket.close();
            };
        }
    }, []);

    return {
        connected,
        botStatus,
        sendMessage,
    };
}
