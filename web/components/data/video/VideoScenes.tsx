
import React, { useState, useEffect, useRef } from 'react';
import styles from './VideoScenes.module.scss';
import Image from 'next/image';

interface Scene {
    scene_id: number;
    audio_id: number;
    scene_title: string;
    scene_description: string;
    scene_tags: string;
    scene_color: string;
    scene_chunks: string[];
    scene_start: number;
    scene_end: number;
    scene_index: number;
}

interface ScenesProps {
    scenes: Scene[];
    handleSceneClick: (scene: Scene | null) => void;
    selectedScene: Scene | null;
}

const VideoScenes: React.FC<ScenesProps> = ({ scenes, handleSceneClick, selectedScene }) => {
    const [localScenes, setLocalScenes] = useState<Scene[]>(scenes);

    const scenesContainerRef = useRef<HTMLDivElement>(null);
    const prevMouseXRef = useRef<number | null>(null);
    const [isDragging, setIsDragging] = useState(false);
    const [draggedScene, setDraggedScene] = useState<Scene | null>(null);
    useEffect(() => {
        setLocalScenes(scenes);
        handleSceneClick(scenes[0]);
    }, [scenes]);

    const handleDragStart = (e: React.DragEvent<HTMLDivElement>, scene: Scene) => {
        setDraggedScene(scene);
        setIsDragging(true);
    };

    const handleDragEnd = () => {
        setDraggedScene(null);
        prevMouseXRef.current = null;
        setIsDragging(false);
    };

    const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
        if (isDragging && draggedScene) {
            const scenesContainer = scenesContainerRef.current;
            if (!scenesContainer) return;

            const scenesContainerRect = scenesContainer.getBoundingClientRect();
            const containerLeft = scenesContainerRect.left;

            const mouseX = e.clientX - containerLeft;

            if (prevMouseXRef.current !== null) {
                const deltaX = mouseX - prevMouseXRef.current;
                if (deltaX !== 0) {
                    const direction = deltaX > 0 ? MoveDirection.RIGHT : MoveDirection.LEFT;
                    const threshold = 1; // Adjust the threshold value as needed
                    if (Math.abs(deltaX) >= threshold) {
                        handleUpdateScene(direction);
                        prevMouseXRef.current = mouseX; // Update the previous mouse position after scene update
                    }
                }
            } else {
                prevMouseXRef.current = mouseX; // Set the initial mouse position
            }
        }
        if (!isDragging && prevMouseXRef.current !== null) {
            handleDragEnd();
        }
    };


    enum MoveDirection {
        LEFT = -1,
        RIGHT = 1,
    }

    const handleUpdateScene = (direction: MoveDirection) => {
        const updatedScenes = [...localScenes];
        const leftIndex = updatedScenes.findIndex(scene => scene.scene_id === draggedScene?.scene_id);
        const leftScene = updatedScenes[leftIndex];
        const rigthScene = updatedScenes[leftIndex + 1];
        if (!rigthScene || !leftScene) {
            return;
        }

        const isMakingSceneLonger = direction === MoveDirection.RIGHT;
        const isMakingSceneShorter = direction === MoveDirection.LEFT;
        let grid_duration = (leftScene.scene_end - leftScene.scene_start) / leftScene.scene_chunks.length;

        // check if both scenes have more than 1 chunk
        if (isMakingSceneLonger && rigthScene.scene_chunks.length <= 1) {
            return;
        }
        if (isMakingSceneShorter && leftScene.scene_chunks.length <= 1) {
        }

        // update scene start and end
        if (isMakingSceneLonger) {
            rigthScene.scene_start -= grid_duration;
            leftScene.scene_end += grid_duration;
        }
        if (isMakingSceneShorter) {
            rigthScene.scene_start += grid_duration;
            leftScene.scene_end -= grid_duration;
        }

        // update scene chunks
        if (isMakingSceneLonger) {
            rigthScene.scene_chunks.shift();
            leftScene.scene_chunks.push(rigthScene.scene_chunks[0]);
        }

        if (isMakingSceneShorter) {
            rigthScene.scene_chunks.unshift(leftScene.scene_chunks.pop() || "");
        }
    };
    return (
        <>
            <div ref={scenesContainerRef}
                onMouseMove={handleMouseMove} className={styles.scenesContainer}>
                {localScenes.map((scene, index) => {
                    let isSceneSelected = false;
                    if (selectedScene) {
                        isSceneSelected = selectedScene.scene_id === scene.scene_id;
                    }

                    let local_duration = (scene.scene_end - scene.scene_start);

                    return (
                        <React.Fragment key={index * 2}>
                            <div
                                className={`${styles.scene} ${isSceneSelected ? styles.selectedScene : ''}`}
                                onClick={() => handleSceneClick(scene)}
                                style={{
                                    width: `${local_duration}%`,
                                    backgroundColor: scene.scene_color,
                                    color: scene.scene_color,
                                }}
                            >
                                <div className={styles.sceneName}>{index + 1}</div>
                            </div>
                            {index !== localScenes.length - 1 && (
                                <div
                                    onDragStart={(e) => handleDragStart(e, scene)}
                                    onDragLeave={handleDragEnd}
                                    onDragEnd={handleDragEnd}
                                    draggable={true}
                                    className={styles.resizeBar}
                                    style={{
                                    }}
                                />
                            )}
                        </React.Fragment>
                    );
                })}
            </div>
            <div className={styles.sceneInfo}>
                <div className={styles.sceneInfoTitle}>{selectedScene?.scene_title}</div>
                <div className={styles.sceneInfoDescription}>{selectedScene?.scene_description}</div>
            </div>
        </>
    );
};

export default VideoScenes;
export type { Scene };


