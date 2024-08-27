import React, { useEffect, useState } from 'react';
import styles from './AudioControls.module.scss';
import ControlButton from '@/components/base/Button';

type AudioControlsProps = {
    fileName: string;
    onReupload: () => void;
    onSwitchView: () => void;
    onToggleEditMode: () => void;
    isPlaying: boolean;
    sceneViewMode: ViewMode;
    onUpdateViewMode: (mode: ViewMode) => void;
    onPlayPause: () => void;
    onVolumeChange: (volume: number) => void;

    currentTime: number;
    audioState: String;

};

export enum ViewMode {
    SCENES,
    CORRELATION,
    FEATURES,
    NONE
}

const AudioControls: React.FC<AudioControlsProps> = ({
    fileName,
    onReupload,
    onSwitchView,
    onToggleEditMode,
    isPlaying,
    sceneViewMode,
    onUpdateViewMode,
    onPlayPause,
    currentTime,
    audioState,
    onVolumeChange
}) => {
    const [currentTimeDisplay, setCurrentTimeDisplay] = useState('00:00');
    const [isVolumeControlOpen, setIsVolumeControlOpen] = useState(false);
    const [volume, setVolume] = useState(100);

    const onOpenVolumeControl = () => {
        setIsVolumeControlOpen(!isVolumeControlOpen);
    };


    const handleVolumeChange = (volume: number) => {
        setVolume(volume);
        onVolumeChange(volume);
    };


    useEffect(() => {
        const minutes = Math.floor(currentTime / 60);
        const seconds = Math.floor(currentTime % 60);
        // display MM:SS:MS
        // always same length
        setCurrentTimeDisplay(
            `${minutes.toString().padStart(2, '0')}:${seconds
                .toString()
                .padStart(2, '0')}`
        );


    }, [currentTime]);

    return (
        <div className={styles.controls}>
            <div className={styles.controlButtonsContainer}>
                <ControlButton onClick={onPlayPause} icon={isPlaying ? "/icons/wave/pause.svg" : "/icons/wave/play.svg"} alt="Play/Pause" />
                <ControlButton onClick={onReupload} icon="/icons/wave/trash.svg" alt="Audio" />
                <ControlButton
                    onContextMenu={
                        (e) => {
                            e.preventDefault();
                            onUpdateViewMode(ViewMode.NONE);
                        }
                    }
                    onClick={
                        () => {
                            if (sceneViewMode === ViewMode.SCENES) {
                                onUpdateViewMode(ViewMode.CORRELATION);
                            } else if (sceneViewMode === ViewMode.CORRELATION) {
                                onUpdateViewMode(ViewMode.FEATURES);
                            } else if (sceneViewMode === ViewMode.FEATURES) {
                                onUpdateViewMode(ViewMode.SCENES);
                            }
                            else {
                                onUpdateViewMode(ViewMode.SCENES);
                            }
                        }
                    }

                    icon={sceneViewMode === ViewMode.SCENES ? "/icons/wave/edit.svg" : (sceneViewMode === ViewMode.CORRELATION ? "/icons/wave/waveform.svg" : (sceneViewMode === ViewMode.FEATURES ? "/icons/wave/mfcc.svg" : "/icons/wave/view.svg"))}
                    text={sceneViewMode === ViewMode.SCENES ? "Scenes" : (sceneViewMode === ViewMode.CORRELATION ? "Correlation" : (sceneViewMode === ViewMode.FEATURES ? "Features" : "Open Eye"))} alt="View" />

                <ControlButton onClick={onOpenVolumeControl} icon={volume === 0 ? "/icons/wave/volume-mute.svg" : (volume < 50 ? "/icons/wave/volume-min.svg" : "/icons/wave/volume-max.svg")} alt="Volume" />
                <ControlSlider isOpen={isVolumeControlOpen} value={volume} onChange={handleVolumeChange} />

                <TagView
                    isLoaded={true}
                    audioState={fileName}
                ></TagView>
            </div>
            <div className={styles.rigthSideControls}>
                <TagView
                    audioState={audioState}
                ></TagView>
                <div className={styles.timeDisplay}>{currentTimeDisplay}</div>
            </div>
        </div >
    );
};

const ControlSlider: React.FC<{ isOpen: boolean; value: number; onChange: (value: number) => void }> = ({
    isOpen,
    value,
    onChange
}) => {
    const [isDragging, setIsDragging] = useState(false);
    const [sliderValue, setSliderValue] = useState(value);

    useEffect(() => {
        if (isOpen) {
            setSliderValue(value);
        }
    }, [isOpen, value]);

    const onSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = Number(e.target.value);
        setSliderValue(newValue);
        onChange(newValue);
    };

    return (
        <div className={`${styles.controlSlider} ${isOpen ? styles.open : styles.closed}`}>
            <input
                type="range"
                min="0"
                max="100"
                value={sliderValue}
                onChange={onSliderChange}
                onMouseDown={() => setIsDragging(true)}
                onMouseUp={() => setIsDragging(false)}
                className={styles.slider}
            />
        </div>
    );
}

const TagView: React.FC<{ audioState: String, isLoaded?: boolean }> = ({ audioState, isLoaded }) => {
    let text = !audioState.length ? "connecting" : audioState;

    if (text.length > 20) {
        text = text.substring(0, 20) + "...";
    }

    return (
        <div className={`${styles.audioState} ${isLoaded ? styles.audioStateLoaded : ''}`}>
            <div className={styles.audioStateText}>
                {text}</div>
        </div>
    );
}


export default AudioControls;
