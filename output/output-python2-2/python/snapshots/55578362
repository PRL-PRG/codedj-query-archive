import pygtk
pygtk.require( '2.0' )
import gtk
import os

from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from GUI.Core.KeyboardStandAlone import KeyboardStandAlone
from Framework import Note


class StandAlonePlayer( gtk.Window ):
    
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_title('TamTam Player')
        self.set_resizable(False)
        
        self.row_counter = 0
        
        CSoundClient.initialize()
        CSoundClient.setMasterVolume(100)
        
        self.connect( "destroy" , self.destroy )
        self.mainWindowBox = gtk.VBox()
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        #self.drawInstrumentList()
        self.drawMainInstrumentButton()
        self.drawMicButton()
        self.drawReverb()
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
        
    def drawMicButton( self ):
        self.micButton = gtk.Button(label='MicRec')
        self.micButton.connect('clicked' , self.handleMicButtonClick)
        self.mainWindowBox.add(self.micButton)        
    
    def drawReverb( self ):
        reverbAdjustment = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = gtk.HScale(adjustment = reverbAdjustment)
        reverbSlider.set_draw_value(False)
        reverbAdjustment.connect("value_changed" , self.setReverb)
        
        reverbLabel = gtk.Label("Reverb")
        self.mainWindowBox.add(reverbSlider)
        self.mainWindowBox.add(reverbLabel)
        
    def drawMainInstrumentButton(self):
        self.mainInstrumentButton = gtk.Image()
        self.mainInstrumentButton.set_from_file('tamtam/GUI/Core/images/' + self.getInstrumentList()[0] + '.png')
        instrumentButton = gtk.Button(label=None)
        instrumentButton.set_image(self.mainInstrumentButton)
        instrumentButton.connect('clicked' , self.handleMainInstrumentButton, 'clicked')
        self.mainWindowBox.add(instrumentButton)
        
    def drawInstrumentWindow(self):
        self.instrumentWindow = gtk.Window()
        self.instrumentWindow.set_decorated(False)
        self.instrumentWindow.set_keep_above(True)
           
        vBox = gtk.VBox()
        hBox_row1 = gtk.HBox()
        hBox_row2 = gtk.HBox()
        hBox_row3 = gtk.HBox()
        
        for instrument in self.getInstrumentList():
            self.row_counter = self.row_counter + 1
            instImage = gtk.Image()
            instButton = gtk.Button(label=None)
            instImage.set_from_file('tamtam/GUI/Core/images/' + instrument + '.png')
            instButton.set_image(instImage)
            #instButton.set_relief(gtk.RELIEF_NONE)
            instButton.connect('clicked' , self.handleWindowButtonsClick , instrument)
            instButton.connect('enter' , self.handleWindowButtonsEnter , instrument)
            if self.row_counter <= 3:
                hBox_row1.add(instButton)
            elif self.row_counter >= 4 and self.row_counter <= 6:
                hBox_row2.add(instButton)
            elif self.row_counter >= 7:
                hBox_row3.add(instButton)
        self.row_counter = 0
        
        vBox.add(hBox_row1)
        vBox.add(hBox_row2)
        vBox.add(hBox_row3)
        self.instrumentWindow.add(vBox)
        
    def handleMainInstrumentButton(self , widget , data):
        if self.instrumentWindow.get_property('visible') is True:
            self.instrumentWindow.hide()
            return        
        pos = self.get_position()
        self.instrumentWindow.move(pos[0] + 68 , pos[1] + 24)
        self.instrumentWindow.show_all()
   
    def handleWindowButtonsClick(self , widget , data):
        self.instrumentWindow.hide()
        
    def handleWindowButtonsEnter(self , widget , instrument):
        self.mainInstrumentButton.set_from_file('tamtam/GUI/Core/images/' + instrument + '.png')
        print instrument
        self.setInstrument(instrument)
        self.playInstrumentNote(instrument)

    def handleMicButtonClick(self , widget , data = None):
        CSoundClient.micRecording(20)        
            
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone()
        
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.connect( "key-press-event", self.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.keyboardStandAlone.onKeyRelease )
        #self.connect( "button-press-event", self.button )
    
    def setInstrument( self , instrument ):
        self.keyboardStandAlone.setInstrument(instrument)
        
    def setReverb(self,adj):
        self.keyboardStandAlone.setReverb(adj.value)
        
    def playInstrumentNote(self , instrument):
        note = Note.note_new(onset = 0, 
                             pitch = 24, 
                             amplitude = 1, 
                             pan = 0.5, 
                             duration = 8, 
                             trackID = 1, 
                             fullDuration = False, 
                             instrument = instrument, 
                             instrumentFlag = instrument,
                             reverbSend = 0)
        Note.note_play(note)
        
    def getInstrumentList(self):
        CSoundInstruments = CSoundConstants.INSTRUMENTS.keys()
        cleanInstrumentList = []
        for instrumentName in CSoundInstruments:
            if not instrumentName[0: 4] == 'drum':# and not instrumentName[0: 3] == 'mic':
               cleanInstrumentList.append( instrumentName )
        cleanInstrumentList.append("drum1kit")
        cleanInstrumentList.sort()
        return cleanInstrumentList
    
    def destroy( self, widget ):
        gtk.main_quit()

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()