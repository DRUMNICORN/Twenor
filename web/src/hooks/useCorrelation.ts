import { useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';

const useCorrelation = (trackId: string | null, token: string, trackState: string): { correlation: number[]; isLoading: boolean; error: string | null } => {
    const [correlation, setCorrelation] = useState<number[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const calculateCorrelation = async () => {
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
                const url = `http://localhost:8000/api/correlation/${trackId}`;
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
                setCorrelation(response.data.correlation_values);
                setIsLoading(false);
            } catch (error) {
                setError('Failed to calculate correlation.');
                setIsLoading(false);
            }
        };

        calculateCorrelation();
    }, [trackId, token, trackState]);

    return { correlation, isLoading, error };
};

export default useCorrelation;
