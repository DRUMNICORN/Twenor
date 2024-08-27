import librosa
import numpy as np
import json
import sys


import logging
import sys
import json
import os

# Function to compute danceability feature for all chunks
def compute_danceability(chunk, sr):
    # Perform onset detection on the chunk
    logging.info("Performing onset detection on the chunk")
    scene_mono = librosa.to_mono(chunk)
    onset_env = librosa.onset.onset_strength(y=scene_mono, sr=sr)

    # Perform beat audioing using the onset strength
    tempo, beat_frames = librosa.beat.beat_track(onset_envelope=onset_env, sr=sr)
    logging.info("Tempo: {}".format(tempo))
    logging.info("Beat Frames: {}".format(beat_frames))

    if len(beat_frames) < 2:
        logging.info("Beat Frames is less than 2. Returning danceability as 0")
        return 0
    

    # Compute the rhythm stability
    rhythm_stability = compute_rhythm_stability(beat_frames)
    logging.info("Rhythm Stability: {}".format(rhythm_stability))

    # Compute the overall regularity
    beat_strength = librosa.feature.rms(y=chunk, frame_length=2048, hop_length=512)
    overall_regularity = compute_overall_regularity(tempo, beat_strength)

    logging.info("Overall Regularity: {}".format(overall_regularity))
    
    # Calculate danceability based on rhythm stability and overall regularity
    danceability = (rhythm_stability + overall_regularity) / 2.0

    return danceability


def compute_rhythm_stability(beat_frames):
    # Calculate the temporal variations between adjacent beat frames
    beat_intervals = np.diff(beat_frames)

    # Calculate the standard deviation of beat intervals
    rhythm_stability = np.std(beat_intervals)

    return rhythm_stability


def compute_overall_regularity(tempo, beat_strength):
    # Calculate the variations in beat strength values
    beat_strength_variations = np.diff(beat_strength)

    # Calculate the overall regularity based on the standard deviation of beat strength variations
    overall_regularity = np.std(beat_strength_variations)
    return overall_regularity

# Function to compute valence scores for all chunks
def compute_valence(chunk, sr):
    scene_mono = librosa.to_mono(chunk)
    spectral_contrast = librosa.feature.spectral_contrast(y=scene_mono, sr=sr)
    valence = np.mean(spectral_contrast)
    return valence

# Function to compute energy scores for all chunks
def compute_energy(chunk, _sr):

    scene_mono = librosa.to_mono(chunk)
    rms_energy = librosa.feature.rms(y=scene_mono)
    energy = np.mean(rms_energy)
    return energy

# Function to compute tempo for each chunk
def compute_tempo(chunk, sr):
    scene_mono = librosa.to_mono(chunk)
    tempo, _ = librosa.beat.beat_track(y=scene_mono, sr=sr)
    return tempo

# Function to compute the loudness of each chunk
def compute_loudness(chunk, _sr):

    scene_mono = librosa.to_mono(chunk)
    S = np.abs(librosa.stft(scene_mono))
    loudness = librosa.amplitude_to_db(S, ref=np.max)
    return np.mean(loudness)

# Function to compute speechiness scores for all chunks
def compute_speechiness(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    spectral_centroid = librosa.feature.spectral_centroid(y=scene_mono)
    speechiness = np.mean(spectral_centroid)
    return speechiness

# Function to compute the spectral flatness
def compute_instrumentalness(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    spectral_contrast = librosa.feature.spectral_contrast(y=scene_mono)
    instrumentalness = np.mean(spectral_contrast)
    return instrumentalness

# Function to compute the liveness feature
def compute_liveness(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    onset_strength = librosa.onset.onset_strength(y=scene_mono)
    liveness = np.mean(onset_strength)
    return liveness

# Function to compute the spectral flatness
def compute_acousticness(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    spectral_contrast = librosa.feature.spectral_contrast(y=scene_mono)
    acousticness = np.mean(spectral_contrast)
    return acousticness

# Function to compute the key of each chunk
def compute_key(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    chroma = librosa.feature.chroma_stft(y=scene_mono)
    key = np.argmax(np.mean(chroma, axis=1))
    return key

# Function to compute the mode of each chunk
def compute_mode(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    chroma = librosa.feature.chroma_stft(y=scene_mono)
    mode = np.argmax(np.mean(chroma, axis=1))
    return mode

# Function to compute the duration of each chunk
def compute_duration(chunk, sr):
    duration = len(chunk) / sr
    return duration

# Function to compute the time signature of each chunk
def compute_time_signature(chunk, _sr):
    scene_mono = librosa.to_mono(chunk)
    tempo = librosa.feature.rhythm.tempo(y=scene_mono)
    time_signature = 4 if tempo % 4 == 0 else 3
    return time_signature

# Define the features and their computation functions
feature_dict = {
    'Danceability': compute_danceability,
    'Valence': compute_valence,
    'Energy': compute_energy,
    'Tempo': compute_tempo,
    'Loudness': compute_loudness,
    'Speechiness': compute_speechiness,
    'Instrumentalness': compute_instrumentalness,
    'Liveness': compute_liveness,
    'Acousticness': compute_acousticness,
    'Key': compute_key,
    'Mode': compute_mode,
    'Duration': compute_duration,
    'Time Signature': compute_time_signature
}

from decimal import Decimal

def process_audio(audio_buffer, sr):
    # Compute the selected features
    features = []
    for feature_name in feature_dict.keys():
        if feature_name in feature_dict:
            computation_function = feature_dict[feature_name]
            feature_value = computation_function(audio_buffer, sr)
            logging.info(f"Computed feature '{feature_name}' with value {feature_value}")
            # Convert float32 to float
            feature_value = float(feature_value)
            features.append(feature_value)
        else:
            logging.warn(f"Warning: Feature '{feature_name}' not found.")

    # Return the features as a JSON string
    features_json = json.dumps(features)
    logging.info(f"Computed features: {features_json}")
    return features_json

# Set up logging
logging.basicConfig(filename='python.log', level=logging.INFO)

if __name__ == "__main__":
    # Args Error handling
    if len(sys.argv) < 2:
        logging.error("Error: Buffer file path is missing.")
        sys.exit(1)

    buffer_file_path = os.path.join(os.path.dirname(__file__), "buffers", "id.wav")
    audio_buffer = np.loadtxt(buffer_file_path, delimiter=",")

    sr = 22050

    logging.info(f"Audio buffer: {audio_buffer}")
    logging.info(f"Sample rate: {sr}")
    
    # Prase Error handling
    if len(audio_buffer) == 0:
        logging.error("Error: Audio buffer is empty.")
        sys.exit(1)

    # check if buffer is all zeros
    if np.all(audio_buffer == 0):
        logging.error("Error: Audio buffer is all zeros.")
        sys.exit(1)

    if sr <= 0:
        logging.error("Error: Sample rate must be positive.")
        sys.exit(1)

    # Process the audio and compute the features
    features = process_audio(audio_buffer, sr)

    # Convert features to JSON format
    print(features)


    # Exit the program
    logging.info("Program finished successfully.")
    sys.exit(0)
