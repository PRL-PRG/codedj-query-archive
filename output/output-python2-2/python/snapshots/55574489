import Config
from Generation.GenerationConstants import GenerationConstants

class CSoundNote :
    NOTE_ID_COUNTER = 0
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self,
            onset, 
            pitch, 
            amplitude, 
            pan, 
            duration, 
            trackId, 
            instrumentId = Config.INSTRUMENTS["flute"].instrumentId, 
            attack = 0.002, 
            decay = 0.098, 
            reverbSend = 0.1, 
            filterType = 0, 
            filterCutoff = 1000,
            tied = False,
            mode = 'edit' ):
        
        self.onset = onset
        self.pitch = pitch
        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackId = trackId
        self.instrumentId = instrumentId
        #temp: catch old code trying to pass in instrument names here
        int(instrumentId)
        self.attack = attack
        self.decay = decay
        self.reverbSend = reverbSend
        self.filterType = filterType
        self.filterCutoff = filterCutoff
        self.tied = tied
        self.mode = mode
        self.nchanges = 0
        self.noteId = self.NOTE_ID_COUNTER
        self.NOTE_ID_COUNTER += 1

    def __getstate__(self):
        return {'onset': self.onset,
                'pitch': self.pitch,
                'amplitude': self.amplitude,
                'pan': self.pan,
                'duration': self.duration,
                'trackId': self.trackId,
                'instrumentId': self.instrumentId,
                'attack': self.attack,
                'decay': self.decay,
                'reverbSend': self.reverbSend,
                'filterType': self.filterType,
                'filterCutoff': self.filterCutoff,
                'tied': self.tied,
                'mode': self.mode }

    def __setstate__(self,dict):
        self.onset = dict['onset']
        self.pitch = dict['pitch']
        self.amplitude = dict['amplitude']
        self.pan = dict['pan']
        self.duration = dict['duration']
        self.trackId = dict['trackId']
        self.instrumentId = dict['instrumentId']
        self.attack = dict['attack']
        self.decay = dict['decay']
        self.reverbSend = dict['reverbSend']
        self.filterType = dict['filterType']
        self.filterCutoff = dict['filterCutoff']
        self.tied = dict['tied']
        self.mode = dict['mode']
        self.nchanges = 0

    def clone( self ):
        return CSoundNote( self.onset, self.pitch, self.amplitude, self.pan, 
                           self.duration, self.trackId, self.instrumentId, 
                           self.attack, self.decay, self.reverbSend, self.filterType, self.filterCutoff, self.tied, self.mode )



