// import { useMemo } from "react";

import { Scene } from "@/components/data/video/VideoScenes";
import { useMemo, useState } from "react";

interface UseVideoHookResult {
    videoFile: HTMLVideoElement | null;
    isLoading: boolean;
    error: string | null;
}

const useVideo = (currentScene: Scene | null, user_id: string | null, audio_id: string | null): UseVideoHookResult => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const MAIN_PATH = "https://api.drumni.com/videos/";
    const videoFile = useMemo(() => {
        if (!currentScene) {
            return null;
        }
        const video = document.createElement("video");
        video.src = MAIN_PATH + user_id + "/" + audio_id + "/" + currentScene.scene_index + ".mp4";
        video.loop = true;
        video.muted = true;
        video.play();
        return video;
    }, [currentScene]);

    return {
        videoFile: videoFile as HTMLVideoElement | null,
        isLoading,
        error,
    };
};


export default useVideo;