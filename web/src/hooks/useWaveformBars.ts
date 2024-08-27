import { useMemo } from "react";

function processAudioData(audioData: Float32Array, numberOfBars: number): number[] {
    const barData: number[] = [];
    const samplesPerBar = Math.floor(audioData.length / numberOfBars);

    for (let i = 0; i < numberOfBars; i++) {
        const start = i * samplesPerBar;
        const end = (i + 1) * samplesPerBar;
        const section = Array.from(audioData.slice(start, end));

        // Calculate the maximum value within the section
        const maxInSection = Math.max(...section);

        // Normalize the values within the section based on the maximum value
        const normalizedSection = section.map((val) => val / maxInSection);

        // Calculate the average of the normalized values
        const average = normalizedSection.reduce((acc, val) => acc + val, 0) / normalizedSection.length;

        barData.push(average);
    }

    let maxInBarData = Math.max(...barData);
    let minInBarData = Math.min(...barData);
    // map the values to a range from 0 to 1
    let newData = barData.map((val) => (val - minInBarData) / (maxInBarData - minInBarData));
    return newData;
}



const useWaveformBars = (audioBuffer, numberOfBars) => {
    const barData = useMemo(() => {
        if (audioBuffer) {
            const audioData = audioBuffer.getChannelData(0); // Get the audio data as Float32Array
            const barData = processAudioData(audioData, numberOfBars);
            return barData;
        }
        return [];
    }, [audioBuffer, numberOfBars]);

    return barData;
};

export default useWaveformBars;