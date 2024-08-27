import React, { useState, useEffect } from 'react';
import styles from './AudioSpectrogram.module.scss';
import { Features } from '@/util/useFeatures';
import Loading from '@/components/util/Loading';

type FeatureDisplay = {
    x: number;
    y: number;
    color: string;
}

const AudioSpectrogram: React.FC<{ features: Features[], currentDisplayTime: number, duration: number }> = ({ features, currentDisplayTime, duration }) => {
    const [renderedFeatures, setRenderedFeatures] = useState<FeatureDisplay[]>([]);
    const [renderedPercentage, setRenderedPercentage] = useState(0);

    useEffect(() => {
        let newRenderedFeatures: any[] = [];
        features.forEach((feature: Features, i) => {
            let keys = Object.keys(feature);
            keys.forEach((key, ii) => {
                let value: number = feature[key as keyof Features] as number;
                let newFeature: FeatureDisplay = {
                    x: i,
                    y: ii,
                    color: mapValueToColor(value)
                };

                newRenderedFeatures.push(newFeature);
            });
        });
        setRenderedFeatures(newRenderedFeatures);
    }, [features]);

    useEffect(() => {
        let percentage = (currentDisplayTime / duration) * 100;
        setRenderedPercentage(percentage);
    }, [currentDisplayTime, duration]);



    // features is a list of objects and we want to display the values as colored boxes in a gid system
    return (
        <div className={styles.featuresWrapper}>
            <div className={styles.featuresContainer}>
                {
                    renderedFeatures.length > 0 ?
                        <>
                            <Loading />
                        </>
                        :
                        renderedFeatures.map((feature: { color: any; y: number; x: number; }, i: number) => {
                            let length = renderedFeatures.length;
                            let percentage = ((1 / (length / 1.58)) * 1000);
                            return (
                                <div
                                    key={i}
                                    className={styles.feature}
                                    style={{
                                        backgroundColor: feature.color,
                                        position: 'absolute',
                                        top: `${feature.y * 0.24}rem`,
                                        left: `${(feature.x) * percentage}%`,
                                        width: `${percentage + 1}%`,
                                        height: `${10000 / renderedFeatures.length} % `
                                    }}
                                />
                            );
                        })
                }
            </div>
            {/* display of current time will be a white bar which will move to current pos*/}
            <div className={styles.timeDisplay} style={{ left: `${renderedPercentage / 1.234}% ` }} />

        </div>
    );
}

const mapValueToColor = (value: number) => {
    let red = 255;
    let blue = 0;
    let green = 0;

    if (value < 0.5) {
        red = 255;
        blue = 255 * (value * 2);
    } else {
        blue = 255;
        red = 255 - (255 * ((value - 0.5) * 2));
    }

    return `rgba(${red}, ${green}, ${blue}, 1)`;
}
export default AudioSpectrogram;