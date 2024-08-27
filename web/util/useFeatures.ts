import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';


export type Features =
    {
        danceability: number,
        valence: number,
        energy: number,
        tempo: number,
        loudness: number,
        speechiness: number,
        instrumentalness: number,
        liveness: number,
        acousticness: number,
        key: number,
        mode: number,
        duration: number,
        time_signature: number,
    }

type FeaturesPackage = {
    audio_id: number,
    features: Features[],
}

const useFeatures = (audioId: string | null, token: string, audioState: string): { features: FeaturesPackage; isLoading: boolean; error: string | null } => {
    const [features, setFeatures] = useState<FeaturesPackage>({ audio_id: 0, features: [] });
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const calculateFeatures = async () => {
            if (!audioId) {
                return;
            }

            if (!token) {
                setError('No token provided.');
                return;
            }

            setIsLoading(true);
            try {
                const url = `https://api.drumni.com/api/features/${audioId}`;
                const response: AxiosResponse = await axios.get(
                    url,
                    {
                        headers: {
                            Authorization: `Bearer ${token}`,
                        },
                    }
                );

                if (response.data.error) {
                    setError(response.data.error);
                    setIsLoading(false);
                    return;
                }
                setFeatures(response.data);
                setError(null);
                setIsLoading(false);
            } catch (error) {
                setError('Failed to calculate features.');
                setIsLoading(false);
            }
        };

        calculateFeatures();
    }, [audioId, token, audioState]);

    return { features, isLoading, error };
};

export default useFeatures;
