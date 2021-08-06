import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import os
import random
import time

import Config

from Util.ThemeWidgets import *
from Util.Credits import Credits
from Util.NoteLooper import NoteLooper
from Util.CSoundNote import CSoundNote

from Player.KeyboardStandAlone import KeyboardStandAlone
from Player.RythmPlayer import RythmPlayer
from Player.RythmGenerator import *
from SynthLab.SynthLabWindow import SynthLabWindow

Tooltips = Config.Tooltips

import thread
import time

def testtiemr():
    m = 0.0
    while True:
        t0 = time.time()
        time.sleep(0.05)
        t1 = time.time()
        if t1 - t0 > 0.75 : 
            print 'critical lagginess: ', t1 - t0
        if m < t1 - t0:
            m = t1 - t0
            print t1, ' timer max = ', m

class StandAlonePlayer( gtk.EventBox ):
    
    def __init__(self, client):
        thread.start_new_thread( testtimer )
        gtk.EventBox.__init__( self)
        self.set_border_width(Config.MAIN_WINDOW_PADDING)
        
        self.csnd = client

        self.instrument = self.getInstrumentList()[0]
        self.timeout_ms = 50
        self.reverb = 0.
        self.volume = 80
        self.regularity = 0.75
        self.beat = 4
        self.tempo = Config.PLAYER_TEMPO
        self.rythmInstrument = 'drum1kit'
        self.tempo2tickrate = Config.TICKS_PER_BEAT / 60.0
        self.noteLooper = NoteLooper( Config.NOTELOOPER_HORIZON, self.tempo * self.tempo2tickrate )
        self.rythmPlayer = RythmPlayer(self.recordStateButton, self.noteLooper.getTick)
        self.notesList = []
        self.csnd.startTime()
        self.noteLooper.startTime()
        time.sleep(0.001)
        self.playbackTimeout = None
        
        self.synthLabWindow1 = SynthLabWindow(self.csnd, 86)
        self.synthLabWindow2 = SynthLabWindow(self.csnd, 87)
        self.synthLabWindow3 = SynthLabWindow(self.csnd, 88)
        self.synthLabWindow4 = SynthLabWindow(self.csnd, 89)

        self.csnd.setMasterVolume(self.volume)
        self.rythmPlayer.beat = self.beat
        
        self.tooltips = gtk.Tooltips()

        self.creditsOpen = False
        self.recstate = False
        
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.rightBox = gtk.VBox()
        self.mainWindowBox.add(self.leftBox)
        self.mainWindowBox.add(self.rightBox)
        self.add(self.mainWindowBox)
       
        self.enableKeyboard()
        self.setInstrument(self.instrument)
        
        self.drawInstrumentButtons()
        self.drawMicBox()
        self.drawSliders()
        #self.drawLogo()
        self.drawGeneration()
        self.show_all()
        self.playStartupSound()
    
    def drawLogo(self):
        eventbox = gtk.EventBox()
        eventbox.connect('button-press-event', self.handleLogoPress)
        logo = gtk.Image()
        logo.set_from_file(Config.IMAGE_ROOT + 'tamtam_rouge.png')
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
        mainSliderBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        mainSliderBox.set_border_width(Config.BORDER_SIZE)
        
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
        
        self.leftBox.add(mainSliderBox)        
        
    def drawGeneration( self ):

        slidersBox = RoundVBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        slidersBox.set_border_width(Config.BORDER_SIZE)
        geneButtonBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        geneButtonBox.set_border_width(Config.BORDER_SIZE)
        transportBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        transportBox.set_border_width(Config.BORDER_SIZE)
            
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
        ROW_LEN = 8
                   
        vBox = gtk.VBox()
        
        intrumentNum = len(self.getInstrumentList())
        rows = ( intrumentNum // ROW_LEN )
        if intrumentNum % ROW_LEN is not 0:    #S'il y a un reste
            rows = rows + 1
        
        self.firstInstButton = None
        for row in range(rows):
            hBox = gtk.HBox()
            for instrument in self.getInstrumentList()[row*ROW_LEN:(row+1)*ROW_LEN]:
                instBox = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
                instBox.set_border_width(Config.BORDER_SIZE)
                instButton = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + instrument + '.png' , Config.IMAGE_ROOT + instrument + 'sel.png', Config.IMAGE_ROOT + instrument + 'sel.png')
                if self.firstInstButton == None:
                    self.firstInstButton = instButton
                instButton.connect('clicked' , self.handleInstrumentButtonClick , instrument)
                instBox.add(instButton)
                hBox.add(instBox)
            vBox.add(hBox)
        self.leftBox.add(vBox)
        
    def drawMicBox( self ):
        hbox = gtk.HBox()
        
        for n in ['mic1','mic2','mic3','mic4']:
            vbox1 = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
            vbox1.set_border_width(Config.BORDER_SIZE)
            
            micBtn = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + n + '.png' , Config.IMAGE_ROOT + n + 'sel.png', Config.IMAGE_ROOT + n + 'sel.png')
            micRecBtn = ImageButton(Config.IMAGE_ROOT + 'record.png' , Config.IMAGE_ROOT + 'recordhi.png', Config.IMAGE_ROOT + 'recordsel.png')
            self.tooltips.set_tip(micRecBtn,Tooltips.RECMIC)
            
            micBtn.connect('clicked', self.handleInstrumentButtonClick, n)
            micRecBtn.connect('clicked', self.handleMicButtonClick, n)
            micRecBtn.connect('pressed', self.handleRecButtonPress, micBtn)
            
            vbox1.add(micRecBtn)
            vbox1.add(micBtn)
            hbox.add(vbox1)
            
        for n in ['lab1','lab2','lab3','lab4']:
            vbox2 = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
            vbox2.set_border_width(Config.BORDER_SIZE)
            
            synthBtn = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + n + '.png', Config.IMAGE_ROOT + n + 'sel.png', Config.IMAGE_ROOT + n + 'sel.png')
            synthRecBtn = ImageButton(Config.IMAGE_ROOT + 'record.png' , Config.IMAGE_ROOT + 'recordhi.png', Config.IMAGE_ROOT + 'recordsel.png')
            self.tooltips.set_tip(synthRecBtn,Tooltips.RECLAB)
            
            synthBtn.connect('clicked', self.handleInstrumentButtonClick, n)
            synthRecBtn.connect('clicked', self.handleSynthButtonClick, n)
            synthRecBtn.connect('pressed', self.handleRecButtonPress, synthBtn)
            
            vbox2.add(synthRecBtn)
            vbox2.add(synthBtn)
            hbox.add(vbox2)
            
        self.leftBox.add(hbox)
    
    def recordStateButton( self, state ):
        self.seqRecordButton.set_active( state )

    def handleInstrumentButtonClick(self , widget , instrument):
        if widget.get_active() == True and self.recstate == False:
            self.setInstrument(instrument)
            self.playInstrumentNote(instrument)         
        
    def handleRecButtonPress(self, widget, recBtn):
        self.recstate = True
        recBtn.set_active(True)
        
    def handleMicButtonClick(self , widget , data):
        self.recstate = False
        self.setInstrument(data)
        if data == 'mic1':
            self.csnd.micRecording(7)
        elif data == 'mic2':
            self.csnd.micRecording(8)
        elif data == 'mic3':
            self.csnd.micRecording(9)
        elif data == 'mic4':
            self.csnd.micRecording(10)
        else:
            return
    
    def handleSynthButtonClick(self , widget , data):
        self.recstate = False
        self.setInstrument(data)
        if data == 'lab1':
            self.synthLabWindow1.show_all()
        elif data == 'lab2':
            self.synthLabWindow2.show_all()
        elif data == 'lab3':
            self.synthLabWindow3.show_all()
        elif data == 'lab4':
            self.synthLabWindow4.show_all()
        else:
            return

    def regenerate(self):
        def flatten(ll):
            rval = []
            for l in ll:
                rval += l
            return rval
        self.notesList = flatten (
                generator(self.rythmInstrument, self.beat, self.regularity, self.reverb, self.csnd))
        self.noteLooper.clear()
        self.noteLooper.setDuration( self.beat * Config.TICKS_PER_BEAT )
        self.noteLooper.insert([(x.onset, x) for x in self.notesList])
  
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
        self.tempo = int(widget.get_adjustment().value)
        self.rythmPlayer.setTempo(self.tempo)
        self.noteLooper.setRate( self.tempo * self.tempo2tickrate )

    def handleTempoSliderChange(self,adj):
        img = int(self.scale( int(adj.value),
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
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None
        else:
            self.noteLooper.setTick(0) 
            self.playbackTimeout = gobject.timeout_add( self.timeout_ms, self.onTimeout )
            self.onTimeout()

    def onTimeout( self ):
        if self.playbackTimeout == None : return False
        cmds = self.noteLooper.next()
        for c in cmds: 
            self.csnd.sendText( c )
        #self.playStartupSound()
        return True

    def handleGenerationDrumBtn(self , widget , data):
        #data is drum1kit, drum2kit, or drum3kit
        self.rythmInstrument = data
        for n in self.notesList :
            n.instrumentFlag = data
            n.nchanges += 1
        
    def handleGenerateBtn(self , widget , data=None):
        if self.playbackTimeout == None :
            self.playStopButton.set_active(True)  #this calls handlePlayButton
            self.playStartupSound()
        else:
            self.regenerate()

    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone( self.csnd, self.rythmPlayer.recording, self.rythmPlayer.adjustDuration, self.rythmPlayer.getCurrentTick ) 
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
    
    def playInstrumentNote(self , instrument, secs_per_tick = 0.025):
        note = CSoundNote( onset = 0, 
                             pitch = 36, 
                             amplitude = 1, 
                             pan = 0.5, 
                             duration = 20, 
                             trackId = 1, 
                             fullDuration = False, 
                             instrument = instrument, 
                             instrumentFlag = instrument,
                             reverbSend = 0)
        self.csnd.sendText(note.getText(secs_per_tick,0))
    
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
    standAlonePlayer = StandAlonePlayer()
    #start the gtk event loop
    gtk.main()
