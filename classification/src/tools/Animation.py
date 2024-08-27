from time import sleep
from PySide2.QtWidgets import QWidget

def opacity(widget: QWidget, from_val=0, to_val=1, step = 0.001):
    # Run the normal application
    opaqueness = from_val
    widget.setWindowOpacity(opaqueness)
    while opaqueness < to_val:
        widget
        widget.setWindowOpacity(opaqueness)
        sleep(step)
        opaqueness+=step