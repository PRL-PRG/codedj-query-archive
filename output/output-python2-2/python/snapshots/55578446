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
        #self.drawInstrumentList()
        self.drawInstrumentButton()
        self.drawReverb()
        #self.drawImage()
        self.drawInstrumentWindow()
       
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
        
    def drawImage(self):
        image = gtk.Image()
        image.set_from_file('/home/Nat/tamtam/GUI/Core/nat_paint_488x585.png')
        self.mainWindowBox.add(image)
        
    def drawInstrumentButton(self):
        instrumentButton = gtk.ToggleButton(label='Instrument')
        instrumentButton.connect('toggled' , self.handleInstrumentButton)
        self.mainWindowBox.add(instrumentButton)
        
    def handleInstrumentButton(self , widget, data = None):
        print self.get_position()
        if widget.get_active() is True:
            pos = self.get_position()
            self.instrumentWindow.move(pos[0] + 6 , pos[1] + 50)
            self.instrumentWindow.show_all()
        else:
            self.instrumentWindow.hide()
    
    def drawInstrumentWindow(self):
        self.instrumentWindow = gtk.Window()
        self.instrumentWindow.set_decorated(False)
        self.instrumentWindow.set_keep_above(True)
           
        hBox = gtk.HBox()
        image = gtk.Image()
        image.set_from_file('/home/Nat/tamtam/GUI/Core/nat_paint_488x585.png')
        image2 = gtk.Image()
        image2.set_from_file('/home/Nat/tamtam/GUI/Core/nat_paint_488x585.png')
        hBox.add(image)
        hBox.add(image2)
       
        self.instrumentWindow.add(hBox)
            
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone()
        
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.connect( "key-press-event", self.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.keyboardStandAlone.onKeyRelease )
        #self.connect( "button-press-event", self.button )
    
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