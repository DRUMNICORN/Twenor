import sounddevice as sd

def listen_audio_from_desktop(callback, duration=3, fs=44100):
    # Check if there are any audio devices
    if len(sd.query_devices()) == 0:
        raise ValueError("No audio device found")

    def audio_callback(indata, frames, time, status):
        callback(indata)

    with sd.InputStream(callback=audio_callback, channels=2, samplerate=fs):
        sd.sleep(int(duration * 1000))  # Listen for the specified duration in milliseconds
