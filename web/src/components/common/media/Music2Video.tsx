import React, { useEffect, useState } from 'react';
import { signIn, useSession } from 'next-auth/react';
import styles from './Music2Video.module.scss';

import VideoDisplay from '@/components/common/media/video/VideoDisplay';
import Settings from '@/components/common/layout/Settings';
import Controls, { ViewMode } from '@/components/common/media/Controls';
import WaveformVisualizer from '@/components/common/media/audio/WaveformVisualizer';
import AudioPlayer from '@/components/common/media/audio/AudioPlayer';
import useFile from '@/hooks/useFile';
import Scenes, { Scene } from '@/components/common/media/Scenes';
import useScenes from '@/hooks/useScenes';
import useCorrelation from '@/hooks/useCorrelation';
import WaveformBars from './audio/WaveformBars';
import useTrackState from '@/hooks/useTrackState';
import useFeatures, { Features } from '../../../hooks/useFeatures';

interface Music2VideoProps {
    session: any;
}

const Music2Video: React.FC<Music2VideoProps> = ({ session }) => {

    const { data: sessionData, status } = useSession();
    const token = sessionData?.accessToken as string || "";

    const client = session.user;
    const [videoFile, setVideoFile] = useState<File | null>(null);
    const [duration, setDuration] = useState(0);

    const [isPlaying, setIsPlaying] = useState(false);

    const [currentScene, setCurrentScene] = useState<Scene | null>(null);
    const [currentTime, setCurrentTime] = useState(0);
    const [currentDisplayTime, setCurrentDisplayTime] = useState(0);


    const [audioFile, setAudioFile] = useState<File | null>(null);

    const { metadata, setMetadata, trackId, error: fileError } = useFile(audioFile, token);
    const { trackState } = useTrackState(token, trackId, 4200);

    const { scenes, isLoading: isScenesLoading, error: scenesError } = useScenes(trackId, token, trackState);
    const { correlation, isLoading: isCorrelationLoading, error: correlationError } = useCorrelation(trackId, token, trackState);
    const [sceneViewMode, setSceneViewMode] = useState(ViewMode.NONE);
    const { features, isLoading: isFeaturesLoading, error: featuresError } = useFeatures(trackId, token, trackState);

    useEffect(() => {
        if (correlation !== null && correlation.length > 0) {
            setSceneViewMode(ViewMode.CORRELATION);
        }
    }, [correlation]);

    useEffect(() => {
        if (features !== null && features.features.length > 0) {
            setSceneViewMode(ViewMode.FEATURES);
        }
    }, [features]);

    useEffect(() => {
        if (scenes !== null && scenes.length > 0) {
            setSceneViewMode(ViewMode.SCENES);
        }
    }, [scenes]);

    const handleSceneClick = (scene: Scene | null) => {
        if (!scene) {
            setCurrentScene(null);
        } else if (scene.scene_id === currentScene?.scene_id) {
            setCurrentScene(null);
        }
        else {
            setCurrentScene(scene);
            // round to nearest 10th of a second
            let actual_duration = scenes[scenes.length - 1].scene_end;
            let perentage = scene.scene_start / actual_duration;

            let new_time = perentage * duration;
            setCurrentTime(new_time);
            setCurrentDisplayTime(new_time);
        }
    };

    if (client.name === null || client.name === null) {
        return (
            <div className={styles.container_wrapper}>
                <div className={styles.container}>
                    <button onClick={() => signIn()}>Sign in</button>
                </div>
            </div>
        );
    }
    // effect to update scene View mode when correlation is loaded


    return (
        <div className={styles.container_wrapper}>
            <div className={styles.container}>
                <VideoDisplay videoSrc={videoFile ? URL.createObjectURL(videoFile) : null} isPlaying={isPlaying} currentTime={currentTime} onTimeChange={(time) => setCurrentTime(time)} />
                <WaveformVisualizer
                    metadata={metadata}
                    file={audioFile}
                    currentTime={currentDisplayTime}
                    scenes={scenes}
                    selectedScene={currentScene}
                    onCurrentTimeChange={(time) => setCurrentTime(time)}
                    handleFileUpload={(file: File) => setAudioFile(file)}
                    sceneViewMode={sceneViewMode}
                />

                <div className={`${styles.corrContainer} ${sceneViewMode != ViewMode.NONE
                    ? styles.corrButtonActive
                    : ''}`}
                >
                    {
                        sceneViewMode === ViewMode.SCENES &&
                        <div className={styles.scenesWrapper}>
                            <Scenes scenes={scenes} handleSceneClick={handleSceneClick} selectedScene={currentScene} />
                        </div>
                    }
                    {
                        sceneViewMode === ViewMode.CORRELATION &&
                        <div className={styles.corrWrapper}>
                            <div className={styles.corrInside}>
                                <WaveformBars
                                    barData={correlation || [] as number[]}
                                    numberOfBars={correlation?.length || 4}
                                    currentTime={currentDisplayTime}
                                    duration={duration}

                                />
                            </div>
                        </div>
                    }
                    {
                        sceneViewMode === ViewMode.FEATURES &&
                        <FeaturesDiv features={features.features} currentDisplayTime={currentDisplayTime} duration={duration} />
                    }
                </div>
                {
                    // check if controls should be displayed
                    metadata &&

                    <Controls
                        onReupload={() => {
                            setVideoFile(null);
                            setAudioFile(null);
                            setMetadata(null);
                        }}
                        onSwitchView={() => { }}
                        onToggleEditMode={() => { }}
                        isPlaying={isPlaying}
                        sceneViewMode={sceneViewMode}
                        onUpdateViewMode={(mode: ViewMode) => {
                            setSceneViewMode(mode);
                            if (mode === ViewMode.CORRELATION) {
                                setCurrentScene(null);
                            }
                        }
                        }
                        onPlayPause={() => setIsPlaying((prev) => !prev)}
                        currentTime={currentDisplayTime}
                        trackState={trackState}
                    />
                }
                <Settings
                    settings={metadata || {}
                    }

                    onChanged={(updatedSettings: Record<string, any>) => setMetadata(updatedSettings)}
                />

                <AudioPlayer file={audioFile} isPlaying={isPlaying} currentTime={currentTime} onTimeChange={
                    (time) => setCurrentDisplayTime(time)}
                    onDurationChange={(duration) => setDuration(duration)}
                />
            </div>
        </div >
    );
}

