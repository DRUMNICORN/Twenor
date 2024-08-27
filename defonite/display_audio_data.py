import time
import logging
import numpy as np
import matplotlib.pyplot as plt

def display_audio_data(audio_data_queue):
    try:
        while True:
            audio_data = audio_data_queue.get()
            visualize_audio_data(audio_data)
            time.sleep(1)
    except Exception as e:
        logging.error("Error occurred in display audio data thread: %s", str(e))

def visualize_audio_data(audio_data):
    plt.figure()
    plt.plot(np.arange(len(audio_data)), audio_data)
    plt.xlabel("Time")
    plt.ylabel("Amplitude")
    plt.title("Audio Data")
    plt.show()
