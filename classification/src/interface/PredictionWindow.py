from src.tools.Console import Console
console = Console()
console.debug("Loaded Console!")

from os.path import join as path_join, basename, join, exists, dirname, expanduser
from os import listdir, makedirs
from shutil import move
console.debug("Loaded os path tools")


from keras.models import load_model
console.debug("Loaded tensorflow modules")

from pickle import load as pickle_load
console.debug("Loaded pickle")

from joblib import load as joblib_load
console.debug("Loaded joblib")

from numpy import array, append
console.debug("Loaded numpy")

from PySide2.QtGui import (
    QBrush,
    QColor,
    QIcon,
    QPainter, 
    QPixmap,
)
console.debug("Loaded PySide2.QtGui")

from PySide2.QtCore import (
    QCoreApplication,
    QDir,
    QPoint,
    Qt,
    QUrl,
)
console.debug("Loaded PySide2.QtCore")

from PySide2.QtMultimedia import (
    QMediaPlayer,
    QMediaContent
)
console.debug("Loaded PySide2.QtMultimedia")

from PySide2.QtWidgets import (
    QFileDialog,
    QHBoxLayout,
    QLabel,
    QMainWindow,
    QPushButton,
    QSizePolicy,
    QSlider,
    QSpacerItem,
    QStyle,
    QVBoxLayout,
    QWidget,
)
console.debug("Loaded PySide2.QtWidgets")


from src.core.Prediction import Prediction
console.debug("Loaded Prediction")

from src.interface.OptionsWindow import OptionsWindow
console.debug("Loaded Options Window")

from src.tools.Coverart import loadCoverart
console.debug("Loaded Coverart loder")

