import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';
import { Scene } from '@/components/common/media/Scenes';
import { headers } from 'next/dist/client/components/headers';

// props will be sceneRequest but ... is used to destructure the object
const useScenes = (trackId: string | null, token: string, track_state: string): { scenes: Scene[]; isLoading: boolean; error: string | null } => {
    const [scenes, setScenes] = useState<Scene[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {


        const loadScenes = async () => {
            if (!trackId) {
                return;
            }

            setIsLoading(true);
            try {
                const url = `http://localhost:8000/api/scenes/${trackId}`;
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
                    return;
                }
                if (!response.data.scenes) {
                    setError('Failed to load scenes.');
                    setIsLoading(false);
                    return;
                }
                setScenes(response.data.scenes);
                setIsLoading(false);
            } catch (error) {
                setError('Failed to load scenes.');
                setIsLoading(false);
            }
        };

        loadScenes();
    }, [token, trackId, track_state]);

    return { scenes, isLoading, error };
};

export default useScenes;