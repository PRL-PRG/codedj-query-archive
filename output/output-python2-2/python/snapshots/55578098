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
from Player.RythmGenerator import *

class StandAlonePlayer( gtk.EventBox ):
    
    def __init__(self):
        gtk.EventBox.__init__( self)
        self.IMAGE_ROOT = Constants.TAM_TAM_ROOT + '/Resources/Images/'

        self.instrument = self.getInstrumentList()[0]
        self.reverbSend = 0.
        self.rythmInstrument = 'drum1kit'
        self.regularity = 0.75
        self.beat = 12
        self.tempo = 120
        self.rythmPlayer = RythmPlayer()
        
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.rightBox = gtk.VBox()
        self.middleBox = gtk.VBox()
        self.mainWindowBox.add(self.leftBox)
        self.mainWindowBox.add(self.middleBox)
        self.mainWindowBox.add(self.rightBox)
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.drawInstrumentButtons()
        self.drawMicBox()
        self.drawLogo()
        self.drawGeneration()
        self.drawReverb()
        self.show_all()      
    
    def drawLogo(self):
        logo = gtk.Image()
        logo.set_from_file(self.IMAGE_ROOT + 'tamtam.png')
        self.middleBox.add(logo)
                
    def drawReverb( self ):     
        reverbSliderBox = gtk.HBox()
        reverbSliderBoxImgTop = gtk.Image()
        reverbSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'small.png')
        reverbSliderBoxImgBottom = gtk.Image()
        reverbSliderBoxImgBottom.set_from_file(self.IMAGE_ROOT + 'large.png')
        reverbAdjustment = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = gtk.HScale(adjustment = reverbAdjustment)
        reverbSlider.set_inverted(False)
        reverbSlider.set_draw_value(False)
        reverbAdjustment.connect("value_changed" , self.setReverb)
        reverbSliderBox.pack_start(reverbSliderBoxImgTop, False, padding=10)
        reverbSliderBox.pack_start(reverbSlider, True)
        reverbSliderBox.pack_start(reverbSliderBoxImgBottom, False, padding=10)
    
        self.leftBox.add(reverbSliderBox)
        
    def drawMicBox( self ):
        hbox = gtk.HBox()
        
        for n in range(1,5):
            vbox = gtk.VBox()
            
            micBtn = gtk.Button()
            micRecBtn = gtk.Button()
            micBtnImg = gtk.Image()
            micBtnImg.set_from_file(self.IMAGE_ROOT + 'mic' + str(n) + '.png')
            micBtn.set_image(micBtnImg)
            
            micBtn.connect('clicked', self.handleWindowButtonsClick, 'mic' + str(n))
            micRecBtn.connect('clicked', self.handleMicButtonClick, n)
            
            vbox.add(micRecBtn)
            vbox.add(micBtn)
            hbox.add(vbox)
        self.leftBox.add(hbox)
            
        
    def drawGeneration( self ):
        vbox = gtk.VBox()
        hboxTop = gtk.HBox()
        
        geneButtonBox = gtk.HBox()
               
        self.playImg = gtk.Image()
        self.playImg.set_from_file(self.IMAGE_ROOT + 'stop.png')
        playButton = gtk.ToggleButton(label=None)
        playButton.set_image(self.playImg)
        playButton.connect('toggled' , self.handlePlayButton)
        
        for n in range(1,4):
            generationDrumImg = gtk.Image()
            generationDrumImg.set_from_file(self.IMAGE_ROOT + 'drum' + str(n) + 'kitsmall.png')
            generationDrumBtn = gtk.Button(label=None)
            generationDrumBtn.set_image(generationDrumImg)
            generationDrumBtn.connect('clicked' , self.handleGenerationDrumBtn , 'drum'+ str(n) + 'kit')
            geneButtonBox.pack_start(generationDrumBtn)
        geneButtonBox.add(playButton)
            
        geneSliderBox = gtk.VBox()
        geneSliderBoxImgTop = gtk.Image()
        geneSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'simple.png')
        geneSliderBoxImgBottom = gtk.Image()
        geneSliderBoxImgBottom.set_from_file(self.IMAGE_ROOT + 'complex.png')
        geneAdjustment = gtk.Adjustment(value=0.75, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = gtk.VScale(adjustment = geneAdjustment)
        geneSlider.set_inverted(True)
        geneSlider.set_draw_value(False)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        geneSliderBox.pack_start(geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(geneSlider, True, 20)
        geneSliderBox.pack_start(geneSliderBoxImgBottom, False, padding=10)
        
        beatSliderBox = gtk.VBox()
        beatSliderBoxImgTop = gtk.Image()
        beatSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'simple.png')
        beatSliderBoxImgBottom = gtk.Image()
        beatSliderBoxImgBottom.set_from_file(self.IMAGE_ROOT + 'complex.png')
        beatAdjustment = gtk.Adjustment(value=12, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        beatSlider = gtk.VScale(adjustment = beatAdjustment)
        beatSlider.set_inverted(True)
        beatSlider.set_draw_value(False)
        beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        beatSliderBox.pack_start(beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(beatSlider, True, 20)
        beatSliderBox.pack_start(beatSliderBoxImgBottom, False, padding=10)
        
        tempoSliderBox = gtk.VBox()
        tempoSliderBoxImgTop = gtk.Image()
        tempoSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'fast.png')
        tempoSliderBoxImgBottom = gtk.Image()
        tempoSliderBoxImgBottom.set_from_file(self.IMAGE_ROOT + 'slow.png')
        tempoAdjustment = gtk.Adjustment(value=120, lower=40, upper=240, step_incr=1, page_incr=0, page_size=0)
        tempoSlider = gtk.VScale(adjustment = tempoAdjustment)
        tempoSlider.set_inverted(True)
        tempoSlider.set_draw_value(False)
        tempoAdjustment.connect("value_changed" , self.setTempo)
        tempoSliderBox.pack_start(tempoSliderBoxImgTop, False, padding=10)
        tempoSliderBox.pack_start(tempoSlider, True)
        tempoSliderBox.pack_start(tempoSliderBoxImgBottom, False, padding=10)
        
        hboxTop.pack_start(geneSliderBox)
        hboxTop.pack_start(beatSliderBox)
        hboxTop.pack_start(tempoSliderBox)
        vbox.pack_start(hboxTop, True)
        vbox.pack_start(geneButtonBox, False)

        self.rightBox.add(vbox)
        
        
    def drawInstrumentButtons(self):
        ROW_LEN = 8
                   
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
                instImage.set_from_file(self.IMAGE_ROOT + instrument + '.png')
                instButton.set_image(instImage)
                #instButton.set_relief(gtk.RELIEF_NONE)
                instButton.connect('clicked' , self.handleWindowButtonsClick , instrument)
                instButton.connect('enter' , self.handleWindowButtonsEnter , instrument)
                hBox.add(instButton)
            vBox.add(hBox)
        self.leftBox.add(vBox)
   
    def handleWindowButtonsClick(self , widget , instrument):
        self.setInstrument(instrument)
        self.playInstrumentNote(instrument)         
        
    def handleWindowButtonsEnter(self , widget , instrument):
        pass

    def handleMicButtonClick(self , widget , data):
        if data == 1:
            CSoundClient.micRecording(7)
            print 'bob'
        elif data == 2:
            CSoundClient.micRecording(8)
        elif data == 3:
            CSoundClient.micRecording(9)
        elif data == 4:
            CSoundClient.micRecording(10)
        else:
            return
            
    def handlePlayButton(self, widget, data=None):
          if widget.get_active():
              self.rythmPlayer.stopPlayback()
              self.playImg.set_from_file(self.IMAGE_ROOT + 'stop.png')
          else:
              self.rythmPlayer.stopPlayback()
              self.playImg.set_from_file(self.IMAGE_ROOT + 'stop.png')

    def handleGenerationSlider(self, adj):
        self.regularity = adj.value
        
    def handleBeatSlider(self, adj):
        self.beat = int(adj.value)        
    
    def handleGenerationDrumBtn(self , widget , data):
       self.rythmPlayer.beat = self.beat
       self.rythmPlayer.notesList = generator( data, self.beat, self.regularity, self.reverbSend)
       self.rythmPlayer.stopPlayback()
       self.rythmPlayer.startPlayback()
    
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone()
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
        
    def setReverb(self,adj):
        self.reverbSend = adj.value
        self.keyboardStandAlone.setReverb(self.reverbSend)
    
    def setTempo(self,adj):
        self.rythmPlayer.setTempo(adj.value)
        
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
        return cleanInstrumentList
    
    def destroy( self, widget ):
        gtk.main_quit()

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()
