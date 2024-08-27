import React from 'react';
import styles from './LoadingDisplay.module.scss';

const LoadingDisplay = () => {
    return (
        <div className={styles.loadingContainer}>
            <div className={styles.loading}>
                <div className={styles.loadingCircle}></div>
            </div>
        </div>

    );
};

export default LoadingDisplay;
