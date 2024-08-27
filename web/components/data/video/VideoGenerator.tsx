import React, { useEffect, useState } from 'react';
import { signIn, useSession } from 'next-auth/react';
import styles from './VideoGenerator.module.scss';
import useFile from '@/util/useFile';
import VideoControls, { ViewMode } from '@/components/data/audio/AudioControls';
import VideoDisplay from '@/components/data/video/VideoDisplay';
import AudioPlayer from '@/components/data/audio/AudioPlayer';
import AudioVisualizer from '@/components/data/audio/AudioVisualizer';
import useScenes from '@/util/useScenes';
import useFeatures from '@/util/useFeatures';
import useAudioState from '@/util/useAudioState';
import useCorrelation from '@/util/useCorrelation';
import VideoSettings from '@/components/data/video/VideoSettings';
import AudioAnalytics from '../audio/AudioAnalytics';
import { Scene } from './VideoScenes';
import useVideo from '@/util/useVideo';
import useVideoMetadata from '@/util/useVideoMetadata';
import AudioDashboard from '../audio/AudioDashboard';
import useUserTerms from '@/util/useUserTerms';
import AudioTerms from '../audio/AudioTerms';
import useAudios, { Audio } from '@/util/useAudios';
import VideoLoading from './VideoLoading';
import useAudioBuffer from '@/util/useAudioBuffer';

interface VideoGeneratorProps {
    session: any;
}

