// In AudioTerms.tsx
import React from 'react';
import styles from './AudioTerms.module.scss';

interface AudioTermsProps {
    onClose: (accepted: boolean) => void;
}

const AudioTerms: React.FC<AudioTermsProps> = ({ onClose }) => {

    const handleAccept = () => {
        onClose(true);
    };

    const handleDecline = () => {
        onClose(false);
    };

    return (
        <div className={styles.overlay}>
            <div className={styles['overlay-content']}>
                <h2 className={styles.title}>Terms and Conditions</h2> {/* Add a class selector */}
                <p>Here are the terms and conditions of using our service.</p>
                <p>Please read and accept them before proceeding.</p>
                <div className={styles.buttons}>
                    <button onClick={handleAccept}>Accept</button>
                    <button onClick={handleDecline}>Decline</button>
                </div>
            </div>
        </div>
    );
};

export default AudioTerms;
