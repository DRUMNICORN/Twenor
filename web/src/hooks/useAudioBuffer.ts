import { useEffect, useState } from "react";

const useAudioBuffer = (file) => {
    const [audioBuffer, setAudioBuffer] = useState<AudioBuffer | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<Error | null>(null);

    useEffect(() => {
        let audioContext: AudioContext | null = null;

        const loadWaveform = async () => {
            setIsLoading(true);

            try {
                if (!file) {
                    throw new Error('No file provided');
                }


                const blob = file ? new Blob([file]) : null; // Create a blob from the file
                if (!blob) {
                    throw new Error('No file provided');
                }
                const blobUrl = URL.createObjectURL(blob);

                const response = await fetch(blobUrl, {
                    method: 'GET',
                    headers: {
                        'Content-Type': file.type || 'audio/mpeg',
                    },
                });

                const arrayBuffer = await response.arrayBuffer();
                audioContext = new AudioContext();
                const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

                setAudioBuffer(audioBuffer);
                setIsLoading(false);
            } catch (error) {
                setError(error);
                setIsLoading(false);
            }
        };

        loadWaveform();

        return () => {
            if (audioContext) {
                audioContext.close();
            }
        };
    }, [file]);

    return { audioBuffer, isLoading, error };
};

export default useAudioBuffer;