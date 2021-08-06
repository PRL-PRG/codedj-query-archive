import pygtk
pygtk.require( '2.0' )
import gtk
from Framework.CSound.CSoundNote import CSoundNote
from GUI.Core.KeyMapping import KEY_MAP

class KeyboardInput:
    def __init__( self , getCurrentTick , trackInstruments ):
        self.active = False
        self.record = False
        self.key_dict = dict()
        self.getCurrentTick = getCurrentTick
        self.trackInstruments = trackInstruments
        
    def volumeFunction(self):
        return 1.0
    def getTempoCallback(self):
        return 60
        
    def onKeyPress(self,widget,event):
        if not self.active:
            return
        
        key = event.hardware_keycode 
        # If the key is already in the dictionnary, exit function (to avoir key repeats)
        if self.key_dict.has_key(key):
                return
        # Assign on which track the note will be created according to the number of keys pressed    
        track = len(self.key_dict)+10
        # If the pressed key is in the keymap
        if KEY_MAP.has_key(key):
            # CsoundNote parameters
            onset = self.getCurrentTick()
            pitch = KEY_MAP[key]
            amplitude = 1
            pan = 0.5
            duration = -1
            trackID = track
            #volumeFunction = False
            #getTempoCallback = False
            tied = False
            print self.trackInstruments
            instrument = self.trackInstruments[0]
            # Create and play the note
            self.key_dict[key] = CSoundNote(onset, pitch, amplitude, pan, duration, trackID, self.volumeFunction, self.getTempoCallback, tied, instrument)
            self.key_dict[key].play()
                
        #print track-9
        
    
    def onKeyRelease(self,widget,event):
        if not self.active:
            return
        key = event.hardware_keycode 
        
        if KEY_MAP.has_key(key):
            self.key_dict[key].duration = 0
            self.key_dict[key].amplitude = 0
            self.key_dict[key].play()
            self.key_dict[key].duration = self.getCurrentTick() - self.key_dict[key].onset
            print "onset",self.key_dict[key].onset
            print "dur",self.key_dict[key].duration
            del self.key_dict[key]
            
    
    def onButtonPress(self,widget,event):
        pass
        
