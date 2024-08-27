import React, { useEffect, useState } from 'react';
import styles from './BeatGrid.module.scss';

type BeatGridProps = {
    bpm: number;
    offset: number;
    duration: number;
};

const BeatGrid: React.FC<BeatGridProps> = ({ bpm, offset, duration }) => {
    const [beats, setBeats] = useState<number[]>([]);

    useEffect(() => {
        // calculate beats with bpm and offset and duration
        let beats: number[] = [];
        let beat: number = offset * 60 / bpm * 16;
        while (beat < duration) {
            beats.push(beat);
            beat += 60 / bpm * 16;
        }
        setBeats(beats);
    }, [bpm, offset, duration]);

    return (
        <div className={styles.beatGrid}>
            {beats.map((beat, index) => (
                <div
                    key={index}
                    className={styles.beat}
                    style={{
                        left: `${(beat / duration) * 100}%`,
                        height: `${index % 4 === 0 ? '60%' : '42%'}`,
                    }} // add z-index here
                />
            ))}
        </div>
    );
};


export default BeatGrid;