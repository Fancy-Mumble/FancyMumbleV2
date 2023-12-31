import React, { useState, useCallback } from 'react';
import { Box, Button, Container, Slider, Typography } from '@mui/material';
import Cropper, { Area } from 'react-easy-crop';

interface ICroppedAreaPixels extends Area {
    width: number;
    height: number;
}

interface ImageCropperProps {
    image: string;
    onSkip: () => void;
    onCrop: (image: string, zoom: number, crop: { x: number, y: number }) => void;
    onCancel: () => void;
}

const ImageCropper: React.FC<ImageCropperProps> = ({ image, onSkip, onCrop, onCancel }) => {
    const [crop, setCrop] = useState<{ x: number, y: number }>({ x: 0, y: 0 });
    const [zoom, setZoom] = useState<number>(1);
    const [croppedAreaPixels, setCroppedAreaPixels] = useState<ICroppedAreaPixels | null>(null);
    const [croppedImage, setCroppedImage] = useState<string | null>(null);

    const onCropComplete = useCallback((_croppedArea: any, croppedAreaPixels: ICroppedAreaPixels) => {
        setCroppedAreaPixels(croppedAreaPixels);
    }, []);

    const showCroppedImage = useCallback(async () => {
        try {
            //         const croppedImage = await getCroppedImg(
            //     /* You need to provide the image source here */,
            //             croppedAreaPixels: any
            //   );
            // setCroppedImage(croppedImage);
        } catch (e) {
            console.error(e);
        }
    }, [croppedAreaPixels]);

    return (
        <Box>
            <Box sx={{ mb: 1 }}>
                <Typography variant="h6">Edit Image</Typography>
            </Box>
            <Box sx={{ width: '640px', height: '480px', position: 'relative' }}>
                <Cropper
                    image={image}
                    crop={crop}
                    zoom={zoom}
                    aspect={3 / 1}
                    onCropChange={setCrop}
                    onCropComplete={onCropComplete}
                    onZoomChange={setZoom}
                    showGrid={false}
                />
            </Box>
            <Box>
                <Container sx={{ my: 2 }}>
                    <Slider
                        value={zoom}
                        min={1}
                        max={3}
                        step={0.1}
                        aria-labelledby="Zoom"
                        onChange={(e, zoom) => setZoom(zoom as number)}
                    />
                </Container>
            </Box>
            <Box sx={{ display: 'flex' }}>
                <Box sx={{ flexGrow: 1 }}>
                    <Button onClick={onSkip} variant="text" color="info">
                        Skip
                    </Button>
                </Box>
                <Box>
                    <Button onClick={onCancel} variant="text" color="error">
                        Cancel
                    </Button>
                    <Button onClick={() => onCrop(image, zoom, crop)} variant="contained" color="primary">
                        Apply
                    </Button>
                </Box>
            </Box>
            {croppedImage && (
                <img alt="Cropped" src={croppedImage} />
            )}
        </Box>
    );
};

export default ImageCropper;