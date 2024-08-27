import React, { useState, useEffect } from 'react';
import styles from './Metadata.module.scss';

interface MetadataProps {
    metadata: Record<string, any>;
    onChanged: (updatedMetadata: Record<string, any>) => void;
}

const Metadata: React.FC<MetadataProps> = ({ metadata, onChanged }) => {
    const [editedMetadata, setEditedMetadata] = useState<Record<string, any>>(metadata);

    useEffect(() => {
        setEditedMetadata(metadata);
    }, [metadata]);

    const handleValueChange = (key: string, value: any) => {
        // Check if the key starts with "STATIC_"
        const updatedMetadata = { ...editedMetadata, [key]: value };
        setEditedMetadata(updatedMetadata);
        onChanged(updatedMetadata);
    };

    const formatMetadataKey = (key: string) => {
        // Remove prefix and suffix from the key
        return key;
    };

    const formatMetadataValue = (key: string, value: any) => {
        return <input type="text" value={value} onChange={(e) => handleValueChange(key, e.target.value)} />;

    };

    return (
        <div className={styles.metadata}>
            {Object.entries(metadata).map(([key, value], index) => (
                <div key={index} className={styles.metadataRow}>
                    <div className={styles.metadataKey}>{formatMetadataKey(key)}</div>
                    <div className={styles.metadataValue}>{formatMetadataValue(key, editedMetadata[key])}</div>
                </div>
            ))}
        </div>
    );
};

export default Metadata;
