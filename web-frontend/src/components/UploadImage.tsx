import { useCallback, useState } from 'react';
import { Button, Typography } from '@mui/material';

enum UploadImageStage {
    None,
    Uploading,
    Success,
    Failure,
}

export function UploadImage({ enabled }: { enabled: boolean }) {
    const [selectedImage, setSelectedImage] = useState<File | null>(null);
    const [stage, setStage] = useState<UploadImageStage>(UploadImageStage.None);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    const handleUpload = useCallback(async () => {
        if (!selectedImage) return;

        const formData = new FormData();
        formData.append('image', selectedImage);

        try {
            setStage(UploadImageStage.Uploading);
            const response = await fetch(
                import.meta.env.VITE_API_URL + '/upload-image',
                {
                    method: 'POST',
                    body: formData,
                }
            );

            if (response.ok) {
                setStage(UploadImageStage.Success);
            } else {
                const message = await response.text();
                setErrorMessage(
                    `Request failed with code ${response.status} ${response.statusText}: ${message}`
                );
                setStage(UploadImageStage.Failure);
            }
        } catch (error) {
            console.error('Error uploading image:', error);
            setErrorMessage(`Error uploading image: ${error}`);
            setStage(UploadImageStage.Failure);
        }
    }, [selectedImage]);

    const fileIsTooBig =
        selectedImage?.size !== undefined &&
        selectedImage?.size > 2 * 1024 * 1024;
    const finalEnabled = enabled && !fileIsTooBig;

    return (
        <div>
            {selectedImage !== null ? (
                <div>
                    <img
                        alt="Selected image not found."
                        width={'500px'}
                        src={URL.createObjectURL(selectedImage)}
                    />
                </div>
            ) : null}

            <input
                type="file"
                style={{ marginBottom: '10px' }}
                onChange={(event) => {
                    if (event.target.files) {
                        setSelectedImage(event.target.files[0]);
                    }
                }}
            />

            {fileIsTooBig ? (
                <Typography style={{ color: 'var(--error-red)' }}>
                    File too big: Max size is 2MB.
                </Typography>
            ) : null}

            {stage === UploadImageStage.None && selectedImage !== null ? (
                <div>
                    <br />
                    <div className="center">
                        <Button
                            variant="contained"
                            disabled={!finalEnabled}
                            onClick={handleUpload}
                        >
                            Upload image
                        </Button>
                    </div>
                </div>
            ) : null}

            {stage === UploadImageStage.Uploading ? (
                <Typography>Uploading...</Typography>
            ) : null}

            {stage === UploadImageStage.Success ? (
                <Typography>Upload succeeded. Waiting for server...</Typography>
            ) : null}

            {stage === UploadImageStage.Failure ? (
                <Typography style={{ color: 'var(--error-red)' }}>
                    Upload failed. Error message: {errorMessage}
                </Typography>
            ) : null}
        </div>
    );
}
