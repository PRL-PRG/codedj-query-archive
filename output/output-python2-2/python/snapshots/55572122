import pygtk
pygtk.require( '2.0' )
import gtk

import gobject

from common.Util.ThemeWidgets import *
from common.Util.Profiler import TP
from common.Util import NoteDB
from common.Util.NoteDB import PARAMETER
from common.Util import ControlStream
from common.Util.CSoundClient import new_csound_client
from common.Util.InstrumentPanel import InstrumentPanel
from common.Util.InstrumentPanel import DrumPanel
from common.Util.CSoundNote import CSoundNote
from EditToolbars import mainToolbar
from EditToolbars import generateToolbar
from gettext import gettext as _
from subprocess import Popen
from sugar.graphics.palette import Palette, WidgetInvoker
import time
import os
import commands
import random

class CONTEXT:
    PAGE = 0
    TRACK = 1
    NOTE = 2

import common.Config as Config
#from SubActivity import SubActivity

from common.Generation.GenerationConstants import GenerationConstants
from Edit.Properties import Properties
from Edit.TrackInterface import TrackInterface, TrackInterfaceParasite
from Edit.TuneInterface import TuneInterface, TuneInterfaceParasite

from common.Generation.Generator import generator1, GenerationParameters

Tooltips = Config.Tooltips()
KEY_MAP_PIANO = Config.KEY_MAP_PIANO

