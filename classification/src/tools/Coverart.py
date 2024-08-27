   
from re import findall
from os.path import (
    basename,
    splitext
)

from src.tools.BingDownloader import download

def loadAudioName(filename):
    filename = basename(filename).lower()
    ext = splitext(filename)[-1]
    filename = splitext(filename)[0]
    words = findall(r'[a-zA-Z0-9]+', filename)
    words = sorted(set(words))
    words = list(filter(lambda word: not (word.isdigit() and int(word) >= 10000) , words))
    words = list(filter(lambda word: word not in ['remix'], words))
    filename = ' '.join(words)
    return filename

def loadCoverart(filename):
    filename = loadAudioName(filename)
    download(filename, limit=1,  output_dir='img', adult_filter_off=True, force_replace=False, timeout=60, verbose=False)
