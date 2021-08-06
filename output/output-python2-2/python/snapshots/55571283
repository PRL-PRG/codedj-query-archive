import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import os
import random
import time
import xdrlib
import commands

from types import *
from math import sqrt
from common.Util.NoteDB import PARAMETER

import common.Util.Network as Net

import common.Config as Config

from Mini.miniToolbars import playToolbar
from Mini.miniToolbars import recordToolbar
from common.Util.ThemeWidgets import *
from common.Util.CSoundNote import CSoundNote
from common.Util import NoteDB
from common.Util.NoteDB import Note
from common.Util.CSoundClient import new_csound_client
from common.Util.LoopSettings import LoopSettings

from Fillin import Fillin
from KeyboardStandAlone import KeyboardStandAlone
from MiniSequencer import MiniSequencer
from Loop import Loop
from RythmGenerator import *
from common.Util.Trackpad import Trackpad
from Mini.InstrumentPanel import InstrumentPanel

from gettext import gettext as _

Tooltips = Config.Tooltips

class miniTamTamMain(gtk.EventBox):

    def __init__(self, activity):
        gtk.EventBox.__init__(self)

        self.activity = activity

        self.set_border_width(Config.MAIN_WINDOW_PADDING)

        self.firstTime = False
        self.playing = False
        self.csnd = new_csound_client()
        self.timeout_ms = 50
        self.instVolume = 50
        self.drumVolume = 0.5
        self.instrument = 'ocarina'
        self.regularity = 0.75
        self.beat = 4
        self.reverb = 0.1
        self.tempo = Config.PLAYER_TEMPO
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.rythmInstrument = 'drum1kit'
        self.muteInst = False
        self.drumFillin = Fillin( self.beat, self.tempo, self.rythmInstrument, self.reverb, self.drumVolume )
        self.sequencer= MiniSequencer(self.recordStateButton, self.recordOverSensitivity)
        self.loop = Loop(self.beat, sqrt( self.instVolume*0.01 ))
        self.csnd.setTempo(self.tempo)
        self.noteList = []
        time.sleep(0.001) # why?
        self.trackpad = Trackpad( self )
        for i in range(21):
            self.csnd.setTrackVolume( 100, i )

        self.volume = 100
        self.csnd.setMasterVolume(self.volume)
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat
        self.tooltips = gtk.Tooltips()

        self.masterVBox = gtk.VBox()
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.leftBox.set_size_request(950,-1)
        self.rightBox = gtk.VBox()
        self.mainWindowBox.pack_start(self.rightBox,True,True)
        self.mainWindowBox.pack_start(self.leftBox,False,False)
        self.masterVBox.pack_start(self.mainWindowBox)
        self.add(self.masterVBox)

        self.enableKeyboard()
        self.setInstrument(self.instrument)

        self.loopSettingsPopup = gtk.Window(gtk.WINDOW_POPUP)
        self.loopSettingsPopup.set_modal(True)
        self.loopSettingsPopup.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        self.loopSettingsPopup.connect("button-release-event", lambda w,e:self.doneLoopSettingsPopup() )
        self.loopSettings = LoopSettings( self.loopSettingsPopup, self.loopSettingsPlayStop, self.loopSettingsChannel, self.doneLoopSettingsPopup )
        self.loopSettingsPopup.add( self.loopSettings )
        self.loopSettingsPlaying = False


        self.drawInstrumentButtons()
        self.drawGeneration()
        self.show_all()
        if 'a good idea' == True:
            self.playStartupSound()

        #self.synthLabWindow = None


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

        #Play button Image
        self.playButtonImg = gtk.Image()
        self.playButtonImg.set_from_icon_name('media-playback-start', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.playButtonImg.show()

        #Stop button Image
        self.stopButtonImg = gtk.Image()
        self.stopButtonImg.set_from_icon_name('media-playback-stop', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.stopButtonImg.show()
        # Toolbar
        self.activity.activity_toolbar.share.show()
        self._playToolbar = playToolbar(self.activity.toolbox, self)
        self._recordToolbar = recordToolbar(self.activity.toolbox, self)
        self.activity.toolbox.add_toolbar(_('Play'), self._playToolbar)
        self.activity.toolbox.add_toolbar(_('Record'), self._recordToolbar)
        self.activity.toolbox.set_current_toolbar(1)
        self._playToolbar.show()
        self._recordToolbar.show()

        self.activity.connect( "shared", self.shared )

        if os.path.isfile("FORCE_SHARE"):    # HOST
            r = random.random()
            #print "::::: Sharing as TTDBG%f :::::" % r
            #self.activity.set_title(_("TTDBG%f" % r))
            print "::::: Sharing as TamTam :::::"
            self.activity.set_title(_("TamTam"))
            self.activity.share()
        elif self.activity._shared_activity: # PEER
            self.activity._shared_activity.connect( "buddy-joined", self.buddy_joined )
            self.activity._shared_activity.connect( "buddy-left", self.buddy_left )
            self.activity.connect( "joined", self.joined )
            self.network.setMode( Net.MD_WAIT )
            #self.activity.activity_toolbar.share.hide()

    def drawGeneration( self ):

        slidersBox = RoundVBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        slidersBox.set_border_width(Config.PANEL_SPACING)
        geneButtonBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        geneButtonBox.set_border_width(Config.PANEL_SPACING)

        geneSliderBox = gtk.VBox()
        self.geneSliderBoxImgTop = gtk.Image()
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex6.png')
        self.geneAdjustment = gtk.Adjustment(value=self.regularity, lower=0, upper=1, step_incr=0.01, page_incr=0, page_size=0)
        self.geneSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutbleu.png", self.geneAdjustment, 5 )
        self.geneSlider.set_inverted(False)
        self.geneSlider.set_size_request(15,305)
        self.geneAdjustment.connect("value_changed" , self.handleGenerationSlider)
        self.geneSlider.connect("button-release-event", self.handleGenerationSliderRelease)
        geneSliderBox.pack_start(self.geneSliderBoxImgTop, False, padding=10)
        geneSliderBox.pack_start(self.geneSlider, True, 20)
        self.tooltips.set_tip(self.geneSlider,Tooltips.COMPL)

        beatSliderBox = gtk.VBox()
        self.beatSliderBoxImgTop = gtk.Image()
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat3.png')
        self.beatAdjustment = gtk.Adjustment(value=self.beat, lower=2, upper=12, step_incr=1, page_incr=0, page_size=0)
        self.beatSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutjaune.png", self.beatAdjustment, 5, snap = 1 )
        self.beatSlider.set_inverted(True)
        self.beatSlider.set_size_request(15,305)
        self.beatAdjustment.connect("value_changed" , self.handleBeatSlider)
        self.beatSlider.connect("button-release-event", self.handleBeatSliderRelease)
        beatSliderBox.pack_start(self.beatSliderBoxImgTop, False, padding=10)
        beatSliderBox.pack_start(self.beatSlider, True, 20)
        self.tooltips.set_tip(self.beatSlider,Tooltips.BEAT)

        self.delayedTempo = 0 # used to store tempo updates while the slider is active
        self.tempoSliderActive = False

        tempoSliderBox = gtk.VBox()
        self.tempoSliderBoxImgTop = gtk.Image()
        self.tempoSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'tempo5.png')
        self.tempoAdjustment = gtk.Adjustment(value=self.tempo, lower=Config.PLAYER_TEMPO_LOWER, upper=Config.PLAYER_TEMPO_UPPER, step_incr=1, page_incr=1, page_size=1)
        tempoSlider = ImageVScale( Config.IMAGE_ROOT + "sliderbutvert.png", self.tempoAdjustment, 5)
        tempoSlider.set_inverted(True)
        tempoSlider.set_size_request(15,305)
        self.tempoAdjustmentHandler = self.tempoAdjustment.connect("value_changed" , self.handleTempoSliderChange)
        tempoSlider.connect("button-press-event", self.handleTempoSliderPress)
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
        volumeSlider.set_size_request(15,305)
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

    def loopSettingsChannel(self, channel, value):
        self.csnd.setChannel(channel, value)

    def loopSettingsPlayStop(self, state, loop):
        if not state:
            if loop:
                self.loopSettingsPlaying = True
                self.csnd.inputMessage(Config.CSOUND_PLAY_LS_NOTE % 5022)
            else:
                self.csnd.inputMessage(Config.CSOUND_PLAY_LS_NOTE % 5023)
        else:
            if loop:
                self.loopSettingsPlaying = False
                self.csnd.inputMessage(Config.CSOUND_STOP_LS_NOTE)

    def doneLoopSettingsPopup(self):
        if self._recordToolbar.loopSetButton.get_active():
            if self.loopSettingsPlaying:
                self.csnd.inputMessage(Config.CSOUND_STOP_LS_NOTE)
                self.loopSettingsPlaying = False
            self._recordToolbar.loopSetButton.set_active(False)

    def handleLoopSettingsBtn(self, widget, data=None):
        if widget.get_active():

            chooser = gtk.FileChooserDialog(title='Edit SoundFile Preference',action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

            #filter = gtk.FileFilter()
            #filter.add_pattern('*.wav')
            #chooser.set_filter(filter)
            chooser.set_current_folder(Config.SNDS_DIR)

            for f in chooser.list_shortcut_folder_uris():
                chooser.remove_shortcut_folder_uri(f)

            if chooser.run() == gtk.RESPONSE_OK:
                try:
                    tempName = chooser.get_filename()
                    soundName = os.path.split(tempName)[1]
                except IOError:
                    print 'ERROR: failed to load Sound from file %s' % chooser.get_filename()
            chooser.destroy()
            #results = commands.getstatusoutput("csound -U sndinfo %s" % tempName)
            results = commands.getstatusoutput("du -b %s" % tempName)
            if results[0] == 0:
                list = results[1].split()
                #pos = list.index('seconds')
                #soundLength = float(list[pos-1])
                soundLength = float(list[0]) / 2 / 16000.
            self.loopSettings.set_name(soundName)
            self.loopSettings.setButtonState()
            self.loopSettingsPopup.show()
            self.loopSettingsPopup.move( 600, 200 )
            self.timeoutLoad = gobject.timeout_add(2000, self.load_ls_instrument, soundName, soundLength)
        else:
            self.loopSettingsPopup.hide()

    def load_ls_instrument(self, soundName, soundLength):
        self.csnd.load_ls_instrument(soundName)
        self.loopSettings.set_values(soundLength)
        gobject.source_remove( self.timeoutLoad )

    def drawInstrumentButtons(self):
        self.instrumentPanelBox = gtk.HBox()
        # InstrumentPanel(elf.setInstrument,self.playInstrumentNote, False, self.micRec, self.synthRec)
        self.leftBox.pack_start(self.instrumentPanelBox,True,True)

    def setInstrumentPanel( self, instrumentPanel ):
        instrumentPanel.configure( self.setInstrument,self.playInstrumentNote, False, self.micRec )
        self.instrumentPanel = instrumentPanel
        self.instrumentPanelBox.pack_start( instrumentPanel )

    def releaseInstrumentPanel( self ):
        self.instrumentPanelBox.remove( self.instrumentPanel )

    def micRec(self, widget, mic):
        os.system('rm ' + Config.SNDS_DIR + '/' + mic)
        self.csnd.inputMessage("i5600 0 4")
        (s1,o1) = commands.getstatusoutput("arecord -f S16_LE -t wav -r 16000 -d 4 " + Config.SNDS_DIR + "/tempMic.wav")
        (s2, o2) = commands.getstatusoutput("csound " + Config.FILES_DIR + "/crop.csd")
        (s3, o3) = commands.getstatusoutput("mv " + Config.SNDS_DIR + "/micTemp " + Config.SNDS_DIR + "/" + mic)
        (s4, o4) = commands.getstatusoutput("rm " + Config.SNDS_DIR + "/tempMic.wav")
        self.micTimeout = gobject.timeout_add(200, self.loadMicInstrument, mic)
        self.instrumentPanel.set_activeInstrument(mic,True)
        self.setInstrument(mic)

    def recordStateButton( self, button, state ):
        if button == 1:
            self._recordToolbar.keyboardRecButton.set_active( state )
        else:
            self._recordToolbar.keyboardRecOverButton.set_active( state )

    def recordOverSensitivity( self, state ):
        self._recordToolbar.keyboardRecOverButton.set_sensitive( state )

    #def synthLabWindowOpen(self):
        #return self.synthLabWindow != None  and self.synthLabWindow.get_property('visible')

    def loadMicInstrument( self, data ):
        self.csnd.load_mic_instrument( data )

    #def closeSynthLab(self):
        #if self.synthLabWindow != None:
            #self.synthLabWindow.destroy()
            #self.synthLabWindow = None

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
        self.recordOverSensitivity( False )
        if self.playing:
            self.csnd.loopStart()

    def adjustDrumVolume(self):
        for n in self.noteList:
            self.csnd.loopUpdate(n[1], PARAMETER.AMPLITUDE, n[1].cs.amplitude*self.drumVolume, 1)

    def handleClose(self,widget):
        if self.playStopButton.get_active() == True:
            self.playStopButton.set_active(False)
        self.sequencer.clearSequencer()
        self.csnd.loopClear()
        self.activity.close()

    def handleGenerationSlider(self, adj):
        img = int(adj.value * 7)+1
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex' + str(img) + '.png')

    def handleGenerationSliderRelease(self, widget, event):
        self.regularity = widget.get_adjustment().value
        self.beatPickup = False
        self.regenerate()
        self.beatPickup = True

    def pickupNewBeat(self):
        self.beat = random.randint(2, 12)
        img = self.scale(self.beat,2,12,1,11)
        self.beatSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'beat' + str(img) + '.png')
        self.beatAdjustment.set_value(self.beat)

        self.regularity = random.randint(50, 100) * 0.01
        img = int(self.regularity * 7)+1
        self.geneSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'complex' + str(img) + '.png')
        self.geneAdjustment.set_value(self.regularity)

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

    def handleTempoSliderPress(self, widget, event):
        self.tempoSliderActive = True

    def handleTempoSliderRelease(self, widget, event):
        self.tempoSliderActive = False
        if self.network.isPeer() and self.delayedTempo != 0:
            if self.tempo != self.delayedTempo:
                self.tempoAdjustment.handler_block( self.tempoAdjustmentHandler )
                self.tempoAdjustment.set_value( self.delayedTempo )
                self._updateTempo( self.delayedTempo )
                self.tempoAdjustment.handler_unblock( self.tempoAdjustmentHandler )
            self.delayedTempo = 0
            self.sendSyncQuery()

    def handleTempoSliderChange(self,adj):
        if self.network.isPeer():
            self.requestTempoChange(int(adj.value))
        else:
            self._updateTempo( int(adj.value) )

    def _updateTempo( self, val ):

        if self.network.isHost():
            t = time.time()
            percent = self.heartbeatElapsed() / self.beatDuration

        self.tempo = val
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.csnd.setTempo(self.tempo)
        self.sequencer.tempo = self.tempo
        self.drumFillin.setTempo(self.tempo)

        if self.network.isHost():
            self.heatbeatStart = t - percent*self.beatDuration
            self.updateSync()
            self.sendTempoUpdate()

        img = int(self.scale( self.tempo,
            Config.PLAYER_TEMPO_LOWER,Config.PLAYER_TEMPO_UPPER,
            1,9))
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
        self._playToolbar.balanceSliderImgLeft.set_from_file(Config.IMAGE_ROOT + 'dru' + str(img) + '.png')
        img2 = int(self.scale(self.instVolume,0,100,0,4.9))
        self._playToolbar.balanceSliderImgRight.set_from_file(Config.IMAGE_ROOT + 'instr' + str(img2) + '.png')

    def handleReverbSlider(self, adj):
        self.reverb = adj.value
        self.drumFillin.setReverb( self.reverb )
        img = int(self.scale(self.reverb,0,1,0,4))
        self._playToolbar.reverbSliderImgRight.set_from_file(Config.IMAGE_ROOT + 'reverb' + str(img) + '.png')
        self.keyboardStandAlone.setReverb(self.reverb)

    def handleVolumeSlider(self, adj):
        self.volume = adj.value
        self.csnd.setMasterVolume(self.volume)
        img = int(self.scale(self.volume,0,200,0,3.9))
        self.volumeSliderBoxImgTop.set_from_file(Config.IMAGE_ROOT + 'volume' + str(img) + '.png')

    def handlePlayButton(self, widget, data = None):
	# use widget.get_active() == False when calling this on 'clicked'
	# use widget.get_active() == True when calling this on button-press-event
        if widget.get_active() == False:
            self.drumFillin.stop()
            self.sequencer.stopPlayback()
            self.csnd.loopPause()
            widget.set_icon_widget(self.playButtonImg)
            self.playing = False
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
            widget.set_icon_widget(self.stopButtonImg)
            self.playing = True


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
        if not self._playToolbar.playButton.get_active():
            self._playToolbar.playButton.set_active(True)

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
                             reverbSend = self.reverb,
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
        self.unpacker.reset(data)
        val = self.unpacker.unpack_int()
        if self.tempoSliderActive:
            self.delayedTempo = val
            return
        self.tempoAdjustment.handler_block( self.tempoAdjustmentHandler )
        self.tempoAdjustment.set_value( val )
        self._updateTempo( val )
        self.tempoAdjustment.handler_unblock( self.tempoAdjustmentHandler )
        self.sendSyncQuery()

    def processPR_SYNC_QUERY( self, sock, message, data ):
        self.packer.pack_float(self.nextHeartbeat())
        self.network.send( Net.HT_SYNC_REPLY, data + self.packer.get_buffer(), sock )
        self.packer.reset()

    def processPR_TEMPO_QUERY( self, sock, message, data ):
        self.packer.pack_int(self.tempo)
        self.network.send( Net.HT_TEMPO_UPDATE, self.packer.get_buffer(), to = sock )
        self.packer.reset()

    def processPR_REQUEST_TEMPO_CHANGE( self, sock, message, data ):
        if self.tempoSliderActive:
            return
        self.unpacker.reset(data)
        val = self.unpacker.unpack_int()
        self.tempoAdjustment.set_value( val )

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
            self.csnd.adjustTick(-err)


if __name__ == "__main__":
    MiniTamTam = miniTamTam()
    #start the gtk event loop
    gtk.main()
