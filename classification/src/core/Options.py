from os.path import (
    join,
)

import src.tools.Utilities

class AudioClassificationModel:
    def __init__(self, 
                 name,
                 segments = 10,
                 seccount = 3
                 tracks = 100,
                 ):
        audio_folder_paths = []
        
        self.segments = segments
        self.seccount = seccount
        
        self.scaled = None
        self.models = []
        self.historys = []
        
        self.data_dir = join(getDocDir(),)
    
    def loadAudioFolders(self, path):
        pass
    
    def loadAudioFolder(self, path):
        pass