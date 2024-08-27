import threading
import time
from queue import Queue
from image_recognition import recognize_text
from animation_generation import generate_animation
from display_animation import display_animation
from audio_bpm_detection import get_bpm
from audio_listener import listen_audio_from_desktop
from display_audio_data import display_audio_data

class Organizer:
    data_queue = Queue()
    audio_source = 'desktop'
    bpm = 0
    audio_data = []

    @classmethod
    def run(cls):
        cls.start_threads()
        cls.listen_audio()

    @classmethod
    def start_threads(cls):
        try:
            threading.Thread(target=cls.image_recognition_thread).start()
            threading.Thread(target=cls.animation_generation_thread).start()
            threading.Thread(target=cls.display_animation_thread).start()
            threading.Thread(target=cls.display_audio_data_thread).start()
        except Exception as e:
            print("Error occurred while starting threads:", str(e))
            exit(1)

    @classmethod
    def image_recognition_thread(cls):
        try:
            while True:
                text_array = recognize_text()
                cls.data_queue.put(('text_array', text_array))
                time.sleep(3)
        except Exception as e:
            print("Error occurred in image recognition thread:", str(e))
            exit(1)

    @classmethod
    def display_audio_data_thread(cls):
        try:
            display_audio_data(cls.data_queue)
        except Exception as e:
            print("Error occurred in display audio data thread:", str(e))
            exit(1)

    @classmethod
    def animation_generation_thread(cls):
        try:
            while True:
                if not cls.data_queue.empty():
                    item = cls.data_queue.get()
                    if item[0] == 'text_array':
                        text_array = item[1]
                        image_directory = generate_animation(text_array)
                        cls.data_queue.put(('image_directory', image_directory))
        except Exception as e:
            print("Error occurred in animation generation thread:", str(e))
            exit(1)

    @classmethod
    def display_animation_thread(cls):
        try:
            while True:
                if not cls.data_queue.empty():
                    item = cls.data_queue.get()
                    if item[0] == 'image_directory':
                        image_directory = item[1]
                        fps = cls.bpm / 60  # Convert BPM to FPS
                        display_animation(image_directory, fps)
        except Exception as e:
            print("Error occurred in display animation thread:", str(e))
            exit(1)

    @classmethod
    def get_current_bpm(cls):
        def audio_callback(indata):
            cls.audio_data = indata.copy()
            cls.bpm = get_bpm(cls.audio_data)
            print("BPM:", cls.bpm)
            cls.data_queue.put(('bpm', cls.bpm))

        try:
            listen_audio_from_desktop(audio_callback)
            return
        except Exception as e:
            print("Error occurred in get_current_bpm:", str(e))
            exit(1)

    @classmethod
    def listen_audio(cls):
        try:
            while True:
                cls.get_current_bpm()
                time.sleep(3)
        except Exception as e:
            print("Error occurred in listen_audio:", str(e))
            exit(1)

if __name__ == '__main__':
    print("Starting Organizer")
    Organizer.run()
