from src.core.Analysis import Analysis

from PySide2.QtCore import (QThread, Signal)
from pandas import DataFrame
from numpy import (
    float64,
    empty,
    array,
    append,
    mean
)

class Prediction(QThread):
    change_value = Signal(int)
    result_value = Signal(object)
    
    def __init__(self, file, options):
        super().__init__()
        self.model = options.model
        self.scaler = options.scaler
        self.file = file
        self.segments = options.segments
        self.seconds = options.seconds
        
    def run(self):
        self.predict()
            
    def predict(self):
        a = Analysis(audio_path=self.file, options=self, _ui_bar = self.change_value)
        
        segments = a.loadFeatures()

        self.segments_df = DataFrame(segments, dtype=float64)
        self.segments_df.drop(['length'], axis=1, inplace=True)
        self.segments_df = self.segments_df.reindex(sorted(self.segments_df.columns), axis=1)

        self.segments_df = DataFrame(self.scaler.transform(self.segments_df), columns=self.segments_df.columns)
        self.segments_df.reset_index(drop=True)
        

        q = self.model.predict(self.segments_df)
        # print(X_train.columns)
        self.result = self.loadBetterGuess(q.T)
        self.result_value.emit(self.result)
        
    def stop(self):
        self.terminate()
        
    def loadBetterGuess(self, q):
        r = empty((0,2))
        for i, d in enumerate(q):
            # print(d)
            g = mean(float64(d))
            r = append(r, array([[int(i), float64(g)]]), axis=0)
        return r
