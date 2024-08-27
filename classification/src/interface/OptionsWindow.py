from os.path import join
from os import listdir

from PySide2.QtGui import (
    QIcon,
)
from PySide2.QtCore import (
    QPoint,
    Qt,
    Signal,
)

from PySide2.QtWidgets import (
    QComboBox,
    QHBoxLayout,
    QLabel,
    QPushButton,
    QSizePolicy,
    QSlider, 
    QSpacerItem,
    QVBoxLayout,
    QWidget,
)

class OptionsWindow(QWidget):
    saved = Signal(str, str, int, int, bool, bool)
    isOpen = False
    
    def __init__(self, storage_location, data_dir, model_dir, segments, autoTolarance, autoMove, autoNext):
        super().__init__()
        isOpen = True
        
        self.storage_location = storage_location
        self.data_dir = data_dir
        self.model_dir = model_dir
        self.segments = segments
        self.autoTolarance = autoTolarance
        self.autoMove = autoMove
        self.autoNext = autoNext
        
        self.WIDTH = 80*5
        self.HEIGHT = 20*5
        
        self.windowHeader = 'Options'
        self.setWindowTitle("Emotions are Real | Options")
        # QMainWindow.__init__(self, None, Qt.WindowStaysOnTopHint)
        self.setWindowFlag(Qt.FramelessWindowHint)
        self.setWindowIcon(QIcon('./imgwindow-icon.png'))
        self.setAttribute(Qt.WA_TranslucentBackground)
        self.setWindowOpacity(1)
        self.resize(self.WIDTH, self.HEIGHT)
        self.setupUi()
    
    def saveOptions(self):
        self.saved.emit(self.data_dir, self.model_dir, self.segments, self.autoTolarance, self.autoMove, self.autoNext)
    
    def updateModelDir(self, dir):
        self.model_dir = dir
        
    def updateDataDir(self, dir):
        self.data_dir = dir
        self.modelSelectorComboBox.clear()
        self.setupModelComBox()
                
    def updateSegments(self, segments):
        self.segments = segments
        self.segmentSliderLabel.setText(f'{str(self.segments)} Segments')
            
    def updateTolorance(self, autoTolarance):
        self.autoTolarance = autoTolarance
        self.toloranceSliderLabel.setText(f'{str(self.autoTolarance)}%')
            
    def setupUi(self):
        self.setupUiElements()
        self.setupUiStructure()
        self.setupUiEvents()
    
    def setupUiElements(self):
        self.expandingVSpacer = QSpacerItem(1, 1, QSizePolicy.Minimum, QSizePolicy.Expanding)
        self.expandingHSpacer = QSpacerItem(1, 1, QSizePolicy.Expanding, QSizePolicy.Minimum)
        
        self.hSpacer = QSpacerItem(20, 10, QSizePolicy.Minimum, QSizePolicy.Maximum)
        self.vSpacer = QSpacerItem(10, 20, QSizePolicy.Maximum, QSizePolicy.Minimum)
        
        self.exitButton = QPushButton(objectName="toolbarButton", clicked =self.quitUi)
        self.exitButton.setIcon(QIcon('./img/close.png'))
        
        self.saveButton = QPushButton(objectName="toolbarButton", clicked =self.saveOptions)
        self.saveButton.setIcon(QIcon('./img/save.png'))
        
        self.datasetSelectorComboBox = QComboBox(objectName="selector")
        self.setupDataComBox()
        
        self.modelSelectorComboBox = QComboBox(objectName="selector")
        self.setupModelComBox()

        self.windowLabel = QLabel(self.windowHeader, objectName="windowLabel")
        
        self.segmentSlider = QSlider(Qt.Horizontal, minimum=1, maximum=50, objectName="positionSlider")
        self.segmentSliderLabel = QLabel(f'{str(self.segments)} Segments', objectName="optionValueLabel")
        self.segmentSlider.setValue(self.segments)
        
        self.toloranceSlider = QSlider(Qt.Horizontal, minimum=0, maximum=100, objectName="positionSlider")
        self.toloranceSliderLabel = QLabel(f'{str(self.autoTolarance)}%', objectName="optionValueLabel")
        self.toloranceSlider.setValue(self.autoTolarance)
        
        self.autoMoveButton = QPushButton(objectName="labelButtons", clicked=self.updateAutoMove)
        self.autoMoveButton.setText('Move files automatic: ' + ('On' if self.autoMove else 'Off'))
        
        self.autoNextButton = QPushButton(objectName="labelButtons", clicked=self.updateAutoNext)
        self.autoNextButton.setText('Next files automatic: ' + ('On' if self.autoNext else 'Off'))


    def updateAutoNext(self):
        self.autoNext = not self.autoNext
        self.autoNextButton.setText('Next files automatic: ' + ('On' if self.autoNext else 'Off'))

    def updateAutoMove(self):
        self.autoMove = not self.autoMove
        self.autoMoveButton.setText('Move files automatic: ' + ('On' if self.autoMove else 'Off'))

        self.autoMoveButton = QPushButton(objectName="labelButtons", clicked=self.updateAutoMove)
        self.autoMoveButton.setText('Move files automatic:' + ('On' if self.autoMove else 'Off'))

    def updateAutoMove(self):
        self.autoMove = not self.autoMove
        self.autoMoveButton.setText('Move files automatic: ' + ('On' if self.autoMove else 'Off'))

    def setupDataComBox(self):
        trained = listdir(self.storage_location)
        for i, train in enumerate(trained):
            if(i == 0):
                self.data_dir = train
            self.datasetSelectorComboBox.addItem(train)
            
        index = self.datasetSelectorComboBox.findText(self.data_dir, Qt.MatchFixedString)
        if index >= 0:
            self.datasetSelectorComboBox.setCurrentIndex(index)

    def setupModelComBox(self):
        models = listdir(join(self.storage_location, self.data_dir))
        for model in models:
            if '.h5' in model:
                if 'best' in model:
                    self.model_dir = model
                self.modelSelectorComboBox.addItem(model)

        index = self.modelSelectorComboBox.findText(self.model_dir)
        if index >= 0:
            self.modelSelectorComboBox.setCurrentIndex(index)
    
    def setupUiEvents(self):
        self.datasetSelectorComboBox.currentTextChanged.connect(self.updateDataDir)
        self.modelSelectorComboBox.currentTextChanged.connect(self.updateModelDir)
        self.segmentSlider.valueChanged.connect(self.updateSegments)
        self.toloranceSlider.valueChanged.connect(self.updateTolorance)

    def setupUiStructure(self):
        segmentsliderLayout = QHBoxLayout()
        segmentsliderLayout.addWidget(self.segmentSliderLabel)
        segmentsliderLayout.addWidget(self.segmentSlider)
        
        
        tolorancesliderLayout = QHBoxLayout()
        tolorancesliderLayout.addWidget(self.toloranceSliderLabel)
        tolorancesliderLayout.addWidget(self.toloranceSlider)
        
        
        optionsContainer = QWidget(objectName = "playerContainer")
        optionsLayout = QVBoxLayout(optionsContainer)
        optionsLayout.addWidget(self.datasetSelectorComboBox)
        optionsLayout.addWidget(self.modelSelectorComboBox)
        optionsLayout.addLayout(segmentsliderLayout)
        optionsLayout.addLayout(tolorancesliderLayout)
        optionsLayout.addWidget(self.autoMoveButton)
        optionsLayout.addWidget(self.autoNextButton)
        optionsLayout.addItem(self.expandingVSpacer)
        
        toolbarContainer = QWidget(objectName = "toolbarContainer")
        toolbarLayout = QHBoxLayout(toolbarContainer)
        toolbarLayout.addWidget(self.windowLabel)
        toolbarLayout.addItem(self.expandingHSpacer)
        toolbarLayout.addWidget(self.saveButton, alignment= Qt.AlignRight | Qt.AlignTop)
        toolbarLayout.addWidget(self.exitButton, alignment= Qt.AlignRight | Qt.AlignTop)

        layoutContainer = QWidget(objectName = "window")
        layout = QVBoxLayout(layoutContainer)
        layout.addWidget(toolbarContainer)
        layout.addWidget(optionsContainer)
        
        layoutLayout = QHBoxLayout()
        layoutLayout.addWidget(layoutContainer)
        self.setLayout(layoutLayout)

    def quitUi(self):
        self.close()
        isOpen = False
                
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