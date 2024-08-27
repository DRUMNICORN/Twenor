// Settings.tsx
import React, { useState, useEffect } from 'react';
import styles from './Settings.module.scss';
import Help from '../../dom/Help';

interface SettingsProps {
    settings: Record<string, any>;
    onChanged: (updatedSettings: Record<string, any>) => void;
}

const Settings: React.FC<SettingsProps> = ({ settings, onChanged }) => {
    const [editedSettings, setEditedSettings] = useState<Record<string, any>>(settings);

    useEffect(() => {
        setEditedSettings(settings);
    }, [settings]);

    const handleValueChange = (key: string, value: any) => {
        const updatedSettings = { ...editedSettings, [key]: value };
        setEditedSettings(updatedSettings);
        onChanged(updatedSettings);
    };

    const formatSettingsKey = (key: string) => {
        return key.charAt(0).toUpperCase() + key.slice(1);
    };

    const formatSettingsValue = (key: string, value: any) => {
        if (!key) {
            return value;
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

    // Calculate the dynamic maxHeight based on content's height
    const containerRef = React.createRef<HTMLDivElement>();
    useEffect(() => {
        if (containerRef.current) {
            const height = containerRef.current.scrollHeight;
        }
    }, [containerRef]);

    if (Object.keys(settings).length == 0) {
        return (
            <div className={styles.settings}>
                <div className={styles.settingsInside}>
                    <Help />
                </div>
            </div>);
    }
    return (
        <div
            className={styles.settings}
            ref={containerRef} // Use ref to access the container element
        >
            <div className={styles.settingsInside}>
                {Object.entries(settings).map(([key, value], index) => (
                    <div key={index} className={`${styles.settingsRow} ${key === 'lyrics' ? styles.lyricsRow : ''}`}>
                        <div className={styles.settingsKey}>{
                            key === 'lyrics' ? 'Lyrics or Description' : formatSettingsKey(key)
                        }</div>
                        <div className={styles.settingsValue}>{formatSettingsValue(key, editedSettings[key])}</div>
                    </div>
                ))}
            </div>
        </div >
    );
};

export default Settings;
