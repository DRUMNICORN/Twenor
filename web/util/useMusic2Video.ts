import { useState } from 'react';

export const useMusic2Video = () => {
    const [videoFile, setVideoFile] = useState<File | null>(null);
    const [duration, setDuration] = useState(0);
    const [isPlaying, setIsPlaying] = useState(false);
    const [currentTime, setCurrentTime] = useState(0);
    const [currentDisplayTime, setCurrentDisplayTime] = useState(0);

    return {
        videoFile,
        setVideoFile,
        duration,
        setDuration,
        isPlaying,
        setIsPlaying,
        currentTime,
        setCurrentTime,
        currentDisplayTime,
        setCurrentDisplayTime,
        // Return additional functions or states here
    };
};
