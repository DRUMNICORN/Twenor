import React, { useState, useEffect } from 'react';
import styles from './VideoDisplay.module.scss';
import Image from 'next/image';
import Loading from '@/components/util/Loading';

interface VideoDisplayProps {
    video: HTMLVideoElement | null;
    isPlaying: boolean;
    currentTime: number;
    onTimeChange: (time: number) => void;
}

const VideoDisplay: React.FC<VideoDisplayProps> = ({
    video,
    isPlaying,
    currentTime,
    onTimeChange
}) => {
    const videoRef = React.useRef<HTMLVideoElement>(null);
    const [videoRatio, setVideoRatio] = useState<number>(16 / 9);
    const [isDisplayOpen, setIsDisplayOpen] = useState<boolean>(false);

    useEffect(() => {
        if (video && videoRef.current) {
            videoRef.current.src = video.src;
        }
    }, [video]);

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
            <button className={`${styles.toggleButton} ${video ? '' : styles.disabled}`} onClick={toggleDisplay}>
                <img
                    src="/icons/arrow.svg"
                    alt="expand"
                    width={"5rem"}
                    height={"5rem"}
                    className={isDisplayOpen ? styles.open : styles.closed}
                />
            </button>
            <div className={`${styles.videoContainer} ${isDisplayOpen ? '' : styles.closed}`}>
                {!video ? (
                    <div className={styles.noVideo}>
                        <Loading />
                    </div>
                ) : (
                    <div className={styles.videoWrapper}>
                        <video
                            ref={videoRef}
                            className={styles.videoPlayer}
                            controls={isPlaying}
                            autoPlay={isPlaying}
                            loop
                            muted
                        // onTimeUpdate={handleTimeUpdate}
                        />
                    </div>
                )}
            </div>
        </>
    );
};

export default VideoDisplay;