class PredictionWindow(QMainWindow):
    def __init__(self):        
        super().__init__()
        self.windowHeader = 'Music Classification Tool'
        

        self.audio_dir = QDir.homePath()
        self.seconds = 3

        self.storage_location = join(expanduser('~/Documents'), 'SoundModels')
        if not exists(self.storage_location):
            makedirs(self.storage_location)
            console.info(f'created folder {self.storage_location}') 

        self.data_dir = None
        self.model_dir = None
        self.segments = 10
        self.autoTolarance = 40
        self.autoMove = False
        self.autoNext = False

        self.volume = 20

        self.WIDTH = 640
        self.HEIGHT = 200

        self.pixmap_temp = [0, 0]

        self.setWindowTitle("Emotions are Real")
        # QMainWindow.__init__(self, None, Qt.WindowStaysOnTopHint)
        self.centralWidget = QWidget()
        self.setCentralWidget(self.centralWidget)
        self.setWindowFlag(Qt.FramelessWindowHint)
        self.setWindowIcon(QIcon('./imgwindow-icon.png'))
        self.setAttribute(Qt.WA_TranslucentBackground)
        self.setWindowOpacity(1)
        self.resize(self.WIDTH, self.HEIGHT)

        self.mediaPlayer = QMediaPlayer()
        self.mediaVolumeChanged(self.volume)

        self.options = OptionsWindow(self.storage_location, self.data_dir, self.model_dir, self.segments, self.autoTolarance ,self.autoMove, self.autoNext)
        self.options.saved.connect(self.updateOptions)
        self.options.saveOptions()
        self.setupUi()
    
    # TODO
    # - ffmpeg
    # - def audio libary
    # - Model to package
    
    ####################################################################
    #   ______               __      __                                #
    #  /      \             /  |    /  |                               #
    # /$$$$$$  |  ______   _$$ |_   $$/   ______   _______    _______  #
    # $$ |  $$ | /      \ / $$   |  /  | /      \ /       \  /       | #
    # $$ |  $$ |/$$$$$$  |$$$$$$/   $$ |/$$$$$$  |$$$$$$$  |/$$$$$$$/  #
    # $$ |  $$ |$$ |  $$ |  $$ | __ $$ |$$ |  $$ |$$ |  $$ |$$      \  #
    # $$ \__$$ |$$ |__$$ |  $$ |/  |$$ |$$ \__$$ |$$ |  $$ | $$$$$$  | #
    # $$    $$/ $$    $$/   $$  $$/ $$ |$$    $$/ $$ |  $$ |/     $$/  #
    #  $$$$$$/  $$$$$$$/     $$$$/  $$/  $$$$$$/  $$/   $$/ $$$$$$$/   #
    #           $$ |                                                   #
    #           $$ |                                                   #
    #           $$/                                                    #
    #                                                                  #
    ####################################################################
    
    def updateOptions(self, data_dir, model_dir, segments, autoTolarance, autoMove, autoNext):
        self.autoMove = autoMove
        self.autoNext = autoNext
        self.autoTolarance = autoTolarance
        self.segments = segments
        
        if data_dir:
            self.data_dir = path_join(self.storage_location, data_dir)
            
        try:
            console.debug("Load Model")
            self.model = load_model(path_join(self.data_dir, model_dir))
        except (OSError, TypeError):
            console.error(f'Model {model_dir} not found in {self.data_dir}')
            self.model = None
    
        try:
            console.debug("Load Scaler")
            self.scaler = joblib_load(path_join(self.data_dir, "scaler.pkl"))
        except (FileNotFoundError, TypeError):
            console.error(f'Scaler not found in {self.data_dir}')
            self.scaler = None

        try: 
            console.debug("Load Labels")
            with open(path_join(self.data_dir, "labels.pkl"), "rb") as a_file:
                self.labels = pickle_load(a_file)
        except (FileNotFoundError, TypeError):
            console.error(f'Labels not found in {self.data_dir}')
            self.labels = []
            
        self.refreshLabelButton()
        console.debug("Updated options")
            
    def openOptionsWindow(self):
        if not self.options and hasattr(self.options, 'isVisible') and self.options.isVisible():
            self.options.quitUi()
            return 0
        self.options.show()
        console.debug("Opened options window")
        

    ######################################################################################################################
    #   ______                       __                                       ______                   __  __            #
    #  /      \                     /  |                                     /      \                 /  |/  |           #
    # /$$$$$$  | _______    ______  $$ | __    __   _______   ______        /$$$$$$  | __    __   ____$$ |$$/   ______   #
    # $$ |__$$ |/       \  /      \ $$ |/  |  /  | /       | /      \       $$ |__$$ |/  |  /  | /    $$ |/  | /      \  #
    # $$    $$ |$$$$$$$  | $$$$$$  |$$ |$$ |  $$ |/$$$$$$$/ /$$$$$$  |      $$    $$ |$$ |  $$ |/$$$$$$$ |$$ |/$$$$$$  | #
    # $$$$$$$$ |$$ |  $$ | /    $$ |$$ |$$ |  $$ |$$      \ $$    $$ |      $$$$$$$$ |$$ |  $$ |$$ |  $$ |$$ |$$ |  $$ | #
    # $$ |  $$ |$$ |  $$ |/$$$$$$$ |$$ |$$ \__$$ | $$$$$$  |$$$$$$$$/       $$ |  $$ |$$ \__$$ |$$ \__$$ |$$ |$$ \__$$ | #
    # $$ |  $$ |$$ |  $$ |$$    $$ |$$ |$$    $$ |/     $$/ $$       |      $$ |  $$ |$$    $$/ $$    $$ |$$ |$$    $$/  #
    # $$/   $$/ $$/   $$/  $$$$$$$/ $$/  $$$$$$$ |$$$$$$$/   $$$$$$$/       $$/   $$/  $$$$$$/   $$$$$$$/ $$/  $$$$$$/   #
    #                                   /  \__$$ |                                                                       #
    #                                   $$    $$/                                                                        #
    #                                    $$$$$$/                                                                         #
    #                                                                                                                    #
    ######################################################################################################################

    def loadCurrentFile(self, _ = None):
        self.progressLabelValue = 0
        console.debug("Load next file")
        self.progressLabel.setText('load...')
        self.playButton.setEnabled(False)
        self.refreshLabelButton()

        # sending_button = self.sender()
        # actual_genre = sending_button.text().split(':')[0]
        if(len(self.files) <= 0):
            console.info(f'no files found in {self.audio_dir}') 
            self.openButton.setEnabled(True)
            return False

        self.updateMedia()

    def loadNextFile(self, _ = None):
        console.debug("Load next file")
        self.progressLabel.setText('load...')

        self.reloadButton.setEnabled(True)
        self.nextButton.setEnabled(True)
        self.playButton.setEnabled(False)
        self.refreshLabelButton()


        # sending_button = self.sender()
        # actual_genre = sending_button.text().split(':')[0]
        if(len(self.files) <= 0):
            console.info("No files found!")
            self.openButton.setEnabled(True)
            return False

        self.fileName = path_join(self.audio_dir, self.files[0])

        console.debug("Load coverart")
        loadCoverart(self.fileName)

        self.files.pop(0)

        self.startThread()
        self.updateMedia()
        
    def updateMedia(self):
        self.trackLabel.setText(basename(self.fileName))
        self.mediaPlayer.setMedia(QMediaContent(QUrl.fromLocalFile(self.fileName)))
        self.playButton.setEnabled(True)
        self.toggleAudio()
        console.info("Loaded Next File")
        self.updateTrackCover()
        

    def startThread(self):
        self.stopThread()
        
        self.thread = Prediction(file=self.fileName, options=self)
        self.thread.result_value.connect(self.updateUi)
        self.thread.change_value.connect(self.updateProgressLabel)
        self.thread.start()
        console.debug("Started Thread")

    def stopThread(self):
        if (type(self.thread) == Prediction) and self.thread.isRunning:
            self.thread.stop()
            self.thread.quit()
            console.debug("Stoped Thread")

    ########################################################################################
    #  __    __                  __              __                      __    __  ______  #
    # /  |  /  |                /  |            /  |                    /  |  /  |/      | #
    # $$ |  $$ |  ______    ____$$ |  ______   _$$ |_     ______        $$ |  $$ |$$$$$$/  #
    # $$ |  $$ | /      \  /    $$ | /      \ / $$   |   /      \       $$ |  $$ |  $$ |   #
    # $$ |  $$ |/$$$$$$  |/$$$$$$$ | $$$$$$  |$$$$$$/   /$$$$$$  |      $$ |  $$ |  $$ |   #
    # $$ |  $$ |$$ |  $$ |$$ |  $$ | /    $$ |  $$ | __ $$    $$ |      $$ |  $$ |  $$ |   #
    # $$ \__$$ |$$ |__$$ |$$ \__$$ |/$$$$$$$ |  $$ |/  |$$$$$$$$/       $$ \__$$ | _$$ |_  #
    # $$    $$/ $$    $$/ $$    $$ |$$    $$ |  $$  $$/ $$       |      $$    $$/ / $$   | #
    #  $$$$$$/  $$$$$$$/   $$$$$$$/  $$$$$$$/    $$$$/   $$$$$$$/        $$$$$$/  $$$$$$/  #
    #           $$ |                                                                       #
    #           $$ |                                                                       #
    #           $$/                                                                        #
    #                                                                                      #
    ########################################################################################

    def updateUi(self, data):
        self.progressLabel.setText('done!')
        self.progressLabelValue = 0
        self.updateLabelButtons(data)
        console.debug("Updated UI")


    def updateProgressLabel(self, value):
        self.progressLabelValue += value;
        self.progressLabel.setText(f'{self.progressLabelValue}/{self.segments}')
        console.debug(f"Loaded Segment {self.progressLabelValue}/{self.segments}")
        
    def updateLabelButtons(self, results):
        self.winner = ''
        self.winner_sus = 0
        for button_pos, (index, result) in enumerate(results):
            self.labelButtons[button_pos].setText(f'{self.labels[index]}: {int((result*100))}%')
            self.labelButtons[button_pos].setEnabled(True)
            if(result > self.winner_sus):
                self.winner_sus = result
                self.winner = self.labels[index]
        self.openButton.setEnabled(True)
                
        if(self.autoTolarance > self.winner_sus * 100):
            self.labelList.update()
            console.info('Manuel action required to continue')
            self.progressLabel.setText('done!')
            return False
                
        if(self.autoMove):
            self.updateFile(self.fileName, self.winner)
        
        if(self.autoNext):
            self.loadNextFile();

    def updateFile(self, path, label):
        folder_dir = join(dirname(path), '#classified')
        if not exists(folder_dir):
            makedirs(folder_dir)
            console.info(f'Created folder {folder_dir}') 
        label_dir = join(folder_dir, label)
        if not exists(label_dir):
            makedirs(label_dir)
            console.info(f'Created folder {label_dir}') 

        file_path = join(label_dir, basename(path))
        try:
            move(path, file_path)
            split = '\\'
            console.info(f'{basename(file_path)} moved to {label_dir.split(split)[-1]}') 
        except FileNotFoundError:
            console.error(f'could not save {basename(path)}')
        
        console.debug(f'Updated File {label}: {basename(path)}')
    
    def updateTrackCover(self):
        pixmap = QPixmap('./img/coverart.jpg')
        pixmap_temp = [int(pixmap.width()), int(pixmap.height())]
        
        if(self.pixmap_temp == pixmap_temp):
            pixmap = QPixmap('./img/default_coverart.jpg')
            self.pixmap_temp = pixmap_temp
            console.debug("Updated coverart")
            
        if pixmap.isNull():
            return
        
        pixmap = pixmap.scaled(100, 100)
        radius = 20

        # create empty pixmap of same size as original 
        rounded = QPixmap(pixmap.size())
        rounded.fill(QColor("transparent"))
        
        # # draw rounded rect on new pixmap using original pixmap as brush
        painter = QPainter(rounded)
        painter.setRenderHint(QPainter.Antialiasing)
        painter.setBrush(QBrush(pixmap))
        painter.setPen(Qt.NoPen)
        painter.drawRoundedRect(pixmap.rect(), radius, radius)
        painter.end()
        
        self.trackCover.setPixmap(rounded)
        console.debug("Created new Pixmap")


    ###############################################################################################################
    #  __       __                  __  __                  _______   __                                          #
    # /  \     /  |                /  |/  |                /       \ /  |                                         #
    # $$  \   /$$ |  ______    ____$$ |$$/   ______        $$$$$$$  |$$ |  ______   __    __   ______    ______   #
    # $$$  \ /$$$ | /      \  /    $$ |/  | /      \       $$ |__$$ |$$ | /      \ /  |  /  | /      \  /      \  #
    # $$$$  /$$$$ |/$$$$$$  |/$$$$$$$ |$$ | $$$$$$  |      $$    $$/ $$ | $$$$$$  |$$ |  $$ |/$$$$$$  |/$$$$$$  | #
    # $$ $$ $$/$$ |$$    $$ |$$ |  $$ |$$ | /    $$ |      $$$$$$$/  $$ | /    $$ |$$ |  $$ |$$    $$ |$$ |  $$/  #
    # $$ |$$$/ $$ |$$$$$$$$/ $$ \__$$ |$$ |/$$$$$$$ |      $$ |      $$ |/$$$$$$$ |$$ \__$$ |$$$$$$$$/ $$ |       #
    # $$ | $/  $$ |$$       |$$    $$ |$$ |$$    $$ |      $$ |      $$ |$$    $$ |$$    $$ |$$       |$$ |       #
    # $$/      $$/  $$$$$$$/  $$$$$$$/ $$/  $$$$$$$/       $$/       $$/  $$$$$$$/  $$$$$$$ | $$$$$$$/ $$/        #
    #                                                                              /  \__$$ |                     #
    #                                                                              $$    $$/                      #
    #                                                                               $$$$$$/                       #
    #                                                                                                             #
    ###############################################################################################################

    def toggleAudio(self):
        if self.mediaPlayer.state() == QMediaPlayer.PlayingState:
            self.mediaPlayer.pause()
            console.debug("Paused audio")
        else:
            self.mediaPlayer.play()
            console.debug("Play audio")

    def mediaStateChanged(self, state):
        if self.mediaPlayer.state() == QMediaPlayer.EndOfMedia:
            console.info('Playback ended!')
            
            if(self.autoNext):
                self.loadNextFile()
            
        if self.mediaPlayer.state() == QMediaPlayer.PlayingState:
            self.playButton.setIcon(
                    self.style().standardIcon(QStyle.SP_MediaPause))
        else:
            self.playButton.setIcon(
                    self.style().standardIcon(QStyle.SP_MediaPlay))

    def positionChanged(self, position):
        self.positionSlider.setValue(position)

    def durationChanged(self, duration):
        self.positionSlider.setRange(0, duration)

    def mediaSlideerChanged(self, position):
        self.mediaPlayer.setPosition(position)
        
    maxVolume = 100
    def mediaVolumeChanged(self, value):
        self.mediaPlayer.setVolume(value)

    ##############################################################################################################################################################
    #  __       __  __                  __                                __       __                                                                    __      #
    # /  |  _  /  |/  |                /  |                              /  \     /  |                                                                  /  |     #
    # $$ | / \ $$ |$$/  _______    ____$$ |  ______   __   __   __       $$  \   /$$ |  ______   __     __  ______   _____  ____    ______   _______   _$$ |_    #
    # $$ |/$  \$$ |/  |/       \  /    $$ | /      \ /  | /  | /  |      $$$  \ /$$$ | /      \ /  \   /  |/      \ /     \/    \  /      \ /       \ / $$   |   #
    # $$ /$$$  $$ |$$ |$$$$$$$  |/$$$$$$$ |/$$$$$$  |$$ | $$ | $$ |      $$$$  /$$$$ |/$$$$$$  |$$  \ /$$//$$$$$$  |$$$$$$ $$$$  |/$$$$$$  |$$$$$$$  |$$$$$$/    #
    # $$ $$/$$ $$ |$$ |$$ |  $$ |$$ |  $$ |$$ |  $$ |$$ | $$ | $$ |      $$ $$ $$/$$ |$$ |  $$ | $$  /$$/ $$    $$ |$$ | $$ | $$ |$$    $$ |$$ |  $$ |  $$ | __  #
    # $$$$/  $$$$ |$$ |$$ |  $$ |$$ \__$$ |$$ \__$$ |$$ \_$$ \_$$ |      $$ |$$$/ $$ |$$ \__$$ |  $$ $$/  $$$$$$$$/ $$ | $$ | $$ |$$$$$$$$/ $$ |  $$ |  $$ |/  | #
    # $$$/    $$$ |$$ |$$ |  $$ |$$    $$ |$$    $$/ $$   $$   $$/       $$ | $/  $$ |$$    $$/    $$$/   $$       |$$ | $$ | $$ |$$       |$$ |  $$ |  $$  $$/  #
    # $$/      $$/ $$/ $$/   $$/  $$$$$$$/  $$$$$$/   $$$$$/$$$$/        $$/      $$/  $$$$$$/      $/     $$$$$$$/ $$/  $$/  $$/  $$$$$$$/ $$/   $$/    $$$$/   #
    #                                                                                                                                                            #
    ##############################################################################################################################################################

    def mousePressEvent(self, event):
        self.moveWindow = event.pos().y() < 69
        if (event.button() == Qt.LeftButton):
            self.oldPos = event.globalPos()

    def mouseMoveEvent(self, event):
        if not self.moveWindow:
            return
        if hasattr(self, 'oldPos'):
            delta = QPoint(event.globalPos() - self.oldPos)
            self.move(self.x() + delta.x(), self.y() + delta.y())
        self.oldPos = event.globalPos()
        
        
    ##############################################################################
    #   ______               __                                __    __  ______  #
    #  /      \             /  |                              /  |  /  |/      | #
    # /$$$$$$  |  ______   _$$ |_    __    __   ______        $$ |  $$ |$$$$$$/  #
    # $$ \__$$/  /      \ / $$   |  /  |  /  | /      \       $$ |  $$ |  $$ |   #
    # $$      \ /$$$$$$  |$$$$$$/   $$ |  $$ |/$$$$$$  |      $$ |  $$ |  $$ |   #
    #  $$$$$$  |$$    $$ |  $$ | __ $$ |  $$ |$$ |  $$ |      $$ |  $$ |  $$ |   #
    # /  \__$$ |$$$$$$$$/   $$ |/  |$$ \__$$ |$$ |__$$ |      $$ \__$$ | _$$ |_  #
    # $$    $$/ $$       |  $$  $$/ $$    $$/ $$    $$/       $$    $$/ / $$   | #
    #  $$$$$$/   $$$$$$$/    $$$$/   $$$$$$/  $$$$$$$/         $$$$$$/  $$$$$$/  #
    #                                         $$ |                               #
    #                                         $$ |                               #
    #                                         $$/                                #
    #                                                                            #
    ##############################################################################

    def setupUi(self):
        self.setupUiElements()
        console.debug('Loaded Elements')
        self.setupUiStructure()
        console.debug('Loaded Structure')
        self.setupUiEvents()
        console.debug('Loaded Events')

    def setupUiStructure(self):
        controlLayout = QHBoxLayout()
        controlLayout.setContentsMargins(0, 0, 0, 0)
        controlLayout.addWidget(self.playButton)
        controlLayout.addWidget(self.reloadButton)
        controlLayout.addWidget(self.nextButton)
        controlLayout.addWidget(self.positionSlider)

        trackLayout = QHBoxLayout()
        trackLayout.addWidget(self.trackCover, alignment=Qt.AlignLeft)
        trackLayout.addItem(self.hSpacer)
        trackLayout.addWidget(self.trackLabel, alignment=Qt.AlignLeft)
        trackLayout.addWidget(self.volumeSlider)

        playerContainer = QWidget(objectName = "playerContainer")
        playerLayout = QVBoxLayout(playerContainer)
        playerLayout.addLayout(trackLayout)
        playerLayout.addLayout(controlLayout)

        labelLayout = QVBoxLayout()
        labelLayout.addLayout(self.labelList)

        observerLayout = QHBoxLayout()
        observerLayout.addItem(self.vSpacer)
        observerLayout.addWidget(playerContainer)
        observerLayout.addItem(self.hSpacer)
        observerLayout.addLayout(labelLayout)
        observerLayout.addItem(self.vSpacer)

        toolbarContainer = QWidget(objectName = "toolbarContainer")
        toolbarLayout = QHBoxLayout(toolbarContainer)
        toolbarLayout.addWidget(self.windowLabel)
        toolbarLayout.addItem(self.expandingHSpacer)
        toolbarLayout.addWidget(self.openButton, alignment= Qt.AlignRight | Qt.AlignTop)
        toolbarLayout.addWidget(self.optionsButton, alignment= Qt.AlignRight | Qt.AlignTop)
        toolbarLayout.addWidget(self.exitButton, alignment= Qt.AlignRight | Qt.AlignTop)

        footprintLayout = QHBoxLayout()
        self.progressLabelValue = 0
        footprintLayout.addWidget(self.progressLabel)
        footprintLayout.addWidget(self.infoLabel)

        layoutContainer = QWidget(objectName = "window")
        layout = QVBoxLayout(layoutContainer)
        layout.addWidget(toolbarContainer)
        layout.addItem(self.vSpacer)
        layout.addLayout(observerLayout)
        layout.addLayout(footprintLayout)
        
        layoutLayout = QHBoxLayout()
        layoutLayout.addWidget(layoutContainer)
        self.centralWidget.setLayout(layoutLayout)

    def setupUiElements(self):
        self.expandingVSpacer = QSpacerItem(1, 1, QSizePolicy.Minimum, QSizePolicy.Expanding)
        self.expandingHSpacer = QSpacerItem(1, 1, QSizePolicy.Expanding, QSizePolicy.Minimum)
        
        self.hSpacer = QSpacerItem(20, 10, QSizePolicy.Minimum, QSizePolicy.Maximum)
        self.vSpacer = QSpacerItem(10, 20, QSizePolicy.Maximum, QSizePolicy.Minimum)

        self.playButton = QPushButton(objectName="playerButton")
        self.playButton.setEnabled(False)
        self.playButton.setIcon(self.style().standardIcon(QStyle.SP_MediaPlay))
        
        self.reloadButton = QPushButton(objectName="playerButton")
        self.reloadButton.setEnabled(False)
        self.reloadButton.setIcon(QIcon('./img/reload.png'))
        
        self.nextButton = QPushButton(objectName="playerButton")
        self.nextButton.setEnabled(False)
        self.nextButton.setIcon(QIcon('./img/next.png'))
        
        self.positionSlider = QSlider(Qt.Horizontal, objectName="positionSlider")
        self.positionSlider.setRange(0, 0)
        
        self.volumeSlider = QSlider(Qt.Vertical, objectName="volumeSlider")
        self.volumeSlider.setRange(0, self.maxVolume)
        self.volumeSlider.setValue(self.volume)
        
        self.labelList = QVBoxLayout(objectName="labelList")
        self.labelList.setObjectName(u"gridLayout")
        self.setupLabelButtons()
        
        self.trackCover = QLabel(objectName="trackCover")
        
        self.trackLabel = QLabel("", objectName="trackLabel")
        self.trackLabel.setWordWrap(True)
        self.trackLabel.setFixedWidth(self.WIDTH * 0.6)
        self.trackLabel.setFixedHeight(self.HEIGHT * 0.7)

        self.openButton = QPushButton(objectName="toolbarButton")
        self.openButton.setIcon(QIcon('./img/album-folder.png'))
        
        self.optionsButton = QPushButton(objectName="toolbarButton")
        self.optionsButton.setIcon(QIcon('./img/settings.png'))
        
        self.exitButton = QPushButton(objectName="toolbarButton")
        self.exitButton.setIcon(QIcon('./img/close.png'))
        
        self.windowLabel = QLabel(self.windowHeader, objectName="windowLabel")
        
        self.progressLabel = QLabel('', alignment=Qt.AlignLeft, objectName="infoLabel")
        self.infoLabel = QLabel('42', alignment=Qt.AlignRight, objectName='infoLabel')
        console.link(self.infoLabel)
        
    
    def setupUiEvents(self):
        self.reloadButton.clicked.connect(self.loadCurrentFile)
        self.nextButton.clicked.connect(self.loadNextFile)
        self.playButton.clicked.connect(self.toggleAudio)
        self.positionSlider.sliderMoved.connect(self.mediaSlideerChanged)
        self.volumeSlider.sliderMoved.connect(self.mediaVolumeChanged)
        self.exitButton.clicked.connect(self.quitUi)
        self.optionsButton.clicked.connect(self.openOptionsWindow)
        self.openButton.clicked.connect(self.loadLibaryFolder)
        self.mediaPlayer.error.connect(self.loadNextFile)
        
        self.mediaPlayer.stateChanged.connect(self.mediaStateChanged)
        self.mediaPlayer.positionChanged.connect(self.positionChanged)
        self.mediaPlayer.durationChanged.connect(self.durationChanged)

    def quitUi(self):
        self.stopThread()
        QCoreApplication.quit()
        
    ##########################################################################################################################
    #  __                  __                  __        _______               __      __                                    #
    # /  |                /  |                /  |      /       \             /  |    /  |                                   #
    # $$ |        ______  $$ |____    ______  $$ |      $$$$$$$  | __    __  _$$ |_  _$$ |_     ______   _______    _______  #
    # $$ |       /      \ $$      \  /      \ $$ |      $$ |__$$ |/  |  /  |/ $$   |/ $$   |   /      \ /       \  /       | #
    # $$ |       $$$$$$  |$$$$$$$  |/$$$$$$  |$$ |      $$    $$< $$ |  $$ |$$$$$$/ $$$$$$/   /$$$$$$  |$$$$$$$  |/$$$$$$$/  #
    # $$ |       /    $$ |$$ |  $$ |$$    $$ |$$ |      $$$$$$$  |$$ |  $$ |  $$ | __ $$ | __ $$ |  $$ |$$ |  $$ |$$      \  #
    # $$ |_____ /$$$$$$$ |$$ |__$$ |$$$$$$$$/ $$ |      $$ |__$$ |$$ \__$$ |  $$ |/  |$$ |/  |$$ \__$$ |$$ |  $$ | $$$$$$  | #
    # $$       |$$    $$ |$$    $$/ $$       |$$ |      $$    $$/ $$    $$/   $$  $$/ $$  $$/ $$    $$/ $$ |  $$ |/     $$/  #
    # $$$$$$$$/  $$$$$$$/ $$$$$$$/   $$$$$$$/ $$/       $$$$$$$/   $$$$$$/     $$$$/   $$$$/   $$$$$$/  $$/   $$/ $$$$$$$/   #
    #                                                                                                                        #
    ##########################################################################################################################
    
    def manuallySelectLabel(self):
        self.winner_man = self.sender().text().split(":")[0]
        self.updateFile(self.fileName, self.winner_man)
        
        self.labelList.update()
        self.openButton.setEnabled(True)
        self.loadNextFile();
        
    def setupLabelButtons(self):
        try:
            self.labelButtons = array([])
            for index in range(len(self.labels)):
                self.labelButtons = append(self.labelButtons, QPushButton(self.labels[index], objectName='labelButtons'))
                self.labelButtons[index].setEnabled(False)
                self.labelButtons[index].setFixedWidth(100)
                self.labelList.addWidget(self.labelButtons[index], alignment=Qt.AlignLeft)
                self.labelButtons[index].clicked.connect(self.manuallySelectLabel)
        except AttributeError:
            console.debug('42 is loaded')

    def refreshLabelButton(self):
        if hasattr(self, 'labelList'):
            self.clearLayout(self.labelList)
        self.setupLabelButtons()
        
    def clearLayout(self, layout):
        while layout.count():
            child = layout.takeAt(0)
            if child.widget():
                child.widget().deleteLater()
                
    ###################################################################################################################
    #  __                        __  __                            ________         __        __                      #
    # /  |                      /  |/  |                          /        |       /  |      /  |                     #
    # $$ |        ______    ____$$ |$$/  _______    ______        $$$$$$$$/______  $$ |  ____$$ |  ______    ______   #
    # $$ |       /      \  /    $$ |/  |/       \  /      \       $$ |__  /      \ $$ | /    $$ | /      \  /      \  #
    # $$ |      /$$$$$$  |/$$$$$$$ |$$ |$$$$$$$  |/$$$$$$  |      $$    |/$$$$$$  |$$ |/$$$$$$$ |/$$$$$$  |/$$$$$$  | #
    # $$ |      $$ |  $$ |$$ |  $$ |$$ |$$ |  $$ |$$ |  $$ |      $$$$$/ $$ |  $$ |$$ |$$ |  $$ |$$    $$ |$$ |  $$/  #
    # $$ |_____ $$ \__$$ |$$ \__$$ |$$ |$$ |  $$ |$$ \__$$ |      $$ |   $$ \__$$ |$$ |$$ \__$$ |$$$$$$$$/ $$ |       #
    # $$       |$$    $$/ $$    $$ |$$ |$$ |  $$ |$$    $$ |      $$ |   $$    $$/ $$ |$$    $$ |$$       |$$ |       #
    # $$$$$$$$/  $$$$$$/   $$$$$$$/ $$/ $$/   $$/  $$$$$$$ |      $$/     $$$$$$/  $$/  $$$$$$$/  $$$$$$$/ $$/        #
    #                                             /  \__$$ |                                                          #
    #                                             $$    $$/                                                           #
    #                                              $$$$$$/                                                            #
    #                                                                                                                 #
    ###################################################################################################################
    
    def loadLibaryFolder(self, audio_dir = None):
        if type(audio_dir) is not str:
            audio_dir = QFileDialog.getExistingDirectory(self, "Select a Audio Libary", self.audio_dir)
        if audio_dir != '':
            self.openButton.setEnabled(False)
        
            self.audio_dir = audio_dir
            self.files = listdir(audio_dir)
            self.files = list(filter(lambda k: '#classified' not in k, self.files))
            self.refreshLabelButton()
            console.info(f'loaded folder ${audio_dir}') 
            self.loadNextFile()
            