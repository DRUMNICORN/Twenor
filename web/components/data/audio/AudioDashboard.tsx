import React from 'react';
import styles from './AudioDashboard.module.scss';
import { Audio } from '@/util/useAudios';
import ControlButton from '@/components/base/Button';
import { on } from 'events';

type AudioDashboardProps = {
    audios: Audio[],
    onAudioClick: (audio: Audio) => void,
    onAudioDelete: (audioId: string) => void,
    onAudioDownload: (audioId: string) => void,
    audioId: string | null,
};

const AudioDashboard: React.FC<AudioDashboardProps> = ({ audios, onAudioClick, onAudioDelete, onAudioDownload, audioId }) => {
    return (
        <div>
            <div className={styles.dashboardStylingElement} />

            <div className={styles.dashboard}>
                {(audios || []).map((audio, index) => {
                    return (
                        <div key={index} className={`${parseInt(audioId || "0") == parseInt(audio.id) ? styles.dashboardButtonDownloading : ''} ${styles.dashboardButton}`} onClick={(e) => onAudioClick(audio)}>
                            <div className={styles.dashboardButtonWrapper} >
                                <div className={styles.dashboardButtonControl}>
                                    <ControlButton
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            onAudioDelete(audio.id);
                                        }}
                                        icon={"/icons/wave/trash.svg"}
                                        alt="Audio" />
                                </div>
                            </div>

                            <div className={styles.dashboardButtonTitle}>
                                {audio.title || 'No title'} | {audio.id}
                            </div>

                            {
                                parseInt(audioId || "0") == parseInt(audio.id) &&
                                <div className={styles.dashboardButtonWrapper}>
                                    <div className={`${styles.dashboardButtonControl} ${styles.dashboardButtonControlDownload}`}>
                                        <ControlButton
                                            icon={"/icons/arrow.svg"}
                                            alt="DOWNLOAD" onClick={() => { onAudioDownload(audio.id) }}
                                        />
                                    </div>
                                </div>
                            }
                            <div className={styles.dashboardButtonState}>
                                {audio.state || 'No state'}
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default AudioDashboard;
