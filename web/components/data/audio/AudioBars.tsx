import React, { useEffect, useState } from 'react';

import styles from './AudioBars.module.scss';
import { CONFIG } from '@/util/styles/theme';

interface Props {
    handleFileUpload?: (file: File) => void;
    barData: number[];
    currentTime?: number;
    audioBuffer?: AudioBuffer | null;
    numberOfBars: number;
    duration?: number;
}

const AudioBars: React.FC<Props> = ({ handleFileUpload, barData, currentTime, audioBuffer, numberOfBars, duration }) => {
    const [currentBarData, setCurrentBarData] = useState<number[]>(barData);
    const [currentNumberOfBars, setCurrentNumberOfBars] = useState<number>(numberOfBars);
    const [currentAudioBuffer, setCurrentAudioBuffer] = useState<AudioBuffer | null>(audioBuffer || null);

    useEffect(() => {
        const animationTimeout = setTimeout(() => {
            setCurrentBarData(barData);
            setCurrentNumberOfBars(numberOfBars);
            setCurrentAudioBuffer(audioBuffer || null);
        }, 100);

        return () => clearTimeout(animationTimeout);
    }, [barData, currentTime, audioBuffer, numberOfBars]);

    const calculateBarColor = (barHeight: number, isPastProgress: boolean): string => {
        const { PrimaryRGB, SecondaryRGB, AddRGB, SubRGB } = CONFIG;
        const primaryColor = PrimaryRGB.split(',').map(Number);
        const secondaryColor = SecondaryRGB.split(',').map(Number);

        let red, green, blue;

        if (isPastProgress) {

            red = Math.round(barHeight * primaryColor[0] + (1 - barHeight) * secondaryColor[0] / 42);
            green = Math.round(barHeight * primaryColor[1] + (1 - barHeight) * secondaryColor[1] / 42);
            blue = Math.round(barHeight * primaryColor[2] + (1 - barHeight) * secondaryColor[2] / 42);
        } else {
            const addColor = AddRGB.split(',').map(Number);
            const subColor = SubRGB.split(',').map(Number);

            red = Math.round(barHeight * primaryColor[0] + (1 - barHeight) * subColor[0]);
            green = Math.round(barHeight * primaryColor[1] + (1 - barHeight) * subColor[1]);
            blue = Math.round(barHeight * primaryColor[2] + (1 - barHeight) * subColor[2]);

            const brightnessIncreaseFactor = 4;

            red = Math.round(Math.min(red * brightnessIncreaseFactor, 255));
            green = Math.round(Math.min(green * brightnessIncreaseFactor, 255));
            blue = Math.round(Math.min(blue * brightnessIncreaseFactor, 255));
        }

        const clamp = (value: number): number => Math.max(0, Math.min(value, 255));
        const clampedRed = clamp(red);
        const clampedGreen = clamp(green);
        const clampedBlue = clamp(blue);

        return `rgb(${clampedRed}, ${clampedGreen}, ${clampedBlue})`;
    };

    const handleDrop = (e: React.DragEvent<HTMLDivElement>): void => {
        e.preventDefault();
        e.stopPropagation();
        if (e.dataTransfer.files && e.dataTransfer.files[0]) {
            if (handleFileUpload) {
                handleFileUpload(e.dataTransfer.files[0]);
            }
        } else {
            // Handle other cases
        }
    };

    // let audio_duration = duration || currentAudioBuffer?.duration;
    useEffect(() => {
        setAudioDuration(duration || currentAudioBuffer?.duration);
    }, [duration, currentAudioBuffer]);

    let [audio_duration, setAudioDuration] = useState(duration || currentAudioBuffer?.duration);

    return (
        <div onDrop={handleDrop} className={styles.audioBarsContainer} style={{ height: '100px' }}>
            {currentBarData.map((barHeight, index) => {
                const barProgress = (index / currentNumberOfBars) * 100;
                let isPastProgress = false;
                let isCurrentProgress = false;

                if (barProgress <= ((currentTime || 1) / (audio_duration || 1)) * 100) {
                    isPastProgress = true;
                }

                // check if its the current bar
                if (barProgress <= ((currentTime || 1) / (audio_duration || 1)) * 100 && barProgress + 100 / currentNumberOfBars >= ((currentTime || 1) / (audio_duration || 1)) * 100) {
                    isCurrentProgress = true;
                }
                const specialAudioColor = calculateBarColor(barHeight, isPastProgress);
                // get red value and add % to it of hight
                const barColor = `rgb(${barHeight * 255}, 0, 0)`;
                combineBarColors(barColor, specialAudioColor);

                return (
                    <div
                        key={index * 2}
                        className={`${styles.bar} ${isPastProgress ? styles.pastProgress : ''} ${isCurrentProgress ? styles.currentProgress : ''}`}
                        style={{
                            position: 'absolute',
                            bottom: 0,
                            left: `${barProgress}%`,
                            height: `${barHeight * 200 + 10}%`,
                            width: `${100 / currentNumberOfBars + 1}%`,
                            backgroundColor: specialAudioColor,
                            transition: 'all 0.3s ease-in-out',
                            opacity: 1,
                        }}
                    />
                );
            })}
        </div>
    );
};

export default AudioBars;

const combineBarColors = (barColor: string, specialAudioColor: string) => {
    const barColorArray = barColor.split(',');
    const specialAudioColorArray = specialAudioColor.split(',');

    const red = Math.round((Number(barColorArray[0].split('(')[1]) + Number(specialAudioColorArray[0].split('(')[1])) / 2);
    const green = Math.round((Number(barColorArray[1]) + Number(specialAudioColorArray[1])) / 2);
    const blue = Math.round((Number(barColorArray[2].split(')')[0]) + Number(specialAudioColorArray[2].split(')')[0])) / 2);

    return `rgb(${red}, ${green}, ${blue})`;
}
