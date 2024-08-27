import librosa

from numpy import (
    array,
    var,
    mean,
    float64
)

from numpy.random import seed
seed(42)
class Segment:
    def __init__(self):
        self.features = {}
        self.results = {}

    def loadAudio(self, path, offset, length, target_sr):
        # TODO add  random offset by seed
        self.audio_data, self.rate = librosa.load(
            path, offset=offset, duration=length, sr=target_sr)
        # self.audio_data, _ = librosa.effects.trim(self.audio_data) # TODO check effects!

    def addFeature(self, name ,function, state=None):
        result = function() if type(function) == type(self.addFeature) else function

        if state is None:
            state = len(array(result).shape)
        self.results[name] = result
        if state == 0:
            self.features[name] = result
        elif state == 1:
            self.features[f'{name}_var'] = var(result, dtype=float64)
            self.features[f'{name}_mean'] = mean(result, dtype=float64)
        elif state == 2:
            for i, value in enumerate(result):
                self.features[f'{name}{i+1}_var'] = var(value, dtype=float64)
                self.features[f'{name}{i+1}_mean'] = mean(value, dtype=float64)
        else:
            raise ValueError(f'Invalid state {state}')

    def loadFeatures(self):
        self.addFeature('length', self.length, 0)
        self.addFeature('tempo', self.tempo, 0)
        
        # self.addFeature('mfcc', self.mfcc, 2)
        self.addFeature('mfcc_perc', self.mfcc_perc, 2)
        self.addFeature('mfcc_harmony', self.mfcc_harmony, 2)
        
        self.addFeature('chroma_stft', self.chroma_stft, 2)
        
        self.addFeature('mfcc_delta', self.mfcc_delta, 1)
        self.addFeature('mfcc_delta_delta', self.mfcc_delta_delta, 1)
        
        self.addFeature('rms', self.rms, 1) # TODO more prozessing
        self.addFeature('tonnetz', self.tonnetz, 1)
        # self.addFeature('spectral_centroid', self.spectral_centroid, 1)
        self.addFeature('spectral_contrast', self.spectral_contrast, 1)
        self.addFeature('zero_crossing_rate', self.zero_crossing_rate, 1)
        # self.addFeature('spectral_bandwidth', self.spectral_bandwidth, 1)
        # self.addFeature('spectral_rolloff', self.spectral_rolloff, 1)
        # self.addFeature('harmony', self.harmony, 1)
        # self.addFeature('perceptr', self.perc, 1)

    tonnetz_ = None
    def tonnetz(self):
        if self.tonnetz_ is not None:
            return self.tonnetz_
        self.tonnetz_ = librosa.feature.tonnetz(y=self.harmonic())
        return self.tonnetz_

    harmonic_ = None
    def harmonic(self):
        if self.harmonic_ is not None:
            return self.harmonic_
        self.harmonic_ = librosa.effects.harmonic(y=self.harmony())
        return self.harmonic_

    mfcc_ = None
    def mfcc(self):
        if self.mfcc_ is not None:
            return self.mfcc_
        self.mfcc_ = librosa.feature.mfcc(y=self.audio_data, sr=self.rate)
        return self.mfcc_

    mfcc_delta_ = None
    def mfcc_delta(self):
        if self.mfcc_delta_ is not None:
            return self.mfcc_delta_
        self.mfcc_delta_ = librosa.feature.delta(self.mfcc(), order = 1)
        return self.mfcc_delta_
    
    mfcc_delta_delta_ = None
    def mfcc_delta_delta(self):
        if self.mfcc_delta_delta_ is not None:
            return self.mfcc_delta_delta_
        self.mfcc_delta_delta_ = librosa.feature.delta(self.mfcc(), order = 2)
        return self.mfcc_delta_delta_

    mfcc_harmony_ = None
    def mfcc_harmony(self):
        if self.mfcc_harmony_ is not None:
            return self.mfcc_harmony_
        self.mfcc_harmony_ = librosa.feature.mfcc(y=self.harmony(), sr=self.rate, n_mfcc=40)[-20:][::3]
        return self.mfcc_harmony_
    
    mfcc_perc_ = None
    def mfcc_perc(self):
        if self.mfcc_perc_ is not None:
            return self.mfcc_perc_
        self.mfcc_perc_ = librosa.feature.mfcc(y=self.perc(), sr=self.rate, n_mfcc=40, lifter=2 * 40)[::3]
        return self.mfcc_perc_

    stft_ = None
    def stft(self):
        if self.stft_ is not None:
            return self.stft_
        self.stft_ = librosa.core.stft(self.audio_data)
        return self.stft_

    chroma_stft_ = None
    def chroma_stft(self):
        if self.chroma_stft_ is not None:
            return self.chroma_stft_
        self.chroma_stft_ = librosa.feature.chroma_stft(S=self.stft(), sr=self.rate)
        return self.chroma_stft_

    spectral_centroid_ = None
    def spectral_centroid(self):
        if self.spectral_centroid_ is not None:
            return self.spectral_centroid_
        self.spectral_centroid_ = librosa.feature.spectral_centroid(self.audio_data)
        return self.spectral_centroid_

    spectral_bandwidth_ = None
    def spectral_bandwidth(self):
        if self.spectral_bandwidth_ is not None:
            return self.spectral_bandwidth_
        self.spectral_bandwidth_ = librosa.feature.spectral_bandwidth(self.audio_data)
        return self.spectral_bandwidth_

    spectral_rolloff_ = None
    def spectral_rolloff(self):
        if self.spectral_rolloff_ is not None:
            return self.spectral_rolloff_
        self.spectral_rolloff_ = librosa.feature.spectral_rolloff(self.audio_data)
        return self.spectral_rolloff_

    spectral_contrast_ = None
    def spectral_contrast(self):
        if self.spectral_contrast_ is not None:
            return self.spectral_contrast_
        self.spectral_contrast_ = librosa.feature.spectral_contrast(y = self.audio_data)
        return self.spectral_contrast_

    zero_crossing_rate_ = None
    def zero_crossing_rate(self):
        if self.zero_crossing_rate_ is not None:
            return self.zero_crossing_rate_
        self.zero_crossing_rate_ = librosa.feature.zero_crossing_rate(self.audio_data)
        return self.zero_crossing_rate_

    rms_ = None
    def rms(self):
        if self.rms_ is not None:
            return self.rms_
        self.rms_ = librosa.feature.rms(self.audio_data)
        return self.rms_

    harmony_ = None
    def harmony(self):
        if self.harmony_ is not None:
            return self.harmony_
        self.hpss()
        return self.harmony_

    perc_ = None
    def perc(self):
        if self.perc_ is not None:
            return self.perc_
        self.hpss()
        return self.perc_

    def hpss(self):
        self.harmony_, self.perc_ = librosa.effects.hpss(y = self.audio_data)

    tempo_ = None
    def tempo(self):
        if self.tempo_ is not None:
            return self.tempo_
        _tempo = librosa.beat.tempo(self.audio_data)[0]
        _tempo = round(_tempo % 180)
        self.tempo_ = _tempo
        return self.tempo_

    length_ = None
    def length(self):
        if self.length_ is not None:
            return self.length_
        self.length_ = len(self.audio_data)
        return self.length_ 