export default Music2Video;


// a component that displays the features of a track
type RenderedFeature = {
    x: number;
    y: number;
    color: string;
};

const FeaturesDiv: React.FC<{ features: Features[], currentDisplayTime: number, duration: number }> = ({ features, currentDisplayTime, duration }) => {
    const [renderedFeatures, setRenderedFeatures] = useState<any[]>([]);
    const [renderedPercentage, setRenderedPercentage] = useState(0);

    useEffect(() => {
        let newRenderedFeatures: any[] = [];
        features.forEach((feature, i) => {
            let keys = Object.keys(feature);
            keys.forEach((key, ii) => {
                let newFeature = {
                    x: i,
                    y: ii,
                    color: mapValueToColor(feature[key])
                };
                newRenderedFeatures.push(newFeature);
            });
        });
        setRenderedFeatures(newRenderedFeatures);
    }, [features]);

    useEffect(() => {
        let percentage = (currentDisplayTime / duration) * 100;
        setRenderedPercentage(percentage);
    }, [currentDisplayTime, duration]);

    // features is a list of objects and we want to display the values as colored boxes in a gid system
    console.log(renderedPercentage);
    return (
        <div className={styles.featuresWrapper}>
            <div className={styles.featuresContainer}>
                {
                    renderedFeatures.map((feature, i) => {
                        let length = renderedFeatures.length;
                        let percentage = ((1 / (length / 1.3)) * 1000);
                        return (
                            <div
                                key={i}
                                className={styles.feature}
                                style={{
                                    backgroundColor: feature.color,
                                    position: 'absolute',
                                    top: `${feature.y * 6}px`,
                                    left: `${feature.x * percentage}%`,
                                    width: `${percentage}vw`,
                                    height: '6px'

                                }}
                            />
                        );
                    })
                }
            </div>
            {/* display of current time will be a white bar which will move to current pos*/}
            <div className={styles.timeDisplay} style={{ left: `${renderedPercentage - 0.2}% ` }} />

        </div>
    );

}


const mapValueToColor = (value: number) => {
    let red = 255;
    let blue = 0;
    let green = 0;

    if (value < 0.5) {
        red = 255;
        blue = 255 * (value * 2);
    } else {
        blue = 255;
        red = 255 - (255 * ((value - 0.5) * 2));
    }

    return `rgba(${red}, ${green}, ${blue}, 1)`;
}

