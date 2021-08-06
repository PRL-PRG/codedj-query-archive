import pygtk
pygtk.require( '2.0' )
import gtk

import Config
#TODO: this is a suprising dependency... what's up??
from Generation.GenerationConstants import GenerationConstants
from Util.NoteDB  import Note
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client

KEY_MAP_PIANO = Config.KEY_MAP_PIANO

class KeyboardStandAlone:
    def __init__( self, recordingFunction, adjustDurationFunction, getCurrentTick, getPlayState ):
        self.csnd = new_csound_client()        
        self.recording = recordingFunction
        self.adjustDuration = adjustDurationFunction
#        self.getCurrentTick = getCurrentTick
        self.getPlayState = getPlayState
        self.key_dict = dict()
        self.onset_dict = dict()
        self.trackCount = 0
        self.instrument = 'flute'
        self.reverb = 0
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        
    def setReverb(self , reverb):
        self.reverb = reverb
        
    def onKeyPress(self,widget,event):
        key = event.hardware_keycode
        # If the key is already in the dictionnary, exit function (to avoir key repeats)
        if self.key_dict.has_key(key):
            return
        # Assign on which track the note will be created according to the number of keys pressed    
        track = self.trackCount
        self.trackCount += 1
        if self.trackCount >= 10:
            self.trackCount = 0
        # If the pressed key is in the keymap
        if KEY_MAP_PIANO.has_key(key):
            # CsoundNote parameters
            pitch = KEY_MAP_PIANO[key]
            duration = -1
            instrument = self.instrument
            
            if event.state == gtk.gdk.MOD1_MASK:
                pitch = pitch+5

            if instrument[ 0: 4 ] == 'drum':
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    pitch = GenerationConstants.DRUMPITCH[ pitch ]

                if Config.INSTRUMENTS[instrument].kit != None:
                    instrument = Config.INSTRUMENTS[instrument].kit[pitch].name
                pitch = 36
                duration = 100

            if Config.INSTRUMENTS[instrument].csoundInstrumentId == Config.INST_PERC:    #Percussions resonance
                duration = 60

            # Create and play the note
            self.key_dict[key] = CSoundNote(onset = 0, 
                                            pitch = pitch, 
                                            amplitude = 1, 
                                            pan = 0.5, 
                                            duration = duration, 
                                            trackId = track, 
                                            fullDuration = False, 
                                            instrumentId = Config.INSTRUMENTS[instrument].instrumentId, 
                                            reverbSend = self.reverb,
                                            tied = True,
                                            mode = 'mini') 
            self.csnd.play(self.key_dict[key], 0.3)
            #self.key_dict[key].playNow(0.3)
            if self.getPlayState():
                recOnset = self.csnd.loopGetTick() / 3
                self.onset_dict[key] = recOnset
                self.recording( CSoundNote(
                                     onset = recOnset, 
                                     pitch = pitch, 
                                     amplitude = 1, 
                                     pan = 0.5, 
                                     duration = 100, 
                                     trackId = track,
                                     decay = .1, 
                                     fullDuration = False, 
                                     instrument = instrument, 
                                     instrumentFlag = instrument,
                                     reverbSend = self.reverb))
            
    def onKeyRelease(self,widget,event):
        key = event.hardware_keycode
        
        if KEY_MAP_PIANO.has_key(key):
            csnote = self.key_dict[key]
            if Config.INSTRUMENTSID[ csnote.instrumentId ].csoundInstrumentId == Config.INST_TIED:
                csnote.duration = .5
                csnote.decay = 0.7
                csnote.amplitude = 1
                csnote.tied = False
                csnote.mode = 'mini'
                self.csnd.play(csnote, 0.3)
                if self.getPlayState():
                    self.adjustDuration(csnote.pitch, self.onset_dict[key])
            del self.key_dict[key]
        if self.getPlayState():
            if self.onset_dict.has_key(key):
                del self.onset_dict[key]
    
    def onButtonPress( self, widget, event ):
        pass
            
