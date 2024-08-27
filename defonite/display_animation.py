import cv2
import os
import time

def display_animation(image_directory, fps):
    image_files = sorted(os.listdir(image_directory))
    frame_delay = int(1000 / fps)  # Calculate frame delay in milliseconds

    for image_file in image_files:
        image_path = os.path.join(image_directory, image_file)
        frame = cv2.imread(image_path)

        cv2.imshow("Animation", frame)
        if cv2.waitKey(frame_delay) == ord('q'):  # Exit the loop if 'q' key is pressed
            break

    cv2.destroyAllWindows()
