import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import os
import random
import time
from types import *

import Config

from Util.ThemeWidgets import *
from Util.CSoundNote import CSoundNote
from Util.NoteDB import Note
from Util.CSoundClient import new_csound_client

from KeyboardStandAlone import KeyboardStandAlone
from RythmPlayer import RythmPlayer
from RythmGenerator import *
from SynthLab.SynthLabWindow import SynthLabWindow
from Util.Trackpad import Trackpad
from Util.InstrumentPanel import InstrumentPanel

Tooltips = Config.Tooltips

class miniTamTamMain( gtk.EventBox ):
    
    def __init__(self):
        gtk.EventBox.__init__( self)
        self.set_border_width(Config.MAIN_WINDOW_PADDING)
        
        self.csnd = new_csound_client()

        self.instrument = self.getInstrumentList()[0]
        self.timeout_ms = 50
        self.reverb = 0.
        self.volume = 80
        self.regularity = 0.75
        self.beat = 4
        self.tempo = Config.PLAYER_TEMPO
        self.rythmInstrument = 'drum1kit'
        self.rythmPlayer = RythmPlayer(self.recordStateButton)
        self.regenerate()
        self.csnd.loopSetTempo(self.tempo)
        self.notesList = []
        time.sleep(0.001)
        self.playbackTimeout = None
        self.trackpad = Trackpad( self, self.csnd )

        loopPointsTable = []        
        sample_names = [name for i in range( len( Config.INSTRUMENTS ) ) for name in Config.INSTRUMENTS.keys() if Config.INSTRUMENTS[ name ].instrumentId == i ] 
        for inst in sample_names:
            loopStart = Config.INSTRUMENTS[ inst ].loopStart
            loopEnd = Config.INSTRUMENTS[ inst ].loopEnd
            crossDur = Config.INSTRUMENTS[ inst ].crossDur
            loopPointsTable.extend( [ loopStart, loopEnd, crossDur ] )
        mess = "f5755 0 512 -2 " + " "  .join([str(n) for n in loopPointsTable])
        self.csnd.inputMessage( mess )

        self.csnd.setMasterVolume(self.volume)
        self.rythmPlayer.beat = self.beat
        
        self.tooltips = gtk.Tooltips()
        
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.rightBox = gtk.VBox()
        self.mainWindowBox.add(self.leftBox)
        self.mainWindowBox.add(self.rightBox)
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.connect('key-press-event',self.handleKeyboard)
        self.setInstrument(self.instrument)
        
        self.drawInstrumentButtons()
        self.drawSliders()
        #self.drawLogo()
        self.drawGeneration()
        self.show_all()
        self.playStartupSound()

        self.synthLabWindow = None
                
    def drawSliders( self ):     
        mainSliderBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        mainSliderBox.set_border_width(Config.PANEL_SPACING)
        
        reverbSliderBox = gtk.HBox()
        self.reverbSliderBoxImgTop = gtk.Image()
        self.reverbSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'reverb0.png')
        reverbAdjustment = gtk.Adjustment(value=self.reverb, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = ImageHScale( Config.IMAGE_ROOT + "sliderbutred.png", reverbAdjustment, 7 )
        reverbSlider.set_inverted(False)
        reverbSlider.set_size_request(350,15)
        reverbAdjustment.connect("value_changed" , self.handleReverbSlider)
        reverbSliderBox.pack_start(reverbSlider, True, 20)
        reverbSliderBox.pack_start(self.reverbSliderBoxImgTop, False, padding=0)
        self.tooltips.set_tip(reverbSlider,Tooltips.REV)

        volumeSliderBox = gtk.HBox()
        self.volumeSliderBoxImgTop = gtk.Image()
        self.volumeSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'volume2.png')
        volumeAdjustment = gtk.Adjustment(value=self.volume, lower=0, upper=100, step_incr=1, page_incr=0, page_size=0)
        volumeSlider = ImageHScale( Config.IMAGE_ROOT + "sliderbutviolet.png", volumeAdjustment, 7 )
        volumeSlider.set_inverted(False)
        volumeSlider.set_size_request(350,15)
        volumeAdjustment.connect("value_changed" , self.handleVolumeSlider)
        volumeSliderBox.pack_start(volumeSlider, True, 20)
        volumeSliderBox.pack_start(self.volumeSliderBoxImgTop, False, padding=0)
        self.tooltips.set_tip(volumeSlider,Tooltips.VOL)
    
        mainSliderBox.pack_start(volumeSliderBox, True, True, 5)
        mainSliderBox.pack_start(reverbSliderBox, True, True, 5)
        
        self.leftBox.pack_start(mainSliderBox,False,False)        
        
    def drawGeneration( self ):

        slidersBox = RoundVBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        slidersBox.set_border_width(Config.PANEL_SPACING)
        geneButtonBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        geneButtonBox.set_border_width(Config.PANEL_SPACING)
        transportBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        transportBox.set_border_width(Config.PANEL_SPACING)
            
        geneSliderBox = gtk.VBox()
        self.geneSliderBoxImgTop = gtk.Image()
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex6.png')
        geneAdjustment = gtk.Adjustment(value=self.regularity, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutbleu.png", geneAdjustment, 5 )
        geneSlider.set_inverted(False)
        geneSlider.set_size_request(15,408)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        geneSlider.connect("button-release-event", self.handleGenerationSliderRelease)
        geneSliderBox.pack_start(self.geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(geneSlider, True, 20)
        self.tooltips.set_tip(geneSlider,Tooltips.COMPL)
                        
        beatSliderBox = gtk.VBox()
        self.beatSliderBoxImgTop = gtk.Image()
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat3.png')
        beatAdjustment = gtk.Adjustment(value=self.beat, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        beatSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutjaune.png", beatAdjustment, 5, snap = 1 )
        beatSlider.set_inverted(True)
        beatSlider.set_size_request(15,408)
        beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        beatSlider.connect("button-release-event", self.handleBeatSliderRelease)
        beatSliderBox.pack_start(self.beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(beatSlider, True, 20)
        self.tooltips.set_tip(beatSlider,Tooltips.BEAT)
                        
        tempoSliderBox = gtk.VBox()
        self.tempoSliderBoxImgTop = gtk.Image()
        self.tempoSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'tempo5.png')
        tempoAdjustment = gtk.Adjustment(value=self.tempo, lower=Config.PLAYER_TEMPO_LOWER, upper=Config.PLAYER_TEMPO_UPPER, step_incr=1, page_incr=1, page_size=1)
        tempoSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutvert.png", tempoAdjustment, 5)
        tempoSlider.set_inverted(True)
        tempoSlider.set_size_request(15,408)
        tempoAdjustment.connect("value_changed" , self.handleTempoSliderChange)
        tempoSlider.connect("button-release-event", self.handleTempoSliderRelease)
        tempoSliderBox.pack_start(self.tempoSliderBoxImgTop, False, padding=10)
        tempoSliderBox.pack_start(tempoSlider, True)
        self.tooltips.set_tip(tempoSlider,Tooltips.TEMPO)
        
        slidersBoxSub = gtk.HBox()        
        slidersBoxSub.pack_start(geneSliderBox)
        slidersBoxSub.pack_start(beatSliderBox)
        slidersBoxSub.pack_start(tempoSliderBox)
        slidersBox.pack_start(slidersBoxSub)
        
        generateBtn = ImageButton(Config.IMAGE_ROOT + 'dice.png', clickImg_path = Config.IMAGE_ROOT + 'diceblur.png')
        generateBtn.connect('clicked', self.handleGenerateBtn)
        slidersBox.pack_start(generateBtn)
        self.tooltips.set_tip(generateBtn,Tooltips.GEN)
        
        #Generation Button Box    
        geneSubBox = gtk.VBox()
        geneSubBoxTop = gtk.HBox()
        
        generationDrumBtn1 = ImageRadioButton(group = None , mainImg_path = Config.IMAGE_ROOT + 'drum1kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum1kitselgen.png')
        generationDrumBtn1.connect('clicked' , self.handleGenerationDrumBtn , 'drum1kit')
        geneSubBoxTop.pack_start(generationDrumBtn1)
        generationDrumBtn2 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum2kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum2kitselgen.png')
        generationDrumBtn2.connect('clicked' , self.handleGenerationDrumBtn , 'drum2kit')
        geneSubBoxTop.pack_start(generationDrumBtn2)
        generationDrumBtn3 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum3kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum3kitselgen.png')
        generationDrumBtn3.connect('clicked' , self.handleGenerationDrumBtn , 'drum3kit')
        geneSubBox.pack_start(geneSubBoxTop, True)
        geneSubBox.pack_start(generationDrumBtn3, True)
        geneButtonBox.pack_start(geneSubBox, True)
        self.tooltips.set_tip(generationDrumBtn1,Tooltips.JAZZ)
        self.tooltips.set_tip(generationDrumBtn2,Tooltips.ARAB)
        self.tooltips.set_tip(generationDrumBtn3,Tooltips.AFRI)
        
        #Transport Button Box
        self.seqRecordButton = ImageToggleButton(Config.IMAGE_ROOT + 'record2.png', Config.IMAGE_ROOT + 'record2sel.png')
        self.seqRecordButton.connect('clicked', self.rythmPlayer.handleRecordButton )

        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'play.png', Config.IMAGE_ROOT + 'stop.png')
        self.playStopButton.connect('clicked' , self.handlePlayButton)
        transportBox.pack_start(self.seqRecordButton)
        transportBox.pack_start(self.playStopButton)
        self.tooltips.set_tip(self.seqRecordButton,Tooltips.SEQ)
        self.tooltips.set_tip(self.playStopButton,Tooltips.PLAY)
        
        self.rightBox.pack_start(slidersBox, True)
        self.rightBox.pack_start(geneButtonBox, True)
        self.rightBox.pack_start(transportBox, True)
 
    def drawInstrumentButtons(self):
        self.instPanel = InstrumentPanel(self.setInstrument,self.playInstrumentNote, False, self.micRec, self.synthRec)
        self.leftBox.pack_start(self.instPanel,True,True)
    
    def micRec(self,mic):
        os.system('rm ' + Config.PREF_DIR + '/' + mic)
        if mic == 'mic1':
            self.csnd.micRecording(7)
        elif mic == 'mic2':
            self.csnd.micRecording(8)
        elif mic == 'mic3':
            self.csnd.micRecording(9)
        elif mic == 'mic4':
            self.csnd.micRecording(10)
        else:
            return  
        self.micTimeout = gobject.timeout_add(5000, self.loadMicInstrument, mic)
        
    def synthRec(self,lab):
        if self.synthLabWindow != None:
            self.synthLabWindow.destroy()
            self.synthLabWindow =None

        self.synthLabWindow = SynthLabWindow( 
                {'lab1':86, 'lab2':87, 'lab3':88, 'lab4':89}[lab],
                self.closeSynthLab)
        self.synthLabWindow.show_all()

    def recordStateButton( self, state ):
        self.seqRecordButton.set_active( state )       
        
    def synthLabWindowOpen(self):
        return self.synthLabWindow != None  and self.synthLabWindow.get_property('visible')

    def loadMicInstrument( self, data ):
        self.csnd.load_mic_instrument( data )

    def closeSynthLab(self):
        if self.synthLabWindow != None:
            self.synthLabWindow.destroy()
            self.synthLabWindow = None

    def regenerate(self):
        def flatten(ll):
            rval = []
            for l in ll:
                rval += l
            return rval
        i = 0
        self.noteList= []
        self.csnd.loopClear()
        for x in flatten( generator(self.rythmInstrument, self.beat, self.regularity, self.reverb) ):
            n = Note(0, x.trackId, i, x)
            self.noteList.append( (x.onset, n) )
            i = i + 1
            self.csnd.loopPlay(n)
        self.csnd.loopSetNumTicks( self.beat * Config.TICKS_PER_BEAT)
               
    def handleGenerationSlider(self, adj):
        img = int(adj.value * 7)+1
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex' + str(img) + '.png')

    def handleGenerationSliderRelease(self, widget, event):
        self.regularity = widget.get_adjustment().value
        self.regenerate()

    def handleBeatSlider(self, adj):
        img = self.scale(int(adj.value),2,12,1,11)
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat' + str(img) + '.png')
        
    def handleBeatSliderRelease(self, widget, event):
        self.beat = int(widget.get_adjustment().value)
        self.rythmPlayer.beat = self.beat
        self.regenerate()

    def handleTempoSliderRelease(self, widget, event):
        #self.tempo = int(widget.get_adjustment().value)
        #self.csnd.loopSetTempo(self.tempo)
        self.rythmPlayer.tempo = widget.get_adjustment().value
        pass

    def handleTempoSliderChange(self,adj):
        self.tempo = int(adj.value)
        self.csnd.loopSetTempo(self.tempo)

        img = int(self.scale( self.tempo,
            Config.PLAYER_TEMPO_LOWER,Config.PLAYER_TEMPO_UPPER,
            1,8))
        self.tempoSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'tempo' + str(img) + '.png')

    def handleVolumeSlider(self, adj):
        self.volume = int(adj.value)
        self.csnd.setMasterVolume(self.volume)
        img = int(self.scale(self.volume,0,100,0,3.9))
        self.volumeSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'volume' + str(img) + '.png')
        
    def handleReverbSlider(self, adj):
        self.reverb = adj.value
        img = int(self.scale(self.reverb,0,1,0,4))
        self.reverbSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'reverb' + str(img) + '.png')
        self.keyboardStandAlone.setReverb(self.reverb)
        
    def handlePlayButton(self, widget, data = None):
        if widget.get_active() == False:
            self.rythmPlayer.stopPlayback()
            self.playbackTimeout = None
            self.csnd.loopStop()
        else:
            self.csnd.loopSetTick(0)
            self.csnd.loopStart()

    def handleGenerationDrumBtn(self , widget , data):
        #data is drum1kit, drum2kit, or drum3kit
        self.rythmInstrument = data
        for (o,n) in self.notesList :
            n.instrumentFlag = data
        self.csnd.loopSet_onset_note( self.notesList)
        
    def handleGenerateBtn(self , widget , data=None):
        self.regenerate()
        if self.playbackTimeout == None :
            self.playStopButton.set_active(True)  #this calls handlePlayButton
            self.playStartupSound()

    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone( self.rythmPlayer.recording, self.rythmPlayer.adjustDuration, self.csnd.loopGetTick, self.rythmPlayer.getPlayState ) 
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
    
    def playInstrumentNote(self , instrument, secs_per_tick = 0.025):
        self.csnd.play( 
                    CSoundNote( onset = 0, 
                             pitch = 36, 
                             amplitude = 1, 
                             pan = 0.5, 
                             duration = 20, 
                             trackId = 1, 
                             fullDuration = False, 
                             instrument = instrument, 
                             instrumentFlag = instrument,
                             reverbSend = 0),
                    secs_per_tick)
        
    def handleKeyboard(self, widget, event):
        if event.hardware_keycode == 65:
            if self.playStopButton.get_active():
                self.playStopButton.set_active(False)
            else:
                self.playStopButton.set_active(True)
    
    def playStartupSound(self):
        r = str(random.randrange(1,11))
        self.playInstrumentNote('guidice' + r)

    def getInstrumentList(self):
        cleanInstrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab' and instrument[0:4] != 'guid']
        cleanInstrumentList.sort(lambda g,l: cmp(Config.INSTRUMENTS[g].category, Config.INSTRUMENTS[l].category) )
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
    MiniTamTam = miniTamTam()
    #start the gtk event loop
    gtk.main()
