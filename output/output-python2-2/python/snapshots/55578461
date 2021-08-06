import pygtk
pygtk.require( '2.0' )
import gtk

from Framework import Note
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from GUI.Core.KeyMapping import KEY_MAP

class KeyboardStandAlone:
    def __init__( self ):        
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
        if KEY_MAP.has_key(key):
            # CsoundNote parameters
            pitch = KEY_MAP[key]
            duration = -1
            instrument = self.instrument
            
            if event.state == gtk.gdk.MOD1_MASK:
                pitch = pitch+5
                
            if instrument == 'drum1kit':
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ GenerationConstants.DRUMPITCH[ pitch ] ]
                else:
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                pitch = 36
                duration = 100

            if CSoundConstants.INSTRUMENTS[instrument].csoundInstrumentID == 102:    #Percussions resonance
                duration = 100
            # Create and play the note
            self.key_dict[key] = Note.note_new(onset = 0, 
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
            Note.note_play(self.key_dict[key])
            
    def onKeyRelease(self,widget,event):
        key = event.hardware_keycode
        
        if KEY_MAP.has_key(key):
            self.key_dict[key]['duration'] = 0
            self.key_dict[key]['amplitude'] = 0
            self.key_dict[key]['dirty'] = True
            Note.note_play(self.key_dict[key])
            del self.key_dict[key]
            
    def onButtonPress( self, widget, event ):
        pass
            
