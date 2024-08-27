import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';

/**
 * 
impl FeaturesPackage {
    pub fn new(track_id: i32, features: Vec<Features>) -> FeaturesPackage {
        FeaturesPackage {
            track_id,
            features,
        }
    }

    pub fn insert(&mut self, feature: Features) {
        self.features.push(feature);
    }
    
}

// Define the SceneRequest type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub danceability: f64,
    pub valence: f64,
    pub energy: f64,
    pub tempo: f64,
    pub loudness: f64,
    pub speechiness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub acousticness: f64,
    pub key: f64,
    pub mode: f64,
    pub duration: f64,
    pub time_signature: f64,
}

 */

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
    track_id: number,
    features: Features[],
}

const useFeatures = (trackId: string | null, token: string, trackState: string): { features: FeaturesPackage; isLoading: boolean; error: string | null } => {
    const [features, setFeatures] = useState<FeaturesPackage>({ track_id: 0, features: [] });
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const calculateFeatures = async () => {
            if (!trackId) {
                setError('No trackId provided.');
                return;
            }

            if (!token) {
                setError('No token provided.');
                return;
            }

            setIsLoading(true);
            try {
                const url = `http://localhost:8000/api/features/${trackId}`;
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
                setIsLoading(false);
            } catch (error) {
                setError('Failed to calculate features.');
                setIsLoading(false);
            }
        };

        calculateFeatures();
    }, [trackId, token, trackState]);

    return { features, isLoading, error };
};

export default useFeatures;