#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.EventBox ):

    def __init__( self, activity ):
        gtk.EventBox.__init__(self)
        self.csnd = new_csound_client()
        self.tooltips = gtk.Tooltips()
        self.activity = activity
        for i in [6,7,8,9,10]:
            self.csnd.setTrackVolume(100, i)
        self.trackCount = 6

        self.scale = GenerationConstants.DEFAULT_SCALE

        # META ALGO: [section, variation or not, nPages] A B A C
        # TODO: Different parameters sets for each tracks
        self.tuneForm = [[0, False, 4], [1, False, 4], [0, True, 4], [2, False, 2]]
        
        def init_data( ):
            TP.ProfileBegin("init_data")
            self._data = {}

            #[ volume, ... ]
            self._data['track_volume'] = [ Config.DEFAULT_VOLUME ] * Config.NUMBER_OF_TRACKS
            self._data['track_mute']   = [ 1.0 ] * Config.NUMBER_OF_TRACKS

            #[ instrument index, ... ]
            self.trackInstrumentDefault = [
                    Config.INSTRUMENTS["kalimba"],
                    Config.INSTRUMENTS["kalimba"],
                    Config.INSTRUMENTS["kalimba"],
                    Config.INSTRUMENTS["kalimba"],
                    Config.INSTRUMENTS["drum2kit"] ]
            self.trackInstrument = self.trackInstrumentDefault[:]
            if len(self.trackInstrument) != Config.NUMBER_OF_TRACKS: raise 'error'
            self.drumIndex = Config.NUMBER_OF_TRACKS - 1

            #second instrument for melodic tracks
            self.trackInstrument2Default = [ None, None, None, None]
            self.trackInstrument2 = self.trackInstrument2Default[:]

            self._data['volume'] = Config.DEFAULT_VOLUME
            self._data['tempo'] = Config.PLAYER_TEMPO

            self.playScope = "Selection"
            self.displayedPage = -1
            self.trackSelected = [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]
            self.trackActive = [ 1 for i in range(Config.NUMBER_OF_TRACKS) ]

            self.pages_playing = []
            self.journalCalled = True

            self.noteDB = NoteDB.NoteDB()
            TP.ProfileEnd("init_data")

        def formatRoundBox( box, fillcolor ):
            box.set_radius( 7 )
            box.set_border_width( 1 )
            box.set_fill_color( fillcolor )
            box.set_border_color( Config.PANEL_BCK_COLOR )
            return box

        def init_GUI():

            self.GUI = {}
            self.GUI["2main"] = gtk.VBox()

            def draw_inst_icons():
                instrumentNames = [ k for k in Config.INSTRUMENTS.keys() if (k[0:4] != 'drum' and k[0:4] != 'guid') or Config.INSTRUMENTS[k].category == "kit" ]
                self.GUI["2instrumentIcons"] = {}
                for instrument in instrumentNames:
                    try:
                        self.GUI["2instrumentIcons"][instrument] = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + instrument + '.png')
                    except:
                        self.GUI["2instrumentIcons"][instrument] = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + 'generic.png')
            TP.ProfileBegin("init_GUI::instrument icons")
            draw_inst_icons()
            TP.ProfileEnd("init_GUI::instrument icons")


            #------------------------------------------------------------------------
            # page
            self.GUI["2page"] = gtk.HBox()
            self.GUI["2main"].pack_start( self.GUI["2page"], False )
            if 1: # + instrument panel
                self.GUI["2instrumentPanel"] = gtk.VBox()
                self.GUI["2instrumentPanel"].set_size_request( 132, -1 )
                self.GUI["2page"].pack_start( self.GUI["2instrumentPanel"], False )
                # + + instrument 1 box
                self.GUI["2instrument1Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument1Box"].set_size_request( -1, 132 )
                self.GUI["2instrument1volBox"] = gtk.VBox()
                self.GUI["2instrument1volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument1volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 0 )
                self.GUI["2instrument1volumeSlider"] = gtk.VScale(self.GUI["2instrument1volumeAdjustment"])
                self.GUI["2instrument1volumeSlider"].set_draw_value(False)
                self.GUI["2instrument1volumeSlider"].set_inverted(True)
                self.GUI["2instrument1volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument1volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 0 )
                self.GUI["2instrument1muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument1muteButton"].connect("toggled",self.handlemuteButton,0)
                self.GUI["2instrument1muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,0)
                self.GUI["2instrument1muteButton"].set_active(True)
                #self.GUI["2instrument1volBox"].pack_start( self.GUI["2instrument1volumeSlider"], True, True, 0 )
                #self.GUI["2instrument1volBox"].pack_start( self.GUI["2instrument1muteButton"], False, False, 5 )
                self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1volBox"], False, False, 0 )
                self.GUI["2instrument1Button"] = InstrumentButton( self, 0, Config.BG_COLOR )
                self.GUI["2instrument1Palette"] = instrumentPalette(_('Track 1 Volume'), 0, self)
                self.GUI["2instrument1Button"].set_palette(self.GUI["2instrument1Palette"])
                self.GUI["2instrument1Button"].setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[0].name] )
                self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1Button"], padding = 3 )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument1Box"] )
                # + + instrument 2 box
                self.GUI["2instrument2Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument2Box"].set_size_request( -1, 132 )
                self.GUI["2instrument2volBox"] = gtk.VBox()
                self.GUI["2instrument2volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument2volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 1 )
                self.GUI["2instrument2volumeSlider"] = gtk.VScale(self.GUI["2instrument2volumeAdjustment"])
                self.GUI["2instrument2volumeSlider"].set_draw_value(False)
                self.GUI["2instrument2volumeSlider"].set_inverted(True)
                self.GUI["2instrument2volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument2volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 1 )
                self.GUI["2instrument2muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument2muteButton"].connect("toggled",self.handlemuteButton,1)
                self.GUI["2instrument2muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,1)
                self.GUI["2instrument2muteButton"].set_active(True)
                #self.GUI["2instrument2volBox"].pack_start( self.GUI["2instrument2volumeSlider"], True, True, 0 )
                #self.GUI["2instrument2volBox"].pack_start( self.GUI["2instrument2muteButton"], False, False, 5 )
                self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2volBox"], False, False, 0 )
                self.GUI["2instrument2Button"] = InstrumentButton( self, 1, Config.BG_COLOR )
                self.GUI["2instrument2Palette"] = instrumentPalette(_('Track 2 Volume'), 1, self)
                self.GUI["2instrument2Button"].set_palette(self.GUI["2instrument2Palette"])
                self.GUI["2instrument2Button"].setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[1].name] )
                self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2Button"], padding = 3 )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument2Box"] )
                # + + instrument 3 box
                self.GUI["2instrument3Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument3Box"].set_size_request( -1, 132 )
                self.GUI["2instrument3volBox"] = gtk.VBox()
                self.GUI["2instrument3volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][2], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument3volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 2 )
                self.GUI["2instrument3volumeSlider"] = gtk.VScale(self.GUI["2instrument3volumeAdjustment"])
                self.GUI["2instrument3volumeSlider"].set_draw_value(False)
                self.GUI["2instrument3volumeSlider"].set_inverted(True)
                self.GUI["2instrument3volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument3volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 2 )
                self.GUI["2instrument3muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument3muteButton"].connect("toggled",self.handlemuteButton,2)
                self.GUI["2instrument3muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,2)
                self.GUI["2instrument3muteButton"].set_active(True)
                #self.GUI["2instrument3volBox"].pack_start( self.GUI["2instrument3volumeSlider"], True, True, 0 )
                #self.GUI["2instrument3volBox"].pack_start( self.GUI["2instrument3muteButton"], False, False, 5 )
                self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3volBox"], False, False, 0 )
                self.GUI["2instrument3Button"] = InstrumentButton( self, 2, Config.BG_COLOR )
                self.GUI["2instrument3Palette"] = instrumentPalette(_('Track 3 Volume'), 2, self)
                self.GUI["2instrument3Button"].set_palette(self.GUI["2instrument3Palette"])
                self.GUI["2instrument3Button"].setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[2].name] )
                self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3Button"], padding = 3 )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument3Box"] )
                # + + instrument 4 box
                self.GUI["2instrument4Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument4Box"].set_size_request( -1, 132 )
                self.GUI["2instrument4volBox"] = gtk.VBox()
                self.GUI["2instrument4volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][3], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument4volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 3 )
                self.GUI["2instrument4volumeSlider"] = gtk.VScale(self.GUI["2instrument4volumeAdjustment"])
                self.GUI["2instrument4volumeSlider"].set_draw_value(False)
                self.GUI["2instrument4volumeSlider"].set_inverted(True)
                self.GUI["2instrument4volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument4volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 3 )
                self.GUI["2instrument4muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument4muteButton"].connect("toggled",self.handlemuteButton,3)
                self.GUI["2instrument4muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,3)
                self.GUI["2instrument4muteButton"].set_active(True)
                #self.GUI["2instrument4volBox"].pack_start( self.GUI["2instrument4volumeSlider"], True, True, 0 )
                #self.GUI["2instrument4volBox"].pack_start( self.GUI["2instrument4muteButton"], False, False, 5 )
                self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4volBox"], False, False, 0 )
                self.GUI["2instrument4Button"] = InstrumentButton( self, 3, Config.BG_COLOR )
                self.GUI["2instrument4Palette"] = instrumentPalette(_('Track 4 Volume'), 3, self)
                self.GUI["2instrument4Button"].set_palette(self.GUI["2instrument4Palette"])
                self.GUI["2instrument4Button"].setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[3].name] )
                self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4Button"], padding = 3 )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument4Box"] )
                # + + drum box
                self.GUI["2drumBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2drumBox"].set_size_request( -1, 165 )
                self.GUI["2drumVolBox"] = gtk.VBox()
                self.GUI["2drumvolumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][4], 0, 100, 1, 1, 0 )
                #self.GUI["2drumvolumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 4 )
                self.GUI["2drumvolumeSlider"] = gtk.VScale(self.GUI["2drumvolumeAdjustment"])
                self.GUI["2drumvolumeSlider"].set_draw_value(False)
                self.GUI["2drumvolumeSlider"].set_inverted(True)
                self.GUI["2drumvolumeSlider"].set_size_request( 30, -1 )
                self.GUI["2drumvolumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 4 )
                self.GUI["2drumMuteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2drumMuteButton"].connect("toggled",self.handlemuteButton,4)
                self.GUI["2drumMuteButton"].connect("button-press-event",self.handlemuteButtonRightClick,4)
                self.GUI["2drumMuteButton"].set_active(True)
                #self.GUI["2drumVolBox"].pack_start( self.GUI["2drumvolumeSlider"], True, True, 0 )
                #self.GUI["2drumVolBox"].pack_start( self.GUI["2drumMuteButton"], False, False, 5 )
                self.GUI["2drumBox"].pack_start( self.GUI["2drumVolBox"], False, False, 0 )
                self.GUI["2drumButton"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[4].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[4].name + '.png')
                self.GUI["2drumPalette"] = instrumentPalette(_('Track 5 Volume'), 4, self)
                self.GUI["2drumButton"].set_palette(self.GUI["2drumPalette"])
                self.GUI["2drumButton"].connect("toggled", self.pickDrum)
                self.GUI["2drumButton"].connect('enter-notify-event', self.blockFocus)
                self.GUI["2drumButton"].connect('leave-notify-event', self.unblockFocus)                
                self.GUI["2drumBox"].pack_start( self.GUI["2drumButton"] )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2drumBox"] )
                self.GUI["2page"].pack_start( self.GUI["2instrumentPanel"], False )
                # + track interface
                self.trackInterface = TrackInterface( self.noteDB, self, self.getScale )
                self.noteDB.addListener( self.trackInterface, TrackInterfaceParasite, True )
                self.trackInterface.set_size_request( 1068, 693 )
                self.GUI["2page"].pack_start( self.trackInterface, False, False )

            #------------------------------------------------------------------------
            # tune interface
            if 1: # + tune interface
                self.GUI["2tuneHBox"] = RoundHBox( fillcolor = Config.TOOLBAR_BCK_COLOR, bordercolor = Config.TOOLBAR_BCK_COLOR, radius = 0 )
                self.GUI["2tuneScrollLeftButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditLeft.png", Config.IMAGE_ROOT+"arrowEditLeftDown.png", Config.IMAGE_ROOT+"arrowEditLeftOver.png", backgroundFill = Config.TOOLBAR_BCK_COLOR )
                self.GUI["2tuneScrollLeftButton"].set_size_request( 25, -1 )
                self.GUI["2tuneScrollLeftButton"].connect( "clicked", lambda a1:self.scrollTune( -1 ) )
                self.GUI["2tuneHBox"].pack_start( self.GUI["2tuneScrollLeftButton"], False, False )
                self.GUI["2tuneVBox"] = gtk.VBox()
                self.GUI["2tuneScrolledWindow"] = gtk.ScrolledWindow()
                self.GUI["2tuneScrolledWindow"].set_policy( gtk.POLICY_NEVER, gtk.POLICY_NEVER )
                self.tuneInterface = TuneInterface( self.noteDB, self, self.GUI["2tuneScrolledWindow"].get_hadjustment() )
                self.noteDB.addListener( self.tuneInterface, TuneInterfaceParasite, True )
                self.GUI["2tuneScrolledWindow"].add_with_viewport( self.tuneInterface )
                self.tuneInterface.get_parent().set_shadow_type( gtk.SHADOW_NONE )
                self.GUI["2tuneVBox"].pack_start( self.GUI["2tuneScrolledWindow"] )
                self.GUI["2tuneSlider"] = gtk.HScrollbar( self.GUI["2tuneScrolledWindow"].get_hadjustment() ) #ImageHScale( Config.IMAGE_ROOT+"sliderEditTempo.png", self.GUI["2tuneScrolledWindow"].get_hadjustment(), 6 )
                self.GUI["2tuneVBox"].pack_start( self.GUI["2tuneSlider"], False, False )
                self.GUI["2tuneHBox"].pack_start( self.GUI["2tuneVBox"] )
                self.GUI["2tuneScrollRightButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditRight.png", Config.IMAGE_ROOT+"arrowEditRightDown.png", Config.IMAGE_ROOT+"arrowEditRightOver.png", backgroundFill = Config.TOOLBAR_BCK_COLOR )
                self.GUI["2tuneScrollRightButton"].set_size_request( 25, -1 )
                self.GUI["2tuneScrollRightButton"].connect( "clicked", lambda a1:self.scrollTune( 1 ) )
                self.GUI["2tuneHBox"].pack_start( self.GUI["2tuneScrollRightButton"], False, False )
                self.GUI["2main"].pack_start( self.GUI["2tuneHBox"] )

            # set tooltips
            for key in self.GUI:
                if Tooltips.Edit.has_key(key):
                    self.tooltips.set_tip(self.GUI[key],Tooltips.Edit[key])

            self.add( self.GUI["2main"] )

            self.skipCleanup = "" # used when jumping between duplicate note/track


            # Popups
            TP.ProfileBegin("init_GUI::popups")
            # + instrument panel
            self.GUI["9instrumentPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            self.GUI["9instrumentPopup"].move( 400, 100 )
            self.GUI["9instrumentPopup"].resize( 800, 452 )
            self.GUI["9instrumentPopup"].set_modal(True)
            self.GUI["9instrumentPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            self.GUI["9instrumentPopup"].connect("button-release-event", lambda w,e:self.cancelInstrumentSelection() )
            # + drum panel
            TP.ProfileBegin("init_GUI::drumPanel")
            self.drumPanel = DrumPanel( self.donePickDrum )
            TP.ProfileEnd("init_GUI::drumPanel")
            self.GUI["9drumPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            self.GUI["9drumPopup"].move( 400, 100 )
            self.GUI["9drumPopup"].resize( 400, 100 )
            self.GUI["9drumPopup"].set_modal(True)
            self.GUI["9drumPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            self.GUI["9drumPopup"].connect("button-release-event", lambda w,e:self.cancelDrumSelection() )
            self.GUI["9drumPopup"].add( self.drumPanel )
            # + generation window
            #TP.ProfileBegin("init_GUI::generationPanel")
            #self.generationPanel = GenerationParametersWindow( self.generate, self.doneGenerationPopup )
            #TP.ProfileEnd("init_GUI::generationPanel")
            #self.GUI["9generationPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            #self.GUI["9generationPopup"].set_modal(True)
            #self.GUI["9generationPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            #self.GUI["9generationPopup"].connect("button-release-event", lambda w,e:self.doneGenerationPopup() )
            #self.GUI["9generationPopup"].add( self.generationPanel )
            # + properties window
            #self.GUI["9propertiesPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            #self.GUI["9propertiesPopup"].set_modal(True)
            #self.GUI["9propertiesPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            #self.GUI["9propertiesPopup"].connect("button-release-event", lambda w,e:self.donePropertiesPopup() )
            #TP.ProfileBegin("init_GUI::propertiesPanel")
            #self.propertiesPanel = Properties( self.noteDB, self.donePropertiesPopup, self.GUI["9propertiesPopup"] )
            #TP.ProfileEnd("init_GUI::propertiesPanel")
            #self.GUI["9propertiesPopup"].add( self.propertiesPanel )
            # + playback scope
            self.GUI["9loopPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            self.GUI["9loopPopup"].move( 100, 100 )
            self.GUI["9loopPopup"].resize( 300, 100 )
            self.GUI["9loopPopup"].set_modal(True)
            self.GUI["9loopPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            self.GUI["9loopPopup"].connect("button-release-event", lambda w,e:self.GUI["2loopButton"].set_active(False) )
            self.GUI["9loopBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
            self.GUI["9loopAllOnce"] = gtk.Button("AO")
            self.GUI["9loopBox"].pack_start( self.GUI["9loopAllOnce"] )
            self.GUI["9loopAllRepeat"] = gtk.Button("AR")
            self.GUI["9loopBox"].pack_start( self.GUI["9loopAllRepeat"] )
            self.GUI["9loopSelectedOnce"] = gtk.Button("SO")
            self.GUI["9loopBox"].pack_start( self.GUI["9loopSelectedOnce"] )
            self.GUI["9loopSelectedRepeat"] = gtk.Button("SR")
            self.GUI["9loopBox"].pack_start( self.GUI["9loopSelectedRepeat"] )
            self.GUI["9loopPopup"].add(self.GUI["9loopBox"])
            TP.ProfileEnd("init_GUI::popups")

        #===================================================
        # begin initialization
        #SubActivity.__init__( self, set_mode )

        # keyboard variables
        self.kb_record = False
        self.kb_keydict = {}

        # playback params
        self.playing = False
        self.playSource = 'Page'
        self.currentpageId = 0
        self.playingTuneIdx = 0

        # timers
        self.playbackTimeout = False

        # FPS stuff
        self.fpsTotalTime = 0
        self.fpsFrameCount = 0
        self.fpsN = 100 # how many frames to average FPS over
        self.fpsLastTime = time.time() # fps will be borked for the first few frames but who cares?

        self.context = -1 # invalidate
        self.contextTrackActive = False
        self.contextNoteActive = False

        init_data()   #above
        init_GUI()    #above

        # register for notification AFTER track and tune interfaces
        self.noteDB.addListener( self, page=True, note=True )

        self.csnd.setMasterVolume( self.getVolume() )
        self.initTrackVolume()

        for tid in range(Config.NUMBER_OF_TRACKS):
            self.handleInstrumentChanged( ( tid, self.trackInstrument[tid] ) )

        instrumentsIds = []
        for inst in self.trackInstrument:
            instrumentsIds.append(inst.instrumentId)

        first = self.noteDB.addPage( -1, NoteDB.Page(4, instruments = instrumentsIds) )
        self.displayPage( first )

        self.createNewTune( None )

        # Toolbar
        self.activity.activity_toolbar.keep.show()
        self._mainToolbar = mainToolbar(self.activity.toolbox, self)
        self._generateToolbar = generateToolbar(self.activity.toolbox, self)
        self.activity.toolbox.add_toolbar(_('Compose'), self._mainToolbar)
        self.activity.toolbox.add_toolbar(_('Generate'), self._generateToolbar)
        self.activity.toolbox.set_current_toolbar(1)
        self._mainToolbar.show()
        self._generateToolbar.show()

        self.show_all()  #gtk command

        self.setContext( CONTEXT.PAGE )

        self.audioRecordState = False

    def createNewTune( self, widget, data=None ):
        self.createNewTune3()

    def createNewTune3( self ):

        if self.playing == True:
            self.handleStop()

        self.tuneInterface.selectPages( self.noteDB.getTune() )

        beats = random.randint(3,8)
        stream = []
        for page in self.noteDB.getTune():
            stream += [ page, beats ]
        if len(stream):
            self.noteDB.updatePages( [ PARAMETER.PAGE_BEATS, len(stream)//2 ] + stream )

        orch = self.newOrchestra()

        instrumentsIds = []
        for inst in orch:
            instrumentsIds.append(inst.instrumentId)

        self.pageDelete( -1, instruments = instrumentsIds )

        initTempo = random.randint(60, 132)
        self._data['tempo'] = initTempo

        formsUsed = []
        for section in self.tuneForm:
            if section[0] not in formsUsed:
                param = self.chooseGenParams()
                self.tuneInterface.selectPages( self.noteDB.getTune() )
                if not formsUsed:
                    for i in range(section[2]-1):
                        self.pageAdd(instruments = instrumentsIds)
                else:
                    for i in range(section[2]):
                        self.pageAdd(instruments = instrumentsIds)
                formsUsed.append(section[0])

                self.tuneInterface.selectPages( self.noteDB.getTune()[-section[2]:] )
                self.generateMode = 'page'
                self.generate( GenerationParameters( density = param[0], rythmRegularity = param[1], step = param[2], pitchRegularity = param[3], articule = param[4], silence = param[5], pattern = param[6], scale = param[7]), section[2] )
            else:
                pageOffset = 0
                pageIds = []
                firstPos = [i[0] for i in self.tuneForm].index(section[0])
                if firstPos == 0:
                    pageOffset = 0
                else:
                    for i in range(firstPos):
                        pageOffset += self.tuneForm[i][2]
                for i in range(section[2]):
                    pageIds.append(self.noteDB.getTune()[pageOffset + i])
                after = self.noteDB.getTune()[-1]
                self.displayPage( self.noteDB.getTune()[pageOffset] )
                self.tuneInterface.selectPages(self.noteDB.getTune())
                self.pageDuplicate(-1, pageIds)

        self.tuneInterface.selectPages( self.noteDB.getTune() )
        self.displayPage( self.noteDB.getTune()[0] )


    def newOrchestra(self):
        stringsPickup = []
        windsPickup = []
        keyboardPickup = []
        fxPickup = []
        drumsPickup = ["drum1kit", "drum2kit", "drum3kit", "drum4kit", "drum5kit"]
        for name in Config.INSTRUMENTS.keys():
            if Config.INSTRUMENTS[name].category == 'strings' and Config.INSTRUMENTS[name].name != 'violin':
                stringsPickup.append(name)
            elif Config.INSTRUMENTS[name].category == 'winds' and Config.INSTRUMENTS[name].name != 'didjeridu':
                windsPickup.append(name)
            elif Config.INSTRUMENTS[name].category == 'keyboard' or Config.INSTRUMENTS[name].category == 'percussions':
                if Config.INSTRUMENTS[name].name != 'zap' and Config.INSTRUMENTS[name].name != 'cling':
                    keyboardPickup.append(name)
        return [
                    Config.INSTRUMENTS[random.choice(stringsPickup)],
                    Config.INSTRUMENTS[random.choice(stringsPickup)],
                    Config.INSTRUMENTS[random.choice(windsPickup)],
                    Config.INSTRUMENTS[random.choice(keyboardPickup)],
                    Config.INSTRUMENTS[random.choice(drumsPickup)] ]

    def chooseGenParams(self):
        choose = random.randint(0,4)
        density = GenerationConstants.RYTHM_DENSITY_BANK[choose]
        rytReg = GenerationConstants.RYTHM_REGU_BANK[choose]
        step = GenerationConstants.PITCH_STEP_BANK[choose]
        pitReg = GenerationConstants.PITCH_REGU_BANK[choose]
        dur = GenerationConstants.DURATION_BANK[choose]
        silence = GenerationConstants.SILENCE_BANK[choose]
        pattern = [random.choice([0,1,1,2,3,3]) for x in range(4)]
        scale = random.randint(0,6)
        return [density, rytReg, step, pitReg, dur, silence, pattern, scale]

    def onActivate( self, arg ):
        #SubActivity.onActivate( self,arg )
        # whatever needs to be done on initialization
        self.csnd.loopPause()
        self.csnd.loopClear()
        for n in self.noteDB.getNotes( ):
            self.csnd.loopPlay(n, 0) #adds all notes to c client in inactive state


    def onDeactivate( self ):
        #SubActivity.onDeactivate( self )
        # clean up things like popups etc
        self.releaseInstrumentPanel()
        self.csnd.loopPause()
        self.csnd.loopClear()

    def setInstrumentPanel( self, instrumentPanel ):
        instrumentPanel.configure( self.donePickInstrument, self.playInstrumentNote, enterMode = True )
        self.instrumentPanel = instrumentPanel
        self.GUI["9instrumentPopup"].add( self.instrumentPanel )

    def releaseInstrumentPanel( self ):
        self.GUI["9instrumentPopup"].remove( self.instrumentPanel )


    def updateFPS( self ):
        t = time.time()
        dt = t - self.fpsLastTime
        self.fpsLastTime = t
        self.fpsTotalTime += dt
        self.fpsFrameCount += 1
        if self.fpsFrameCount == self.fpsN:
            fps = self.fpsN/self.fpsTotalTime
            avgMS = 1000/fps
            fps = "FPS %d ms %.2f" % (fps, avgMS)
            #self.fpsText.set_text(fps )
            if (Config.DEBUG > 2):  print fps
            self.fpsTotalTime = 0
            self.fpsFrameCount = 0

    #=========================================================
    # Popup Windows

    def doneGenerationPopup( self ):
        if self.GUI["2pageGenerateButton"].get_active():
            self.GUI["2pageGenerateButton"].set_active( False )
        if self.GUI["2trackGenerateButton"].get_active():
            self.GUI["2trackGenerateButton"].set_active( False )

    def donePropertiesPopup( self ):
        if self.GUI["2pagePropertiesButton"].get_active():
            self.GUI["2pagePropertiesButton"].set_active( False )
        if self.GUI["2trackPropertiesButton"].get_active():
            self.GUI["2trackPropertiesButton"].set_active( False )
        if self.GUI["2notePropertiesButton"].get_active():
            self.GUI["2notePropertiesButton"].set_active( False )

    def cancelPopup( self, w, event, popup ):
        popup.hide()


    def handleLoopButton( self, w ):
        if w.get_active(): self.GUI["9loopPopup"].show_all()
        else: self.GUI["9loopPopup"].hide()

    #-----------------------------------
    # playback functions
    #-----------------------------------

    def updatePageSelection( self, selectedIds ):
        if not self.playing:
            return

        if self.playScope == "All":
            return

        if self.displayedPage in selectedIds:
            startPage = self.displayedPage
        else:
            startPage = selectedIds[0]

        self._playPages( selectedIds, startPage, self.trackInterface.getPlayhead() )

    def updatePagesPlaying( self ):
        if not self.playing:
            return

        curTick = self.csnd.loopGetTick()

        pageTick = self.page_onset[self.displayedPage]
        if curTick < pageTick:
            pageTick = 0
            ind = 0
        else:
            ind = self.pages_playing.index(self.displayedPage)

        localTick = curTick - pageTick

        self._playPages( self.tuneInterface.getSelectedIds(), ind, localTick )

    def handleAudioRecord( self, widget, data=None ):
        if widget.get_active() == True:
            chooser = gtk.FileChooserDialog(
                title='Save tune as Audio file',
                action=gtk.FILE_CHOOSER_ACTION_SAVE,
                buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))
            filter = gtk.FileFilter()
            filter.add_pattern('*.ogg')
            chooser.set_filter(filter)
            chooser.set_current_folder(Config.TUNE_DIR)

            for f in chooser.list_shortcut_folder_uris():
                chooser.remove_shortcut_folder_uri(f)

            if chooser.run() == gtk.RESPONSE_OK:
                if self.playing:
                    self.handleStop()
                else:
                    self.handleRewind()

                self.audioRecordState = True
                self.audioFileName = chooser.get_filename()
                if self.audioFileName[-4:] != '.ogg':
                    self.audioFileName += '.ogg'

                self.audioRecordTimeout = gobject.timeout_add( 500, self._startAudioRecord )
                self.audioRecordTick = -1
            chooser.destroy()
        else:
            self.audioRecordState = False

    def _startAudioRecord( self ):
        if not self.playing:
            self.handlePlay()
        return False

    def handlePlay( self, widget = None ):

        if widget:
            widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event

        if self.audioRecordState:
            self.csnd.inputMessage( "i5400 0 -1" )
            time.sleep( 0.01 )

        if self.playScope == "All":
            toPlay = self.noteDB.getTune()
        else:
            toPlay = self.tuneInterface.getSelectedIds()

        self._playPages( toPlay, self.displayedPage, self.trackInterface.getPlayhead() )

        self.playing = True

    def _playPages( self, pages, startPage, startTick ):

        self.pages_playing = pages[:]

        trackset = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackActive[i] ] )

        numticks = 0
        self.page_onset = {}
        for pid in self.pages_playing:
            self.page_onset[pid] = numticks
            numticks += self.noteDB.getPage(pid).ticks

        # check for a second instrument on melodic tracks
        stream = []
        for page in self.pages_playing:
            for track in trackset:
                if track != self.drumIndex:
                    if self.trackInstrument2[track] != None:
                        if len(self.noteDB.getNotesByTrack(page, track)):
                            stream += [ page, track, NoteDB.PARAMETER.INSTRUMENT2, len(self.noteDB.getNotesByTrack(page, track)) ]
                            for n in self.noteDB.getNotesByTrack(page, track):
                                stream += [ n.id, self.trackInstrument2[track].instrumentId ]
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

        self.csnd.loopClear()
        for page in self.pages_playing:
            for track in trackset:
                for n in self.noteDB.getNotesByTrack( page, track ):
                    self.csnd.loopPlay(n, 1)
                    self.csnd.loopUpdate(n, NoteDB.PARAMETER.ONSET, n.cs.onset + self.page_onset[n.page] , 1)

        self.csnd.loopSetNumTicks( numticks )

        self.csnd.loopSetTick( self.page_onset[startPage] + startTick )
        self.csnd.setTempo(self._data['tempo'])
        if (Config.DEBUG > 3): print "starting from tick", startTick, 'at tempo', self._data['tempo']
        self.csnd.loopStart()

        if not self.playbackTimeout:
            self.playbackTimeout = gobject.timeout_add( 50, self.onTimeout )



    def handleStop( self, widget = None, rewind = True ):

        if widget:
            widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event

        if self.audioRecordState:
            self.csnd.inputMessage( "i5401 4 1" )
            time.sleep( 0.01 )

        if self.playbackTimeout:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = False

        self.csnd.loopPause()
        self.csnd.loopDeactivate()

        if self.audioRecordState:
            time.sleep(4)
            self.csnd.__del__()
            time.sleep(0.5)
            self.audioRecordState = False
            command = "gst-launch-0.10 filesrc location=" + Config.PREF_DIR + "/perf.wav ! wavparse ! audioconvert ! vorbisenc ! oggmux ! filesink location=" + self.audioFileName
            command2 = "rm /home/olpc/.sugar/default/tamtam/perf.wav"
            (status, output) = commands.getstatusoutput(command)
            (status2, output2) = commands.getstatusoutput(command2)
            self.csnd.__init__()
            time.sleep(0.1)
            self.csnd.connect(True)
            time.sleep(0.1)
            self.waitToSet()
            self.csnd.load_instruments()
            self.GUI["2recordButton"].set_active(False)
        self.playing = False

        if rewind: self.handleRewind()

    def handleRewind( self, widget = None ):
        if self.playScope == "All": id = self.noteDB.getPageByIndex(0)
        else: id = self.tuneInterface.getFirstSelected()
        self.trackInterface.setPlayhead( 0 )
        self.displayPage( id )

    def handleClose(self,widget):
        self.activity.close()

    def onTimeout(self):
        self.updateFPS()

        curTick = self.csnd.loopGetTick()

        pageTick = self.page_onset[self.displayedPage]
        if curTick < pageTick:
            pageTick = 0
            ind = 0
        else:
            ind = self.pages_playing.index(self.displayedPage)

        localTick = curTick - pageTick
        pageLength = self.noteDB.getPage(self.pages_playing[ind]).ticks
        max = len(self.pages_playing)
        while localTick > pageLength:
            ind += 1
            if ind == max: ind = 0
            localTick -= pageLength
            pageLength = self.noteDB.getPage(self.pages_playing[ind]).ticks

        self.trackInterface.setPlayhead( localTick )

        if self.pages_playing[ind] != self.displayedPage:
            if ind + 1 < max: predraw = self.pages_playing[ind+1]
            else: predraw = self.pages_playing[0]
            self._displayPage( self.pages_playing[ind], predraw )
        else:
            self.trackInterface.predrawPage()

        if self.audioRecordState:
            if self.audioRecordTick > curTick: # we've looped around
                self.handleStop()
            else:
                self.audioRecordTick = curTick


        return True

    def onMuteTrack( self, widget, trackId ):
        self._data['track_mute'][trackId] = not self._data['track_mute'][trackId]
        #if self._data['track_mute'][trackId]:
            #self.noteLooper.setMute( trackId, 0.0 )
        #else:
            #self.noteLooper.setMute( trackId, 1.0 )

    def onTrackVolumeChanged( self, widget, trackId ):
        v =  widget.get_value() / 100.0
        self._data['track_volume'][trackId] = v
        #self.noteLooper.setVolume( trackId, v )

    def clearInstrument( self, id, primary = True ):
        btn = self.GUI["2instrument%dButton" % (id+1)]
        if primary:
            if self.trackInstrument2[id] == None:
                return
            self.handleInstrumentChanged( ( id, self.trackInstrument2[id] ), True )
            self.handleInstrumentChanged( ( id, None ), False )
            btn.setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[id].name] )
            btn.setSecondary( None )
        else:
            self.handleInstrumentChanged( ( id, None ), False )
            btn.setSecondary( None )
            pages = self.tuneInterface.getSelectedIds()
            self.noteDB.setInstrument2( pages, id, -1 )

    # data is tuple ( trackId, instrumentName )
    def handleInstrumentChanged( self, data, primary = True ):
        (id, instrument) = data
        if primary:
            self.trackInstrument[id] = instrument
        else:
            self.trackInstrument2[id] = instrument


        if primary: # TODO handle secondary instruments properly
            if (Config.DEBUG > 3): print "handleInstrumentChanged", id, instrument.name, primary

            pages = self.tuneInterface.getSelectedIds()
            self.noteDB.setInstrument( pages, id, instrument.instrumentId )

    def getScale(self):
        return self.scale

    def handleVolume( self, widget ):
        self._data["volume"] = round( widget.get_value() )
        self.csnd.setMasterVolume(self._data["volume"])
        img = min(3,int(4*self._data["volume"]/100)) # volume 0-3
        #self.GUI["2volumeImage"].set_from_file( Config.IMAGE_ROOT+"volume"+str(img)+".png" )

    def initTrackVolume( self ):
        for i in range(Config.NUMBER_OF_TRACKS):
            self.csnd.setTrackVolume(self._data["track_volume"][i], i)

    def handleTrackVolume( self, widget, track ):
        self._data["track_volume"][track] = round( widget.get_value() )
        self.csnd.setTrackVolume(self._data["track_volume"][track], track)

    def getTrackInstrument( self, track ):
        return self.trackInstrument[track]

    def getTrackVolume( self, track ):
        return self._data["track_volume"][track]

    def handleTempo( self, widget ):
        self._data['tempo'] = round( widget.get_value() )
        img = min(7,int(8*(self._data["tempo"]-widget.lower)/(widget.upper-widget.lower)))+1# tempo 1-8
        #self.GUI["2tempoImage"].set_from_file( Config.IMAGE_ROOT+"tempo"+str(img)+".png" )
        if self.playing:
            self.csnd.setTempo(self._data['tempo'])

    def handleToolClick( self, widget, mode ):
        if widget.get_active(): self.trackInterface.setInterfaceMode( mode )

    def getTool( self ):
        if self.GUI["2toolPointerButton"].get_active(): return "default"
        else: return "draw"

    def handleKeyboardRecordButton( self, widget, data=None ):
        self.kb_record = widget.get_active()

    def pickInstrument( self, widget, num, primary = True ):
        self.last_clicked_instTrackID = num
        self.last_clicked_instPrimary = primary
        self.instrumentPanel.selectFirstCat()
        if primary or self.trackInstrument2[num] == None:
            self.instrumentPanel.set_activeInstrument( self.trackInstrument[num].name, True )
        else:
            self.instrumentPanel.set_activeInstrument( self.trackInstrument2[num].name, True )
        winLoc = self.parent.window.get_position()
        alloc = widget.parent.get_allocation()
        x = alloc.x + alloc.width + winLoc[0]
        y = alloc.y + winLoc[1]
        self.GUI["9instrumentPopup"].move( x, y )
        self.GUI["9instrumentPopup"].show()

    def cancelInstrumentSelection( self ):
        self.GUI["9instrumentPopup"].hide()

    def donePickInstrument( self, instrumentName ):
        self.handleInstrumentChanged( (self.last_clicked_instTrackID, Config.INSTRUMENTS[instrumentName]), self.last_clicked_instPrimary )
        btn = self.GUI["2instrument%dButton" % (self.last_clicked_instTrackID+1)]
        if self.last_clicked_instPrimary:
            btn.setPrimary( self.GUI["2instrumentIcons"][instrumentName] )
        else:
            btn.setSecondary( self.GUI["2instrumentIcons"][instrumentName] )
        self.GUI["9instrumentPopup"].hide()


    def pickDrum( self, widget , data = None ):
        if widget.get_active(): # show the panel
            winLoc = self.parent.window.get_position()
            alloc = widget.get_allocation()
            x = alloc.x + alloc.width + winLoc[0]
            y = alloc.y + winLoc[1]
            self.drumPanel.set_activeInstrument( self.trackInstrument[Config.NUMBER_OF_TRACKS-1].name, True )
            self.GUI["9drumPopup"].move( x, y )
            self.GUI["9drumPopup"].show()
        else: # hide the panel
            self.GUI["9drumPopup"].hide()

    def cancelDrumSelection( self ):
        self.GUI["2drumButton"].set_active( False )

    def donePickDrum( self, drumName ):
        self.handleInstrumentChanged( ( self.drumIndex, Config.INSTRUMENTS[drumName] ) )
        self.GUI["2drumButton"].setImage( "main", self.GUI["2instrumentIcons"][drumName] )
        self.GUI["2drumButton"].setImage( "alt", self.GUI["2instrumentIcons"][drumName] )
        self.GUI["2drumButton"].set_active( False )

    def playInstrumentNote( self, instrumentName, secs_per_tick = 0.025):
        self.csnd.play(
                    CSoundNote( onset = 0,
                             pitch = 36,
                             amplitude = 1,
                             pan = 0.5,
                             duration = 20,
                             trackId = 1,
                             instrumentId = Config.INSTRUMENTS[instrumentName].instrumentId,
                             reverbSend = 0),
                    secs_per_tick)

    def handlemuteButton(self,widget,track):
        if widget.get_active():
            self.trackActive[track] = True
        else:
            self.trackActive[track] = False
        self.updatePagesPlaying()

    def handlemuteButtonRightClick(self,widget,event,track):
        if event.button == 3:
            widget.set_active(True)
            #if the other tracks are inactive
            if self.trackActive.count(False) == Config.NUMBER_OF_TRACKS - 1:
                for i in range(Config.NUMBER_OF_TRACKS):
                    if i == 4:
                        #self.GUI["2drumMuteButton"].set_active(True)
                        self.GUI["2drumPalette"].muteButton.set_active(True)
                    else:
                        #self.GUI["2instrument" + str(i+1) + "muteButton"].set_active(True)
                        self.GUI["2instrument" + str(i+1) + "Palette"].muteButton.set_active(True)
            else:
                for i in range(Config.NUMBER_OF_TRACKS):
                    if i != track:
                        if i == 4:
                            #self.GUI["2drumMuteButton"].set_active(False)
                            self.GUI["2drumPalette"].muteButton.set_active(False)
                        else:
                            #self.GUI["2instrument" + str(i+1) + "muteButton"].set_active(False)
                            self.GUI["2instrument" + str(i+1) + "Palette"].muteButton.set_active(False)
            self.updatePagesPlaying()
            
    def blockFocus(self, widget = None, data = None):
        self.activity.handler_block(self.activity.focusOutHandler)
        self.activity.handler_block(self.activity.focusInHandler)

    def unblockFocus(self, widget = None, data = None):
        self.activity.handler_unblock(self.activity.focusOutHandler)
        self.activity.handler_unblock(self.activity.focusInHandler)

    #-----------------------------------
    # generation functions
    #-----------------------------------

    def recompose( self, algo, params, nPagesCycle = 4):
        if self.generateMode == "track":
            if self.trackSelected == [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]:
                newtracks = set(range(Config.NUMBER_OF_TRACKS))
            else:
                newtracks = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ] )
            newpages  = self.tuneInterface.getSelectedIds()
        else: # page mode
            newtracks = set(range(Config.NUMBER_OF_TRACKS))
            newpages = self.tuneInterface.getSelectedIds()

        dict = {}
        for t in newtracks:
            dict[t] = {}
            for p in newpages:
                dict[t][p] = self.noteDB.getCSNotesByTrack( p, t )

        beatsOfPages = {}
        for pageId in newpages:
            beatsOfPages[pageId] = self.noteDB.pages[pageId].beats

        instruments = self.noteDB.getInstruments(newpages)

        #[ i.name for i in self.trackInstrument ],
        algo(
                params,
                self._data['track_volume'][:],
                instruments,
                self._data['tempo'],
                beatsOfPages,
                newtracks,
                newpages,
                dict, nPagesCycle)

        # filter & fix input ...WTF!?
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    intdur = int(note.duration)
                    note.duration = intdur
                    note.pageId = page
                    note.trackId = track

        # prepare the new notes
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += dict[tid][pid]

        # delete the notes and add the new
        self.noteDB.deleteNotesByTrack( newpages, newtracks )

        stream = []
        for page in newpages:
            for track in newtracks:
                stream += [ page, track, len(dict[track][page]) ]
                stream += dict[track][page]
        stream += [-1]
        self.noteDB.addNotes( stream )

    def generate( self, params, nPagesCycle = 4 ):
        self.recompose( generator1, params, nPagesCycle)

    #=======================================================
    # Clipboard Functions

    def getClipboardArea( self, page = -1 ):
        if page == -1: page = self.displayedPage
        ids = self.tuneInterface.getSelectedIds()
        return self.noteDB.getClipboardArea( ids.index(page) )

    def pasteClipboard( self, offset, trackMap ):
        pages = self.tuneInterface.getSelectedIds()
        instrumentMap = {}
        for t in trackMap:
            if t != trackMap[t]: instrumentMap[t] = self.trackInstrument[t].instrumentId
        return self.noteDB.pasteClipboard( pages, offset, trackMap, instrumentMap )

    def cleanupClipboard( self ):
        self.trackInterface.donePaste()


    #=======================================================
    # Note Functions

    def noteProperties( self, widget ):
        if widget.get_active():
            ids = self.trackInterface.getSelectedNotes()
            notes = { self.displayedPage: {} }
            for t in range(Config.NUMBER_OF_TRACKS):
                if len(ids[t]):
                    notes[self.displayedPage][t] = [ self.noteDB.getNote( self.displayedPage, t, id ) for id in ids[t] ]

            self.propertiesPanel.setContext("note", self.generationPanel.scale, notes = notes )
            winLoc = self.parent.window.get_position()
            balloc = self.GUI["2contextBox"].get_allocation()
            walloc = self.GUI["9propertiesPopup"].get_allocation()
            if walloc.height != 1: # hack to deal with showing the window before first allocation T_T
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 30, balloc.y - walloc.height + winLoc[1] )
            else:
                self.GUI["9propertiesPopup"].move(0, 2048) # off the screen
            self.GUI["9propertiesPopup"].show()
            if walloc.height == 1:
                walloc = self.GUI["9propertiesPopup"].get_allocation()
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 30, balloc.y - walloc.height + winLoc[1] )
        else:
            self.GUI["9propertiesPopup"].hide()

    def noteDelete( self ):
        ids = self.trackInterface.getSelectedNotes()
        stream = []
        for t in range(Config.NUMBER_OF_TRACKS):
            N = len(ids[t])
            if not N: continue
            stream += [ self.displayedPage, t, N ] + ids[t]
        if len(stream):
            self.noteDB.deleteNotes( stream + [-1] )

    def noteDuplicate( self ):
        ids = self.trackInterface.getSelectedNotes()
        stream = []
        for t in range(Config.NUMBER_OF_TRACKS):
            N = len(ids[t])
            if not N: continue
            stream += [ self.displayedPage, t, N ] + ids[t]
        if len(stream):
            self.skipCleanup = "note"
            self.skipCleanup = ""
            self.noteDB.notesToClipboard( stream + [-1] )
            self.trackInterface.setInterfaceMode("paste_notes")
            return True
        return False

    def noteDuplicateWidget( self, widget ):
        if widget.get_active():
            if self.noteDuplicate(): # duplicate succeeded
                return
            # cancel duplicate
            widget.set_active(False)
            self.trackInterface.setInterfaceMode("tool")
        else:
            self.trackInterface.setInterfaceMode("tool")

    def noteOnset( self, step ):
        self.trackInterface.noteStepOnset( step )

    def notePitch( self, step ):
        # TODO
        return

    def noteDuration( self, step ):
        # TODO
        return

    def noteVolume( self, step ):
        # TODO
        return

    #=======================================================
    # Track Functions

    def toggleTrack( self, trackN, exclusive ):
        if exclusive:
            for i in range(Config.NUMBER_OF_TRACKS):
                if self.trackSelected[i]:
                    self.trackSelected[i] = False
                    self.trackInterface.trackToggled( i )
                    self.tuneInterface.trackToggled( i )
            self.trackSelected[trackN] = True
            self.trackInterface.trackToggled( trackN )
            self.tuneInterface.trackToggled( trackN )
            self.setContextState( CONTEXT.TRACK, True )
            self.setContext( CONTEXT.TRACK )
        else:
            self.trackSelected[trackN] = not self.trackSelected[trackN]
            self.trackInterface.trackToggled( trackN )
            self.tuneInterface.trackToggled( trackN )
            trackSelected = False
            for i in range(Config.NUMBER_OF_TRACKS):
                if self.trackSelected[i]:
                    self.setContextState( CONTEXT.TRACK, True )
                    self.setContext( CONTEXT.TRACK )
                    trackSelected = True
                    break
            if not trackSelected:
                self.setContextState( CONTEXT.TRACK, False )

    def setTrack( self, trackN, state ):
        if self.trackSelected[trackN] != state:
            self.trackSelected[trackN] = state
            self.trackInterface.trackToggled( trackN )

    def clearTracks( self ):
        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackSelected[i]:
                self.trackSelected[i]= False
                self.trackInterface.trackToggled( i )
                self.tuneInterface.trackToggled( i )

        self.setContextState( CONTEXT.TRACK, False )

    def getTrackSelected( self, trackN ):
        return self.trackSelected[trackN]

    def trackGenerate( self, widget ):
        if widget.get_active():
            self.generateMode = "track"
            winLoc = self.parent.window.get_position()
            balloc = self.GUI["2contextBox"].get_allocation()
            walloc = self.GUI["9generationPopup"].get_allocation()
            if walloc.height != 1: # hack to make deal with showing the window before first allocation T_T
                self.GUI["9generationPopup"].move( balloc.x + winLoc[0], balloc.y - walloc.height + winLoc[1] )
            else:
                self.GUI["9generationPopup"].move(0, 2048) # off the screen
            self.GUI["9generationPopup"].show()
            if walloc.height == 1:
                walloc = self.GUI["9generationPopup"].get_allocation()
                self.GUI["9generationPopup"].move( balloc.x + winLoc[0], balloc.y - walloc.height + winLoc[1] )
        else:
            self.GUI["9generationPopup"].hide()


    def trackProperties( self, widget ):
        if widget.get_active():
            self.propertiesPanel.setContext( "track", self.generationPanel.scale, self.tuneInterface.getSelectedIds(), [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ] )
            winLoc = self.parent.window.get_position()
            balloc = self.GUI["2contextBox"].get_allocation()
            walloc = self.GUI["9propertiesPopup"].get_allocation()
            if walloc.height != 1: # hack to make deal with showing the window before first allocation T_T
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 30, balloc.y - walloc.height + winLoc[1] )
            else:
                self.GUI["9propertiesPopup"].move(0, 2048) # off the screen
            self.GUI["9propertiesPopup"].show()
            if walloc.height == 1:
                walloc = self.GUI["9propertiesPopup"].get_allocation()
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 30, balloc.y - walloc.height + winLoc[1] )
        else:
            self.GUI["9propertiesPopup"].hide()

    def trackDelete( self, pageIds = -1, trackIds = -1 ):

        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()
        if trackIds == -1: trackIds = [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ]

        self.noteDB.deleteNotesByTrack( pageIds, trackIds )

    def trackDuplicate( self, pageIds = -1, trackIds = -1 ):

        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()
        if trackIds == -1: trackIds = [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ]

        if len(trackIds):
            self.skipCleanup = "track"
            self.skipCleanup = ""
            self.noteDB.tracksToClipboard( pageIds, trackIds )
            self.trackInterface.setInterfaceMode("paste_tracks")
            return True
        return False

    def trackDuplicateWidget( self, widget ):
        if widget.get_active():
            if self.trackDuplicate(): # duplicate succeeded
                return
            # cancel duplicate
            widget.set_active(False)
            self.trackInterface.setInterfaceMode("tool")
        else:
            self.trackInterface.setInterfaceMode("tool")

    #-----------------------------------
    # tune/page functions
    #-----------------------------------

    def scrollTune( self, direction ):
        adj = self.GUI["2tuneScrolledWindow"].get_hadjustment()
        if direction > 0:
            adj.set_value( min( adj.value + Config.PAGE_THUMBNAIL_WIDTH, adj.upper - adj.page_size ) )
        else:
            adj.set_value( max( adj.value - Config.PAGE_THUMBNAIL_WIDTH, 0) )

    def displayPage( self, pageId, nextId = -1 ):
        if self.playing:
            if self.displayedPage != pageId and pageId in self.pages_playing:
                self.csnd.loopSetTick( self.page_onset[pageId] )

        self._displayPage( pageId, nextId )


    # only called locally!
    def _displayPage( self, pageId, nextId = -1 ):

        self.displayedPage = pageId

        page = self.noteDB.getPage(pageId)
        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackInstrument[i].instrumentId != page.instruments[i]:
                self.trackInstrument[i] = Config.INSTRUMENTSID[page.instruments[i]]
                if i == Config.NUMBER_OF_TRACKS-1:
                    btn = self.GUI["2drumButton"]
                    btn.setImage( "main", self.GUI["2instrumentIcons"][self.trackInstrument[i].name] )
                    btn.setImage( "alt", self.GUI["2instrumentIcons"][self.trackInstrument[i].name] )
                else:
                    btn = self.GUI["2instrument%dButton"%(i+1)]
                    btn.setPrimary( self.GUI["2instrumentIcons"][self.trackInstrument[i].name] )
                    if self.trackInstrument2[i] != None:
                        btn.setSecondary( self.GUI["2instrumentIcons"][self.trackInstrument2[i].name] )
                    else:
                        btn.setSecondary( None )
        self.tuneInterface.displayPage( pageId )
        self.trackInterface.displayPage( pageId, nextId )

    def predrawPage( self, pageId ):
        if self.playbackTimeout: return # we're playing, predrawing is already handled
        if self.trackInterface.setPredrawPage( pageId ): # page needs to be drawn
            self.trackInterface.predrawPage()

    def abortPredrawPage( self ):
        self.trackInterface.abortPredrawPage()

    def pageGenerate( self, widget ):
        if widget.get_active():
            self.generateMode = "page"
            winLoc = self.parent.window.get_position()
            balloc = self.GUI["2contextBox"].get_allocation()
            walloc = self.GUI["9generationPopup"].get_allocation()
            if walloc.height != 1: # hack to make deal with showing the window before first allocation T_T
                self.GUI["9generationPopup"].move( balloc.x + winLoc[0], balloc.y - walloc.height + winLoc[1] )
            else:
                self.GUI["9generationPopup"].move(0, 2048) # off the screen
            self.GUI["9generationPopup"].show()
            if walloc.height == 1:
                walloc = self.GUI["9generationPopup"].get_allocation()
                self.GUI["9generationPopup"].move( balloc.x + winLoc[0], balloc.y - walloc.height + winLoc[1] )
        else:
            self.GUI["9generationPopup"].hide()

    def setPageGenerateMode(self, mode):
        self.generateMode = mode

    def pageProperties( self, widget ):
        if widget.get_active():
            self.propertiesPanel.setContext( "page", self.generationPanel.scale, self.tuneInterface.getSelectedIds() )
            winLoc = self.parent.window.get_position()
            balloc = self.GUI["2contextBox"].get_allocation()
            walloc = self.GUI["9propertiesPopup"].get_allocation()
            if walloc.height != 1: # hack to make deal with showing the window before first allocation T_T
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 100, balloc.y - walloc.height + winLoc[1] )
            else:
                self.GUI["9propertiesPopup"].move(0, 2048) # off the screen
            self.GUI["9propertiesPopup"].show()
            if walloc.height == 1:
                walloc = self.GUI["9propertiesPopup"].get_allocation()
                self.GUI["9propertiesPopup"].move( balloc.x + winLoc[0] - 100, balloc.y - walloc.height + winLoc[1] )
        else:
            self.GUI["9propertiesPopup"].hide()

    def pageDelete( self, pageIds = -1, instruments = False ):

        if pageIds == -1:
            pageIds = self.tuneInterface.getSelectedIds()

        if instruments == False:
            instruments = []
            for inst in self.trackInstrument:
                instruments.append(inst.instrumentId)

        self.noteDB.deletePages( pageIds[:], instruments )

    def pageDuplicate( self, after = -1, pageIds = False ):

        if after == -1: after = self.tuneInterface.getLastSelected()
        if not pageIds: pageIds = self.tuneInterface.getSelectedIds()

        new = self.noteDB.duplicatePages( pageIds[:], after )
        self.displayPage( new[self.displayedPage] )
        self.tuneInterface.selectPages( new.values() )

    def pageAdd( self, after = -1, beats = False, color = False, instruments = False ):

        if after == -1: after = self.tuneInterface.getLastSelected()
        page = self.noteDB.getPage( self.displayedPage )
        if not beats: beats = page.beats
        if not color: color = page.color
        if not instruments: instruments = page.instruments

        # TODO think about network mode here...
        self.displayPage( self.noteDB.addPage( -1, NoteDB.Page(beats,color,instruments), after ) )

    def pageBeats( self, pageIds = -1 ):

        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()

        # TODO change the beats

    #=======================================================
    # NoteDB notifications

    def notifyPageAdd( self, id, at ):
        return

    def notifyPageDelete( self, which, safe ):
        if self.displayedPage in which:
            self.displayPage( safe )

    def notifyPageDuplicate( self, new, at ):
        return

    def notifyPageMove( self, which, low, high ):
        return

    def notifyPageUpdate( self, page, parameter, value ):
        pass

    def notifyNoteAdd( self, page, track, id ):
        if (Config.DEBUG > 3) : print 'INFO: adding note to loop', page, track, id
        n = self.noteDB.getNote(page, track, id)
        self.csnd.loopPlay(n,0)
        if self.playing and (n.page in self.page_onset ):
            onset = n.cs.onset + self.page_onset[n.page]
            self.csnd.loopUpdate(n, NoteDB.PARAMETER.ONSET, onset, 1) #set onset + activate

    def notifyNoteDelete( self, page, track, id ):
        if (Config.DEBUG > 3) : print 'INFO: deleting note from loop', page, track, id
        self.csnd.loopDelete1(page,id)
    def notifyNoteUpdate( self, page, track, id, parameter, value ):
        if (Config.DEBUG > 3) : print 'INFO: updating note ', page, id, parameter, value
        note = self.noteDB.getNote(page, track, id)
        self.csnd.loopUpdate(note, parameter, value, -1)

    #-----------------------------------
    # load and save functions
    #-----------------------------------

    def waitToSet(self):
        self.csnd.setMasterVolume(self._data['volume'])
        self.csnd.setTempo(self._data['tempo'])
        self.initTrackVolume()

    def handleSave(self, widget = None):

        chooser = gtk.FileChooserDialog(
                title='Save Tune',
                action=gtk.FILE_CHOOSER_ACTION_SAVE,
                buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))
        filter = gtk.FileFilter()
        filter.add_pattern('*.tam')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.TUNE_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            ofilename = chooser.get_filename()
            if ofilename[-4:] != '.tam':
                ofilename += '.tam'
            try:
                ofile = open(ofilename, 'w')
                ofilestream = ControlStream.TamTamOStream (ofile)
                self.noteDB.dumpToStream(ofilestream)
                ofilestream.track_vol(self._data['track_volume'])
                ofilestream.master_vol(self._data['volume'])
                ofilestream.tempo(self._data['tempo'])
                ofile.close()
            except OSError,e:
                print 'ERROR: failed to open file %s for writing\n' % ofilename
        chooser.destroy()

    def handleJournalSave(self, file_path):
        ofile = open(file_path, 'w')
        ofilestream = ControlStream.TamTamOStream (ofile)
        self.noteDB.dumpToStream(ofilestream)
        ofilestream.track_vol(self._data['track_volume'])
        ofilestream.master_vol(self._data['volume'])
        ofilestream.tempo(self._data['tempo'])
        ofile.close()

    def _loadFile( self, path ):
        try:
            oldPages = self.noteDB.getTune()[:]

            ifile = open(path, 'r')
            ttt = ControlStream.TamTamTable ( self.noteDB )
            ttt.parseFile(ifile)
            self.trackInstrument = self.trackInstrumentDefault[:] # these will get set correctly in displayPage
            self._data['track_volume'] = ttt.tracks_volume
            self._data['volume'] = float(ttt.masterVolume)
            self._data['tempo'] = float(ttt.tempo)
            #self.GUI["2volumeAdjustment"].set_value(self._data['volume'])
            #self.GUI["2tempoAdjustment"].set_value(self._data['tempo'])
            for i in range(Config.NUMBER_OF_TRACKS):
                if i == 4:
                    string = '2drumvolumeAdjustment'
                else:
                    string = '2instrument' + str(i+1) + 'volumeAdjustment'
                self.GUI[string].set_value(self._data['track_volume'][i])
            ifile.close()

            self.noteDB.deletePages( oldPages )

            self.tuneInterface.selectPages( self.noteDB.getTune() )
        except OSError,e:
            print 'ERROR: failed to open file %s for reading\n' % ofilename

    def handleLoad(self, widget):
        chooser = gtk.FileChooserDialog(
                title='Load Tune',
                action=gtk.FILE_CHOOSER_ACTION_OPEN,
                buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        filter = gtk.FileFilter()
        filter.add_pattern('*.tam')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.TUNE_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            print 'DEBUG: loading file: ', chooser.get_filename()
            self._loadFile( chooser.get_filename() )

        chooser.destroy()
        self.delay = gobject.timeout_add(1000, self.waitToSet)

    def handleJournalLoad(self,file_path):
        self.journalCalled = True
        self._loadFile( file_path )

    #-----------------------------------
    # Record functions
    #-----------------------------------
    def handleMicRecord( self, widget, data ):
        self.csnd.micRecording( data )
    def handleCloseMicRecordWindow( self, widget = None, data = None ):
        self.micRecordWindow.destroy()
        self.micRecordButton.set_active( False )

    #-----------------------------------
    # callback functions
    #-----------------------------------
    def handleKeyboardShortcuts(self,event):
        keyval = event.keyval

        # backspace and del keys
        if keyval == gtk.keysyms.Delete or keyval == gtk.keysyms.BackSpace:
            if self.context == CONTEXT.PAGE: self.pageDelete()
            if self.context == CONTEXT.TRACK: self.trackDelete()
            if self.context == CONTEXT.NOTE: self.noteDelete()
        # plus key
        if keyval == gtk.keysyms.equal:
            self.pageAdd()
        # duplicate ctrl-c
        if event.state == gtk.gdk.CONTROL_MASK and keyval == gtk.keysyms.c:
            if self.context == CONTEXT.PAGE: self.pageDuplicate()
            if self.context == CONTEXT.TRACK: self.trackDuplicate()
            if self.context == CONTEXT.NOTE: self.noteDuplicate()
        #Arrows
        if event.state == gtk.gdk.SHIFT_MASK:
            # up/down arrows volume
            if keyval == gtk.keysyms.Up: self.trackInterface.noteStepVolume(0.1)
            if keyval == gtk.keysyms.Down: self.trackInterface.noteStepVolume(-0.1)
            # left/right arrows onset
            if keyval == gtk.keysyms.Left: self.trackInterface.noteStepDuration(-1)
            if keyval == gtk.keysyms.Right: self.trackInterface.noteStepDuration(1)
        else:
            # up/down arrows pitch
            if keyval == gtk.keysyms.Up: self.trackInterface.noteStepPitch(1)
            if keyval == gtk.keysyms.Down: self.trackInterface.noteStepPitch(-1)
            # left/right arrows duration
            if keyval == gtk.keysyms.Left: self.trackInterface.noteStepOnset(-1)
            if keyval == gtk.keysyms.Right: self.trackInterface.noteStepOnset(1)
        #Save Loop
        if event.state == gtk.gdk.CONTROL_MASK and keyval == gtk.keysyms.s:
            self.handleSave()


    def onKeyPress(self,widget,event):
        self.handleKeyboardShortcuts(event)
        Config.ModKeys.keyPress( event.hardware_keycode )
        key = event.hardware_keycode

        # If the key is already in the dictionnary, exit function (to avoir key repeats)
        if self.kb_keydict.has_key(key):
                return

        # Assign on which track the note will be created according to the number of keys pressed
        if self.trackCount >= 9:
            self.trackCount = 6
        fakeTrack = self.trackCount
        self.trackCount += 1

        # If the pressed key is in the keymap
        if KEY_MAP_PIANO.has_key(key):
            pitch = KEY_MAP_PIANO[key]
            duration = -1

            # get instrument from top selected track if a track is selected
            if True in self.trackSelected:
                index = self.trackSelected.index(True)
                instrument = self.getTrackInstrument(index).name
            else:
                return

            tid = index

            # pitch, inst and duration for drum recording
            if tid == Config.NUMBER_OF_TRACKS-1:
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    pitch = GenerationConstants.DRUMPITCH[pitch]
                if Config.INSTRUMENTS[instrument].kit != None:
                    instrument = Config.INSTRUMENTS[instrument].kit[pitch].name
                duration = 100

            # Create and play the note
            self.kb_keydict[key] = CSoundNote(onset = 0,
                                        pitch = pitch,
                                        amplitude = 1,
                                        pan = 0.5,
                                        duration = duration,
                                        trackId = fakeTrack,
                                        instrumentId = Config.INSTRUMENTS[instrument].instrumentId,
                                        tied = False,
                                        mode = 'edit')
            self.csnd.play(self.kb_keydict[key], 0.3)

            # doesn't keep track of keys for drum recording
            if tid == Config.NUMBER_OF_TRACKS-1:
                del self.kb_keydict[key]

            # remove previosly holded key from dictionary
            if len(self.kb_keydict) > 1:
                for k in self.kb_keydict.keys():
                    if k != key:
                        gobject.source_remove( self.durUpdate )
                        self.durUpdate = False
                        self.kb_keydict[k].duration = 0.5
                        self.kb_keydict[k].amplitude = 0
                        self.kb_keydict[k].decay = 0.7
                        self.kb_keydict[k].tied = False
                        self.csnd.play(self.kb_keydict[k], 0.3)
                        if not self.kb_record:
                            del self.kb_keydict[k]
                            return
                        self.removeRecNote(self.csId)

            if not self.kb_record:
                return

            #record the note on track
            pageList = self.tuneInterface.getSelectedIds()
            pid = self.displayedPage
            minOnset = self.page_onset[pid]
            onsetQuantized = Config.DEFAULT_GRID * int((self.csnd.loopGetTick() - minOnset) / Config.DEFAULT_GRID + 0.5)

            maxOnset = self.noteDB.getPage(pid).ticks
            if onsetQuantized >= maxOnset:
                if pid == pageList[-1]:
                    pid = pageList[0]
                else:
                    if len(pageList) > 1:
                        pidPos = pageList.index(pid)
                        pid = pageList[pidPos+1]
                onsetQuantized = 0

            if tid < Config.NUMBER_OF_TRACKS-1:
                for n in self.noteDB.getNotesByTrack( pid, tid ):
                    if onsetQuantized < n.cs.onset:
                        break
                    if onsetQuantized >= n.cs.onset + n.cs.duration:
                        continue
                    if onsetQuantized < n.cs.onset + n.cs.duration - 2:
                        self.noteDB.deleteNote(n.page, n.track, n.id)
                    elif onsetQuantized - n.cs.onset < 1:
                        self.noteDB.deleteNote(n.page, n.track, n.id)
                    else:
                        self.noteDB.updateNote( n.page, n.track, n.id, PARAMETER.DURATION, onsetQuantized - n.cs.onset )
                    break
            else:
                for n in self.noteDB.getNotesByTrack( pid, tid ):
                    if onsetQuantized < n.cs.onset:
                        break
                    if onsetQuantized == n.cs.onset:
                        if pitch < n.cs.pitch:
                            break
                        if pitch == n.cs.pitch:
                            return # don't bother with a new note

            csnote = CSoundNote(onset = 0,
                                        pitch = pitch,
                                        amplitude = 1,
                                        pan = 0.5,
                                        duration = duration,
                                        trackId = index,
                                        instrumentId = Config.INSTRUMENTS[instrument].instrumentId,
                                        tied = False,
                                        mode = 'edit')

            csnote.onset = onsetQuantized
            csnote.duration = 1
            csnote.pageId = pid
            id = self.noteDB.addNote(-1, pid, tid, csnote)
            # csId: PageId, TrackId, Onset, Key, DurationSetOnce
            self.csId = [pid, tid, id, csnote.onset, key, False ]
            if tid < Config.NUMBER_OF_TRACKS-1:
                self.durUpdate = gobject.timeout_add( 25, self.durationUpdate )

    def onKeyRelease(self,widget,event):
        Config.ModKeys.keyRelease( event.hardware_keycode )
        key = event.hardware_keycode

        if True in self.trackSelected:
            index = self.trackSelected.index(True)
            if index == Config.NUMBER_OF_TRACKS-1:
                return
        else:
            return

        if KEY_MAP_PIANO.has_key(key) and self.kb_keydict.has_key(key):
            if self.kb_record and self.durUpdate:
                gobject.source_remove( self.durUpdate )
                self.durUpdate = False

            if Config.INSTRUMENTSID[ self.kb_keydict[key].instrumentId ].csoundInstrumentId == Config.INST_TIED:
                self.kb_keydict[key].duration = 0.5
                self.kb_keydict[key].amplitude = 0
                self.kb_keydict[key].decay = 0.5
                self.kb_keydict[key].tied = False
                self.csnd.play(self.kb_keydict[key], 0.3)
            if not self.kb_record:
                del self.kb_keydict[key]
                return

            self.removeRecNote(self.csId)

    def removeRecNote(self, csId):
        newDuration = (int(self.csnd.loopGetTick()) - self.page_onset[csId[0]]) - csId[3]
        maxTick = self.noteDB.getPage(csId[0]).ticks

        if not csId[5]: # handle notes that were created right at the end of a page
            if newDuration > maxTick//2:
                newDuration = 1
            else:
                csId[5] = True

        if newDuration < -Config.DEFAULT_GRID_DIV2: # we looped around
            newDuration = maxTick - self.csId[3]
        elif newDuration < 1:
            newDuration = 1

        if (csId[3] + newDuration) > maxTick:
            newDuration = maxTick - csId[3]

        for n in self.noteDB.getNotesByTrack( csId[0], csId[1] ):
            if n.id == csId[2]:
                continue
            if csId[3] + newDuration <= n.cs.onset:
                break
            if csId[3] >= n.cs.onset + n.cs.duration:
                continue
            self.noteDB.deleteNote(n.page, n.track, n.id)
            break

        self.noteDB.updateNote( csId[0], csId[1], csId[2], PARAMETER.DURATION, newDuration)

        del self.kb_keydict[csId[4]]

    def durationUpdate(self):
        newDuration = (int(self.csnd.loopGetTick()) - self.page_onset[self.csId[0]]) - self.csId[3]

        maxTick = self.noteDB.getPage(self.csId[0]).ticks
        stop = False

        if not self.csId[5]: # handle notes that were created right at the end of a page
            if newDuration > maxTick//2:
                newDuration = 1
            else:
                self.csId[5] = True

        if newDuration < -Config.DEFAULT_GRID_DIV2: # we looped around
            newDuration = maxTick - self.csId[3]
            stop = True
        elif newDuration < 1:
            newDuration = 1

        if (self.csId[3] + newDuration) > maxTick:
            stop = True
            newDuration = maxTick - self.csId[3]

        for n in self.noteDB.getNotesByTrack( self.csId[0], self.csId[1] ):
            if n.id == self.csId[2]:
                continue
            if self.csId[3] + newDuration <= n.cs.onset:
                break
            if self.csId[3] >= n.cs.onset + n.cs.duration:
                continue
            self.noteDB.deleteNote(n.page, n.track, n.id)
            break

        self.noteDB.updateNote( self.csId[0], self.csId[1], self.csId[2], PARAMETER.DURATION, newDuration)

        if stop:
            key = self.csId[4]
            if Config.INSTRUMENTSID[ self.kb_keydict[key].instrumentId ].csoundInstrumentId == Config.INST_TIED:
                self.kb_keydict[key].duration = 0.5
                self.kb_keydict[key].amplitude = 0
                self.kb_keydict[key].decay = 0.5
                self.kb_keydict[key].tied = False
                self.csnd.play(self.kb_keydict[key], 0.3)

            del self.kb_keydict[key]
            return False
        return True

    def delete_event( self, widget, event, data = None ):
        return False

    def onDestroy( self ):

        if (Config.DEBUG > 1): print TP.PrintAll()

    def setContextState( self, context, state ):
        if context == CONTEXT.TRACK:
            self.contextTrackActive = state
            if not state:
                if self.context == CONTEXT.TRACK:
                    if self.contextNoteActive:
                        self.setContext( CONTEXT.NOTE )
                    else:
                        self.setContext( CONTEXT.PAGE )
        else:
            self.contextNoteActive = state
            if not state:
                if self.context == CONTEXT.NOTE:
                    self.prevContext()

    def setContext( self, context, force = False ):

        if self.context == context and not force: return

        self.context = context

        if self.context == CONTEXT.NOTE:
            self._generateToolbar.generationButton.set_sensitive(False)
        else:
            self._generateToolbar.generationButton.set_sensitive(True)

    def getContext(self):
        return self.context

    def prevContext( self ):
        if self.context == CONTEXT.TRACK:
            self.setContext( CONTEXT.PAGE )
        elif self.contextTrackActive:
            self.setContext( CONTEXT.TRACK )
        else:
            self.setContext( CONTEXT.PAGE )

    def nextContext( self ):
        if self.context == CONTEXT.TRACK:
            self.setContext( CONTEXT.NOTE )
        elif self.contextTrackActive:
            self.setContext( CONTEXT.TRACK )
        else:
            self.setContext( CONTEXT.NOTE )

    #-----------------------------------
    # access functions (not sure if this is the best way to go about doing this)
    #-----------------------------------
    def getVolume( self ):
        return self._data["volume"]

    def getTempo( self ):
        return self._data["tempo"]
        #return round( self.tempoAdjustment.value, 0 )

    def getBeatsPerPage( self ):
        return int(round( self.beatsPerPageAdjustment.value, 0 ))

    def getWindowTitle( self ):
        return "Tam-Tam [Volume %i, Tempo %i, Beats/Page %i]" % ( self.getVolume(), self.getTempo(), self.getBeatsPerPage() )


class InstrumentButton( gtk.DrawingArea ):

    def __init__( self, owner, index, backgroundFill ):
        gtk.DrawingArea.__init__( self )

        self.index = index
        self.owner = owner

        self.win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( self.win )

        colormap = self.get_colormap()
        self.color = { "background":   colormap.alloc_color( backgroundFill, True, True ),
                       "divider":      colormap.alloc_color( "#000", True, True ),
                       "+/-":          colormap.alloc_color( Config.FG_COLOR, True, True ),
                       "+/-Highlight": colormap.alloc_color( "#FFF", True, True ) }

        self.pixmap = None
        self.primary = None
        self.primaryWidth = self.primaryHeight = 1
        self.secondary = None
        self.secondaryWidth = self.secondaryHeight = 1

        self.clicked = None
        self.hover = None

        self.add_events( gtk.gdk.BUTTON_PRESS_MASK
                       | gtk.gdk.BUTTON_RELEASE_MASK
                       | gtk.gdk.POINTER_MOTION_MASK
                       | gtk.gdk.POINTER_MOTION_HINT_MASK
                       | gtk.gdk.LEAVE_NOTIFY_MASK 
                       | gtk.gdk.ENTER_NOTIFY_MASK )
        self.connect( "size-allocate", self.size_allocate )
        self.connect( "button-press-event", self.button_press )
        self.connect( "button-release-event", self.button_release )
        self.connect( "motion-notify-event", self.motion_notify )
        self.connect( "enter-notify-event", self.enter_notify )
        self.connect( "leave-notify-event", self.leave_notify )
        self.connect( "expose-event", self.expose )

    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
        self.pixmap = gtk.gdk.Pixmap( self.win, allocation.width, allocation.height )
        self.primaryX = (self.alloc.width - self.primaryWidth) // 2
        self.primaryY = (self.alloc.height - self.primaryHeight) // 2
        self.secondaryX = (self.alloc.width - self.secondaryWidth) // 2
        self.secondaryY = self.alloc.height//2

        self.hotspots = [ [ self.alloc.width-24, self.alloc.height-29, self.alloc.width-8, self.alloc.height-13 ],
                          [ self.alloc.width-24, self.alloc.height//2-23, self.alloc.width-8, self.alloc.height//2-7 ] ]

        self.hotspots[0] += [ (self.hotspots[0][0]+self.hotspots[0][2])//2, (self.hotspots[0][1]+self.hotspots[0][3])//2 ]
        self.hotspots[1] += [ (self.hotspots[1][0]+self.hotspots[1][2])//2, (self.hotspots[1][1]+self.hotspots[1][3])//2 ]

        self._updatePixmap()

    def button_press( self, widget, event ):

        self.clicked = "PRIMARY"
        self.hover = None

        if     event.x >= self.hotspots[0][0] and event.x <= self.hotspots[0][2] \
           and event.y >= self.hotspots[0][1] and event.y <= self.hotspots[0][3]:
            self.clicked = "HOTSPOT_0"

        elif self.secondary != None:

            if     event.x >= self.hotspots[1][0] and event.x <= self.hotspots[1][2] \
               and event.y >= self.hotspots[1][1] and event.y <= self.hotspots[1][3]:
                self.clicked = "HOTSPOT_1"

            elif event.y > self.alloc.height//2:
                self.clicked = "SECONDARY"

    def button_release( self, widget, event ):
        if self.clicked == "PRIMARY":
            self.owner.pickInstrument( self, self.index, True )
        elif self.clicked == "SECONDARY":
            self.owner.pickInstrument( self, self.index, False )
        elif self.clicked == "HOTSPOT_0":
            if self.secondary != None: # remove secondary
                self.owner.clearInstrument( self.index, False )
            else: # add secondary
                self.owner.pickInstrument( self, self.index, False )
        else: # HOTSPOT_1, remove primary
            self.owner.clearInstrument( self.index, True )

        self.clicked = None

    def motion_notify( self, widget, event ):

        if self.clicked != None:
            return

        if event.is_hint:
            x, y, state = widget.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        if     event.x >= self.hotspots[0][0] and event.x <= self.hotspots[0][2] \
           and event.y >= self.hotspots[0][1] and event.y <= self.hotspots[0][3]:
            if self.hover != "HOTSPOT_0":
                self.hover = "HOTSPOT_0"
                self.queue_draw()


        elif    self.secondary != None \
           and event.x >= self.hotspots[1][0] and event.x <= self.hotspots[1][2] \
           and event.y >= self.hotspots[1][1] and event.y <= self.hotspots[1][3]:
            if self.hover != "HOTSPOT_1":
                self.hover = "HOTSPOT_1"
                self.queue_draw()
        else:
            if self.hover != None:
                self.hover = None
                self.queue_draw()

    def leave_notify( self, widget, event ):
        if event.mode != gtk.gdk.CROSSING_NORMAL:
            return
        if self.hover != None:
            self.hover = None
            if self.clicked == None:
                self.queue_draw()
        
        self.owner.activity.handler_unblock(self.owner.activity.focusOutHandler)
        self.owner.activity.handler_unblock(self.owner.activity.focusInHandler)
        
    def enter_notify(self, widget, event):
        # Block the Focus Out event so that the sound does'nt stop when a Palette is invoked.
        self.owner.activity.handler_block(self.owner.activity.focusOutHandler)
        self.owner.activity.handler_block(self.owner.activity.focusInHandler)

    def setPrimary( self, img ):
        self.primary = img
        self.primaryWidth = img.get_width()
        self.primaryHeight = img.get_height()
        if self.pixmap:
            self.primaryX = (self.alloc.width - self.primaryWidth) // 2
            self.primaryY = (self.alloc.height - self.primaryHeight) // 2
            self._updatePixmap()

    def setSecondary( self, img ):
        self.secondary = img
        if img != None:
            self.secondaryWidth = img.get_width()
            self.secondaryHeight = img.get_height()
            self.secondaryOffset = self.secondaryHeight//2
            if self.pixmap:
                self.secondaryX = (self.alloc.width - self.secondaryWidth) // 2
                self.secondaryY = self.alloc.height//2
        if self.pixmap:
            self._updatePixmap()

    def _updatePixmap( self ):
        self.gc.foreground = self.color["background"]
        self.pixmap.draw_rectangle( self.gc, True, 0, 0, self.alloc.width, self.alloc.height )
        if self.secondary != None:
            self.pixmap.draw_pixbuf( self.gc, self.primary, 0, 0, self.primaryX, self.primaryY, self.primaryWidth, self.primaryHeight//2, gtk.gdk.RGB_DITHER_NONE )
            self.pixmap.draw_pixbuf( self.gc, self.secondary, 0, self.secondaryOffset, self.secondaryX, self.secondaryY, self.secondaryWidth, self.secondaryHeight//2, gtk.gdk.RGB_DITHER_NONE )
            self.gc.foreground = self.color["divider"]
            self.gc.set_line_attributes( 2, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            self.pixmap.draw_line( self.gc, 2, self.alloc.height//2, self.alloc.width-4, self.alloc.height//2 )
        else:
            self.pixmap.draw_pixbuf( self.gc, self.primary, 0, 0, self.primaryX, self.primaryY, self.primaryWidth, self.primaryHeight, gtk.gdk.RGB_DITHER_NONE )
        self.queue_draw()

    def expose( self, widget, event ):
        self.window.draw_drawable( self.gc, self.pixmap, 0, 0, 0, 0, self.alloc.width, self.alloc.height )
        self.gc.set_line_attributes( 4, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_MITER )
        if self.secondary != None:
            if self.clicked == "HOTSPOT_0" or (self.clicked == None and self.hover == "HOTSPOT_0" ):
                self.gc.foreground = self.color["+/-Highlight"]
            else:
                self.gc.foreground = self.color["+/-"]
            self.window.draw_line( self.gc, self.hotspots[0][0], self.hotspots[0][5], self.hotspots[0][2], self.hotspots[0][5] )
            if self.clicked == "HOTSPOT_1" or (self.clicked == None and self.hover == "HOTSPOT_1" ):
                self.gc.foreground = self.color["+/-Highlight"]
            else:
                self.gc.foreground = self.color["+/-"]
            self.window.draw_line( self.gc, self.hotspots[1][0], self.hotspots[1][5], self.hotspots[1][2], self.hotspots[1][5] )
        else:
            if self.clicked == "HOTSPOT_0" or self.hover == "HOTSPOT_0":
                self.gc.foreground = self.color["+/-Highlight"]
            else:
                self.gc.foreground = self.color["+/-"]
            self.window.draw_line( self.gc, self.hotspots[0][0], self.hotspots[0][5], self.hotspots[0][2], self.hotspots[0][5] )
            self.window.draw_line( self.gc, self.hotspots[0][4], self.hotspots[0][1], self.hotspots[0][4], self.hotspots[0][3] )
            
    def set_palette(self, palette):
        self._palette = palette
        self._palette.props.invoker = WidgetInvoker(self)
        self._palette.props.invoker._position_hint = WidgetInvoker.AT_CURSOR #This is a hack, will change with newer Palette API
            
        
class instrumentPalette(Palette):
    def __init__(self, label, trackID, edit):
        Palette.__init__(self, label)
     
        self.trackID = trackID
        self.edit = edit
        
        self.tooltips = gtk.Tooltips()
        
        self.volumeBox = gtk.HBox()
        
        self.muteButton = gtk.CheckButton()
        self.muteButton.connect("toggled",self.edit.handlemuteButton, self.trackID)
        self.muteButton.connect("button-press-event",self.edit.handlemuteButtonRightClick, self.trackID)
        self.muteButton.set_active(True)
        self.tooltips.set_tip(self.muteButton, _('Left click to mute, right click to solo'))

        if self.trackID < 4:
            exec "self.volumeSliderAdj = self.edit.GUI['2instrument%svolumeAdjustment']" % str(self.trackID+1)
        else:
            self.volumeSliderAdj = self.edit.GUI["2drumvolumeAdjustment"]
        self.volumeSliderAdj.connect( "value-changed", self.edit.handleTrackVolume, self.trackID)
        self.volumeSlider =  gtk.HScale(adjustment = self.volumeSliderAdj)
        self.volumeSlider.set_size_request(250, -1)
        self.volumeSlider.set_inverted(False)
        self.volumeSlider.set_draw_value(False)

        self.volumeBox.pack_start(self.muteButton, padding = 5)
        self.volumeBox.pack_start(self.volumeSlider, padding = 5)
        self.volumeBox.show_all()

        self.set_content(self.volumeBox)
    