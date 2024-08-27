import { useEffect, useState } from "react";
import styles from './VideoLoading.module.scss';

interface VideoLoadingProps {
    loads: boolean[],
    errors: (string | null)[]
}

const VideoLoading: React.FC<VideoLoadingProps> = ({ loads, errors }) => {

    const [orbColors, setOrbColors] = useState<string[]>([]);

    useEffect(() => {
        const colors = loads.map((load, i) => {
            if (load) return 'success';
            if (errors[i]) return 'error';
            return 'loading';
        });
        setOrbColors(colors);
    }, [loads, errors]);

    return (
        <div className={styles.loading_showcase}>
            {orbColors.map((color, i) => (
                <div key={i} className={`${styles.loading_orb} ${styles[color]}`}></div>
            ))}
        </div>
    );

}

export default VideoLoading;