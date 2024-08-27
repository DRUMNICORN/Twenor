import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';
import { Scene } from '@/components/data/video/VideoScenes';

// props will be sceneRequest but ... is used to destructure the object
const useScenes = (audioId: string | null, token: string, audio_state: string): { scenes: Scene[]; isLoading: boolean; error: string | null } => {
    const [scenes, setScenes] = useState<Scene[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const loadScenes = async () => {
            if (!audioId) {
                return;
            }

            setIsLoading(true);
            try {
                const url = `https://api.drumni.com/api/scenes/${audioId}`;
                const response: AxiosResponse = await axios.post(
                    url,
                    {
                        headers: {
                            Authorization: `Bearer ${token}`,
                            'Content-Type': 'application/json',
                        },
                    }
                );

                if (response.data.error) {
                    setError(response.data.error);
                    setIsLoading(false);
                    setScenes([]);
                    return;
                }
                if (!response.data.scenes && response.data.length != 0) {
                    setError('Failed to load scenes.');
                    setIsLoading(false);
                    return;
                }
                for (let i = 0; i < response.data.scenes.length; i++) {
                    response.data.scenes[i].scene_index = i;
                }
                setScenes(response.data.scenes || []);
                setIsLoading(false);
                setError('');
            } catch (error) {
                setScenes([]);
                setError(`Failed to load because of: ${error}`);
                setIsLoading(false);
            }
        };

        loadScenes();
    }, [token, audioId, audio_state]);

    return { scenes, isLoading, error };
};

export default useScenes;