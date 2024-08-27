import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';

const useCorrelation = (audioId: string | null, token: string, audioState: string): { correlation: number[]; isLoading: boolean; error: string | null } => {
    const [correlation, setCorrelation] = useState<number[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const calculateCorrelation = async () => {
            if (!audioId) {
                return;
            }

            if (!token) {
                setError('No token provided.');
                return;
            }

            setIsLoading(true);
            try {
                const url = `https://api.drumni.com/api/correlation/${audioId}`;
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
                setError(null);
                setCorrelation(response.data.correlation_values);
                setIsLoading(false);
            } catch (error) {
                setError('Failed to calculate correlation.');
                setIsLoading(false);
            }
        };

        calculateCorrelation();
    }, [audioId, token, audioState]);

    return { correlation, isLoading, error };
};

export default useCorrelation;
