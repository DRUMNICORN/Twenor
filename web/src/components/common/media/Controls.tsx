import React, { useEffect, useState } from 'react';
import styles from './Controls.module.scss';

type ControlsProps = {
    onReupload: () => void;
    onSwitchView: () => void;
    onToggleEditMode: () => void;
    isPlaying: boolean;
    sceneViewMode: ViewMode;
    onUpdateViewMode: (mode: ViewMode) => void;
    onPlayPause: () => void;
    currentTime: number;
    trackState: String;
};

export enum ViewMode {
    SCENES,
    CORRELATION,
    FEATURES,
    NONE
}

const Controls: React.FC<ControlsProps> = ({
    onReupload,
    onSwitchView,
    onToggleEditMode,
    isPlaying,
    sceneViewMode,
    onUpdateViewMode,
    onPlayPause,
    currentTime,
    trackState
}) => {
    const [currentTimeDisplay, setCurrentTimeDisplay] = useState('00:00');

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
                <ControlButton onClick={onReupload} icon="/icons/wave/trash.svg" alt="Waveform" />
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

                    icon={sceneViewMode === ViewMode.SCENES ? "/icons/wave/view.svg" : (sceneViewMode === ViewMode.CORRELATION ? "/icons/wave/waveform.svg" : "/icons/wave/feature.svg")} alt="Waveform" />
                <TrackStateView
                    trackState={trackState}
                ></TrackStateView>
            </div>
            <div className={styles.timeDisplay}>{currentTimeDisplay}</div>
        </div>
    );
};

type ControlButtonProps = {
    onClick: () => void;
    onContextMenu?: (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
    icon: string;
    alt: string;
};

const ControlButton: React.FC<ControlButtonProps> = ({ onClick, onContextMenu, icon, alt }) => {
    return (
        <button className={styles.controlButton} onClick={onClick} onContextMenu={onContextMenu}>
            <div className={styles.icon}>
                <img src={icon} width={16} height={16} alt={alt} className={styles.invertedImage} />
            </div>
        </button >
    );
};

const TrackStateView: React.FC<{ trackState: String }> = ({ trackState }) => {
    if (trackState === "done") {
        return (
            <></>
        );
    }

    return (
        <div className={styles.trackState}>
            <div className={styles.trackStateText}>{trackState}</div>
        </div>
    );
}


export default Controls;
