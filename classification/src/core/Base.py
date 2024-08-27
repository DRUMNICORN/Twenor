from pandas import DataFrame
from pandas import read_csv
from tqdm.std import tqdm
from os import (
    makedirs
)
from os.path import (
    join,
    exists,
    splitext
)

class Base:
    SAVE_ENABLED = False
    def __init__(self, data_dir,  segments = 10, tracks = 100, offset = 0, seconds = 3, rate = 22050, batch_size = 128):
        self.tracks = tracks
        self.segments = segments
        self.offset = offset
        self.seconds = seconds
        self.rate = rate
        
        # [32, 64] - CPU
        # [128, 256] - GPU for more boost
        self.batch_size = batch_size

        self.data_dir = join('data', data_dir)
        print(f'Data Directory: {self.data_dir}')
        if not exists(self.data_dir):
            makedirs(self.data_dir)
        
        self.df = DataFrame(columns=['filename'])
        self.library = {}
        self.database = {}
        # pass
    
    def generateInformation(self):
        print("Dataset has",self.df.shape)
        print("Count of Positive and Negative samples")
        print("Columns with NA values are",list(self.df.columns[self.df.isnull().any()]))
        print("Expected duration is", {self.df.length[0]})
        print("Columns with DIFFRENT values are", list(self.df.length.loc[self.df.length != self.df.length[0]]))
        print(self.df.label.value_counts().reset_index())
        print(self.df.head())
        print(len(self.df.columns))
    
    def dropMissingLabel(self, label):
        self.df = self.df.loc[self.df['label'] != label]
    
    def normalizeDataFrame(self):  # sourcery skip: avoid-builtin-shadow
        min = self.df.label.value_counts().min()
        self.df = self.df.groupby('label').head(min).reset_index()
        self.SAVE_ENABLED = False
    
    def saveDataFrame(self):
        col = self.df.pop("filename")
        self.df.insert(0, col.name, col)
        if(self.SAVE_ENABLED == False):
            self.df.to_csv(join(self.data_dir, 'db.min.csv'), index=False)
        else:
            self.df.to_csv(join(self.data_dir, 'db.csv'), index=False)
    
    def loadDataFrame(self):
        expectedColumns = self.database[list(self.database.keys())[0]][0][0].keys()
        self.df = DataFrame(columns=expectedColumns)
        self.loadDataFrameFile()
        self.updateDataFrame()
        self.saveDataFrame()
        
    def loadDataFrameFile(self):
        if exists(join(self.data_dir, 'db.csv')):
            self.df = read_csv(join(self.data_dir, 'db.csv'))
            # print(self.df.head())
                
    def updateDataFrame(self):
        for label in list(self.database.keys()):
            for file in self.database[label]:
                for segment in file:
                    if(len(self.df.loc[self.df['filename'] == segment['filename']]) == 0):
                        segment.update({
                            "label": label
                        })
                        self.df = self.df.append(segment, ignore_index=True)
    
    def getCountInsideDataFrame(self, col: str, value: str):
        try:
            return len(self.df.loc[self.df[col] == value])
        except Exception:
            return 0
    
    def validateFile(self, file_path, filename):
        acceppted_audio_formats = ['wav', 'mp3'] 
        if (splitext(file_path)[-1] in acceppted_audio_formats):
            return False
        
        if(self.getCountInsideDataFrame('filename', self.getFilename(filename, 0)) > 0):
            return False
        
        return True
    
    def validateFolder(self, label):
        files = len(self.library[label])
        if(files == 0):
            print(f' -> {label}: library empty.')
            return False
        
        if(len(self.library[label]) < self.tracks):
            print(f' -> {label}: library missing {self.tracks - len(self.library[label])} files.')
            # return False
        
        if(self.getCountInsideDataFrame("label", label) / self.segments >= self.tracks):
            print(f' -> {label}: dataframe satisfied with {self.getCountInsideDataFrame("label", label)} rows.')
            return False
    
        return True
    
    def loadProgressBar(self, label, init = 0, max = 0) -> tqdm:
        # sourcery skip: avoid-builtin-shadow
        if max == 0:
            max = self.segments * self.tracks
        return tqdm(total=max, desc=label, bar_format="{desc}\t{bar:20} {n_fmt}/{total_fmt} [{elapsed} -> {remaining}]", initial=init)

    def getFilename(self, filename, i = ''):
        words = splitext(filename)
        return ''.join([words[0], str(i), words[-1]])  
