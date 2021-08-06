import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import os
import random
import time
import xdrlib
import commands

from gettext import gettext as _gettext

from types import *
from math import sqrt
from Util.NoteDB import PARAMETER

import Util.Network
Net = Util.Network # convinience assignment

import Config

from Util.ThemeWidgets import *
from Util.CSoundNote import CSoundNote
from Util import NoteDB
from Util.NoteDB import Note
from Util.CSoundClient import new_csound_client
from Util.LoopSettings import LoopSettings

from Fillin import Fillin
from KeyboardStandAlone import KeyboardStandAlone
from MiniSequencer import MiniSequencer
from Loop import Loop
from RythmGenerator import *
from SynthLab.SynthLabWindow import SynthLabWindow
from Util.Trackpad import Trackpad
from Util.InstrumentPanel import InstrumentPanel

Tooltips = Config.Tooltips

from SubActivity import SubActivity
    
class miniTamTamMain(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)

        self.activity = activity

        self.set_border_width(Config.MAIN_WINDOW_PADDING)

        self.firstTime = False
        self.csnd = new_csound_client()
        self.timeout_ms = 50
        self.instVolume = 50
        self.drumVolume = 0.5
        self.instrument = 'ocarina'
        self.regularity = 0.75
        self.beat = 4
        self.reverb = 0.
        self.tempo = Config.PLAYER_TEMPO
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.rythmInstrument = 'drum1kit'
        self.muteInst = False
        self.drumFillin = Fillin( self.beat, self.tempo, self.rythmInstrument, self.reverb, self.drumVolume )
        self.sequencer= MiniSequencer(self.recordStateButton)
        self.loop = Loop(self.beat, sqrt( self.instVolume*0.01 ))
        self.csnd.loopSetTempo(self.tempo)
        self.noteList = []
        time.sleep(0.001)
        self.trackpad = Trackpad( self )
        for i in range(21):
            self.csnd.setTrackVolume( 100, i )

        self.volume = 150
        self.csnd.setMasterVolume(self.volume)
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat 
        self.tooltips = gtk.Tooltips()
        
        self.masterVBox = gtk.VBox()
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.leftBox.set_size_request(950,-1)
        self.rightBox = gtk.VBox()
        self.mainWindowBox.pack_start(self.leftBox,False,False)
        self.mainWindowBox.pack_start(self.rightBox,True,True)
        self.masterVBox.pack_start(self.mainWindowBox)
        self.add(self.masterVBox)
       
        self.enableKeyboard()
        self.setInstrument(self.instrument)
        
        self.loopSettingsPopup = gtk.Window(gtk.WINDOW_POPUP)
        self.loopSettingsPopup.set_modal(True)
        self.loopSettingsPopup.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        self.loopSettingsPopup.connect("button-release-event", lambda w,e:self.doneLoopSettingsPopup() )
        self.loopSettings = LoopSettings( self.loopSettingsPopup )
        self.loopSettingsPopup.add( self.loopSettings )        
        
        
        self.drawInstrumentButtons()
        self.drawSliders()
        self.drawGeneration()
        self.show_all()
        if 'a good idea' == True:
            self.playStartupSound()

        self.synthLabWindow = None
        
        self.beatPickup = True
        #self.regenerate()

        self.heartbeatStart = time.time()
        self.syncQueryStart = {}
        self.syncTimeout = None

        self.network = Net.Network()
        self.network.addWatcher( self.networkStatusWatcher )
        self.network.connectMessage( Net.HT_SYNC_REPLY, self.processHT_SYNC_REPLY )
        self.network.connectMessage( Net.HT_TEMPO_UPDATE, self.processHT_TEMPO_UPDATE )
        self.network.connectMessage( Net.PR_SYNC_QUERY, self.processPR_SYNC_QUERY )
        self.network.connectMessage( Net.PR_TEMPO_QUERY, self.processPR_TEMPO_QUERY )
        self.network.connectMessage( Net.PR_REQUEST_TEMPO_CHANGE, self.processPR_REQUEST_TEMPO_CHANGE )

        # data packing classes
        self.packer = xdrlib.Packer()
        self.unpacker = xdrlib.Unpacker("")
    
        #-- handle forced networking ---------------------------------------
        if self.network.isHost():
            self.updateSync()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
        elif self.network.isPeer():
            self.sendTempoQuery()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
        #-------------------------------------------------------------------

       
        if os.path.isfile("FORCE_SHARE"):    # HOST
            r = random.random()
            #print "::::: Sharing as TTDBG%f :::::" % r
            #self.activity.set_title(_gettext("TTDBG%f" % r))
            print "::::: Sharing as TamTam :::::"
            self.activity.set_title(_gettext("TamTam"))
            self.activity.connect( "shared", self.shared )
            self.activity.share()
        elif self.activity._shared_activity: # PEER
            self.activity._shared_activity.connect( "buddy-joined", self.buddy_joined )
            self.activity._shared_activity.connect( "buddy-left", self.buddy_left )
            self.activity.connect( "joined", self.joined )
            self.network.setMode( Net.MD_WAIT )
                
    def drawSliders( self ):     
        mainLowBox = gtk.HBox()
        mainSliderBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        mainSliderBox.set_border_width(Config.PANEL_SPACING)
        
        reverbSliderBox = gtk.HBox()
        self.reverbSliderBoxImgTop = gtk.Image()
        self.reverbSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'reverb0.png')
        reverbAdjustment = gtk.Adjustment(value=self.reverb, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        reverbSlider = ImageHScale( Config.IMAGE_ROOT + "sliderbutred.png", reverbAdjustment, 7 )
        reverbSlider.set_inverted(False)
        reverbSlider.set_size_request(250,15)
        reverbAdjustment.connect("value_changed" , self.handleReverbSlider)
        reverbSliderBox.pack_start(reverbSlider, True, 20)
        reverbSliderBox.pack_start(self.reverbSliderBoxImgTop, False, padding=0)
        self.tooltips.set_tip(reverbSlider,Tooltips.REV)

        balSliderBox = gtk.HBox()
        self.balSliderBoxImgBot = gtk.Image()
        self.balSliderBoxImgTop = gtk.Image()
        self.balSliderBoxImgBot.set_from_file(Config.IMAGE_ROOT + 'dru2.png')
        self.balSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'instr2.png')
        balAdjustment = gtk.Adjustment(value=self.instVolume, lower=0, upper=100, step_incr=1, page_incr=0, page_size=0)
        balSlider = ImageHScale( Config.IMAGE_ROOT + "sliderbutviolet.png", balAdjustment, 7 )
        balSlider.set_inverted(False)
        balSlider.set_size_request(250,15)
        balAdjustment.connect("value_changed" , self.handleBalanceSlider)
        balSliderBox.pack_start(self.balSliderBoxImgBot, False, padding=0)
        balSliderBox.pack_start(balSlider, True, 20)
        balSliderBox.pack_start(self.balSliderBoxImgTop, False, padding=0)
        self.tooltips.set_tip(balSlider,Tooltips.BAL)
        
        micRecordBox = gtk.HBox()
        for i in [1,2,3,4]:
            recordButton = ImageButton(Config.IMAGE_ROOT + 'synthRecord' + str(i) + '.png', Config.IMAGE_ROOT + 'synthRecord' + str(i) + 'Down.png', Config.IMAGE_ROOT + 'synthRecord' + str(i) + 'Over.png')
            target = 'mic' + str(i)
            recordButton.connect("clicked", self.micRec, target)
            micRecordBox.pack_start(recordButton, False, False, 2)
            self.tooltips.set_tip(recordButton, Tooltips.MT_RECORDBUTTONS[i-1])
            
        #Transport Button Box
        transportBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        transportBox.set_border_width(Config.PANEL_SPACING)
        self.seqRecordButton = ImageToggleButton(Config.IMAGE_ROOT + 'krecord.png', Config.IMAGE_ROOT + 'krecordDown.png', Config.IMAGE_ROOT + 'krecordOver.png')
        self.seqRecordButton.connect('button-press-event', self.sequencer.handleRecordButton )

        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        self.playStopButton.connect('button-press-event' , self.handlePlayButton)
        transportBox.pack_start(self.seqRecordButton)
        transportBox.pack_start(self.playStopButton)
        closeButton = ImageButton(Config.IMAGE_ROOT + 'close.png')
        closeButton.connect('pressed',self.handleClose)
        transportBox.pack_start(closeButton)
        self.tooltips.set_tip(self.seqRecordButton,Tooltips.SEQ)
        self.tooltips.set_tip(self.playStopButton,Tooltips.PLAY)
    
        mainSliderBox.pack_start(balSliderBox, padding = 5)
        mainSliderBox.pack_start(reverbSliderBox, padding = 5)
        mainSliderBox.pack_start(micRecordBox, padding = 5)
        
        mainLowBox.pack_start(mainSliderBox)
        mainLowBox.pack_start(transportBox)
        
        self.masterVBox.pack_start(mainLowBox)        
        
    def drawGeneration( self ):

        slidersBox = RoundVBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        slidersBox.set_border_width(Config.PANEL_SPACING)
        geneButtonBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        geneButtonBox.set_border_width(Config.PANEL_SPACING)
            
        geneSliderBox = gtk.VBox()
        self.geneSliderBoxImgTop = gtk.Image()
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex6.png')
        geneAdjustment = gtk.Adjustment(value=self.regularity, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        geneSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutbleu.png", geneAdjustment, 5 )
        geneSlider.set_inverted(False)
        geneSlider.set_size_request(15,320)
        geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        geneSlider.connect("button-release-event", self.handleGenerationSliderRelease)
        geneSliderBox.pack_start(self.geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(geneSlider, True, 20)
        self.tooltips.set_tip(geneSlider,Tooltips.COMPL)
                        
        beatSliderBox = gtk.VBox()
        self.beatSliderBoxImgTop = gtk.Image()
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat3.png')
        self.beatAdjustment = gtk.Adjustment(value=self.beat, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        self.beatSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutjaune.png", self.beatAdjustment, 5, snap = 1 )
        self.beatSlider.set_inverted(True)
        self.beatSlider.set_size_request(15,320)
        self.beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        self.beatSlider.connect("button-release-event", self.handleBeatSliderRelease)
        beatSliderBox.pack_start(self.beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(self.beatSlider, True, 20)
        self.tooltips.set_tip(self.beatSlider,Tooltips.BEAT)
                        
        tempoSliderBox = gtk.VBox()
        self.tempoSliderBoxImgTop = gtk.Image()
        self.tempoSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'tempo5.png')
        self.tempoAdjustment = gtk.Adjustment(value=self.tempo, lower=Config.PLAYER_TEMPO_LOWER, upper=Config.PLAYER_TEMPO_UPPER, step_incr=1, page_incr=1, page_size=1)
        #tempoSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutvert.png", self.tempoAdjustment, 5)
        tempoSlider = gtk.VScale( self.tempoAdjustment)
        #TEMP
        tempoSlider.set_inverted(True)
        tempoSlider.set_size_request(15,320)
        self.tempoAdjustmentHandler = self.tempoAdjustment.connect("value_changed" , self.handleTempoSliderChange)
        tempoSlider.connect("button-release-event", self.handleTempoSliderRelease)
        tempoSliderBox.pack_start(self.tempoSliderBoxImgTop, False, padding=10)
        tempoSliderBox.pack_start(tempoSlider, True)
        self.tooltips.set_tip(tempoSlider,Tooltips.TEMPO)

        volumeSliderBox = gtk.VBox()
        self.volumeSliderBoxImgTop = gtk.Image()
        self.volumeSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'volume2.png')
        self.volumeAdjustment = gtk.Adjustment(value=self.volume, lower=0, upper=200, step_incr=1, page_incr=1, page_size=1)
        volumeSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutbleu.png", self.volumeAdjustment, 5)
        volumeSlider.set_inverted(True)
        volumeSlider.set_size_request(15,320)
        self.volumeAdjustment.connect("value_changed" , self.handleVolumeSlider)
        #volumeSlider.connect("button-release-event", self.handleVolumeSliderRelease)
        volumeSliderBox.pack_start(self.volumeSliderBoxImgTop, False, padding=10)
        volumeSliderBox.pack_start(volumeSlider, True)
        self.tooltips.set_tip(volumeSlider,Tooltips.VOL)
 
        
        slidersBoxSub = gtk.HBox()        
        slidersBoxSub.pack_start(beatSliderBox)
        slidersBoxSub.pack_start(geneSliderBox)
        slidersBoxSub.pack_start(tempoSliderBox)
        slidersBoxSub.pack_start(volumeSliderBox)
        slidersBox.pack_start(slidersBoxSub)
        
        generateBtnSub = gtk.HBox()
        generateBtn = ImageButton(Config.IMAGE_ROOT + 'dice.png', clickImg_path = Config.IMAGE_ROOT + 'diceblur.png')
        generateBtn.connect('button-press-event', self.handleGenerateBtn)
        self.loopSettingsBtn = ImageToggleButton(Config.IMAGE_ROOT + 'dice.png', Config.IMAGE_ROOT + 'diceblur.png')
        self.loopSettingsBtn.connect('toggled', self.handleLoopSettingsBtn)
        generateBtnSub.pack_start(self.loopSettingsBtn)
        generateBtnSub.pack_start(generateBtn)
        slidersBox.pack_start(generateBtnSub)
        self.tooltips.set_tip(generateBtn,Tooltips.GEN)
        
        #Generation Button Box    
        geneVBox = gtk.VBox()
        geneTopBox = gtk.HBox()
        geneMidBox = gtk.HBox()
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
        generationDrumBtn5 = ImageRadioButton(group = generationDrumBtn1 , mainImg_path = Config.IMAGE_ROOT + 'drum5kit.png' , altImg_path = Config.IMAGE_ROOT + 'drum5kitselgen.png')
        generationDrumBtn5.connect('clicked' , self.handleGenerationDrumBtn , 'drum5kit')
        geneMidBox.pack_start(generationDrumBtn5, True)
        geneVBox.pack_start(geneTopBox, True)
        geneVBox.pack_start(geneMidBox, True)
        geneVBox.pack_start(geneLowBox, True)
        geneButtonBox.pack_start(geneVBox,True)
        self.tooltips.set_tip(generationDrumBtn1,Tooltips.JAZZ)
        self.tooltips.set_tip(generationDrumBtn2,Tooltips.ARAB)
        self.tooltips.set_tip(generationDrumBtn3,Tooltips.AFRI)
        self.tooltips.set_tip(generationDrumBtn4,Tooltips.ELEC)
        self.tooltips.set_tip(generationDrumBtn5,Tooltips.BRES)
        
        self.rightBox.pack_start(slidersBox, True)
        self.rightBox.pack_start(geneButtonBox, True)

    def doneLoopSettingsPopup(self):
        if self.loopSettingsBtn.get_active():
            self.loopSettingsBtn.set_active(False)
    
    def handleLoopSettingsBtn(self, widget, data=None):
        if widget.get_active():
            self.loopSettingsPopup.show()
            self.loopSettingsPopup.move( 600, 400 )
        else:
            self.loopSettingsPopup.hide()        
    
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

    def micRec(self, widget, mic):
        os.system('rm ' + Config.PREF_DIR + '/' + mic)
        self.csnd.inputMessage("i5600 0 4")
        (s1,o1) = commands.getstatusoutput("arecord -f S16_LE -t wav -r 16000 -d 4 " + Config.PREF_DIR + "/tempMic.wav")
        (s2, o2) = commands.getstatusoutput("csound " + Config.FILES_DIR + "/crop.csd")
        (s3, o3) = commands.getstatusoutput("mv " + Config.PREF_DIR + "/micTemp " + Config.PREF_DIR + "/" + mic)
        (s4, o4) = commands.getstatusoutput("rm " + Config.PREF_DIR + "/tempMic.wav") 
        self.micTimeout = gobject.timeout_add(200, self.loadMicInstrument, mic)
        
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
        if self.beatPickup:
            self.pickupNewBeat()
        noteOnsets = []
        notePitchs = []
        i = 0
        self.noteList= []
        self.csnd.loopClear()
        for x in flatten( generator(self.rythmInstrument, self.beat, 0.8, self.regularity, self.reverb) ):
            x.amplitude = x.amplitude * self.drumVolume
            noteOnsets.append(x.onset)
            notePitchs.append(x.pitch)
            n = Note(0, x.trackId, i, x)
            self.noteList.append( (x.onset, n) )
            i = i + 1
            self.csnd.loopPlay(n,1)                    #add as active
        self.csnd.loopSetNumTicks( self.beat * Config.TICKS_PER_BEAT)
        self.drumFillin.unavailable( noteOnsets, notePitchs )

    def adjustDrumVolume(self):
        for n in self.noteList:
            self.csnd.loopUpdate(n[1], PARAMETER.AMPLITUDE, n[1].cs.amplitude*self.drumVolume, 1)
            
    def handleClose(self,widget):
        if self.playStopButton.get_active() == True:
            self.playStopButton.set_active(False)  
        self.sequencer.clearSequencer()
        self.csnd.loopClear()
        self.set_mode('quit')
               
    def handleGenerationSlider(self, adj):
        img = int(adj.value * 7)+1
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex' + str(img) + '.png')

    def handleGenerationSliderRelease(self, widget, event):
        self.regularity = widget.get_adjustment().value
        self.regenerate()

    def pickupNewBeat(self):
        self.beat = random.randint(2, 12)
        img = self.scale(self.beat,2,12,1,11)
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat' + str(img) + '.png')
        self.beatAdjustment.set_value(self.beat)
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat
        self.drumFillin.setBeats( self.beat )
        
    def handleBeatSlider(self, adj):
        img = self.scale(int(adj.value),2,12,1,11)
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat' + str(img) + '.png')
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat
        self.drumFillin.setBeats( self.beat )
        
    def handleBeatSliderRelease(self, widget, event):
        self.beat = int(widget.get_adjustment().value)
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat
        self.drumFillin.setBeats( self.beat )
        self.beatPickup = False
        self.regenerate()
        self.beatPickup = True

    def handleTempoSliderRelease(self, widget, event):
        #self.tempo = int(widget.get_adjustment().value)
        #self.csnd.loopSetTempo(self.tempo)
        #self.sequencer.tempo = widget.get_adjustment().value
        #self.drumFillin.setTempo(self.tempo)
        pass

    def handleTempoSliderChange(self,adj):
        print "handleTempoSliderChange"
        if self.network.isPeer():
            self.requestTempoChange(int(adj.value))
        else: 
            self._updateTempo( int(adj.value), True )

    def _updateTempo( self, val, propagate = False ):

        if self.network.isHost():
            t = time.time()
            percent = self.heartbeatElapsed() / self.beatDuration

        self.tempo = val 
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.csnd.loopSetTempo(self.tempo)
        self.sequencer.tempo = self.tempo 
        self.drumFillin.setTempo(self.tempo)

        if self.network.isHost():
            self.heatbeatStart = t - percent*self.beatDuration
            self.updateSync()
            self.sendTempoUpdate()
 
        img = int(self.scale( self.tempo,
            Config.PLAYER_TEMPO_LOWER,Config.PLAYER_TEMPO_UPPER,
            1,8))
        self.tempoSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'tempo' + str(img) + '.png')

    def handleBalanceSlider(self, adj):
        self.instVolume = int(adj.value)
        self.drumVolume = sqrt( (100-self.instVolume)*0.01 )
        self.adjustDrumVolume()
        self.drumFillin.setVolume(self.drumVolume)
        instrumentVolume = sqrt( self.instVolume*0.01 )
        self.loop.adjustLoopVolume(instrumentVolume)
        self.sequencer.adjustSequencerVolume(instrumentVolume)
        img = int(self.scale(self.instVolume,100,0,0,4.9))
        self.balSliderBoxImgBot.set_from_file(Config.IMAGE_ROOT + 'dru' + str(img) + '.png')
        img2 = int(self.scale(self.instVolume,0,100,0,4.9))
        self.balSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'instr' + str(img2) + '.png')
        
    def handleReverbSlider(self, adj):
        self.reverb = adj.value
        self.drumFillin.setReverb( self.reverb )
        img = int(self.scale(self.reverb,0,1,0,4))
        self.reverbSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'reverb' + str(img) + '.png')
        self.keyboardStandAlone.setReverb(self.reverb)

    def handleVolumeSlider(self, adj):
        self.volume = adj.value
        self.csnd.setMasterVolume(self.volume)
        img = int(self.scale(self.volume,0,200,0,3.9))
        self.volumeSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'volume' + str(img) + '.png')
        
    def handlePlayButton(self, widget, data = None):
	# use widget.get_active() == False when calling this on 'clicked'
	# use widget.get_active() == True when calling this on button-press-event
        if self.playStopButton.get_active() == True:
            self.drumFillin.stop()
            self.sequencer.stopPlayback()
            self.csnd.loopPause()
        else:
            if not self.firstTime:
                self.regenerate()
                self.firstTime = True
            self.drumFillin.play()
            #self.csnd.loopSetTick(0)
            nextInTicks = self.nextHeartbeatInTicks()
            #print "play:: next beat in %f ticks. bpb == %d. setting ticks to %d" % (nextInTicks, self.beat, Config.TICKS_PER_BEAT*self.beat - int(round(nextInTicks)))
            self.csnd.loopSetTick( Config.TICKS_PER_BEAT*self.beat - int(round(nextInTicks)) )
            self.csnd.loopStart()

    def handleGenerationDrumBtn(self , widget , data):
        #data is drum1kit, drum2kit, or drum3kit
        #print 'HANDLE: Generate Button'
        self.rythmInstrument = data
        instrumentId = Config.INSTRUMENTS[data].instrumentId
        for (o,n) in self.noteList :
            self.csnd.loopUpdate(n, NoteDB.PARAMETER.INSTRUMENT, instrumentId, -1)
        self.drumFillin.setInstrument( self.rythmInstrument )
        
    def handleGenerateBtn(self , widget , data=None):
        self.regenerate()
        if not self.playStopButton.get_active():
                self.handlePlayButton(self, widget)
                self.playStopButton.set_active(True) 

        #this calls sends a 'clicked' event, 
        #which might be connected to handlePlayButton
        self.playStartupSound()

    def enableKeyboard( self ):
        self.keyboardStandAlone = KeyboardStandAlone( self.sequencer.recording, self.sequencer.adjustDuration, self.csnd.loopGetTick, self.sequencer.getPlayState, self.loop ) 
        self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
    
    def setInstrument( self , instrument ):
        self.instrument = instrument
        self.keyboardStandAlone.setInstrument(instrument)
    
    def playInstrumentNote(self , instrument, secs_per_tick = 0.025):
        if not self.muteInst:
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
        if event.hardware_keycode == 219: #'/*' button to reset drum loop
            if self.playStopButton.get_active() == True:
                self.handlePlayButton(self.playStopButton)
                self.playStopButton.set_active(False)
                self.handlePlayButton(self.playStopButton)
                self.playStopButton.set_active(True)

        if event.hardware_keycode == 37:
            if self.muteInst:
                self.muteInst = False
            else:
                self.muteInst = True
                
        if event.hardware_keycode == 65: #what key is this? what feature is this?
            pass
            #if self.playStopButton.get_active():
                #self.playStopButton.set_active(False)
            #else:
                #self.playStopButton.set_active(True)
                
        self.keyboardStandAlone.onKeyPress(widget, event, sqrt( self.instVolume*0.01 ))

    def onKeyRelease(self, widget, event):
        self.keyboardStandAlone.onKeyRelease(widget, event)
    
    def playStartupSound(self):
        r = str(random.randrange(1,11))
        self.playInstrumentNote('guidice' + r)

    def getInstrumentList(self):
        cleanInstrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab' and instrument[0:4] != 'guid']
        cleanInstrumentList.sort(lambda g,l: cmp(Config.INSTRUMENTS[g].category, Config.INSTRUMENTS[l].category) )
        return cleanInstrumentList + ['drum1kit', 'drum2kit', 'drum3kit']
    
    def onActivate( self, arg ):
        self.csnd.loopPause()
        self.csnd.loopClear()

    def onDeactivate( self ):
        SubActivity.onDeactivate( self )
        self.releaseInstrumentPanel()
        self.csnd.loopPause()
        self.csnd.loopClear()

    def onDestroy( self ):
        self.network.shutdown()
        
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
    
     
    #-----------------------------------------------------------------------
    # Network

    #-- Activity -----------------------------------------------------------

    def shared( self, activity ):
        if Config.DEBUG: print "miniTamTam:: successfully shared, start host mode"
        self.activity._shared_activity.connect( "buddy-joined", self.buddy_joined )
        self.activity._shared_activity.connect( "buddy-left", self.buddy_left )
        self.network.setMode( Net.MD_HOST )
        self.updateSync()
        self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )

    def joined( self, activity ):
        print "miniTamTam:: joined activity!!"
        for buddy in self.activity._shared_activity.get_joined_buddies():
            print buddy.props.ip4_address

    def buddy_joined( self, activity, buddy ):
        print "buddy joined " + str(buddy)
        try:
            print buddy.props.ip4_address
        except:
            print "bad ip4_address"
        if self.network.isHost():
            # TODO how do I figure out if this buddy is me?
            if buddy.props.ip4_address:
                self.network.introducePeer( buddy.props.ip4_address )
            else:
                print "miniTamTam:: new buddy does not have an ip4_address!!"

    def buddy_left( self, activity, buddy):
        print "buddy left"

    #def joined( self, activity ):
    #    if Config.DEBUG: print "miniTamTam:: successfully joined, wait for host"
    #    self.net.waitForHost()

    #-- Senders ------------------------------------------------------------

    def sendSyncQuery( self ):
        self.packer.pack_float(random.random())
        hash = self.packer.get_buffer()
        self.packer.reset()
        self.syncQueryStart[hash] = time.time()
        self.network.send( Net.PR_SYNC_QUERY, hash)

    def sendTempoUpdate( self ):
        self.packer.pack_int(self.tempo)
        self.network.sendAll( Net.HT_TEMPO_UPDATE, self.packer.get_buffer() )
        self.packer.reset()

    def sendTempoQuery( self ):
        self.network.send( Net.PR_TEMPO_QUERY )

    def requestTempoChange( self, val ):
        print "requestTempoChange", val
        self.packer.pack_int(val)
        self.network.send( Net.PR_REQUEST_TEMPO_CHANGE, self.packer.get_buffer() )
        self.packer.reset()

    #-- Handlers -----------------------------------------------------------

    def networkStatusWatcher( self, mode ):
        if mode == Net.MD_OFFLINE:
            if self.syncTimeout:
                gobject.source_remove( self.syncTimeout )
                self.syncTimeout = None
        if mode == Net.MD_PEER:
            self.updateSync()
            if not self.syncTimeout:
                self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
            self.sendTempoQuery()
            
    def processHT_SYNC_REPLY( self, sock, message, data ):
        t = time.time()
        hash = data[0:4]
        latency = t - self.syncQueryStart[hash]
        self.unpacker.reset(data[4:8])
        nextBeat = self.unpacker.unpack_float()
        #print "mini:: got sync: next beat in %f, latency %d" % (nextBeat, latency*1000)
        self.heartbeatStart = t + nextBeat - self.beatDuration - latency/2
        self.correctSync()
        self.syncQueryStart.pop(hash)

    def processHT_TEMPO_UPDATE( self, sock, message, data ):
        print "got tempo update"
        self.unpacker.reset(data)
        self.tempoAdjustment.handler_block( self.tempoAdjustmentHandler )
        val = self.unpacker.unpack_int()
        self.tempoAdjustment.set_value( val )
        self._updateTempo( val )
        self.tempoAdjustment.handler_unblock( self.tempoAdjustmentHandler )
        self.sendSyncQuery()
        print "done"
 
    def processPR_SYNC_QUERY( self, sock, message, data ):
        self.packer.pack_float(self.nextHeartbeat())
        self.network.send( Net.HT_SYNC_REPLY, data + self.packer.get_buffer(), sock )
        self.packer.reset()

    def processPR_TEMPO_QUERY( self, sock, message, data ):
        print "processPR_TEMPO_QUERY"
        self.packer.pack_int(self.tempo)
        self.network.send( Net.HT_TEMPO_UPDATE, self.packer.get_buffer(), to = sock )
        self.packer.reset()
        print "done"

    def processPR_REQUEST_TEMPO_CHANGE( self, sock, message, data ):
        self.unpacker.reset(data)
        val = self.unpacker.unpack_int()
        print "got tempo change", val
        self.tempoAdjustment.set_value( val )
        print "done"

    #-----------------------------------------------------------------------
    # Sync

    def nextHeartbeat( self ):
        delta = time.time() - self.heartbeatStart
        return self.beatDuration - (delta % self.beatDuration)

    def nextHeartbeatInTicks( self ):
        delta = time.time() - self.heartbeatStart
        next = self.beatDuration - (delta % self.beatDuration)
        return self.ticksPerSecond*next

    def heartbeatElapsed( self ):
        delta = time.time() - self.heartbeatStart
        return delta % self.beatDuration

    def heartbeatElapsedTicks( self ):
        delta = time.time() - self.heartbeatStart
        return self.ticksPerSecond*(delta % self.beatDuration)
        
    def updateSync( self ):
        if self.network.isOffline():
            return False
        elif self.network.isWaiting():
            return True
        elif self.network.isHost():
            self.correctSync()
        else:
            self.sendSyncQuery()
        return True

    def correctSync( self ):
        curTick = self.csnd.loopGetTick()
        curTicksIn = curTick % Config.TICKS_PER_BEAT
        ticksIn = self.heartbeatElapsedTicks()
        err = curTicksIn - ticksIn
        if err > Config.TICKS_PER_BEAT_DIV2: 
            err -= Config.TICKS_PER_BEAT
        elif err < -Config.TICKS_PER_BEAT_DIV2:
            err += Config.TICKS_PER_BEAT
        correct = curTick - err
        ticksPerLoop = Config.TICKS_PER_BEAT*self.beat
        if correct > ticksPerLoop:
            correct -= ticksPerLoop
        elif correct < 0:
            correct += ticksPerLoop
        #print "correct:: %f ticks, %f ticks in, %f expected, %f err, correct %f" % (curTick, curTicksIn, ticksIn, err, correct)
        if abs(err) > 0.25:
            self.csnd.loopAdjustTick(-err)
        

if __name__ == "__main__": 
    MiniTamTam = miniTamTam()
    #start the gtk event loop
    gtk.main()