const VideoGenerator: React.FC<VideoGeneratorProps> = ({ session }) => {
    const { data: sessionData, status } = useSession();
    const [isSessionLoading, setIsSessionLoading] = useState(true);
    const [sessionError, setSessionError] = useState(null);

    useEffect(() => {
        if (status === 'loading') {
            setIsSessionLoading(true);
        } else if (status === 'unauthenticated') {
            signIn();
        } else if (status === 'authenticated') {
            setIsSessionLoading(false);
        } else {
            setSessionError(status);
        }
    }, [status]);

    const extractToken = (sessionData: any) => {
        return sessionData?.user?.token as string || "";
    }
    const token = extractToken(sessionData);
    const client = session.user;

    const [duration, setDuration] = useState(0);
    const [volume, setVolume] = useState(100);
    const [isPlaying, setIsPlaying] = useState(false);

    const [currentScene, setCurrentScene] = useState<Scene | null>(null);
    const [currentTime, setCurrentTime] = useState(0);
    const [currentViewMode, setSceneViewMode] = useState(ViewMode.NONE);
    const [currentDisplayTime, setCurrentDisplayTime] = useState(0);

    const { audioId, isLoading: isFileLoading, error: fileError, downloadFile, uploadFile, file } = useFile(token, client.name || "Guest");
    const { audioState, isLoading: isAudioStateLoading, error: audioStateError } = useAudioState(token, audioId, 4200);
    const { transferedMetadata: metadata, setMetadata } = useVideoMetadata(token, audioId, audioState);
    const { scenes, isLoading: isScenesLoading, error: scenesError } = useScenes(audioId, token, audioState);
    const { correlation, isLoading: isCorrelationLoading, error: correlationError } = useCorrelation(audioId, token, audioState);
    const { features, isLoading: isFeaturesLoading, error: featuresError } = useFeatures(audioId, token, audioState);
    const { videoFile: video, isLoading: isVideoLoading, error: audioError } = useVideo(currentScene, client.id, audioId);
    const { termsAccepted, acceptTerms, isLoading: isUserTermsLoading, error: userTermsError } = useUserTerms(client.id, token);
    const { audios, isLoading: isAudiosLoading, error: audiosError, deleteAudio, reloadAudios } = useAudios(token);
    const [nextSceneTimer, setNextSceneTimer] = useState<NodeJS.Timeout | null>(null);
    const { audioBuffer, isLoading: isAudioBufferLoading, error: audioBufferError } = useAudioBuffer(file);


    useEffect(() => {
        if (file && metadata) {
            let temp_metadata = metadata;
            if (temp_metadata.title.length <= 0) {
                let file_name = file.name.split(".")[0];
                temp_metadata.title = file_name;
            }
            setMetadata(temp_metadata);
        }
    }, [file]);

    const [isTermsAccepted, setIsTermsAccepted] = useState(true);

    // reloadAudios // when metadata is not loaded and audio is uploaded
    const handleRealDownload = (audioId: string) => {
        if (!file) {
            return;
        }
        // send file to user
        const url = window.URL.createObjectURL(file);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${file.name} | ${audioId}`;
        a.click();
        window.URL.revokeObjectURL(url);

        // reload audios
        reloadAudios();


    };

    useEffect(() => {
        if (audioId) {
            reloadAudios();
        }
    }, [audioId]);

    useEffect(() => {
        if (metadata?.title) {
            reloadAudios();
        }
    }, [metadata]);


    useEffect(() => {
        if (termsAccepted) {
            setIsTermsAccepted(true);
        } else {
            setIsTermsAccepted(false);
        }
    }, [termsAccepted]);

    useEffect(() => {
        // check for currentTime and get Current Scene
        if (scenes && scenes.length > 0) {
            if (currentScene) {
                if (nextSceneTimer) {
                    clearTimeout(nextSceneTimer);
                }
                let currentSceneIndex = currentScene.scene_index;
                let nextScene = scenes[currentSceneIndex + 1];
                let durationLeft = currentScene.scene_end - currentTime;
                let timeout = durationLeft * 500;
            }
        }
    }, [currentTime, scenes]);

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

    const handleAudioClick = (audio: Audio) => {
        // setfile(audio.file);
        console.log(audio);
        // clear all scenes
        setMetadata(null);
        setCurrentScene(null);
        setSceneViewMode(ViewMode.NONE);
        downloadFile(audio.id);
    };

    const handleAudioDelete = (audioId: string) => {
        deleteAudio(audioId);

        // reload audios   
        reloadAudios();

    };


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

    if (client == undefined || client.id == undefined) {
        return (
            <div className={styles.container_wrapper}>
                <div className={styles.container}>
                    <h1>Not logged in</h1>
                </div>
            </div>
        );
    }




    return (
        <div className={styles.container_wrapper}>
            {
                !isTermsAccepted && (
                    <AudioTerms
                        onClose={(isAccepted: boolean) => {
                            if (isAccepted) {
                                acceptTerms();
                            }
                        }
                        }
                    />)
            }

            <div className={styles.container}>

                {/* Video Display */}
                <VideoDisplay
                    video={video ? video : null}
                    isPlaying={isPlaying}
                    currentTime={currentTime}
                    onTimeChange={(time) => setCurrentTime(time)}
                />

                {/* Audio Visualizer */}
                <AudioVisualizer
                    metadata={metadata}
                    audioBuffer={audioBuffer}
                    isLoading={isAudioBufferLoading}
                    currentTime={currentDisplayTime}
                    scenes={scenes}
                    selectedScene={currentScene}
                    onCurrentTimeChange={(time) => setCurrentTime(time)}
                    handleFileUpload={(file: File) => uploadFile(file)}
                    sceneViewMode={currentViewMode}
                />

                {/* Loading */}
                {
                    (!isFileLoading && !fileError) &&
                    <VideoLoading
                        loads={
                            [
                                isFileLoading,
                                isUserTermsLoading,
                                isAudioStateLoading,
                                isScenesLoading,
                                isCorrelationLoading,
                                isFeaturesLoading,
                                isVideoLoading,
                            ]
                        }
                        errors={
                            [
                                fileError,
                                userTermsError,
                                audioStateError,
                                scenesError,
                                correlationError,
                                featuresError,
                                audioError,
                            ]
                        }
                    />}

                {/* Controls */}
                {file && (
                    <VideoControls
                        fileName={`${file.name} | ${audioId}` || "Video.mp4"}
                        onReupload={() => {
                            // setVideoFile(null);
                            uploadFile(null);
                            setMetadata(null);
                        }}
                        onSwitchView={() => { }}
                        onToggleEditMode={() => { }}
                        isPlaying={isPlaying}
                        sceneViewMode={currentViewMode}
                        onUpdateViewMode={(mode: ViewMode) => {
                            setSceneViewMode(mode);
                            if (mode === ViewMode.CORRELATION) {
                                setCurrentScene(null);
                            }
                        }}
                        onPlayPause={() => setIsPlaying((prev) => !prev)}
                        currentTime={currentDisplayTime}
                        audioState={audioState}
                        onVolumeChange={(volume) => setVolume(volume)}
                    />
                )}

                {/* Correlation and Features Visualization */}
                {(!isFileLoading && !fileError) &&
                    <AudioAnalytics currentViewMode={currentViewMode} scenes={scenes} handleSceneClick={handleSceneClick} currentScene={currentScene} correlation={correlation} currentDisplayTime={currentDisplayTime} duration={duration} features={features} />
                }


                {/* Settings */}
                {(!isFileLoading && !fileError) && metadata && (< VideoSettings
                    settings={metadata || {}}
                    onChanged={(updatedSettings: Record<string, any>) => setMetadata(updatedSettings)}
                />
                )}

                {/* Audio Player */}
                <AudioPlayer
                    file={file}
                    isPlaying={isPlaying}
                    currentTime={currentTime}
                    onTimeChange={(time) => setCurrentDisplayTime(time)}
                    onDurationChange={(duration) => setDuration(duration)}
                    volume={volume}

                />

                {
                    !isAudiosLoading &&
                    <AudioDashboard audios={audios || []} onAudioClick={handleAudioClick} onAudioDelete={handleAudioDelete} audioId={audioId} onAudioDownload={(audioId: string) => handleRealDownload(audioId)} />
                }


                {/* loading */}
                {/* {isSessionLoading && <p>Loading...</p>}
                {isLoading && <p>Loading...</p>} */}
                {/* {isError && <p>Something went wrong</p>} */}

            </div>
        </div>
    );

}

export default VideoGenerator;


// a component that displays the features of a audio
type RenderedFeature = {
    x: number;
    y: number;
    color: string;
};
