import React, { useState, useEffect } from 'react';
import VideoScenes, { Scene } from "../video/VideoScenes";
import AudioBars from "./AudioBars";
import AudioSpectrogram from "./AudioSpectrogram";

import styles from './AudioAnalytics.module.scss';
import { ViewMode } from './AudioControls';
import { Features } from '@/util/useFeatures';
import Loading from '@/components/util/Loading';

interface AudioAnalyticsProps {
    currentViewMode: ViewMode;
    scenes: Scene[];
    handleSceneClick: (scene: Scene | null) => void;
    currentScene: Scene | null;
    correlation: number[];
    currentDisplayTime: number;
    duration: number;
    features: {
        audio_id: number;
        features: Features[];
    };
}

const AudioAnalytics: React.FC<AudioAnalyticsProps> = ({
    currentViewMode,
    scenes,
    handleSceneClick,
    currentScene,
    correlation,
    currentDisplayTime,
    duration,
    features,
}) => {
    const [viewModeState, setViewModeState] = useState<ViewMode>(ViewMode.NONE);

    // Effect to update the view mode state when it changes
    useEffect(() => {
        if (currentViewMode !== ViewMode.NONE)
            setViewModeState(currentViewMode);
    }, [currentViewMode]);

    return (
        <div className={styles.container}>
            <div className={`${currentViewMode !== ViewMode.NONE ? styles.active : styles.inactive}`} >
                {viewModeState === ViewMode.SCENES && (
                    <div className={styles.wrapper}>
                        {
                            scenes.length > 0 ?
                                <VideoScenes scenes={scenes} handleSceneClick={handleSceneClick} selectedScene={currentScene} />
                                : <Loading />
                        }
                    </div>
                )}

                {viewModeState === ViewMode.CORRELATION && (
                    <div className={styles.wrapper}>
                        {

                            correlation.length > 0 ?
                                <div className={styles.corrInside}>
                                    <AudioBars
                                        barData={correlation || []}
                                        numberOfBars={correlation?.length || 4}
                                        currentTime={currentDisplayTime}
                                        duration={duration} />
                                </div>
                                : <Loading />
                        }
                    </div>
                )}

                {viewModeState === ViewMode.FEATURES && (
                    <div className={styles.wrapper}>
                        {
                            (features.features.length > 0) ?
                                <AudioSpectrogram
                                    features={features.features}
                                    currentDisplayTime={currentDisplayTime}
                                    duration={duration} />
                                : <Loading />
                        }
                    </div>
                )}
            </div>
        </div >
    );
};

export default AudioAnalytics;
