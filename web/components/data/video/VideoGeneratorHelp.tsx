// help text component

import React from 'react';
import styles from './VideoGeneratorHelp.module.scss';

const VideoGeneratorHelp = () => {
    return (
        <div className={styles.settings}>
            <div className={styles.settingsInside}>
                <h1>Guide</h1>
                <p>1. Upload your music audi</p>
                <p>2. Update your metadata</p>
                <p>3. Generate your video</p>
            </div>
        </div>
    )
}

export default VideoGeneratorHelp;