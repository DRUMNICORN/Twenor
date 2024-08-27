// useMetadata.ts

import axios, { AxiosResponse } from 'axios';
import { useEffect, useState, useCallback } from 'react';

type Metadata = Record<string, any> | null;

const getMetadata = async (token: string, trackId?: string): Promise<Metadata> => {
    try {
        const url_bas = 'http://localhost:8000/api/metadata';
        // If trackId is provided, else crash
        if (!trackId) {
            throw new Error('trackId is required');
        }
        const url = trackId ? `${url_bas}/${trackId}` : url_bas;
        const response: AxiosResponse = await axios.get(url, {
            headers: {
                Authorization: `Bearer ${token}`,
            },
        });

        const data = response.data;

        if (Object.keys(data).length > 0) {
            return data;
        } else {
            return null;
        }
    } catch (error) {
        return null;
    }
};
const useMetadata = (token: string | null, trackId: string | null): [Metadata, (metadata: Metadata) => void] => {
    const [metadata, setMetadataState] = useState<Metadata>(null);
    const [metadataToUpdate, setMetadataToUpdate] = useState<Metadata | null>(null);

    const updateMetadata = useCallback(
        async (metadata: Metadata) => {
            try {
                const url_bas = 'http://localhost:8000/api/metadata';
                if (!trackId) {
                    throw new Error('YES trackId is required');
                }
                const url = trackId ? `${url_bas}/${trackId}` : url_bas;
                await axios.post(url, metadata, {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                });
                setMetadataToUpdate(null); // Clear metadataToUpdate after sending
            } catch (error) {
            }
        },
        [token, trackId, setMetadataToUpdate]
    );

    const setMetadata = useCallback(
        (metadata: Metadata) => {
            setMetadataToUpdate(metadata); // Update metadataToUpdate when metadata changes
            setMetadataState(metadata);
        },
        [setMetadataState]
    );

    useEffect(() => {
        const debouncedUpdate = setTimeout(() => {
            if (metadataToUpdate !== null) {
                updateMetadata(metadataToUpdate);
            } else {
            }
        }, 1234);

        return () => clearTimeout(debouncedUpdate);
    }, [metadataToUpdate, updateMetadata]);

    useEffect(() => {
        const fetchMetadata = async () => {
            if (!token || !trackId) {
                return;
            }
            const metadataResponse: Metadata = await getMetadata(token, trackId);
            setMetadataState(metadataResponse);
        };

        fetchMetadata();
    }, [token, setMetadataState, trackId]);

    return [metadata, setMetadata];
};

export default useMetadata;