import pygtk
pygtk.require( '2.0' )
import gtk
from Framework.CSound.CSoundNote import CSoundNote
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.CSound.CSoundClient import CSoundClient
from GUI.Core.KeyMapping import KEY_MAP

class KeyboardInputStandAlone:
    def __init__(self):
        self.window = gtk.Window()
        #self.window.set_size_request(100,100)
        self.window.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.window.connect( "key-press-event", self.onKeyPress )
        self.window.connect( "key-release-event", self.onKeyRelease )
        self.window.connect( "button-press-event", self.onButtonPress )
        self.window.connect( "destroy", self.destroy )
        
        self.key_dict = dict()
        
        CSoundClient.initialize()
        CSoundNote.getVolumeCallback = self.getTrackVolume
        
        self.instrument = 'flute'
                
        self.instrumentMenu = gtk.Menu()
        self.CSoundInstruments = CSoundConstants.INSTRUMENTS.keys()
        self.cleanInstrumentList = []
        for instrumentName in self.CSoundInstruments:
            if not instrumentName[0: 4] == 'drum' and not instrumentName[0: 3] == 'mic':
               self.cleanInstrumentList.append( instrumentName )
        self.cleanInstrumentList.append("drum1kit")
        self.cleanInstrumentList.sort()
        for instrument in self.cleanInstrumentList:
            self.menuItem = gtk.MenuItem(instrument)
            self.menuItem.connect_object("activate" , self.setInstrument , instrument)
            self.instrumentMenu.append(self.menuItem)

        
        self.instrumentMenuBar = gtk.MenuBar()
        self.window.add(self.instrumentMenuBar)
        self.menuBarItem = gtk.MenuItem("Instrument")
        self.menuBarItem.set_submenu(self.instrumentMenu)
        self.instrumentMenuBar.append(self.menuBarItem)
        self.window.show_all()
                
    def getTrackVolume( self, trackID ):
        return 1
    def setInstrument(self , instrumentName):
        self.instrument = instrumentName
        
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
            onset = 0
            pitch = KEY_MAP[key]
            amplitude = 1
            pan = 0.5
            duration = -1
            trackID = track
            tied = False
            instrument = self.instrument
            if instrument == 'drum1kit':
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ GenerationConstants.DRUMPITCH[ pitch ] ]
                else:
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                pitch = 36

            instrumentID = CSoundConstants.INSTRUMENTS[instrument].csoundInstrumentID
            if instrumentID == 103 or instrumentID == 102:
                duration = 100
            # Create and play the note
            self.key_dict[key] = CSoundNote(onset, pitch, amplitude, pan, duration, trackID, tied, instrument)
            self.key_dict[key].play()
            
    def onKeyRelease(self,widget,event):
        key = event.hardware_keycode
        
        if KEY_MAP.has_key(key):
            self.key_dict[key].duration = 0
            self.key_dict[key].amplitude = 0
            self.key_dict[key].play()
            del self.key_dict[key]
            
    def onButtonPress( self, widget, event ):
        pass
            
    def destroy( self, widget ):
        gtk.main_quit()
        
if __name__ == "__main__": 
    keyboardInputStandAlone = KeyboardInputStandAlone()
    #start the gtk event loop
    gtk.main()