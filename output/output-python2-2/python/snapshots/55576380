import pygtk
pygtk.require( '2.0' )
import gtk

import Config
#TODO: this is a suprising dependency... what's up??
from Generation.GenerationConstants import GenerationConstants
from Util.CSoundNote import CSoundNote

KEY_MAP_PIANO = Config.KEY_MAP_PIANO

class KeyboardStandAlone:
    def __init__( self, client, recordingFunction, adjustDurationFunction, getCurrentTick, getPlayState ):
        self.csnd = client        
        self.recording = recordingFunction
        self.adjustDuration = adjustDurationFunction
        self.getCurrentTick = getCurrentTick
        self.getPlayState = getPlayState
        self.key_dict = dict()
        self.onset_dict = dict()
        self.trackCount = 10
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
        if self.trackCount >= 20:
            self.trackCount = 10
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

                if instrument == 'drum1kit':
                    instrument = Config.DRUM1INSTRUMENTS[ pitch ]
                if instrument == 'drum2kit':
                    instrument = Config.DRUM2INSTRUMENTS[ pitch ]
                if instrument == 'drum3kit':
                    instrument = Config.DRUM3INSTRUMENTS[ pitch ]

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
                                            instrument = instrument, 
                                            instrumentFlag = instrument,
                                            reverbSend = self.reverb)
            self.key_dict[key].playNow(0.3)
            if self.getPlayState():
                recOnset = self.getCurrentTick() / 3
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
            if Config.INSTRUMENTS[ self.key_dict[key].instrument].csoundInstrumentId == Config.INST_TIED:
                self.key_dict[key].duration = 1
                self.key_dict[key].decay = 0.7
                self.key_dict[key].amplitude = 1
                self.key_dict[key].playNow(0.3)
                if self.getPlayState():
                    self.adjustDuration(self.key_dict[key].pitch, self.onset_dict[key])
                del self.key_dict[key]
            else:
                del self.key_dict[key]
        if self.getPlayState():
            if self.onset_dict.has_key(key):
                del self.onset_dict[key]
    
    def onButtonPress( self, widget, event ):
        pass
            
