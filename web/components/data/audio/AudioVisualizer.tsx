import React, { useEffect, useRef, useState } from 'react';
import styles from './AudioVisualizer.module.scss';
import AudioBars from './AudioBars';
import Upload from '@/components/util/Upload';
import { Scene } from '@/components/data/video/VideoScenes';
import useAudioBuffer from '@/util/useAudioBuffer';
import AudioGrid from './AudioGrid';

type AudioVisualizerProps = {
    metadata: Record<string, any> | null;
    audioBuffer: AudioBuffer | null;
    currentTime: number;
    onCurrentTimeChange: (time: number) => void;
    handleFileUpload: (file: File) => void;
    scenes: Scene[];
    sceneViewMode: number;
    selectedScene: Scene | null;
    isLoading: boolean;
};

const AudioVisualizer: React.FC<AudioVisualizerProps> = ({
    metadata,
    audioBuffer,
    currentTime,
    onCurrentTimeChange,
    handleFileUpload,
    scenes,
    sceneViewMode,
    selectedScene,
    isLoading
}) => {

    const numberOfBars = 420;
    const audioRef = useRef<HTMLDivElement>(null);
    const playerRef = useRef<HTMLAudioElement>(null);

    const [barData, setBarData] = useState<number[]>(Array(numberOfBars).fill(0));

    const [isPlaying, setIsPlaying] = useState(false);
    const [isMouseDown, setIsMouseDown] = useState(false);

    const handleAudioClick = (event: React.MouseEvent<HTMLDivElement>) => {

        const rect = audioRef.current?.getBoundingClientRect();
        if (!rect) return;

        const offsetX = event.clientX - rect.left;
        const percentage = offsetX / rect.width;
        onCurrentTimeChange(percentage * (audioBuffer?.duration || 1));
        // create a event listener for wait the player to have mouse up
        if (isMouseDown) {
            setIsMouseDown(false);
            return;
        }

        setIsMouseDown(true);
    };

    const handleToggleEditMode = () => {
    };

    const handleSwitchView = () => {
    };


    const [isLoadingData, setIsLoadingData] = useState(true);

    // Function to generate a fake sine wave dataset
    const generateFakeData = (length: number) => {
        const data: number[] = [];
        const frequency = 2; // Number of waves in the fake data
        const amplitude = 0.5; // Amplitude of the sine wave

        for (let i = 0; i < length; i++) {
            const value = amplitude * Math.sin((Math.PI * 2 * i * frequency) / length);
            data.push(value);
        }

        return data;
    };

    useEffect(() => {
        if (!isLoading && audioBuffer) {
            setIsLoadingData(false);
        }
    }, [isLoading, audioBuffer]);

    const handleAudioDrop = (event: React.DragEvent<HTMLDivElement>) => {
        event.preventDefault();
        event.stopPropagation();
        if (event.dataTransfer.files && event.dataTransfer.files[0]) {
            handleFileUpload(event.dataTransfer.files[0]);
        } else {
        }
    };



    useEffect(() => {
        const audio = playerRef.current;
        if (audio) {
            const onTimeUpdate = () => {
                onCurrentTimeChange(audio.currentTime || 0);
            };

            audio.addEventListener('timeupdate', onTimeUpdate);
            return () => {
                audio.removeEventListener('timeupdate', onTimeUpdate);
            }
        }
    }, [audioBuffer, onCurrentTimeChange]);

    useEffect(() => {
        const audio = playerRef.current;
        if (audio) {
            const onReadyToPlay = () => {
                setIsPlaying(true);
                audio.play();
            }

            audio.addEventListener('canplay', onReadyToPlay);

            return () => {
                audio.removeEventListener('canplay', onReadyToPlay);
            }
        }
    }, []);

    useEffect(() => {
        // calculate bar data

        if (!audioBuffer) return;
        const data = audioBuffer.getChannelData(0);
        const step = Math.floor(data.length / numberOfBars) || 1;
        const newData: number[] = [];
        for (let i = 0; i < numberOfBars; i++) {
            let sum = 0;
            for (let j = 0; j < step; j++) {
                sum += Math.abs(data[i * step + j]);
            }
            let average = sum / step;
            newData.push(average);

        }
        setBarData(newData);
    }, [audioBuffer, numberOfBars]);

    return (
        <div className={styles.audioWrapper}>
            <div className={styles.audioContainer}>
                {audioBuffer ? (
                    <div ref={audioRef} className={styles.audio} onClick={handleAudioClick}>
                        {/* Display loading animation if still loading audio data */}

                        {/* Render actual audio bars once audio data is loaded */}
                        <div className={styles.currentTimeLine} style={{ left: `${(currentTime / (audioBuffer?.duration || 1)) * 100}%` }} />
                        <AudioBars
                            handleFileUpload={(event => {
                                handleFileUpload(event);
                                setIsPlaying(false);
                            })}
                            barData={barData}
                            currentTime={currentTime}
                            audioBuffer={audioBuffer}
                            numberOfBars={numberOfBars}
                        />
                        <AudioGrid bpm={parseFloat(metadata?.bpm) || 160} offset={parseFloat(metadata?.offset) || 0} duration={audioBuffer?.duration || 1} />
                        <SceneDisplay selectedScene={selectedScene} scenes={scenes} />
                    </div>
                ) : (
                    <Upload onFileUpload={handleFileUpload} uploading={isLoading} acceptedTypes=".mp3,.wav,.ogg,.m4a,.aac,.flac" />
                )}
            </div>
        </div>
    );
};

export default AudioVisualizer;

// React Code
const SceneDisplay: React.FC<{ selectedScene: Scene | null, scenes: Scene[] }> = ({ selectedScene, scenes }) => {
    if (!selectedScene) {
        return null;
    }

    return (
        <div className={styles.sceneDisplayWrapper}>
            {
                scenes.map((scene, index) => {
                    const isSelected = selectedScene.scene_start === scene.scene_start;
                    const opacityBackground = isSelected ? 1.0 : 0.2;
                    const opacityName = isSelected ? 1.0 : 0.8;

                    return (
                        <div className={styles.sceneDisplay} key={index} style={{
                            left: `${scene.scene_start * 100}%`,
                            width: `${scene.scene_end * 100}%`,
                            height: '20%' // Set a fixed height for each scene
                        }}>
                            <div
                                className={styles.sceneDisplayBackground}
                                style={{
                                    backgroundColor: scene.scene_color,
                                    opacity: opacityBackground,
                                }}
                            />
                            <div
                                className={styles.sceneDisplayName}
                                style={{
                                    opacity: opacityName,
                                }}
                            >
                                {scene.scene_title}
                            </div>
                        </div>
                    );
                })
            }
        </div>
    );
}