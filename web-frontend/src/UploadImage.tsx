import { useState } from 'react';

export function UploadImage() {
    const [selectedImage, setSelectedImage] = useState<File | null>(null);

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
                onChange={(event) => {
                    if (event.target.files) {
                        setSelectedImage(event.target.files[0]);
                    }
                }}
            />
        </div>
    );
}
