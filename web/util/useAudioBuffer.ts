import { useEffect, useState } from "react";

const useAudioBuffer = (file: File | null) => {
    const [audioBuffer, setAudioBuffer] = useState<AudioBuffer | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [error, setError] = useState<Error | null>(null);

    useEffect(() => {
        let audioContext: AudioContext | null = null;

        console.log('loading audio');

        const loadAudio = async () => {
            setIsLoading(true);

            try {
                if (!file) {
                    console.log('No file provided');
                    setIsLoading(false);
                    setAudioBuffer(null);
                    return;
                }

                // const blob = file ? new Blob([file]) : null; // Create a blob from the file
                // if (!blob) {
                //     console.log('No blob created');
                //     setIsLoading(false);
                //     setAudioBuffer(null);
                //     return;
                // }
                // const blobUrl = URL.createObjectURL(blob);

                // const response = await fetch(blobUrl, {
                //     method: 'GET',
                //     headers: {
                //         'Content-Type': file.type,
                //     },
                // });

                // const arrayBuffer = await response.arrayBuffer();
                // audioContext = new AudioContext();
                // const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);


                try {

                    let audioContext = new AudioContext();
                    const arrayBuffer = await file.arrayBuffer();
                    console.log('arrayBuffer', arrayBuffer);
                    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
                    console.log('audioBuffer', audioBuffer);
                    setAudioBuffer(audioBuffer);
                }
                catch (e) {
                    console.warn('weror', e);
                    const blob = file ? new Blob([file], { type: file.type }) : null; // Create a blob from the file
                    if (!blob) {
                        console.log('No blob created');
                        setIsLoading(false);
                        setAudioBuffer(null);
                        return;
                    }
                    const blobUrl = URL.createObjectURL(blob);
                    console.log(file.type)
                    const response = await fetch(blobUrl, {
                        method: 'GET',
                        headers: {
                            'Content-Type': file.type,
                        },
                    });
                    const arrayBuffer = await response.arrayBuffer();
                    audioContext = new AudioContext();
                    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
                    setAudioBuffer(audioBuffer);
                }


                // display first 100 chars of arrayBuffer and audioBuffer

                setIsLoading(false);
                setError(null);
                console.log('done loading audio');
            } catch (error: any) {
                setError(error);
                setIsLoading(false);
                setAudioBuffer(null);
                console.error(error);
            }
        };

        loadAudio();

        return () => {
            if (audioContext) {
                audioContext.close();
            }
        };
    }, [file]);

    return { audioBuffer, isLoading, error };
};

export default useAudioBuffer;
