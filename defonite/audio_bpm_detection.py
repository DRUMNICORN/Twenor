import librosa
import numpy as np

def get_bpm(audio_buffer, sr=44100):
    try:
        # Load audio from buffer
        y = np.array(audio_buffer, dtype=np.float32)

        # Run the onset detection
        onset_env = librosa.onset.onset_strength(y, sr)
        tempo = librosa.beat.tempo(onset_envelope=onset_env, sr=sr)

        return tempo[0]
    except Exception as e:
        print("Error occurred while processing audio data: ", str(e))
        return 0
