from PySide2.QtGui import (
    QPixmap,
    QMovie
)

from PySide2.QtWidgets import (
    QSplashScreen,
)

class SplashScreen(QSplashScreen):
    def __init__(self, animation, flags):

        # run event dispatching in another thread
        QSplashScreen.__init__(self, QPixmap(), flags)
        self.movie = QMovie(animation)
        self.movie.frameChanged.connect(self.onNextFrame)
        self.movie.start()


    def onNextFrame(self):
        pixmap = self.movie.currentPixmap()
        self.setPixmap(pixmap)
        self.setMask(pixmap.mask())
