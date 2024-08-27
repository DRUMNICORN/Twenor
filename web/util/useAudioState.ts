import axios from 'axios';
import { useEffect, useState } from 'react';

const useAudioState = (token: string | null, audioId: string | null, updateInterval: number) => {
    const [audioState, setAudioState] = useState<string>('loading');
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    useEffect(() => {
        if (!audioId) {
            setAudioState('');
            setError('No audio ID provided');
            setIsLoading(false);
            return;
        }

        const loadAudioState = async () => {
            setIsLoading(true);
            setError('');

            try {
                const response = await axios.get(`https://api.drumni.com/api/audiostate/${audioId}`, {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                });

                if (response.data.error || !response.data) {
                    setError('Error fetching audio state');
                } else {
                    const state_str = response.data;
                    setError('');
                    setAudioState(state_str);
                }
            } catch (error) {
                setError('Error fetching audio state');
            } finally {
                setIsLoading(false);
            }
        };

        loadAudioState();

        // Set up the interval for auto-updates
        const intervalId = setInterval(loadAudioState, updateInterval);

        // Clean up the interval when the component unmounts
        return () => clearInterval(intervalId);
    }, [token, audioId, updateInterval]);

    return { audioState, isLoading, error };
};

export default useAudioState;
