from sys import exit

from src.interface.PredictionWindow import PredictionWindow
from src.interface.SplashScreen import SplashScreen
from src.tools.Animation import opacity

from PySide2.QtCore import (
    Qt,
)

from PySide2.QtWidgets import (
    QApplication,
)

from src.tools.ColorTheme import ColorTheme

# logging.basicConfig(format="%(message)s", level=logging.INFO)
if __name__ == "__main__":
    app = QApplication()
    app.setApplicationName("emotions are Real")
    
    color_theme = ColorTheme()
    app.setStyleSheet(color_theme.loadData())
    
    window = PredictionWindow()
    window.setWindowOpacity(0)
        
    splash = SplashScreen('./img/load.gif', Qt.WindowStaysOnTopHint)
    splash.setWindowOpacity(0)
    
    splash.show()
    opacity(splash, 0, 1, 0.001)
    opacity(splash, 1, 0, 0.001)
    
    window.show()
    opacity(window, 0, 1, 0.001)
    
    splash.finish(window)
    exit(app.exec_())
    
    
