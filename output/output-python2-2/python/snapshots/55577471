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
from GUI.Core.ThemeWidgets import *
from GUI.Credits import Credits

class StandAlonePlayer( gtk.EventBox ):
    
    def __init__(self, client):
        gtk.EventBox.__init__( self)
        self.set_border_width(5)
        
        self.csnd = client
                
        self.IMAGE_ROOT = Constants.TAM_TAM_ROOT + '/Resources/Images/'

        self.INST_ICON_SIZE = 112
        self.INST_BOX_COLOR = '#ACB9A5'
        self.BOX_BCK_COLOR =  '#FFFFFF'
        self.BOX_COLOR = '#8F9588'
        self.BOX_SPACING = 2

        self.instrument = self.getInstrumentList()[0]
        self.reverb = 0.
        self.volume = 80
        self.regularity = 0.75
        self.beat = 4
        self.tempo = 120
        self.rythmPlayer = RythmPlayer()
        self.rythmInstrument = 'drum1kit'
        
        self.creditsOpen = False
        
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.rightBox = gtk.VBox()
        self.mainWindowBox.add(self.leftBox)
        self.mainWindowBox.add(self.rightBox)
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        
        #self.drawInstrumentButtons()
        self.drawMicBox()
        self.drawSliders()
        #self.drawLogo()
        self.drawGeneration()
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
                
    def drawSliders( self ):     
        mainSliderBox = RoundHBox(fillcolor = self.BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
        mainSliderBox.set_border_width(self.BOX_SPACING)
        
        reverbSliderBox = gtk.HBox()
        self.reverbSliderBoxImgTop = gtk.Image()
        self.reverbSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'reverb0.png')
        reverbAdjustment = gtk.Adjustment(value=self.reverb, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = ImageHScale( self.IMAGE_ROOT + "sliderbutred.png", reverbAdjustment, 7 )
        reverbSlider.set_inverted(False)
        reverbSlider.set_size_request(350,15)
        reverbAdjustment.connect("value_changed" , self.handleReverbSlider)
        reverbSliderBox.pack_start(reverbSlider, True, 20)
        reverbSliderBox.pack_start(self.reverbSliderBoxImgTop, False, padding=0)

        volumeSliderBox = gtk.HBox()
        self.volumeSliderBoxImgTop = gtk.Image()
        self.volumeSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'volume2.png')
        volumeAdjustment = gtk.Adjustment(value=self.volume, lower=0, upper=100, step_incr=1, page_incr=0, page_size=0)
        volumeSlider = ImageHScale( self.IMAGE_ROOT + "sliderbutviolet.png", volumeAdjustment, 7 )
        volumeSlider.set_inverted(False)
        volumeSlider.set_size_request(350,15)
        volumeAdjustment.connect("value_changed" , self.handleVolumeSlider)
        volumeSliderBox.pack_start(volumeSlider, True, 20)
        volumeSliderBox.pack_start(self.volumeSliderBoxImgTop, False, padding=0)
    
        mainSliderBox.add(volumeSliderBox)
        mainSliderBox.add(reverbSliderBox)
        
        self.leftBox.add(mainSliderBox)        
        
    def drawGeneration( self ):

        slidersBox = RoundVBox(fillcolor = self.BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
        slidersBox.set_border_width(self.BOX_SPACING)
        geneButtonBox = RoundHBox(fillcolor = self.BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
        geneButtonBox.set_border_width(self.BOX_SPACING)
        transportBox = RoundHBox(fillcolor = self.BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
        transportBox.set_border_width(self.BOX_SPACING)
            
        geneSliderBox = gtk.VBox()
        self.geneSliderBoxImgTop = gtk.Image()
        self.geneSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'complex6.png')
        geneAdjustment = gtk.Adjustment(value=self.regularity, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = ImageVScale( self.IMAGE_ROOT + "sliderbutbleu.png", geneAdjustment, 5 )
        geneSlider.set_inverted(False)
        geneSlider.set_size_request(15,366)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        geneSliderBox.pack_start(self.geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(geneSlider, True, 20)
                        
        beatSliderBox = gtk.VBox()
        self.beatSliderBoxImgTop = gtk.Image()
        self.beatSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'beat3.png')
        beatAdjustment = gtk.Adjustment(value=self.beat, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        beatSlider = ImageVScale( self.IMAGE_ROOT + "sliderbutjaune.png", beatAdjustment, 5 )
        beatSlider.set_inverted(True)
        beatSlider.set_size_request(15,366)
        beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        beatSliderBox.pack_start(self.beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(beatSlider, True, 20)
                        
        tempoSliderBox = gtk.VBox()
        self.tempoSliderBoxImgTop = gtk.Image()
        self.tempoSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'tempo4.png')
        tempoAdjustment = gtk.Adjustment(value=self.tempo, lower=40, upper=240, step_incr=1, page_incr=1, page_size=1)
        tempoSlider = ImageVScale( self.IMAGE_ROOT + "sliderbutvert.png", tempoAdjustment, 5)
        tempoSlider.set_inverted(True)
        tempoSlider.set_size_request(15,366)
        tempoAdjustment.connect("value_changed" , self.setTempo)
        tempoSliderBox.pack_start(self.tempoSliderBoxImgTop, False, padding=10)
        tempoSliderBox.pack_start(tempoSlider, True)
        
        slidersBoxSub = gtk.HBox()        
        slidersBoxSub.pack_start(geneSliderBox)
        slidersBoxSub.pack_start(beatSliderBox)
        slidersBoxSub.pack_start(tempoSliderBox)
        slidersBox.pack_start(slidersBoxSub)
        
        generateBtn = ImageButton(self.IMAGE_ROOT + 'dice.png', click_image_path = self.IMAGE_ROOT + 'diceblur.png')
        slidersBox.pack_start(generateBtn)
        
        #Generation Button Box    
        geneSubBox = gtk.VBox()
        geneSubBoxTop = gtk.HBox()
        
        generationDrumBtn1 = ImageRadioButton(group = None , mainImg_path = self.IMAGE_ROOT + 'drum1kit.png' , altImg_path = self.IMAGE_ROOT + 'drum1kitsel.png')
        generationDrumBtn1.connect('clicked' , self.handleGenerationDrumBtn , 'drum1kit')
        geneSubBoxTop.pack_start(generationDrumBtn1)
        generationDrumBtn2 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = self.IMAGE_ROOT + 'drum2kit.png' , altImg_path = self.IMAGE_ROOT + 'drum2kitsel.png')
        generationDrumBtn2.connect('clicked' , self.handleGenerationDrumBtn , 'drum2kit')
        geneSubBoxTop.pack_start(generationDrumBtn2)
        generationDrumBtn3 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = self.IMAGE_ROOT + 'drum3kit.png' , altImg_path = self.IMAGE_ROOT + 'drum3kitsel.png')
        generationDrumBtn3.connect('clicked' , self.handleGenerationDrumBtn , 'drum3kit')
        geneSubBox.pack_start(geneSubBoxTop)
        geneSubBox.pack_start(generationDrumBtn3)
        geneButtonBox.pack_start(geneSubBox)
        
        #Transport Button Box
        playPauseButton = ImageToggleButton(self.IMAGE_ROOT + 'play.png', self.IMAGE_ROOT + 'pause.png')
        playPauseButton.connect('clicked' , self.handlePlayButton)
        stopButton = ImageButton(self.IMAGE_ROOT + 'stop.png')
        stopButton.connect('clicked' , self.handleStopButton)
        transportBox.pack_start(stopButton)
        transportBox.pack_start(playPauseButton)
        
        self.rightBox.pack_start(slidersBox, True)
        self.rightBox.pack_start(geneButtonBox, True)
        self.rightBox.pack_start(transportBox, True)
 
        
    def drawInstrumentButtons(self):
        ROW_LEN = 8
                   
        vBox = gtk.VBox()
        
        intrumentNum = len(self.getInstrumentList())
        rows = ( intrumentNum // ROW_LEN )
        if intrumentNum % ROW_LEN is not 0:    #S'il y a un reste
            rows = rows + 1
                    
        for row in range(rows):
            hBox = gtk.HBox()
            for instrument in self.getInstrumentList()[row*ROW_LEN:(row+1)*ROW_LEN]:
                instBox = RoundVBox(fillcolor = self.INST_BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
                instBox.set_border_width(self.BOX_SPACING)
                instButton = ImageButton(self.IMAGE_ROOT + instrument + '.png')
                #instButton.set_size_request(self.INST_ICON_SIZE,self.INST_ICON_SIZE)
                instButton.connect('clicked' , self.handleWindowButtonsClick , instrument)
                instBox.add(instButton)
                hBox.add(instBox)
            vBox.add(hBox)
        self.leftBox.add(vBox)
        
    def drawMicBox( self ):
        hbox = gtk.HBox()
        
        for n in range(1,5):
            vbox1 = RoundVBox(fillcolor = self.INST_BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
            vbox1.set_border_width(self.BOX_SPACING)
            
            micBtn = ImageButton(self.IMAGE_ROOT + 'mic' + str(n) + '.png')
            micRecBtn = ImageButton(self.IMAGE_ROOT + 'record.png')
            
            micBtn.connect('clicked', self.handleWindowButtonsClick, 'mic' + str(n))
            micRecBtn.connect('clicked', self.handleMicButtonClick, n)
            
            vbox1.add(micRecBtn)
            vbox1.add(micBtn)
            hbox.add(vbox1)
            
        for n in range(1,5):
            vbox2 = RoundVBox(fillcolor = self.INST_BOX_COLOR, bordercolor = self.BOX_BCK_COLOR)
            vbox2.set_border_width(self.BOX_SPACING)
            
            synthBtn = ImageButton(self.IMAGE_ROOT + 'lab' + str(n) + '.png')
            synthRecBtn = ImageButton(self.IMAGE_ROOT + 'record.png')
            
            synthBtn.connect('clicked', self.handleWindowButtonsClick, 'lab' + str(n))
            synthRecBtn.connect('clicked', self.handleSynthButtonClick, n)
            
            vbox2.add(synthRecBtn)
            vbox2.add(synthBtn)
            hbox.add(vbox2)
            
        self.leftBox.add(hbox)
   
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
    
    def handleSynthButtonClick(self , widget , data):
        pass
            
    def handlePlayButton(self, widget, data = None):
          self.rythmPlayer.stopPlayback()
          
    def handleStopButton(self, widget, data = None):
        pass

    def handleGenerationSlider(self, adj):
        self.regularity = adj.value
        img = int(adj.value * 7)+1
        self.geneSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'complex' + str(img) + '.png')
        
    def handleBeatSlider(self, adj):
        self.beat = int(adj.value)
        img = int(adj.value)-1  
        self.beatSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'beat' + str(img) + '.png')
        
    def handleVolumeSlider(self, adj):
        self.volume = int(adj.value)
        img = self.scale(self.volume,0,100,0,3)
        self.volumeSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'volume' + str(img) + '.png')
        
    def handleReverbSlider(self, adj):
        self.reverb = adj.value
        img = int(self.scale(self.reverb,0,1,0,4))
        self.reverbSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'reverb' + str(img) + '.png')
        self.keyboardStandAlone.setReverb(self.reverb)
        
    def handleGenerationDrumBtn(self , widget , data):
        self.rythmPlayer.beat = self.beat
        self.rythmPlayer.notesList = generator( data, self.beat, self.regularity, self.reverbSend, self.csnd)
        self.rythmPlayer.stopPlayback()
        self.rythmPlayer.startPlayback()
    
    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone( self.csnd )
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
    
    def setTempo(self,adj):
        self.rythmPlayer.setTempo(int(adj.value))
        img = int((adj.value - 40) /26.)+1
        self.tempoSliderBoxImgTop.set_from_file(self.IMAGE_ROOT + 'tempo' + str(img) + '.png')
        
    def playInstrumentNote(self , instrument):
        note = NoteStdAlone( client = self.csnd,
                             onset = 0, 
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
        cleanInstrumentList = [instrument for instrument in CSoundConstants.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab']
        cleanInstrumentList.sort(lambda g,l: cmp(CSoundConstants.INSTRUMENTS[g].category, CSoundConstants.INSTRUMENTS[l].category) )
        return cleanInstrumentList + ['drum1kit', 'drum2kit', 'drum3kit']
    
    def destroy( self, widget ):
        gtk.main_quit()
        
    def scale(self, input,input_min,input_max,output_min,output_max):
        range_input = input_max - input_min
        range_output = output_max - output_min
        result = (input - input_min) * range_output / range_input + output_min
    
        if (input_min > input_max and output_min > output_max) or (output_min > output_max and input_min < input_max):
            if result > output_min:
                return output_min
            elif result < output_max:
                return output_max
            else:
                return result
    
        if (input_min < input_max and output_min < output_max) or (output_min < output_max and input_min > input_max):
            if result > output_max:
                return output_max
            elif result < output_min:
                return output_min
            else:
                return result

if __name__ == "__main__": 
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()
