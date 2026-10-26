import { Typography } from '@mui/material';
import { UploadImage } from './UploadImage.tsx';

export default function DisplayBotUploadAvailable({
    connected,
    awaitingReset,
}: {
    connected: boolean;
    awaitingReset: boolean;
}) {
    const message = awaitingReset
        ? 'DrawBot is currently waiting to be reset. Upload an image to have it be drawn when DrawBot is ready.'
        : 'DrawBot is currently available. Upload an image to have it be drawn.';

    return (
        <div>
            <Typography>{message}</Typography>
            <br />
            <UploadImage enabled={connected} />
        </div>
    );
}
