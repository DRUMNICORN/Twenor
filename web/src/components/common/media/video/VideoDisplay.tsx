import React, { useState, useEffect } from 'react';
import styles from './VideoDisplay.module.scss';
import LoadingDisplay from '@/components/common/state/LoadingDisplay';
import Image from 'next/image';

interface VideoDisplayProps {
    videoSrc: string | null;
    isPlaying: boolean;
    currentTime: number;
    onTimeChange: (time: number) => void;
}

const VideoDisplay: React.FC<VideoDisplayProps> = ({
    videoSrc,
    isPlaying,
    currentTime,
    onTimeChange
}) => {
    const videoRef = React.useRef<HTMLVideoElement>(null);
    const [videoRatio, setVideoRatio] = useState<number>(16 / 9);
    const [isDisplayOpen, setIsDisplayOpen] = useState<boolean>(false);

    const handleTimeUpdate = () => {
        if (videoRef.current) {
            onTimeChange(videoRef.current.currentTime);
        }
    };

    useEffect(() => {
        if (videoRef.current) {
            videoRef.current.currentTime = currentTime;
        }
    }, [currentTime]);

    useEffect(() => {
        const updateVideoRatio = () => {
            if (videoRef.current) {
                const { videoWidth, videoHeight } = videoRef.current;
                const ratio = videoWidth / videoHeight;
                setVideoRatio(ratio);
            }
        };

        window.addEventListener('resize', updateVideoRatio);
        updateVideoRatio();

        return () => {
            window.removeEventListener('resize', updateVideoRatio);
        };
    }, []);

    const toggleDisplay = () => {
        setIsDisplayOpen((prevState) => !prevState);
    };

    return (
        <>
            <button className={styles.toggleButton} onClick={toggleDisplay}>
                <img
                    src="/icons/arrow.svg"
                    alt="expand"
                    width={18}
                    height={18}
                    className={isDisplayOpen ? styles.open : styles.closed}
                />
            </button>
            <div className={`${styles.videoContainer} ${isDisplayOpen ? '' : styles.closed}`}>
                {!videoSrc ? (
                    <div className={styles.noVideo}>
                        <LoadingDisplay />
                    </div>
                ) : (
                    <div className={styles.videoWrapper}>
                        <video
                            ref={videoRef}
                            className={styles.videoPlayer}
                            src={videoSrc}
                            controls={isPlaying}
                            autoPlay={isPlaying}
                            loop
                            muted
                            onTimeUpdate={handleTimeUpdate}
                        />
                    </div>
                )}
            </div>
        </>
    );
};

export default VideoDisplay;