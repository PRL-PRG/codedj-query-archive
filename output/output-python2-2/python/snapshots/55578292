import pygtk
pygtk.require( '2.0' )
import gtk
import os

from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Player.KeyboardStandAlone import KeyboardStandAlone
from Player.NoteStdAlone import NoteStdAlone
from Player.RythmPlayer import RythmPlayer
from Player.RythmGenerator import RythmGenerator

class StandAlonePlayer( gtk.Window ):
    
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_title('TamTam Player')
        self.set_resizable(False)
        
        self.rythmPlayer = RythmPlayer()

        self.instrument = self.getInstrumentList()[0]
        #self.setInstrument(self.instrument)
        
        CSoundClient.initialize()
        CSoundClient.setMasterVolume(100)
        
        self.connect( "destroy" , self.destroy )
        self.mainWindowBox = gtk.VBox()
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.drawInstrumentButtons()
        self.drawMicButton()
        self.drawReverb()
        self.drawTempo()
        self.drawPlayButton()
        self.drawGeneration()
       
        self.show_all()      
        
    def drawMicButton( self ):
        micButtonImg = gtk.Image()
        micButtonImg.set_from_file(Constants.TAM_TAM_ROOT + '/GUI/Core/images/record.png')
        
        self.micButton = gtk.Button()
        self.micButton.set_image(micButtonImg)
        self.micButton.connect('pressed' , self.handleMicButtonClick)
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
        
    def drawTempo( self ):
        tempoAdjustment = gtk.Adjustment(value=90, lower=40, upper=240, step_incr=1, page_incr=0, page_size=0)
        tempoSlider = gtk.HScale(adjustment = tempoAdjustment)
        tempoSlider.set_draw_value(False)
        tempoAdjustment.connect("value_changed" , self.setTempo)
        
        tempoLabel = gtk.Label("Tempo")
        self.mainWindowBox.add(tempoSlider)
        self.mainWindowBox.add(tempoLabel)
        
    def drawPlayButton( self ):
        playButton = gtk.ToggleButton(label='Play/Stop')
        playButton.connect('toggled' , self.handlePlayButton)
        self.mainWindowBox.add(playButton)
        
    def drawGeneration( self ):
        generationButton = gtk.Button(label='Generate')
        generationButton.connect('clicked' , self.handleGenerateButton)
        
        geneAdjustment = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = gtk.HScale(adjustment = geneAdjustment)
        geneSlider.set_draw_value(False)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        
        self.mainWindowBox.add(generationButton)
        self.mainWindowBox.add(geneSlider)
        
    def drawInstrumentButtons(self):
        ROW_LEN = 4
                   
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
        self.mainWindowBox.add(vBox)
   
    def handleWindowButtonsClick(self , widget , instrument):
        self.setInstrument(instrument)
        if instrument[0:3] == 'mic':
            self.micButton.show()
        else:
            self.micButton.hide()            
        
    def handleWindowButtonsEnter(self , widget , instrument):
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
            
    def handlePlayButton(self, widget, data=None):
          if widget.get_active():
              self.rythmPlayer.startPlayback()
          else:
              self.rythmPlayer.stopPlayback()

    def handleGenerateButton(self , widget , data=None):
        pass
    
    def handleGenerationSlider(self, adj):
        pass
    
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
    
    def setTempo(self,adj):
        pass
        
    def playInstrumentNote(self , instrument):
        note = NoteStdAlone( onset = 0, 
                             pitch = 36, 
                             amplitude = 1, 
                             pan = 0.5, 
                             duration = 20, 
                             trackID = 1, 
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
