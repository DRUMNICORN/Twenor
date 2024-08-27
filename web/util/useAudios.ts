import { useEffect, useState } from "react";

export interface Audio {
    id: string;
    title: string;
    state: string;
}

interface UseAudiosHookResult {
    isLoading: boolean;
    audios: Audio[] | null;
    error: string | null;
    deleteAudio: (audioId: string) => void;
    reloadAudios: () => void;
}

const getAudios = async (token: string): Promise<Audio[] | null> => {
    const response = await fetch(`https://api.drumni.com/api/audios`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`,
        },
    });
    if (response.ok) {
        const data = await response.json();
        return data;
    }


    return null;
};


const useAudios = (token: string | null): UseAudiosHookResult => {
    const [audios, setAudios] = useState<Audio[] | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        const fetchAudios = async () => {
            if (isLoading) {
                setError('Already fetching');
                return;
            }

            if (!token) {
                setError('No token provided');
                return;
            }

            setIsLoading(true);
            const audios = await getAudios(token);
            if (audios) {
                setAudios(audios);
            } else {
                setError('Error fetching audios');
            }
            setIsLoading(false);
        };

        if (!token) {
            setError('No token provided');
        } else if (token) {
            fetchAudios();
        }
    }, [token]);

    const reloadAudios = async () => {
        if (!token) {
            setError('No token provided');
            return;
        }

        const audios = await getAudios(token);
        if (audios) {
            setAudios(audios);
        } else {
            setError('Error fetching audios');
        }
    };

    const deleteAudio = async (audioId: string) => {
        if (!token) {
            setError('No token provided');
            return;
        }

        const response = await fetch(`https://api.drumni.com/api/audio/${audioId}`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                Authorization: `Bearer ${token}`,
            },
        });
        if (response.ok) {
            const audios = await getAudios(token);
            if (audios) {
                setAudios(audios);
            } else {
                setError('Error fetching audios');
            }
        } else {
            setError('Error deleting audio');
        }
    };




    return { audios, error, isLoading, deleteAudio, reloadAudios };
}

export default useAudios;