import React from 'react';
import styles from './ErrorDisplay.module.scss';

interface ErrorDisplayProps {
    error: string;
    onClose: () => void;
}

const ErrorDisplay: React.FC<ErrorDisplayProps> = ({ error, onClose }) => {
    return (
        <div className={styles.errorDisplay}>
            <p className={styles.errorText}>{error}</p>
            <button className={styles.closeButton} onClick={onClose}>
                <span className={styles.closeIcon}>&times;</span>
            </button>
        </div>
    );
};

export default ErrorDisplay;
