import pygtk
pygtk.require( '2.0' )
import gtk

from Player.NoteStdAlone import NoteStdAlone
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from GUI.Core.KeyMapping import KEY_MAP_PIANO


class KeyboardStandAlone:
    def __init__( self, client ):
        self.csnd = client        
        self.key_dict = dict()
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
        track = len(self.key_dict)+10
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
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                if instrument == 'drum2kit':
                    instrument = CSoundConstants.DRUM2INSTRUMENTS[ pitch ]
                if instrument == 'drum3kit':
                    instrument = CSoundConstants.DRUM3INSTRUMENTS[ pitch ]

                pitch = 36
                duration = 100

            if CSoundConstants.INSTRUMENTS[instrument].csoundInstrumentID == CSoundConstants.INST_PERC:    #Percussions resonance
                duration = 60
            # Create and play the note
            self.key_dict[key] = NoteStdAlone(client = self.csnd,
                                            onset = 0, 
                                            pitch = pitch, 
                                            amplitude = 1, 
                                            pan = 0.5, 
                                            duration = duration, 
                                            trackID = track, 
                                            fullDuration = False, 
                                            instrument = instrument, 
                                            instrumentFlag = instrument,
                                            reverbSend = self.reverb)
            #self.key_dict[key].play()
            self.key_dict[key].play()
            
    def onKeyRelease(self,widget,event):
        key = event.hardware_keycode
        
        if KEY_MAP_PIANO.has_key(key):
            if CSoundConstants.INSTRUMENTS[ self.key_dict[key].instrument].csoundInstrumentID == CSoundConstants.INST_TIED:
                self.key_dict[key].duration = 1
                self.key_dict[key].decay = 0.88
                self.key_dict[key].amplitude = 1
                self.key_dict[key].play()
                del self.key_dict[key]
            else:
                del self.key_dict[key]
            
    def onButtonPress( self, widget, event ):
        pass
            
