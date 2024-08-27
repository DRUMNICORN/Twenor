"use client"

import axios, { AxiosResponse } from 'axios';
import { useEffect, useState, useCallback } from 'react';

type Metadata = Record<string, any> | null;

const getMetadata = async (token: string, audioId?: string): Promise<Metadata> => {
    try {
        const url_bas = 'https://api.drumni.com/api/metadata';
        if (!audioId) {
            throw new Error('audioId is required');
        }

        const url = audioId ? `${url_bas}/${audioId}` : url_bas;

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



interface UseMetadataHookResult {
    transferedMetadata: Metadata,
    setMetadata: (metadata: Metadata) => void;
    isLoading: boolean;
    error: string | null;
};

const useVideoMetadata = (token: string | null, audio_id: string | null, audio_state: string): UseMetadataHookResult => {
    const [metadata, setMetadataState] = useState<Metadata>(null);
    const [metadataToUpdate, setMetadataToUpdate] = useState<Metadata | null>(null);
    const [metadata_id, setMetadata_id] = useState<string | null>(null);
    const [user_id, setUser_id] = useState<string | null>(null);

    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);


    const [transferedMetadata, setTransferedMetadata] = useState<Metadata | null>(null);

    const updateMetadata = useCallback(
        async (metadata: Metadata) => {
            setIsLoading(true);
            metadata = {
                ...metadata,
                metadata_id: metadata_id || -1,
                user_id: user_id || -1,
                audio_id: audio_id || -1,
            };

            metadata.bpm = Number(metadata.bpm);
            metadata.offset = Number(metadata.offset);

            try {
                const url_bas = 'https://api.drumni.com/api/metadata';
                if (!audio_id) {
                    console.log('audioId is required');
                    throw new Error('audioId is required');
                }

                const url = audio_id ? `${url_bas}/${audio_id}` : url_bas;
                await axios.post(url, metadata, {
                    headers: {
                        Authorization: `Bearer ${token}`,
                    },
                });
                setMetadataToUpdate(null);
            } catch (error) {
                console.log(error);
                setError("Error updating metadata");
            } finally {
                setIsLoading(false);
            }
        },
        [token, audio_id, setMetadataToUpdate]
    );

    useEffect(() => {
        if (metadata) {
            const { metadata_id, audio_id, user_id, ...rest } = metadata;
            if (metadata?.bpm > 420) {
                if (metadata) { metadata.bpm = 128; }
            }
            setTransferedMetadata(rest);
        }
    }, [metadata]);


    const setMetadata = useCallback(
        (metadata: Metadata) => {
            if (metadata?.bpm > 420) {
                if (metadata) { metadata.bpm = 128; }
            }
            setMetadataToUpdate(metadata);
            setMetadataState(metadata);
        },
        [setMetadataState]
    );

    useEffect(() => {
        const debouncedUpdate = setTimeout(() => {
            if (metadataToUpdate !== null) {
                if (metadataToUpdate?.bpm > 420) {
                    if (metadataToUpdate) { metadataToUpdate.bpm = 128; }
                }
                updateMetadata(metadataToUpdate);
            }
        }, 1234);

        return () => clearTimeout(debouncedUpdate);
    }, [metadataToUpdate, updateMetadata]);

    useEffect(() => {

        const fetchMetadata = async () => {
            if (!token || !audio_id) {
                setMetadataState(null);
                setTransferedMetadata(null);
                setMetadata_id(null);
                setUser_id(null);
                return;
            }
            setIsLoading(true);
            const metadataResponse: Metadata = await getMetadata(token, audio_id);
            setMetadata_id(metadataResponse?.metadata_id || -1);
            setUser_id(metadataResponse?.user_id || -1);
            setMetadataState(metadataResponse);

            delete metadataResponse?.metadata_id;
            delete metadataResponse?.user_id;
            delete metadataResponse?.audio_id;

            if (metadataResponse?.bpm) {
                metadataResponse.bpm = metadataResponse.bpm.toString();
            }
            if (metadataResponse?.offset) {
                metadataResponse.offset = metadataResponse.offset.toString();
            }
            setTransferedMetadata(metadataResponse);
            setIsLoading(false);
        };

        fetchMetadata();
    }, [token, setMetadataState, audio_id, audio_state]);

    return { transferedMetadata, setMetadata, isLoading, error };
};

export default useVideoMetadata;