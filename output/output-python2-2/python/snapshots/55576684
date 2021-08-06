import pygtk
pygtk.require( '2.0' )
import gtk
import os

from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Player.KeyboardStandAlone import KeyboardStandAlone
from Player.NoteStdAlone import NoteStdAlone

raise 'dont use this class, its the old one'

class StandAlonePlayer( gtk.Window ):
    
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_title('TamTam Player')
        self.set_resizable(False)
        
        self.instrument = self.getInstrumentList()[0]
        #self.setInstrument(self.instrument)
        
        CSoundClient.initialize()
        CSoundClient.setMasterVolume(100)
        
        self.connect( "destroy" , self.destroy )
        self.mainWindowBox = gtk.VBox()
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.drawMainInstrumentButton()
        self.drawMicButton()
        self.drawReverb()
        self.drawInstrumentWindow()
       
        self.show_all()      
        
    def drawMicButton( self ):
        micButtonImg = gtk.Image()
        micButtonImg.set_from_file(Constants.TAM_TAM_ROOT + '/GUI/Core/images/record.png')
        
        self.micButton = gtk.Button()
        self.micButton.set_image(micButtonImg)
        self.micButton.connect('clicked' , self.handleMicButtonClick)
        self.mainWindowBox.add(self.micButton)
        self.micButton.set_no_show_all(True)
        self.micButton.hide()        
    
    def drawReverb( self ):
        reverbAdjustment = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = gtk.HScale(adjustment = reverbAdjustment)
        reverbSlider.set_draw_value(False)
        reverbAdjustment.connect("value_changed" , self.setReverb)
        
        reverbLabel = gtk.Label("Reverb")
        self.mainWindowBox.add(reverbSlider)
        self.mainWindowBox.add(reverbLabel)
        
    def drawMainInstrumentButton(self):
        self.mainInstrumentButtonImg = gtk.Image()
        self.mainInstrumentButtonImg.set_from_file(Constants.TAM_TAM_ROOT + '/GUI/Core/images/' + self.getInstrumentList()[0] + '.png')
        instrumentButton = gtk.Button(label=None)
        instrumentButton.set_image(self.mainInstrumentButtonImg)
        instrumentButton.connect('clicked' , self.handleMainInstrumentButton, 'clicked')
        self.mainWindowBox.add(instrumentButton)
        
    def drawInstrumentWindow(self):
        ROW_LEN = 4
        self.instrumentWindow = gtk.Window()
        self.instrumentWindow.set_decorated(False)
        self.instrumentWindow.set_keep_above(True)
           
        vBox = gtk.VBox()
        
        intrumentNum = len(self.getInstrumentList())
        rows = ( intrumentNum / ROW_LEN )
        if intrumentNum % ROW_LEN is not 0:    #S'il y a un reste
            rows = rows + 1
                    
        for row in range(rows):
            hBox = gtk.HBox()
            for instrument in self.getInstrumentList()[row*ROW_LEN:(row+1)*ROW_LEN]:
                instImage = gtk.Image()
                instButton = gtk.Button(label=None)
                instImage.set_from_file(Constants.TAM_TAM_ROOT + '/GUI/Core/images/' + instrument + '.png')
                instButton.set_image(instImage)
                #instButton.set_relief(gtk.RELIEF_NONE)
                instButton.connect('clicked' , self.handleWindowButtonsClick , instrument)
                instButton.connect('enter' , self.handleWindowButtonsEnter , instrument)
                hBox.add(instButton)
            vBox.add(hBox)
        self.instrumentWindow.add(vBox)
        
    def handleMainInstrumentButton(self , widget , data):
        if self.instrumentWindow.get_property('visible') is True:
            if self.instrument[0:3] is not 'mic':
                self.micButton.hide()
            self.instrumentWindow.hide()
            return        
        pos = self.get_position()
        self.instrumentWindow.move(pos[0] + 68 , pos[1] + 24)
        self.instrumentWindow.show_all()
   
    def handleWindowButtonsClick(self , widget , instrument):
        if instrument[0:3] == 'mic':
            self.micButton.show()
        else:
            self.micButton.hide()
        self.instrumentWindow.hide()
            
        
    def handleWindowButtonsEnter(self , widget , instrument):
        self.mainInstrumentButtonImg.set_from_file(Constants.TAM_TAM_ROOT + '/GUI/Core/images/' + instrument + '.png')
        self.setInstrument(instrument)
        self.playInstrumentNote(instrument)

    def handleMicButtonClick(self , widget , data = None):
        if self.instrument == 'mic1':
            CSoundClient.micRecording(7)
        elif self.instrument == 'mic2':
            CSoundClient.micRecording(8)
        elif self.instrument == 'mic3':
            CSoundClient.micRecording(9)
        elif self.instrument == 'mic4':
            CSoundClient.micRecording(10)
        else:
            print 'Bad instrument'        
            
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone()
        
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.connect( "key-press-event", self.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.keyboardStandAlone.onKeyRelease )
        #self.connect( "button-press-event", self.button )
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
        
    def setReverb(self,adj):
        self.keyboardStandAlone.setReverb(adj.value)
        
    def playInstrumentNote(self , instrument):
        note = NoteStdAlone( onset = 0, 
                             pitch = 36, 
                             amplitude = 1, 
                             pan = 0.5, 
                             duration = 20, 
                             trackId = 1, 
                             fullDuration = False, 
                             instrument = instrument, 
                             instrumentFlag = instrument,
                             reverbSend = 0)
        note.play()
  
    def getInstrumentList(self):
        CSoundInstruments = CSoundConstants.INSTRUMENTS.keys()
        cleanInstrumentList = []
        for instrumentName in CSoundInstruments:
            if not instrumentName[0: 4] == 'drum' and not instrumentName[0: 3] == 'mic':
               cleanInstrumentList.append( instrumentName )
        cleanInstrumentList.append('drum1kit')
        cleanInstrumentList.append('drum2kit')
        cleanInstrumentList.append('drum3kit')
        cleanInstrumentList.sort()
        for n in range(4):
            cleanInstrumentList.append('mic' + str(n+1))
        return cleanInstrumentList
    
    def destroy( self, widget ):
        gtk.main_quit()

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()
