import { useCallback, useEffect, useState } from 'react';
import axios from 'axios';
import useMetadata from './useMetadata';

type Metadata = Record<string, any> | null;

interface UseFileHookResult {
    metadata: Metadata;
    setMetadata: (metadata: Metadata) => void;
    trackId: string | null;
    error: string | null;
}

const useFile = (file: File | null, token: string | null): UseFileHookResult => {
    const [trackId, setTrackId] = useState<string | null>(null);
    const [metadata, setMetadata] = useMetadata(token, trackId);
    const [error, setError] = useState<string | null>(null);

    const uploadFileInformation = useCallback(async () => {
        if (!file || !token) {
            return false;
        }

        const fileInfo = {
            name: file.name || '',
            size: file.size || 0,
            mime_type: file.type || '',
        };

        try {
            const response = await axios.post(
                'http://localhost:8000/api/upload',
                fileInfo,
                {
                    headers: {
                        Authorization: `Bearer ${token}`,
                        'Content-Type': 'application/json',
                    },
                }
            );

            const data = response.data;
            if (data) {
                setTrackId(data);
                return true;
            } else {
                setError('No data returned from server');
                return false;
            }
        } catch (error) {
            setError(error.message);
            return false;
        }
    }, [file, token]);

    const uploadActualFile = useCallback(async () => {
        if (!file || !token || !trackId) {
            return false;
        }

        const formData = new FormData();
        formData.append('file_data', file);

        try {
            await axios.post(`http://localhost:8000/api/file/${trackId}`, formData, {
                headers: {
                    Authorization: `Bearer ${token}`,
                    'Content-Type': 'multipart/form-data',
                },
            });
            return true;
        } catch (error) {
            setError(error.message);
            return false;
        }
    }, [file, token, trackId]);

    useEffect(() => {
        const uploadAndFetchMetadata = async () => {
            const fileInfoUploaded = await uploadFileInformation();
            if (fileInfoUploaded) {
                await uploadActualFile();
            }
        };

        if (file && token) {
            uploadAndFetchMetadata();
        }
    }, [file, token, uploadFileInformation, uploadActualFile]);

    return { metadata, setMetadata, trackId, error };
};

export default useFile;
