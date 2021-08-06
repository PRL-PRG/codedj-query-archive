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
from Util import NoteDB
from Util.NoteDB import Note
from Util.CSoundClient import new_csound_client

from Fillin import Fillin
from KeyboardStandAlone import KeyboardStandAlone
from MiniSequencer import MiniSequencer
from RythmGenerator import *
from SynthLab.SynthLabWindow import SynthLabWindow
from Util.Trackpad import Trackpad
from Util.InstrumentPanel import InstrumentPanel

Tooltips = Config.Tooltips

from SubActivity import SubActivity
    
class miniTamTamMain(SubActivity):
    
    def __init__(self, set_mode):
        SubActivity.__init__(self, set_mode)
        self.set_border_width(Config.MAIN_WINDOW_PADDING)

        self.csnd = new_csound_client()
        self.timeout_ms = 50
        self.volume = 80
        self.instrument = 'ocarina'
        self.regularity = 0.75
        self.beat = 4
        self.reverb = 0.
        self.tempo = Config.PLAYER_TEMPO
        self.rythmInstrument = 'drum1kit'
        self.drumFillin = Fillin( self.beat, self.tempo, self.rythmInstrument, self.reverb )
        self.regenerate()
        self.sequencer= MiniSequencer(self.recordStateButton)
        self.csnd.loopSetTempo(self.tempo)
        self.noteList = []
        time.sleep(0.001)
        self.playbackTimeout = None
        self.trackpad = Trackpad( self )
        for i in range(21):
            self.csnd.setTrackVolume( 100, i )

        self.csnd.setMasterVolume(self.volume)
        self.sequencer.beat = self.beat
        
        self.tooltips = gtk.Tooltips()
        
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.leftBox.set_size_request(950,-1)
        self.rightBox = gtk.VBox()
        self.mainWindowBox.pack_start(self.leftBox,False,False)
        self.mainWindowBox.pack_start(self.rightBox,True,True)
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.setInstrument(self.instrument)
        
        self.drawInstrumentButtons()
        self.drawSliders()
        self.drawGeneration()
        self.show_all()
        if 'a good idea' == True:
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
        
        self.leftBox.pack_start(mainSliderBox, False, False)        
        
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
        geneVBox = gtk.VBox()
        geneTopBox = gtk.HBox()
        geneLowBox = gtk.HBox()
        
        generationDrumBtn1 = ImageRadioButton(group = None , mainImg_path = Config.IMAGE_ROOT + 'drum1kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum1kitselgen.png')
        generationDrumBtn1.connect('clicked' , self.handleGenerationDrumBtn , 'drum1kit')
        geneTopBox.pack_start(generationDrumBtn1)
        generationDrumBtn2 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum2kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum2kitselgen.png')
        generationDrumBtn2.connect('clicked' , self.handleGenerationDrumBtn , 'drum2kit')
        geneTopBox.pack_start(generationDrumBtn2)
        generationDrumBtn3 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum3kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum3kitselgen.png')
        generationDrumBtn3.connect('clicked' , self.handleGenerationDrumBtn , 'drum3kit')
        generationDrumBtn4 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum4kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum4kitselgen.png')
        generationDrumBtn4.connect('clicked' , self.handleGenerationDrumBtn , 'drum4kit')
        geneLowBox.pack_start(generationDrumBtn3, True)
        geneLowBox.pack_start(generationDrumBtn4, True)
        geneVBox.pack_start(geneTopBox, True)
        geneVBox.pack_start(geneLowBox, True)
        geneButtonBox.pack_start(geneVBox,True)
        self.tooltips.set_tip(generationDrumBtn1,Tooltips.JAZZ)
        self.tooltips.set_tip(generationDrumBtn2,Tooltips.ARAB)
        self.tooltips.set_tip(generationDrumBtn3,Tooltips.AFRI)
        self.tooltips.set_tip(generationDrumBtn4,Tooltips.ELEC)
        
        #Transport Button Box
        self.seqRecordButton = ImageToggleButton(Config.IMAGE_ROOT + 'record2.png', Config.IMAGE_ROOT + 'record2sel.png')
        self.seqRecordButton.connect('button-press-event', self.sequencer.handleRecordButton )

        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        self.playStopButton.connect('clicked' , self.handlePlayButton)
        transportBox.pack_start(self.seqRecordButton)
        transportBox.pack_start(self.playStopButton)
        closeButton = ImageButton(Config.IMAGE_ROOT + 'close.png')
        closeButton.connect('pressed',self.handleClose)
        transportBox.pack_start(closeButton)
        self.tooltips.set_tip(self.seqRecordButton,Tooltips.SEQ)
        self.tooltips.set_tip(self.playStopButton,Tooltips.PLAY)
        
        self.rightBox.pack_start(slidersBox, True)
        self.rightBox.pack_start(geneButtonBox, True)
        self.rightBox.pack_start(transportBox, True)
 
    def drawInstrumentButtons(self):
        self.instrumentPanelBox = gtk.HBox()
        # InstrumentPanel(elf.setInstrument,self.playInstrumentNote, False, self.micRec, self.synthRec)
        self.leftBox.pack_start(self.instrumentPanelBox,True,True)
    
    def setInstrumentPanel( self, instrumentPanel ):
        instrumentPanel.configure( self.setInstrument,self.playInstrumentNote, False, self.micRec, self.synthRec )
        self.instrumentPanel = instrumentPanel
        self.instrumentPanelBox.pack_start( instrumentPanel )

    def releaseInstrumentPanel( self ):
        self.instrumentPanelBox.remove( self.instrumentPanel )

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
        noteOnsets = []
        notePitchs = []
        i = 0
        self.noteList= []
        self.csnd.loopClear()
        for x in flatten( generator(self.rythmInstrument, self.beat, 0.8, self.regularity, self.reverb) ):
            noteOnsets.append(x.onset)
            notePitchs.append(x.pitch)
            n = Note(0, x.trackId, i, x)
            self.noteList.append( (x.onset, n) )
            i = i + 1
            self.csnd.loopPlay(n,1)                    #add as active
        self.csnd.loopSetNumTicks( self.beat * Config.TICKS_PER_BEAT)
        self.drumFillin.unavailable( noteOnsets, notePitchs )

    def handleClose(self,widget):
        if self.playStopButton.get_active() == True:
            self.playStopButton.set_active(False)  
        self.sequencer.clearSequencer()
        self.csnd.loopClear()
        self.set_mode('welcome')
               
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
        self.sequencer.beat = self.beat
        self.drumFillin.setBeats( self.beat )
        self.regenerate()

    def handleTempoSliderRelease(self, widget, event):
        #self.tempo = int(widget.get_adjustment().value)
        #self.csnd.loopSetTempo(self.tempo)
        self.sequencer.tempo = widget.get_adjustment().value
        self.drumFillin.setTempo(self.tempo)
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
        self.drumFillin.setReverb( self.reverb )
        img = int(self.scale(self.reverb,0,1,0,4))
        self.reverbSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'reverb' + str(img) + '.png')
        self.keyboardStandAlone.setReverb(self.reverb)
        
    def handlePlayButton(self, widget, data = None):
        if widget.get_active() == False:
            self.drumFillin.stop()
            self.sequencer.stopPlayback()
            self.playbackTimeout = None
            self.csnd.loopPause()
        else:
            self.drumFillin.play()
            self.csnd.loopSetTick(0)
            self.csnd.loopStart()

    def handleGenerationDrumBtn(self , widget , data):
        #data is drum1kit, drum2kit, or drum3kit
        print 'HANDLE: Generate Button'
        self.rythmInstrument = data
        instrumentId = Config.INSTRUMENTS[data].instrumentId
        for (o,n) in self.noteList :
            self.csnd.loopUpdate(n, NoteDB.PARAMETER.INSTRUMENT, instrumentId, -1)
        self.drumFillin.setInstrument( self.rythmInstrument )
        
    def handleGenerateBtn(self , widget , data=None):
        self.regenerate()
        if self.playbackTimeout == None :
            self.playStopButton.set_active(True)  #this calls handlePlayButton
            self.playStartupSound()

    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone( self.sequencer.recording, self.sequencer.adjustDuration, self.csnd.loopGetTick, self.sequencer.getPlayState ) 
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
                             instrumentId = Config.INSTRUMENTS[instrument].instrumentId, 
                             reverbSend = 0,
                             tied = False,
                             mode = 'mini'),
                    secs_per_tick)
        
    def onKeyPress(self, widget, event):
        if event.hardware_keycode == 65: #what key is this? what feature is this?
            if self.playStopButton.get_active():
                self.playStopButton.set_active(False)
            else:
                self.playStopButton.set_active(True)
        self.keyboardStandAlone.onKeyPress(widget, event)
    def onKeyRelease(self, widget, event):
        self.keyboardStandAlone.onKeyRelease(widget, event)
    
    def playStartupSound(self):
        r = str(random.randrange(1,11))
        self.playInstrumentNote('guidice' + r)

    def getInstrumentList(self):
        cleanInstrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab' and instrument[0:4] != 'guid']
        cleanInstrumentList.sort(lambda g,l: cmp(Config.INSTRUMENTS[g].category, Config.INSTRUMENTS[l].category) )
        return cleanInstrumentList + ['drum1kit', 'drum2kit', 'drum3kit']
    
    def onActivate( self ):
        self.csnd.loopPause()
        self.csnd.loopClear()

    def onDeactivate( self ):
        SubActivity.onDeactivate( self )
        self.releaseInstrumentPanel()
        self.csnd.loopPause()
        self.csnd.loopClear()

    def onDestroy( self ):
        #this gets called when the whole app is being destroyed
        #QUESTION is this called before or after onDeactivate()
        pass
        
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
