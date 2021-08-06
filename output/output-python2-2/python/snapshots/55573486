import pygtk
pygtk.require( '2.0' )
import gtk

import gobject

from Util.ThemeWidgets import *
from Util.Profiler import TP
from Util import NoteDB
from Util.NoteDB import PARAMETER
from Util import ControlStream
from Util.CSoundClient import new_csound_client
from Util.InstrumentPanel import InstrumentPanel
from Util.InstrumentPanel import DrumPanel
from Util.CSoundNote import CSoundNote
from subprocess import Popen
import time
import os
import commands

class CONTEXT:
    PAGE = 0
    TRACK = 1
    NOTE = 2

import Config
from SubActivity import SubActivity

from Generation.GenerationConstants import GenerationConstants
from Generation.GenerationParametersWindow import GenerationParametersWindow
from Edit.Properties import Properties
from Edit.TrackInterface import TrackInterface, TrackInterfaceParasite
from Edit.TuneInterface import TuneInterface, TuneInterfaceParasite

from Generation.Generator import generator1, variate, GenerationParameters

Tooltips = Config.Tooltips()
KEY_MAP_PIANO = Config.KEY_MAP_PIANO

#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( SubActivity ):

    def __init__( self, set_mode ):
        self.csnd = new_csound_client()
        self.tooltips = gtk.Tooltips()
        for i in [6,7,8,9,10]:
            self.csnd.setTrackVolume(100, i)
        self.trackCount = 6

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

            self._data['volume'] = Config.DEFAULT_VOLUME
            self._data['tempo'] = Config.PLAYER_TEMPO

            self.playScope = "Selection"
            self.displayedPage = -1
            self.trackSelected = [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]
            self.trackActive = [ 1 for i in range(Config.NUMBER_OF_TRACKS) ]

            self.pages_playing = []

            self.noteDB = NoteDB.NoteDB()
            TP.ProfileEnd("init_data")

        def formatRoundBox( box, fillcolor ):
            box.set_radius( 7 )
            box.set_border_width( 1 )
            box.set_fill_color( fillcolor )
            box.set_border_color( "#FFF" )
            return box

        def init_GUI():
            
            self.GUI = {}
            self.GUI["2main"] = gtk.HBox()

            def draw_inst_icons():
                instrumentNames = [ k for k in Config.INSTRUMENTS.keys() if (k[0:4] != 'drum' and k[0:4] != 'guid') or Config.INSTRUMENTS[k].category == "kit" ]
                self.GUI["2instrumentIcons"] = {}
                for instrument in instrumentNames:
                    self.GUI["2instrumentIcons"][instrument] = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + instrument + '.png')
            
            TP.ProfileBegin("init_GUI::instrument icons")
            draw_inst_icons()
            TP.ProfileEnd("init_GUI::instrument icons")
   
            
            #------------------------------------------------------------------------
            # left panel
            TP.ProfileBegin("init_GUI::left panel")
            self.GUI["2leftPanel"] = gtk.VBox()
            self.GUI["2leftPanel"].set_size_request( 137, -1 )
            if 1: # + instrument panel
                self.GUI["2instrumentPanel"] = gtk.VBox()
                # + + instrument 1 box
                self.GUI["2instrument1Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument1Box"].set_size_request( -1, 137 )
                self.GUI["2instrument1volBox"] = gtk.VBox()
                self.GUI["2instrument1volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument1volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 0 )
                self.GUI["2instrument1volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst1.png", self.GUI["2instrument1volumeAdjustment"], 6 )
                self.GUI["2instrument1volumeSlider"].set_inverted(True)
                self.GUI["2instrument1volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument1volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 0 )
                self.GUI["2instrument1muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument1muteButton"].connect("toggled",self.handlemuteButton,0)
                self.GUI["2instrument1muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,0)
                self.GUI["2instrument1muteButton"].set_active(True)
                self.GUI["2instrument1volBox"].pack_start( self.GUI["2instrument1volumeSlider"], True, True, 0 )
                self.GUI["2instrument1volBox"].pack_start( self.GUI["2instrument1muteButton"], False, False, 5 )
                self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1volBox"], False, False, 0 )
                self.GUI["2instrument1Button"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[0].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[0].name + '.png')
                self.GUI["2instrument1Button"].connect("toggled", self.pickInstrument, 0 )
                self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1Button"] )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument1Box"] )
                # + + instrument 2 box
                self.GUI["2instrument2Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument2Box"].set_size_request( -1, 137 )
                self.GUI["2instrument2volBox"] = gtk.VBox()
                self.GUI["2instrument2volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument2volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 1 )
                self.GUI["2instrument2volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst2.png", self.GUI["2instrument2volumeAdjustment"], 6 )
                self.GUI["2instrument2volumeSlider"].set_inverted(True)
                self.GUI["2instrument2volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument2volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 1 )
                self.GUI["2instrument2muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument2muteButton"].connect("toggled",self.handlemuteButton,1)
                self.GUI["2instrument2muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,1)
                self.GUI["2instrument2muteButton"].set_active(True)
                self.GUI["2instrument2volBox"].pack_start( self.GUI["2instrument2volumeSlider"], True, True, 0 )
                self.GUI["2instrument2volBox"].pack_start( self.GUI["2instrument2muteButton"], False, False, 5 )
                self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2volBox"], False, False, 0 )
                self.GUI["2instrument2Button"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[1].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[1].name + '.png')
                self.GUI["2instrument2Button"].connect("toggled", self.pickInstrument, 1 )
                self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2Button"] )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument2Box"] )
                # + + instrument 3 box
                self.GUI["2instrument3Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument3Box"].set_size_request( -1, 137 )
                self.GUI["2instrument3volBox"] = gtk.VBox()
                self.GUI["2instrument3volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][2], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument3volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 2 )
                self.GUI["2instrument3volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst3.png", self.GUI["2instrument3volumeAdjustment"], 6 )
                self.GUI["2instrument3volumeSlider"].set_inverted(True)
                self.GUI["2instrument3volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument3volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 2 )
                self.GUI["2instrument3muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument3muteButton"].connect("toggled",self.handlemuteButton,2)
                self.GUI["2instrument3muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,2)
                self.GUI["2instrument3muteButton"].set_active(True)
                self.GUI["2instrument3volBox"].pack_start( self.GUI["2instrument3volumeSlider"], True, True, 0 )
                self.GUI["2instrument3volBox"].pack_start( self.GUI["2instrument3muteButton"], False, False, 5 )
                self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3volBox"], False, False, 0 )
                self.GUI["2instrument3Button"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[2].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[2].name + '.png')
                self.GUI["2instrument3Button"].connect("toggled", self.pickInstrument, 2 )
                self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3Button"] )                
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument3Box"] )
                # + + instrument 4 box
                self.GUI["2instrument4Box"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2instrument4Box"].set_size_request( -1, 137 )
                self.GUI["2instrument4volBox"] = gtk.VBox()
                self.GUI["2instrument4volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][3], 0, 100, 1, 1, 0 )
                #self.GUI["2instrument4volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 3 )
                self.GUI["2instrument4volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst4.png", self.GUI["2instrument4volumeAdjustment"], 6 )
                self.GUI["2instrument4volumeSlider"].set_inverted(True)
                self.GUI["2instrument4volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument4volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 3 )
                self.GUI["2instrument4muteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2instrument4muteButton"].connect("toggled",self.handlemuteButton,3)
                self.GUI["2instrument4muteButton"].connect("button-press-event",self.handlemuteButtonRightClick,3)
                self.GUI["2instrument4muteButton"].set_active(True)
                self.GUI["2instrument4volBox"].pack_start( self.GUI["2instrument4volumeSlider"], True, True, 0 )
                self.GUI["2instrument4volBox"].pack_start( self.GUI["2instrument4muteButton"], False, False, 5 )
                self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4volBox"], False, False, 0 )
                self.GUI["2instrument4Button"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[3].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[3].name + '.png')
                self.GUI["2instrument4Button"].connect("toggled", self.pickInstrument, 3 )
                self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4Button"] )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument4Box"] )
                # + + drum box
                self.GUI["2drumBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2drumBox"].set_size_request( -1, 165 )
                self.GUI["2drumVolBox"] = gtk.VBox()
                self.GUI["2drumvolumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][4], 0, 100, 1, 1, 0 )
                #self.GUI["2drumvolumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 4 )
                self.GUI["2drumvolumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderDrum.png", self.GUI["2drumvolumeAdjustment"], 6 )
                self.GUI["2drumvolumeSlider"].set_inverted(True)
                self.GUI["2drumvolumeSlider"].set_size_request( 30, -1 )
                self.GUI["2drumvolumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 4 )
                self.GUI["2drumMuteButton"] = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
                self.GUI["2drumMuteButton"].connect("toggled",self.handlemuteButton,4)
                self.GUI["2drumMuteButton"].connect("button-press-event",self.handlemuteButtonRightClick,4)
                self.GUI["2drumMuteButton"].set_active(True)
                self.GUI["2drumVolBox"].pack_start( self.GUI["2drumvolumeSlider"], True, True, 0 )
                self.GUI["2drumVolBox"].pack_start( self.GUI["2drumMuteButton"], False, False, 5 )
                self.GUI["2drumBox"].pack_start( self.GUI["2drumVolBox"], False, False, 0 )
                self.GUI["2drumButton"] = ImageToggleButton(Config.IMAGE_ROOT + self.trackInstrument[4].name + '.png', Config.IMAGE_ROOT + self.trackInstrument[4].name + '.png')
                self.GUI["2drumButton"].connect("toggled", self.pickDrum)
                self.GUI["2drumBox"].pack_start( self.GUI["2drumButton"] )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2drumBox"] )
                self.GUI["2leftPanel"].pack_start( self.GUI["2instrumentPanel"], False )
                # + volume panel
                self.GUI["2volumePanel"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                # + + volume box
                self.GUI["2volumeBox"] = gtk.VBox()
                self.GUI["2volumeImage"] = gtk.Image()
                self.GUI["2volumeImage"].set_from_file( Config.IMAGE_ROOT+"volume2.png" )
                self.GUI["2volumeBox"].pack_start( self.GUI["2volumeImage"], False )
                self.GUI["2volumeAdjustment"] = gtk.Adjustment( self._data["volume"], 0, 100, 1, 1, 0 )
                self.GUI["2volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderEditVolume.png", self.GUI["2volumeAdjustment"], 6 )
                self.GUI["2volumeSlider"].set_inverted(True)
                self.GUI["2volumeAdjustment"].connect( "value-changed", self.handleVolume )
                self.GUI["2volumeBox"].pack_start( self.GUI["2volumeSlider"] )
                self.GUI["2volumePanel"].pack_start( self.GUI["2volumeBox"] )
                # + + tempo box
                self.GUI["2tempoBox"] = gtk.VBox()
                self.GUI["2tempoImage"] = gtk.Image()
                self.GUI["2tempoImage"].set_from_file( Config.IMAGE_ROOT+"tempo3.png" )
                self.GUI["2tempoBox"].pack_start( self.GUI["2tempoImage"], False )
                self.GUI["2tempoAdjustment"] = gtk.Adjustment( self._data["tempo"], 40, 240, 1, 1, 0 )
                self.GUI["2tempoSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderEditTempo.png", self.GUI["2tempoAdjustment"], 6 )
                self.GUI["2tempoSlider"].set_inverted(True)
                self.GUI["2tempoAdjustment"].connect( "value-changed", self.handleTempo )
                self.GUI["2tempoBox"].pack_start( self.GUI["2tempoSlider"] )
                self.GUI["2volumePanel"].pack_start( self.GUI["2tempoBox"] )
                self.GUI["2leftPanel"].pack_start( self.GUI["2volumePanel"] )
                self.GUI["2main"].pack_start( self.GUI["2leftPanel"], False )
            TP.ProfileEnd("init_GUI::left panel")

            #------------------------------------------------------------------------
            # right panel
            TP.ProfileBegin("init_GUI::right panel")
            self.GUI["2rightPanel"] = gtk.VBox()
            if 1: # + track interface
                #self.GUI["2XYSliderFixed"] = formatRoundBox( RoundFixed(), Config.BG_COLOR )
                #self.GUI["2XYSliderFixed"].set_size_request( -1, 713 )
                #self.GUI["2XYSliderButton"] =  ImageToggleButton( Config.IMAGE_ROOT+"pointer.png", Config.IMAGE_ROOT+"pointerDown.png" )
                #self.GUI["2XYSliderXAdjustment"] = gtk.Adjustment( 650, 500, 1000, 1, 1, 1 )
                #self.GUI["2XYSliderYAdjustment"] = gtk.Adjustment( 650, 500, 1000, 1, 1, 1 )
                #self.GUI["2XYSlider"] = XYSlider( self.GUI["2XYSliderFixed"], self.GUI["2XYSliderButton"], self.GUI["2XYSliderXAdjustment"], self.GUI["2XYSliderYAdjustment"], True, True )
                #self.GUI["2rightPanel"].pack_start( self.GUI["2XYSlider"], False, False, 0 )
                self.trackInterface = TrackInterface( self.noteDB, self, self.getScale )
                self.noteDB.addListener( self.trackInterface, TrackInterfaceParasite, True )
                self.trackInterface.set_size_request( -1, 713 )
                self.GUI["2rightPanel"].pack_start( self.trackInterface, False, False, 0 )
                # + tool panel
                toolPanelHeight = 82
                self.GUI["2toolPanel"] = gtk.HBox()
                self.GUI["2toolPanel"].set_size_request( -1, toolPanelHeight )
                # + + tool box
                self.GUI["2toolBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2toolBox"].set_size_request( 204, -1 )
                self.GUI["2toolPointerButton"] = ImageRadioButton( None, Config.IMAGE_ROOT+"pointer.png", Config.IMAGE_ROOT+"pointerDown.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2toolPointerButton"].connect( "clicked", self.handleToolClick , "default" )
                self.GUI["2toolBox"].pack_start( self.GUI["2toolPointerButton"] )
                self.GUI["2toolPencilButton"] = ImageRadioButton( self.GUI["2toolPointerButton"], Config.IMAGE_ROOT+"pencil.png", Config.IMAGE_ROOT+"pencilDown.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2toolPencilButton"].connect( "clicked", self.handleToolClick , "draw" )
                self.GUI["2toolBox"].pack_start( self.GUI["2toolPencilButton"] )
                self.GUI["2toolPencilButton"] = ImageRadioButton( self.GUI["2toolPointerButton"], Config.IMAGE_ROOT+"brush.png", Config.IMAGE_ROOT+"brushDown.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2toolPencilButton"].connect( "clicked", self.handleToolClick , "paint" )
                self.GUI["2toolBox"].pack_start( self.GUI["2toolPencilButton"] )

                self.GUI["2toolPanel"].pack_start( self.GUI["2toolBox"], False, False )
                self.GUI["2rightPanel"].pack_start( self.GUI["2toolPanel"], False )
                # + + context box (for context sensitive buttons, nothing to do with CAIRO)
                contextWidth = 594
                self.GUI["2contextBox"] = formatRoundBox( RoundFixed(), Config.BG_COLOR )
                self.GUI["2contextBox"].set_size_request( contextWidth, -1 )
                self.GUI["2contextPrevButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditLeft.png", Config.IMAGE_ROOT+"arrowEditLeftDown.png", Config.IMAGE_ROOT+"arrowEditLeftOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2contextPrevButton"].set_size_request( 25, toolPanelHeight )
                self.GUI["2contextPrevButton"].connect( "clicked", lambda a1:self.prevContext() )
                self.GUI["2contextBox"].put( self.GUI["2contextPrevButton"], 0, 0 )
                self.GUI["2contextNextButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditRight.png", Config.IMAGE_ROOT+"arrowEditRightDown.png", Config.IMAGE_ROOT+"arrowEditRightOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2contextNextButton"].set_size_request( 25, toolPanelHeight )
                self.GUI["2contextNextButton"].connect( "clicked", lambda a1:self.nextContext() )
                self.GUI["2contextBox"].put( self.GUI["2contextNextButton"], contextWidth-25, 0 )
                # + + + page box
                self.GUI["2pageBox"] = gtk.HBox()
                self.GUI["2pageBox"].set_size_request( contextWidth-50, 73 )
                self.GUI["2pageGenerateButton"] = ImageToggleButton( Config.IMAGE_ROOT+"genPage.png", Config.IMAGE_ROOT+"genPageDown.png", Config.IMAGE_ROOT+"genPageOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pageGenerateButton"].connect( "toggled", self.pageGenerate )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageGenerateButton"] )
                self.GUI["2pagePropertiesButton"] = ImageToggleButton( Config.IMAGE_ROOT+"propPage.png", Config.IMAGE_ROOT+"propPageDown.png", Config.IMAGE_ROOT+"propPageOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pagePropertiesButton"].connect( "toggled", self.pageProperties )
                self.GUI["2pageBox"].pack_start( self.GUI["2pagePropertiesButton"] )
                self.GUI["2pageDeleteButton"] = ImageButton( Config.IMAGE_ROOT+"delPage.png", Config.IMAGE_ROOT+"delPageDown.png", Config.IMAGE_ROOT+"delPageOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pageDeleteButton"].connect( "clicked", lambda a1:self.pageDelete() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageDeleteButton"] )
                self.GUI["2pageDuplicateButton"] = ImageButton( Config.IMAGE_ROOT+"dupPage.png", Config.IMAGE_ROOT+"dupPageDown.png", Config.IMAGE_ROOT+"dupPageOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pageDuplicateButton"].connect( "clicked", lambda a1:self.pageDuplicate() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageDuplicateButton"] )
                self.GUI["2pageNewButton"] = ImageButton( Config.IMAGE_ROOT+"addPage.png", Config.IMAGE_ROOT+"addPageDown.png", Config.IMAGE_ROOT+"addPageOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pageNewButton"].connect( "clicked", lambda a1:self.pageAdd() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageNewButton"] )
                self.GUI["2contextBox"].put( self.GUI["2pageBox"], 25, 0 )
                # + + + track box
                self.GUI["2trackBox"] = gtk.HBox()
                self.GUI["2trackBox"].set_size_request( contextWidth-50, 73 )
                self.GUI["2trackGenerateButton"] = ImageToggleButton( Config.IMAGE_ROOT+"genTrack.png", Config.IMAGE_ROOT+"genTrackDown.png", Config.IMAGE_ROOT+"genTrackOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2trackGenerateButton"].connect( "toggled", self.trackGenerate )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackGenerateButton"] )
                self.GUI["2trackPropertiesButton"] = ImageToggleButton( Config.IMAGE_ROOT+"propTrack.png", Config.IMAGE_ROOT+"propTrackDown.png", Config.IMAGE_ROOT+"propTrackOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2trackPropertiesButton"].connect( "toggled", self.trackProperties )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackPropertiesButton"] )
                self.GUI["2trackDeleteButton"] = ImageButton( Config.IMAGE_ROOT+"delTrack.png", Config.IMAGE_ROOT+"delTrackDown.png", Config.IMAGE_ROOT+"delTrackOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2trackDeleteButton"].connect( "clicked", lambda a1:self.trackDelete() )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackDeleteButton"] )
                self.GUI["2trackDuplicateButton"] = ImageToggleButton( Config.IMAGE_ROOT+"dupTrack.png", Config.IMAGE_ROOT+"dupTrackDown.png", Config.IMAGE_ROOT+"dupTrackOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2trackDuplicateButton"].connect( "toggled", self.trackDuplicateWidget )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackDuplicateButton"] )
                self.GUI["2contextBox"].put( self.GUI["2trackBox"], 25, 0 )
                # + + + note box
                self.GUI["2noteBox"] = gtk.HBox()
                self.GUI["2noteBox"].set_size_request( contextWidth-50, 73 )
                self.GUI["2notePropertiesButton"] = ImageToggleButton( Config.IMAGE_ROOT+"propNote.png", Config.IMAGE_ROOT+"propNoteDown.png", Config.IMAGE_ROOT+"propNoteOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2notePropertiesButton"].connect( "toggled", self.noteProperties )
                self.GUI["2noteBox"].pack_start( self.GUI["2notePropertiesButton"] )
                self.GUI["2noteDeleteButton"] = ImageButton( Config.IMAGE_ROOT+"delNote.png", Config.IMAGE_ROOT+"delNoteDown.png", Config.IMAGE_ROOT+"delNoteOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2noteDeleteButton"].connect( "clicked", lambda a1:self.noteDelete() )
                self.GUI["2noteBox"].pack_start( self.GUI["2noteDeleteButton"] )
                self.GUI["2noteDuplicateButton"] = ImageToggleButton( Config.IMAGE_ROOT+"dupNote.png", Config.IMAGE_ROOT+"dupNoteDown.png", Config.IMAGE_ROOT+"dupNoteOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2noteDuplicateButton"].connect( "toggled", self.noteDuplicateWidget )
                self.GUI["2noteBox"].pack_start( self.GUI["2noteDuplicateButton"] )
                self.GUI["2contextBox"].put( self.GUI["2noteBox"], 25, 0 )
                self.GUI["2toolPanel"].pack_start( self.GUI["2contextBox"], False )
                # + + transport box
                self.GUI["2transportBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2keyRecordButton"] = ImageToggleButton( Config.IMAGE_ROOT+"krecord.png", Config.IMAGE_ROOT+"krecordDown.png", Config.IMAGE_ROOT+"krecordOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2keyRecordButton"].connect("clicked", self.handleKeyboardRecordButton )
                self.GUI["2recordButton"] = ImageToggleButton( Config.IMAGE_ROOT+"record2.png", Config.IMAGE_ROOT+"record2Down.png", Config.IMAGE_ROOT+"record2Over.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2recordButton"].connect("clicked", self.handleAudioRecord )
                self.GUI["2transportBox"].pack_start( self.GUI["2keyRecordButton"] )
                self.GUI["2transportBox"].pack_start( self.GUI["2recordButton"] )
                self.GUI["2playpauseBox"] = gtk.HBox()
                self.GUI["2playpauseBox"].set_size_request( 90, -1 )
                self.GUI["2playBox"] = gtk.HBox()
                self.GUI["2rewindButton"] = ImageButton( Config.IMAGE_ROOT+"rewind.png", Config.IMAGE_ROOT+"rewindDown.png", Config.IMAGE_ROOT+"rewindOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2rewindButton"].connect( "clicked", self.handleRewind )
                self.GUI["2playBox"].pack_start( self.GUI["2rewindButton"] )
                self.GUI["2playButton"] = ImageButton( Config.IMAGE_ROOT+"play.png", Config.IMAGE_ROOT+"playDown.png", Config.IMAGE_ROOT+"playOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2playBox"].pack_start( self.GUI["2playButton"] )
                self.GUI["2playButton"].connect( "clicked", self.handlePlay )
                self.GUI["2playpauseBox"].pack_start( self.GUI["2playBox"] )
                self.GUI["2transportBox"].pack_start( self.GUI["2playpauseBox"], False, False )
                self.GUI["2pauseBox"] = gtk.HBox()
                self.GUI["2stopButton"] = ImageButton( Config.IMAGE_ROOT+"stop.png", Config.IMAGE_ROOT+"stopDown.png", Config.IMAGE_ROOT+"stopOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2stopButton"].connect( "clicked", self.handleStop )
                self.GUI["2pauseBox"].pack_start( self.GUI["2stopButton"] )
                self.GUI["2pauseButton"] = ImageButton( Config.IMAGE_ROOT+"pause.png", Config.IMAGE_ROOT+"pauseDown.png", Config.IMAGE_ROOT+"pauseOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2pauseButton"].connect( "clicked", self.handleStop, False )
                self.GUI["2pauseBox"].pack_start( self.GUI["2pauseButton"] )
                self.GUI["2pauseBox"].show_all()
                #self.GUI["2loopButton"] = ImageToggleButton( Config.IMAGE_ROOT+"loop.png", Config.IMAGE_ROOT+"loop.png", Config.IMAGE_ROOT+"loop.png", backgroundFill = Config.BG_COLOR )
                #self.GUI["2loopButton"].connect( "toggled", self.handleLoopButton )
                self.GUI["2closeButton"] = ImageButton( Config.IMAGE_ROOT+"close.png" )
                self.GUI["2closeButton"].connect( "pressed", self.handleClose)
                self.GUI["2transportBox"].pack_start( self.GUI["2closeButton"] )
                self.GUI["2toolPanel"].pack_start( self.GUI["2transportBox"] )
                # + load/save box
                self.GUI["2tuneBox"] = formatRoundBox( RoundHBox(), Config.BG_COLOR )
                self.GUI["2saveButton"] = ImageButton( Config.IMAGE_ROOT+"save.png", backgroundFill=Config.BG_COLOR )
                self.GUI["2saveButton"].connect("clicked", self.handleSave )
                self.GUI["2tuneBox"].pack_start( self.GUI["2saveButton"], False, False )
                self.GUI["2loadButton"] = ImageButton( Config.IMAGE_ROOT+"load.png", backgroundFill=Config.BG_COLOR )
                self.GUI["2loadButton"].connect("clicked", self.handleLoad )
                self.GUI["2tuneBox"].pack_start( self.GUI["2loadButton"], False, False )
                # + tune box
                self.GUI["2tuneHBox"] = gtk.HBox()
                self.GUI["2tuneScrollLeftButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditLeft.png", Config.IMAGE_ROOT+"arrowEditLeftDown.png", Config.IMAGE_ROOT+"arrowEditLeftOver.png", backgroundFill = Config.BG_COLOR )
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
                self.GUI["2tuneScrollRightButton"] = ImageButton( Config.IMAGE_ROOT+"arrowEditRight.png", Config.IMAGE_ROOT+"arrowEditRightDown.png", Config.IMAGE_ROOT+"arrowEditRightOver.png", backgroundFill = Config.BG_COLOR )
                self.GUI["2tuneScrollRightButton"].set_size_request( 25, toolPanelHeight )
                self.GUI["2tuneScrollRightButton"].connect( "clicked", lambda a1:self.scrollTune( 1 ) )
                self.GUI["2tuneHBox"].pack_start( self.GUI["2tuneScrollRightButton"], False, False )
                self.GUI["2tuneBox"].pack_start( self.GUI["2tuneHBox"] )
                self.GUI["2rightPanel"].pack_start( self.GUI["2tuneBox"] )
                self.GUI["2main"].pack_start( self.GUI["2rightPanel"] )
            TP.ProfileEnd("init_GUI::right panel")
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
            TP.ProfileBegin("init_GUI::generationPanel")
            self.generationPanel = GenerationParametersWindow( self.generate, self.variate, self.doneGenerationPopup )
            TP.ProfileEnd("init_GUI::generationPanel")
            self.GUI["9generationPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            self.GUI["9generationPopup"].set_modal(True)
            self.GUI["9generationPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            self.GUI["9generationPopup"].connect("button-release-event", lambda w,e:self.doneGenerationPopup() )
            self.GUI["9generationPopup"].add( self.generationPanel )
            # + properties window
            self.GUI["9propertiesPopup"] = gtk.Window(gtk.WINDOW_POPUP)
            self.GUI["9propertiesPopup"].set_modal(True)
            self.GUI["9propertiesPopup"].add_events( gtk.gdk.BUTTON_PRESS_MASK )
            self.GUI["9propertiesPopup"].connect("button-release-event", lambda w,e:self.donePropertiesPopup() )
            TP.ProfileBegin("init_GUI::propertiesPanel")
            self.propertiesPanel = Properties( self.noteDB, self.donePropertiesPopup, self.GUI["9propertiesPopup"] )
            TP.ProfileEnd("init_GUI::propertiesPanel")
            self.GUI["9propertiesPopup"].add( self.propertiesPanel )
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
        SubActivity.__init__( self, set_mode )

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

        first = self.noteDB.addPage( -1, NoteDB.Page(4) )
        self.displayPage( first )

        self.show_all()  #gtk command

        #self.GUI["2pageBox"].hide()
        self.GUI["2trackBox"].hide()
        self.GUI["2noteBox"].hide()
        self.setContext( CONTEXT.PAGE )
 
        self.pageAdd() 
        self.pageAdd() 
        self.pageAdd() 
        self.tuneInterface.selectPages( [1,2,3,4] )
        self.displayPage(1)
        self.generateMode = 'page' 
        self.generate( GenerationParameters() )

        self.audioRecordState = False
 
    def onActivate( self, arg ):
        SubActivity.onActivate( self,arg )
        # whatever needs to be done on initialization
        self.csnd.loopPause()
        self.csnd.loopClear()
        for n in self.noteDB.getNotes( ):
            self.csnd.loopPlay(n, 0) #adds all notes to c client in inactive state


    def onDeactivate( self ):
        SubActivity.onDeactivate( self )
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

        self._playPages( selectedIds, self.displayedPage, self.trackInterface.getPlayhead() )

    def updatePagesPlaying( self ):

        self.csnd.loopDeactivate()

        trackset = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackActive[i] ] )

        notes = []
        if len(trackset) == 0 or len(trackset) == Config.NUMBER_OF_TRACKS:
            for page in self.pages_playing:
                notes += self.noteDB.getNotesByPage( page )
        else:
            for page in self.pages_playing:
                for track in trackset:
                    notes += self.noteDB.getNotesByTrack( page, track )

        numticks = 0
        self.page_onset = {}
        for pid in self.pages_playing:
            self.page_onset[pid] = numticks
            numticks += self.noteDB.getPage(pid).ticks

        #print self.pages_playing
        for n in notes:
            self.csnd.loopUpdate(n, NoteDB.PARAMETER.DURATION, n.cs.duration , 1)

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
                self.audioRecordState = True
                self.audioFileName = chooser.get_filename()
                if self.audioFileName[-4:] != '.ogg':
                    self.audioFileName += '.ogg'
            chooser.destroy()
        else:
            self.audioRecordState = False

    def handlePlay( self, widget ):

        widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
        self.GUI["2playpauseBox"].remove( self.GUI["2playBox"] )
        self.GUI["2playpauseBox"].pack_start( self.GUI["2pauseBox"] )

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

        notes = []
        for page in self.pages_playing:
            for track in trackset:
                notes += self.noteDB.getNotesByTrack( page, track )

        if (Config.DEBUG > 3):
            print 'rebuild note loop'
            print 'pages : ', self.pages_playing
            print 'trackset : ', trackset
            print 'numticks : ', numticks
            print 'notes : ', len(notes), 'notes'
        self.csnd.loopClear()
        for n in notes:
            self.csnd.loopPlay(n, 1)
            self.csnd.loopUpdate(n, NoteDB.PARAMETER.ONSET, n.cs.onset + self.page_onset[n.page] , 1)

        self.csnd.loopSetNumTicks( numticks )

        self.csnd.loopSetTick( self.page_onset[startPage] + startTick )
        self.csnd.loopSetTempo(self._data['tempo'])
        if (Config.DEBUG > 3): print "starting from tick", startTick, 'at tempo', self._data['tempo']
        self.csnd.loopStart()

        if not self.playbackTimeout:
            self.playbackTimeout = gobject.timeout_add( 50, self.onTimeout )

      

    def handleStop( self, widget, rewind = True ):

        widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
        self.GUI["2playpauseBox"].remove( self.GUI["2pauseBox"] )
        self.GUI["2playpauseBox"].pack_start( self.GUI["2playBox"] )

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
        self.set_mode("welcome")

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
            self.displayPage( self.pages_playing[ind], predraw )
        else:
            self.trackInterface.predrawPage()

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

    # data is tuple ( trackId, instrumentName )
    def handleInstrumentChanged( self, data ):
        (id, instrument) = data
        self.trackInstrument[id] = instrument
        if (Config.DEBUG > 3): print "handleInstrumentChanged", id, instrument.name
        
        pages = self.tuneInterface.getSelectedIds()
        self.noteDB.setInstrument( pages, id, instrument.instrumentId )

        #self.noteLooper.setInstrument(id, instrumentName)

        #recordButton = self.instrumentRecordButtons[ id ]
        #if instrumentName in Config.RECORDABLE_INSTRUMENTS:
        #    recordButton.show()
        #    recordButton.connect( "clicked",
        #                          self.handleMicRecord,
        #                          Config.RECORDABLE_INSTRUMENT_CSOUND_IDS[ instrumentName ] )
        #else:
        #    recordButton.hide()

    def getScale(self):
        return self.generationPanel.scale

    def handleVolume( self, widget ):
        self._data["volume"] = round( widget.get_value() )
        self.csnd.setMasterVolume(self._data["volume"])
        img = min(3,int(4*self._data["volume"]/100)) # volume 0-3
        self.GUI["2volumeImage"].set_from_file( Config.IMAGE_ROOT+"volume"+str(img)+".png" )

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
        self.GUI["2tempoImage"].set_from_file( Config.IMAGE_ROOT+"tempo"+str(img)+".png" )


    def handleToolClick( self, widget, mode ):
        if widget.get_active(): self.trackInterface.setInterfaceMode( mode )

    def getTool( self ):
        if self.GUI["2toolPointerButton"].get_active(): return "default"
        else: return "draw"

    def handleKeyboardRecordButton( self, widget, data=None ):
        self.kb_record = self.GUI["2keyRecordButton"].get_active()

    def pickInstrument( self, widget, num ):
        if widget.get_active(): # show the panel
            self.last_clicked_instTrackID = num
            self.instrumentPanel.selectFirstCat()
            self.instrumentPanel.set_activeInstrument( self.trackInstrument[num].name, True )
            winLoc = self.parent.window.get_position()
            alloc = widget.get_allocation()
            x = alloc.x + alloc.width + winLoc[0]
            y = alloc.y + winLoc[1]
            self.GUI["9instrumentPopup"].move( x, y )
            self.GUI["9instrumentPopup"].show()
        else: # hide the panel
            self.GUI["9instrumentPopup"].hide()
            
    def cancelInstrumentSelection( self ):
        self.GUI["2instrument" + str(self.last_clicked_instTrackID+1) + "Button"].set_active(False)

    def donePickInstrument( self, instrumentName ):
        self.handleInstrumentChanged( (self.last_clicked_instTrackID, Config.INSTRUMENTS[instrumentName]) )
        btn = self.GUI["2instrument%dButton" % (self.last_clicked_instTrackID+1)]
        btn.load_pixmap( "main", self.GUI["2instrumentIcons"][instrumentName] )
        btn.load_pixmap( "alt", self.GUI["2instrumentIcons"][instrumentName] )
        btn.set_active( False )
          
    
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
        self.GUI["2drumButton"].load_pixmap( "main", self.GUI["2instrumentIcons"][drumName] )
        self.GUI["2drumButton"].load_pixmap( "alt", self.GUI["2instrumentIcons"][drumName] )
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
                        self.GUI["2drumMuteButton"].set_active(True)
                    else:
                        self.GUI["2instrument" + str(i+1) + "muteButton"].set_active(True)
            else:
                for i in range(Config.NUMBER_OF_TRACKS):
                    if i != track:
                        if i == 4:
                            self.GUI["2drumMuteButton"].set_active(False)
                        else:
                            self.GUI["2instrument" + str(i+1) + "muteButton"].set_active(False)
            self.updatePagesPlaying()
                    
    #-----------------------------------
    # generation functions
    #-----------------------------------

    def recompose( self, algo, params, genOrVar):
        if self.generateMode == "track":
            if self.trackSelected == [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]:
                newtracks = set(range(Config.NUMBER_OF_TRACKS))
            else:
                newtracks = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ] )
            newpages  = self.tuneInterface.getSelectedIds()
        else: # page mode
            newtracks = set(range(Config.NUMBER_OF_TRACKS))
            newpages = self.tuneInterface.getSelectedIds()

        if genOrVar == 0:
            dict = {}
            for t in newtracks:
                dict[t] = {}
                for p in newpages:
                    dict[t][p] = self.noteDB.getCSNotesByTrack( p, t )
        else:
            dict = {}
            for t in newtracks:
                dict[t] = {}
                dict[t][1] = self.noteDB.getCSNotesByTrack( 1, t )

        beatsOfPages = {}        
        for pageId in newpages:
            beatsOfPages[pageId] = self.noteDB.pages[pageId].beats

        algo(
                params,
                self._data['track_volume'][:],
                [ i.name for i in self.trackInstrument ],
                self._data['tempo'],
                beatsOfPages,
                newtracks,
                newpages,
                dict)

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

    def generate( self, params ):
        self.recompose( generator1, params, 0)

    def variate( self, params ):
        self.recompose( variate, params, 1)

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
        if self.skipCleanup != "note" and self.GUI["2noteDuplicateButton"].get_active():
            self.GUI["2noteDuplicateButton"].set_active(False)
        if self.skipCleanup != "track" and self.GUI["2trackDuplicateButton"].get_active():
            self.GUI["2trackDuplicateButton"].set_active(False)
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
            if self.GUI["2trackDuplicateButton"].get_active():
                self.GUI["2trackDuplicateButton"].set_active( False )
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
            if self.GUI["2noteDuplicateButton"].get_active():
                self.GUI["2noteDuplicateButton"].set_active( False )
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

        self.displayedPage = pageId
        
        page = self.noteDB.getPage(pageId)
        print pageId, page.instruments
        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackInstrument[i].instrumentId != page.instruments[i]:
                self.trackInstrument[i] = Config.INSTRUMENTSID[page.instruments[i]]
                if i == Config.NUMBER_OF_TRACKS-1: btn = self.GUI["2drumButton"]
                else: btn = self.GUI["2instrument%dButton"%(i+1)]
                btn.load_pixmap( "main", self.GUI["2instrumentIcons"][self.trackInstrument[i].name] )
                btn.load_pixmap( "alt", self.GUI["2instrumentIcons"][self.trackInstrument[i].name] )

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

    def pageDelete( self, pageIds = -1 ):

        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()

        self.noteDB.deletePages( pageIds[:] )

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
        self.csnd.loopSetTempo(self._data['tempo'])
        self.initTrackVolume()

    def handleSave(self, widget):

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
            print 'INFO: serialize to file %s' % ofilename
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
            print 'DEBUG: clearing noteDB'
            self.noteDB.deletePages( self.noteDB.pages.keys() )
            # still leaves an empty page at start... grrr
            print 'DEBUG: loading file: ', chooser.get_filename()
            try:
                ifile = open(chooser.get_filename(), 'r')
                ttt = ControlStream.TamTamTable ( self.noteDB )
                ttt.parseFile(ifile)
                self.trackInstrument = self.trackInstrumentDefault[:] # these will get set correctly in displayPage
                self._data['track_volume'] = ttt.tracks_volume
                self._data['volume'] = float(ttt.masterVolume)
                self._data['tempo'] = float(ttt.tempo)
                self.GUI["2volumeAdjustment"].set_value(self._data['volume'])
                self.GUI["2tempoAdjustment"].set_value(self._data['tempo'])
                for i in range(Config.NUMBER_OF_TRACKS):
                    if i == 4:
                        string = '2drumvolumeAdjustment'
                    else:
                        string = '2instrument' + str(i+1) + 'volumeAdjustment'  
                    self.GUI[string].set_value(self._data['track_volume'][i])
                for tid in range(Config.NUMBER_OF_TRACKS):
                    self.last_clicked_instTrackID = tid
                    if tid == 4:
                        self.donePickDrum(self.trackInstrument[tid].name)
                    else:
                        self.donePickInstrument(self.trackInstrument[tid].name)
                ifile.close()

                self.tuneInterface.selectPages( self.noteDB.tune )
                #self.displayPage(1)

                # TODO: if deletePages() worked the first time, we wouldn't need
                # this
                self.noteDB.deletePages( self.noteDB.tune[0:1] )
            except OSError,e:
                print 'ERROR: failed to open file %s for reading\n' % ofilename

        chooser.destroy()
        self.delay = gobject.timeout_add(1000, self.waitToSet)
        
    def handleJournalLoad(self,file_path):
        self.noteDB.deletePages( self.noteDB.pages.keys() )

        ifile = open(file_path, 'r')
        ttt = ControlStream.TamTamTable ( self.noteDB )
        ttt.parseFile(ifile)
        self.trackInstrument = self.trackInstrumentDefault[:] # these will get set correctly in displayPage
        self._data['track_volume'] = ttt.tracks_volume
        self._data['volume'] = float(ttt.masterVolume)
        self._data['tempo'] = float(ttt.tempo)
        self.GUI["2volumeAdjustment"].set_value(self._data['volume'])
        self.GUI["2tempoAdjustment"].set_value(self._data['tempo'])
        for i in range(Config.NUMBER_OF_TRACKS):
            if i == 4:
                string = '2drumvolumeAdjustment'
            else:
                string = '2instrument' + str(i+1) + 'volumeAdjustment'  
            self.GUI[string].set_value(self._data['track_volume'][i])
        for tid in range(Config.NUMBER_OF_TRACKS):
            self.last_clicked_instTrackID = tid
            if tid == 4:
                self.donePickDrum(self.trackInstrument[tid].name)
            else:
                self.donePickInstrument(self.trackInstrument[tid].name)
        ifile.close()

        self.tuneInterface.selectPages( self.noteDB.tune )
        #self.displayPage(1)

        # TODO: if deletePages() worked the first time, we wouldn't need
        # this
        self.noteDB.deletePages( self.noteDB.tune[0:1] )
            
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
        key = event.hardware_keycode
    
        # backspace and del keys
        if key == 22 or key == 107:
            if self.context == CONTEXT.PAGE: self.pageDelete()
            if self.context == CONTEXT.TRACK: self.trackDelete()
            if self.context == CONTEXT.NOTE: self.noteDelete()
        # plus key
        if key == 21:
            self.pageAdd()
        # duplicate ctrl-c
        if event.state == gtk.gdk.CONTROL_MASK and key == 54:
            if self.context == CONTEXT.PAGE: self.pageDuplicate()
            if self.context == CONTEXT.TRACK: self.trackDuplicate()
            if self.context == CONTEXT.NOTE: self.noteDuplicate()            
        #Arrows
        if event.state == gtk.gdk.SHIFT_MASK:
            # up/down arrows volume
            if key == 111: self.trackInterface.noteStepVolume(0.1)
            if key == 116: self.trackInterface.noteStepVolume(-0.1)
            # left/right arrows onset
            if key == 113: self.trackInterface.noteStepDuration(-1)
            if key == 114: self.trackInterface.noteStepDuration(1)
        else:
            # up/down arrows pitch
            if key == 111: self.trackInterface.noteStepPitch(1)
            if key == 116: self.trackInterface.noteStepPitch(-1)
            # left/right arrows duration
            if key == 113: self.trackInterface.noteStepOnset(-1)
            if key == 114: self.trackInterface.noteStepOnset(1)
            
        
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

            # pitch, inst and duration for drum recording
            if instrument[0:4] == 'drum':
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
            if instrument[0:4] == 'drum':
                del self.kb_keydict[key]

            # remove previosly holded key from dictionary
            if len(self.kb_keydict) > 1:
                for k in self.kb_keydict.keys():
                    if k != key:
                        gobject.source_remove( self.durUpdate )
                        self.kb_keydict[k].duration = 0.5
                        self.kb_keydict[k].amplitude = 0
                        self.kb_keydict[k].decay = 0.7
                        self.kb_keydict[k].tied = False
                        self.csnd.play(self.kb_keydict[k], 0.3)
                        if not self.kb_record:
                            del self.kb_keydict[k]
                            return
                        oldId = []
                        for i in self.csId:
                            oldId.append(i)
                        self.removeRecNote(k, oldId)

            if not self.kb_record:
                return

            #record the note on track
            pageList = self.tuneInterface.getSelectedIds()
            pid = self.displayedPage
            pidOffset = pageList.index(pid)+1
            self.beats = self.noteDB.getPage( pid ).beats
            tid = index
            minOnset = (pidOffset-1) * self.beats * Config.TICKS_PER_BEAT
            maxOnset = self.beats * Config.TICKS_PER_BEAT
            onsetQuantized = 3 * int((self.csnd.loopGetTick() - minOnset) / 3. + 0.5)

            if onsetQuantized > maxOnset-3:
                if pid == pageList[:-1]:
                    pid = pageList[0]
                else:
                    if len(pageList) > 1:
                        pidPos = pageList.index(pid)
                        pid = pageList[pidPos+1]
                onsetQuantized = 0

            if instrument[0:4] != 'drum':
                for n in self.noteDB.getNotesByTrack( pid, tid ): 
                    if onsetQuantized >= n.cs.onset and ((n.cs.onset + n.cs.duration) - onsetQuantized) in [1,2]:
                        adjustedDuration = onsetQuantized - n.cs.onset
                        if adjustedDuration == 0:
                            return
                        self.noteDB.updateNote( n.page, n.track, n.id, PARAMETER.DURATION, adjustedDuration)
                    if onsetQuantized >= n.cs.onset and (onsetQuantized+2) <= (n.cs.onset + n.cs.duration):
                        self.noteDB.deleteNote(n.page, n.track, n.id)
                        #return
 
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
            self.csId = [pid, tid, id, csnote.onset ]
            if instrument[0:4] != 'drum':
                self.durUpdate = gobject.timeout_add( 25, self.durationUpdate )

    def onKeyRelease(self,widget,event):
        Config.ModKeys.keyRelease( event.hardware_keycode )
        key = event.hardware_keycode

        if True in self.trackSelected:
            index = self.trackSelected.index(True)
            if index == 4:
                return
        else:
            return

        if self.kb_record:
            gobject.source_remove( self.durUpdate )

        if KEY_MAP_PIANO.has_key(key) and self.kb_keydict.has_key(key):
            if Config.INSTRUMENTSID[ self.kb_keydict[key].instrumentId ].csoundInstrumentId == Config.INST_TIED:
                self.kb_keydict[key].duration = 0.5
                self.kb_keydict[key].amplitude = 0
                self.kb_keydict[key].decay = 0.5
                self.kb_keydict[key].tied = False
                self.csnd.play(self.kb_keydict[key], 0.3)
            if not self.kb_record:
                del self.kb_keydict[key]
                return

            self.removeRecNote(key, self.csId)

    def removeRecNote(self, key, csId):
        pageList = self.tuneInterface.getSelectedIds()
        pidOffset = pageList.index(csId[0])+1
 
        newDuration = (self.csnd.loopGetTick() - ((pidOffset-1) * self.beats * Config.TICKS_PER_BEAT)) - csId[3]
        if newDuration < 1:
            newDuration = 1

        maxTick = self.beats * Config.TICKS_PER_BEAT
        if (csId[3] + newDuration) >= maxTick:
            newDuration = maxTick - csId[3]

        for n in self.noteDB.getNotesByTrack( csId[0], csId[1] ): 
            if csId[3] < n.cs.onset and (csId[3] + newDuration) >= n.cs.onset:
                self.noteDB.deleteNote(n.page, n.track, n.id)
                #newDuration = n.cs.onset - csId[3]
                break

        self.noteDB.updateNote( csId[0], csId[1], csId[2], PARAMETER.DURATION, newDuration)

        del self.kb_keydict[key]

    def durationUpdate(self):
        pageList = self.tuneInterface.getSelectedIds()
        pidOffset = pageList.index(self.csId[0])+1
        newDuration = (self.csnd.loopGetTick() - ((pidOffset-1) * self.beats * Config.TICKS_PER_BEAT)) - self.csId[3]

        maxTick = self.beats * Config.TICKS_PER_BEAT
        if (self.csId[3] + newDuration) > maxTick:
            newDuration = maxTick - self.csId[3]

        for n in self.noteDB.getNotesByTrack( self.csId[0], self.csId[1] ): 
            if self.csId[3] < n.cs.onset and (self.csId[3] + newDuration) > n.cs.onset:
                self.noteDB.deleteNote(n.page, n.track, n.id)
                #newDuration = n.cs.onset - self.csId[3]
                break

        self.noteDB.updateNote( self.csId[0], self.csId[1], self.csId[2], PARAMETER.DURATION, newDuration)

        return True

    def delete_event( self, widget, event, data = None ):
        return False

    def onDestroy( self ):

        if (Config.DEBUG > 1): print TP.PrintAll()

    def updateContextNavButtons( self ):
        if self.context == CONTEXT.PAGE:
            self.GUI["2contextPrevButton"].hide()
            if self.contextTrackActive or self.contextNoteActive:
                self.GUI["2contextNextButton"].show()
            else:
                self.GUI["2contextNextButton"].hide()
        elif self.context == CONTEXT.TRACK:
            self.GUI["2contextPrevButton"].show()
            if self.contextNoteActive:
                self.GUI["2contextNextButton"].show()
            else:
                self.GUI["2contextNextButton"].hide()
        else:
            self.GUI["2contextPrevButton"].show()
            self.GUI["2contextNextButton"].hide()

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
                    self.updateContextNavButtons()
        else:
            self.contextNoteActive = state
            if not state:
                if self.context == CONTEXT.NOTE:
                    self.prevContext()
                else:
                    self.updateContextNavButtons()

    def setContext( self, context, force = False ):

        if self.context == context and not force: return

        if self.context == CONTEXT.PAGE: self.GUI["2pageBox"].hide()
        elif self.context == CONTEXT.TRACK: self.GUI["2trackBox"].hide()
        else: self.GUI["2noteBox"].hide()

        self.context = context
        self.updateContextNavButtons()

        if self.context == CONTEXT.PAGE: self.GUI["2pageBox"].show()
        elif self.context == CONTEXT.TRACK: self.GUI["2trackBox"].show()
        else: self.GUI["2noteBox"].show()

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
        return round( self.tempoAdjustment.value, 0 )

    def getBeatsPerPage( self ):
        return int(round( self.beatsPerPageAdjustment.value, 0 ))

    def getWindowTitle( self ):
        return "Tam-Tam [Volume %i, Tempo %i, Beats/Page %i]" % ( self.volumeAdjustment.value, self.getTempo(), self.getBeatsPerPage() )
