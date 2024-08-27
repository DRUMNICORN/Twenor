// VideoSettings.tsx
import React, { useState, useEffect } from 'react';
import styles from './VideoSettings.module.scss';

interface VideoSettingsProps {
    settings: Record<string, any>;
    onChanged?: (updatedVideoSettings: Record<string, any>) => void;
}

const VideoSettings: React.FC<VideoSettingsProps> = ({ settings, onChanged }) => {
    const [editedMetadata, setEditedMetadata] = useState<Record<string, any>>(settings); // Initialize with 'settings' prop

    const handleValueChange = (key: string, value: any) => {
        const updatedVideoSettings = { ...editedMetadata, [key]: value };
        setEditedMetadata(updatedVideoSettings);
        if (onChanged) {
            onChanged(updatedVideoSettings);
        }
    };

    const formatKey = (key: string) => {
        return key.charAt(0).toUpperCase() + key.slice(1);
    };

    const formatValue = (key: string, value: any) => {
        if (!key) {
            return;
        }
        if (key === 'lyrics') {
            return (
                <textarea
                    value={value}
                    onChange={(e) => handleValueChange(key, e.target.value)}
                    className={styles.lyricsInput}
                />
            );
        }
        return (
            <input type="text"
                value={value}
                onChange={(e) => handleValueChange(key, e.target.value)}
            />
        );
    };

    useEffect(() => {
        setEditedMetadata(settings);
    }, [settings]);

    return (
        <div
            className={styles.settings}
        >
            <div className={styles.settingsInside}>
                {Object.entries(settings).map(([key, value], index) => (
                    <div key={index} className={`${styles.settingsRow} ${key === 'lyrics' ? styles.lyricsRow : ''}`}>
                        <div className={styles.settingsKey}>{
                            key === 'lyrics' ? 'Lyrics or Description' : formatKey(key)
                        }</div>
                        <div className={styles.settingsValue}>{formatValue(key, editedMetadata[key])}</div>
                    </div>
                ))}
            </div>
        </div >
    );
};

export default VideoSettings;
