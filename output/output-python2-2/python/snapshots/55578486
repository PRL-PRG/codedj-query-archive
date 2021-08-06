import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from GUI.Core.KeyboardStandAlone import KeyboardStandAlone


class StandAlonePlayer( gtk.Window ):
    
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_title('TamTam Player')
        
        CSoundClient.initialize()
        CSoundClient.setMasterVolume(100)
        
        self.connect( "destroy" , self.destroy )
        self.mainWindowBox = gtk.VBox()
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.drawInstrumentList()
        self.drawReverb()
        
        self.show_all()       
         
    def drawInstrumentList( self ):
        instrumentMenu = gtk.Menu()
        CSoundInstruments = CSoundConstants.INSTRUMENTS.keys()
        cleanInstrumentList = []
        for instrumentName in CSoundInstruments:
            if not instrumentName[0: 4] == 'drum' and not instrumentName[0: 3] == 'mic':
               cleanInstrumentList.append( instrumentName )
        cleanInstrumentList.append("drum1kit")
        cleanInstrumentList.sort()
        for instrument in cleanInstrumentList:
            menuItem = gtk.MenuItem(instrument)
            menuItem.connect_object("activate" , self.setInstrument , instrument)
            instrumentMenu.append(menuItem)

        instrumentMenuBar = gtk.MenuBar()
        
        menuBarItem = gtk.MenuItem('Choose Instrument')
        menuBarItem.set_submenu(instrumentMenu)
        instrumentMenuBar.append(menuBarItem)
        
        self.mainWindowBox.add(instrumentMenuBar)
        self.instrumentLabel = gtk.Label('flute')
        self.mainWindowBox.add(self.instrumentLabel)
    
    def drawReverb( self ):
        reverbAdjustment = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = gtk.HScale(adjustment = reverbAdjustment)
        reverbSlider.set_draw_value(False)
        reverbAdjustment.connect("value_changed" , self.setReverb)
        
        reverbLabel = gtk.Label("Reverb")
        self.mainWindowBox.add(reverbSlider)
        self.mainWindowBox.add(reverbLabel)
            
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone()
        
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.connect( "key-press-event", self.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.keyboardStandAlone.onKeyRelease )
        self.connect( "button-press-event", self.keyboardStandAlone.onButtonPress )
    
    def setInstrument( self , instrument ):
        self.keyboardStandAlone.setInstrument(instrument)
        self.instrumentLabel.set_text(instrument)
        
    def setReverb(self,adj):
        self.keyboardStandAlone.setReverb(adj.value)
    
    def destroy( self, widget ):
        gtk.main_quit()

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()