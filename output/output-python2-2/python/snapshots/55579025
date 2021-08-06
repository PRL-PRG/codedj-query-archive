import pygtk
pygtk.require( '2.0' )
import gtk
import os

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Player.KeyboardStandAlone import KeyboardStandAlone
from Player.NoteStdAlone import NoteStdAlone
from Player.RythmPlayer import RythmPlayer
from Player.RythmGenerator import *
from GUI.Credits import Credits

class StandAlonePlayer( gtk.EventBox ):
    
    def __init__(self, client):
        self.csnd = client
        gtk.EventBox.__init__( self)
        self.set_property("border_width", 30)
                
        self.IMAGE_ROOT = Constants.TAM_TAM_ROOT + '/Resources/Images/'
        self.INST_ICON_SIZE = 100

        self.instrument = self.getInstrumentList()[0]
        self.reverbSend = 0.
        self.rythmInstrument = 'drum1kit'
        self.regularity = 0.75
        self.beat = 12
        self.tempo = 120
        self.rythmPlayer = RythmPlayer()
        
        self.creditsOpen = False
        
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
        eventbox = gtk.EventBox()
        eventbox.connect('button-press-event', self.handleLogoPress)
        logo = gtk.Image()
        logo.set_from_file(self.IMAGE_ROOT + 'tamtam_rouge.png')
        eventbox.add(logo)
        self.middleBox.add(eventbox)
    
    def handleLogoPress(self, widget, event):
        pos = widget.window.get_origin()
        if self.creditsOpen is False:
            credits = Credits(self.handleCreditsClose , pos)
        self.handleCreditsClose(True)
        
    def handleCreditsClose(self , state):
        self.creditsOpen = state
                
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
            micRecBtnImg = gtk.Image()
            micRecBtnImg.set_from_file(self.IMAGE_ROOT + 'recsmall_rouge.png')
            micRecBtn.set_image(micRecBtnImg)
            
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
        playButton = gtk.Button(label=None)
        playButton.set_image(self.playImg)
        playButton.connect('clicked' , self.handlePlayButton)
        
        for n in range(1,4):
            generationDrumImg = gtk.Image()
            generationDrumImg.set_from_file(self.IMAGE_ROOT + 'drum' + str(n) + 'kit.png')
            generationDrumBtn = gtk.Button(label=None)
            generationDrumBtn.set_image(generationDrumImg)
            generationDrumBtn.connect('clicked' , self.handleGenerationDrumBtn , 'drum'+ str(n) + 'kit')
            geneButtonBox.pack_start(generationDrumBtn)
        geneButtonBox.add(playButton)
            
        geneSliderBox = gtk.VBox()
        self.geneSliderBoxImgTop = gtk.Image()
        self.geneSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'complex6F.png')
        geneAdjustment = gtk.Adjustment(value=0.75, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = gtk.VScale(adjustment = geneAdjustment)
        geneSlider.set_inverted(True)
        geneSlider.set_draw_value(False)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        geneSliderBox.pack_start(self.geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(geneSlider, True, 20)
        
        beatSliderBox = gtk.VBox()
        self.beatSliderBoxImgTop = gtk.Image()
        self.beatSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'beat11F.png')
        beatAdjustment = gtk.Adjustment(value=12, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        beatSlider = gtk.VScale(adjustment = beatAdjustment)
        beatSlider.set_inverted(True)
        beatSlider.set_draw_value(False)
        beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        beatSliderBox.pack_start(self.beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(beatSlider, True, 20)
        
        tempoSliderBox = gtk.VBox()
        self.tempoSliderBoxImgTop = gtk.Image()
        self.tempoSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'tempo4F.png')
        tempoAdjustment = gtk.Adjustment(value=120, lower=40, upper=240, step_incr=1, page_incr=1, page_size=1)
        tempoSlider = gtk.VScale(adjustment = tempoAdjustment)
        tempoSlider.set_inverted(True)
        tempoSlider.set_draw_value(False)
        tempoAdjustment.connect("value_changed" , self.setTempo)
        tempoSliderBox.pack_start(self.tempoSliderBoxImgTop, False, padding=10)
        tempoSliderBox.pack_start(tempoSlider, True)
        
        hboxTop.pack_start(geneSliderBox)
        hboxTop.pack_start(beatSliderBox)
        hboxTop.pack_start(tempoSliderBox)
        vbox.pack_start(hboxTop, True, padding=15)
        vbox.pack_start(geneButtonBox, False)

        self.rightBox.add(vbox)
        
        
    def drawInstrumentButtons(self):
        ROW_LEN = 6
                   
        vBox = gtk.VBox()
        
        intrumentNum = len(self.getInstrumentList())
        rows = ( intrumentNum // ROW_LEN )
        if intrumentNum % ROW_LEN is not 0:    #S'il y a un reste
            rows = rows + 1
                    
        for row in range(rows):
            hBox = gtk.HBox()
            for instrument in self.getInstrumentList()[row*ROW_LEN:(row+1)*ROW_LEN]:
                instImage = gtk.Image()
                instButton = gtk.Button(label=None)
                instButton.set_size_request(self.INST_ICON_SIZE,self.INST_ICON_SIZE)
                instImage.set_from_file(self.IMAGE_ROOT + instrument + '.png')
                instButton.add(instImage)
                instButton.set_image(instImage)
                #instButton.set_relief(gtk.RELIEF_NONE)
                instButton.connect('clicked' , self.handleWindowButtonsClick , instrument)
                #instButton.connect('enter' , self.handleWindowButtonsEnter , instrument)
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
            self.csnd.micRecording(7)
            self.setInstrument('mic1')
        elif data == 2:
            self.csnd.micRecording(8)
            self.setInstrument('mic2')
        elif data == 3:
            self.csnd.micRecording(9)
            self.setInstrument('mic3')
        elif data == 4:
            self.csnd.micRecording(10)
            self.setInstrument('mic4')
        else:
            return
            
    def handlePlayButton(self, widget, data=None):
          self.rythmPlayer.stopPlayback()

    def handleGenerationSlider(self, adj):
        self.regularity = adj.value
        img = int(adj.value * 7)+1
        self.geneSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'complex' + str(img) + 'F.png')
        
    def handleBeatSlider(self, adj):
        self.beat = int(adj.value)
        img = int(adj.value)-1  
        self.beatSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'beat' + str(img) + 'F.png')
        
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
        self.rythmPlayer.setTempo(int(adj.value))
        img = int((adj.value - 40) /26.)+1
        self.tempoSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'tempo' + str(img) + 'F.png')
        
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
        cleanInstrumentList = filter( lambda x: (x[0:4] != 'drum') and (x[0:3] != 'mic'), CSoundConstants.INSTRUMENTS.keys())
        cleanInstrumentList.sort(lambda g,l: cmp(CSoundConstants.INSTRUMENTS[g].category, CSoundConstants.INSTRUMENTS[l].category) )
        return cleanInstrumentList + ['drum1kit', 'drum2kit', 'drum3kit']
    
    def destroy( self, widget ):
        gtk.main_quit()

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()
