import axios from "axios";
import { useCallback, useEffect, useState } from "react";

const useTrackState = (token: string | null, trackId: string | null, updateInterval: number): { trackState: string } => {
    const [currentTrackState, setCurrentTrackState] = useState<string>("?");

    useEffect(() => {
        setCurrentTrackState("");
    }, [trackId]);


    useEffect(() => {
        const loadTrackState = async () => {
            if (!trackId) {
                return;
            }

            try {
                let token_str = token;
                const response = await axios.get(`http://localhost:8000/api/trackstate/${trackId}`, {
                    headers: {
                        Authorization: `Bearer ${token_str}`,
                    },
                });

                if (response.data.error || !response.data) {

                } else {

                    let state_str = response.data;
                    setCurrentTrackState(state_str);
                }
            } catch (error) {
            }
        };

        loadTrackState();

        // Set up the interval for auto-updates
        const intervalId = setInterval(loadTrackState, updateInterval);

        // Clean up the interval when the component unmounts
        return () => clearInterval(intervalId);
    }, [currentTrackState, token, trackId, updateInterval]);

    return { trackState: currentTrackState }
};

export default useTrackState;
