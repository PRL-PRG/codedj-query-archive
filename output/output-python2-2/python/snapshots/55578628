from Framework.Core.Event import Event
from Framework.Constants import Constants 
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.Generation.GenerationConstants import GenerationConstants
#----------------------------------------------------------------------
# TODO: extend this hierarchy to include a Note base class
# 		i.e. Event -> Note -> CSoundNote
#		most classes should only deal with Events and Notes, 
#		and not CSoundNotes
#----------------------------------------------------------------------
#----------------------------------------------------------------------
# An Event subclass that represents a CSound note event
#----------------------------------------------------------------------

class CSoundNote( Event ):
    # These callbacks need to be set externally
    getVolumeCallback = None # set in TrackPlayerBase.__init__() [signature: functionName( trackID )]
    
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, onset, 
                                    pitch, 
                                    amplitude, 
                                    pan, 
                                    duration, 
                                    trackID, 
                                    fullDuration = False, 
                                    instrument = CSoundConstants.FLUTE, 
                                    attack = 0.002, 
                                    decay = 0.098, 
                                    reverbSend = 0.1, 
                                    filterType = 0, 
                                    filterCutoff = 1000,
                                    tied = False,
                                    overlap = False  ):
        Event.__init__( self, onset )
        
        self.onset = onset
        self.pitch = pitch
        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackID = trackID
        self.instrument = instrument
        self.fullDuration = fullDuration
        self.attack = attack
        self.decay = decay
        self.reverbSend = reverbSend
        self.filterType = filterType
        self.filterCutoff = filterCutoff
        self.tied = tied
        self.overlap = overlap

    def __getstate__(self):
        return {'pitch': self.pitch,
                'amplitude': self.amplitude,
                'pan': self.pan,
                'duration': self.duration,
                'trackID': self.trackID,
                'instrument': self.instrument,
                'fullDuration': self.fullDuration,
                'attack': self.attack,
                'decay': self.decay,
                'reverbSend': self.reverbSend,
                'filterType': self.filterType,
                'filterCutoff': self.filterCutoff,
                'onset': self.onset,
                'tied': self.tied,
                'overlap': self.overlap }

    def __setstate__(self,dict):
        Event.__init__(self, dict['onset'])
        self.pitch = dict['pitch']
        self.amplitude = dict['amplitude']
        self.pan = dict['pan']
        self.duration = dict['duration']
        self.trackID = dict['trackID']
        self.instrument = dict['instrument']
        self.fullDuration = dict['fullDuration']
        self.attack = dict['attack']
        self.decay = dict['decay']
        self.reverbSend = dict['reverbSend']
        self.filterType = dict['filterType']
        self.filterCutoff = dict['filterCutoff']
        self.tied = dict['tied']
        self.overlap = dict['overlap']

    def clone( self ):
        return CSoundNote( self.onset, self.pitch, self.amplitude, self.pan, 
                           self.duration, self.trackID, self.fullDuration,  self.instrument, 
                           self.attack, self.decay, self.reverbSend, self.filterType, self.filterCutoff, self.tied, self.overlap )

    def play( self ):
        CSoundClient.sendText( self.getText(120, 0) )
        
    def getText( self, tempo, delay ):
        # duration for CSound is in seconds
        newPitch = self.getTranspositionFactor( self.pitch )
        oneTickDuration = (Constants.MS_PER_MINUTE / 1000)  / tempo / Constants.TICKS_PER_BEAT
        newDuration = oneTickDuration * self.duration
        # condition for tied notes
        if CSoundConstants.INSTRUMENTS[ self.instrument ].csoundInstrumentID  == 101  and self.tied and self.fullDuration:
            newDuration = -1
        # condition for overlaped notes
        if CSoundConstants.INSTRUMENTS[ self.instrument ].csoundInstrumentID == 102 and self.overlap:
            newDuration = oneTickDuration * self.duration + 1.

        newAmplitude = self.amplitude * self.getVolumeCallback( self.trackID )

        newAttack = newDuration * self.attack
        if newAttack <= 0.002:
            newAttack = 0.002

        newDecay = newDuration * self.decay
        if newDecay <= 0.002:
            newDecay = 0.002

        return CSoundConstants.PLAY_NOTE_COMMAND % ( CSoundConstants.INSTRUMENTS[ self.instrument ].csoundInstrumentID, 
                                                     self.trackID, 
                                                     delay,
                                                     newDuration, 
                                                     newPitch, 
                                                     self.reverbSend, 
                                                     newAmplitude, 
                                                     self.pan, 
                                                     CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ self.instrument ].instrumentID,
                                                     newAttack,
                                                     newDecay,
                                                     self.filterType,
                                                     self.filterCutoff )
    def play ( self ):
        CSoundClient.sendText( self.getText(120,0) )

    def getTranspositionFactor( self, pitch ):
        return pow( GenerationConstants.TWO_ROOT_TWELVE, pitch - 36 )
    
    #-----------------------------------
    # adjustment functions
    #-----------------------------------
    def adjustDuration( self, amount ):
        self.duration += amount
        
    def adjustAmplitude( self, amount ):
        self.amplitude += amount
            
    def adjustPitch( self, amount ):
        self.pitch += amount
        
