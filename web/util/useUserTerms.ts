import { useCallback, useState, useEffect } from 'react';
import axios, { AxiosResponse } from 'axios';

const API_URL = 'https://api.drumni.com/api/terms';

const useUserTerms = (client_id: string | null, token: string | null) => {
    const [termsAccepted, setTermsAccepted] = useState<boolean | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    const loadTerms = useCallback(async () => {
        if (!client_id) {
            return;
        }

        if (!token) {
            setError('No token provided.');
            return;
        }

        setIsLoading(true);
        try {
            const url = `${API_URL}/${client_id}`;
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
            setTermsAccepted(response.data);
            setError(null);
            setIsLoading(false);
        } catch (error) {
            setError('Failed to load terms.');
            setIsLoading(false);
        }

    }, [client_id, token]);

    useEffect(() => {
        loadTerms();
    }, [loadTerms]);

    const acceptTerms = useCallback(async () => {
        if (!client_id) {
            setError('No client ID provided.');
            return;
        }

        if (!token) {
            setError('No token provided.');
            return;
        }

        try {
            const url = `${API_URL}/${client_id}/accept`;
            await axios.post(
                url,
                {},
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                }
            );
            setTermsAccepted(true);
            setError(null);
        } catch (error) {
            setError('Failed to accept terms.');
        }
    }, [client_id, token]);

    return { termsAccepted, isLoading, error, acceptTerms };
};

export default useUserTerms;
