import React, { useEffect, useRef, FC } from 'react';

type AudioPlayerProps = {
    file: File | null;
    isPlaying: boolean;
    currentTime: number;
    onTimeChange: (time: number) => void;
    onDurationChange: (duration: number) => void;
};

const AudioPlayer: FC<AudioPlayerProps> = ({ file, isPlaying, currentTime, onTimeChange, onDurationChange }) => {
    const audioRef = useRef<HTMLAudioElement | null>(null);
    const [disabled, setDisabled] = React.useState(false);
    useEffect(() => {
        const audio = audioRef.current;
        // if (!audio) {
        //     return;
        // }
        // if (!file) {
        //     setDisabled(true);
        //     return;
        // }
        if (audio) {
            audio.src = URL.createObjectURL(file ? file : new File([], 'empty'));
            // setDisabled(false);
            let duration = 0;
            audio.addEventListener('loadedmetadata', () => {
                duration = audio.duration;
                onDurationChange(duration);
            });

            if (isPlaying) {
                audio.play();
                audio.currentTime = currentTime;
            } else {
                audio.pause();
                audio.currentTime = currentTime;
            }
        }

        return () => {
            if (audio) {
                URL.revokeObjectURL(audio.src);
            }
        };
    }, [currentTime, file, isPlaying]);

    useEffect(() => {
        const audio = audioRef.current;
        if (audio) {
            audio.currentTime = currentTime;
        }
    }, [currentTime]);

    const handlePlay = () => {
        const audio = audioRef.current;
        if (audio) {
            // will play and update currentTime with onTimeChange
            audio.play();
            // init event listener
            audio.addEventListener('timeupdate', () => {
                onTimeChange(audio.currentTime);
            });
        }
    };

    const handlePause = () => {
        const audio = audioRef.current;
        if (audio) {
            audio.pause();
            // remove event listener
            audio.removeEventListener('timeupdate', () => {
                // onTimeChange(audio.currentTime);
            });
        }
    };

    return (
        <div>
            {
                disabled ? <div></div> :
                    <audio ref={audioRef} onPlay={handlePlay} onPause={handlePause} />
            }
        </div>
    );
};

export default AudioPlayer;
