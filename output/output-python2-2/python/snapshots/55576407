import pygtk
pygtk.require( '2.0' )
import gtk

import gobject
from Util.ThemeWidgets import *

import time

class CONTEXT:
    PAGE = 0
    TRACK = 1
    NOTE = 2

import Config

from Edit.MixerWindow import MixerWindow
from Generation.GenerationConstants import GenerationConstants
from Generation.GenerationParametersWindow import GenerationParametersWindow
from Util.NoteDB import NoteDB
from Edit.TrackInterface import TrackInterface, TrackInterfaceParasite
from Edit.TuneInterface import TuneInterface, TuneInterfaceParasite

from Util.Profiler import TP

from Generation.Generator import generator1, variate

#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.EventBox ):

    def __init__( self, CSoundClient ):
        self.csnd = CSoundClient

        def init_data( ):
            self._data = {}

            #[ volume, ... ]
            self._data['track_volume'] = [ Config.DEFAULT_VOLUME ] * Config.NUMBER_OF_TRACKS
            self._data['track_mute']   = [ 1.0 ] * Config.NUMBER_OF_TRACKS

            #[ instrument index, ... ]
            track_inst = [
                    Config.FLUTE,
                    Config.KOTO,
                    Config.GAM,
                    Config.GUIT,
                    Config.DRUM1KIT ]
            if len(track_inst) != Config.NUMBER_OF_TRACKS: raise 'error'

            self._data['track_inst'] = track_inst + [Config.FLUTE] * (Config.NUMBER_OF_TRACKS - len( track_inst) )
            #{ pageId: { [track 0 = note list], [track 2 = note list], ... ] }
            npages = 40
            nbeats = 4

            self._data['volume'] = Config.DEFAULT_VOLUME
            self._data['page_beats'] = [nbeats  for p in range(npages)]
            self._data['tempo'] = Config.PLAYER_TEMPO

            self.trackSelected = [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]

            self.noteDB = NoteDB()
            self.noteDB.addListener( self, page=True ) # register for page notifications

        def formatRoundBox( box, fillcolor ):
            box.set_radius( 7 )
            box.set_border_width( 1 )
            box.set_fill_color( fillcolor )
            box.set_border_color( "#FFF" )
            return box

        def init_GUI():
            self.GUI = {}
            self.GUI["2main"] = gtk.HBox()

            def track_menu(trackId, lbl):
                instrumentMenuItem = gtk.MenuItem( lbl )
                instrumentMenu = gtk.Menu()
                instrumentMenuItem.set_submenu( instrumentMenu )

                instrumentNames = [ k for k in Config.INSTRUMENTS.keys() if k[0:4] != 'drum' ] + ['drum1kit']
                instrumentNames.sort()
                for i in instrumentNames:
                    menuItem = gtk.MenuItem( i )
                    menuItem.connect_object( "activate", self.handleInstrumentChanged, ( trackId, i ) )
                    instrumentMenu.append( menuItem )

                instrumentMenuBar = gtk.MenuBar()
                instrumentMenuBar.append( instrumentMenuItem )
                return instrumentMenuBar

            #-------------------------------------------------------------------------
            # left panel
            self.GUI["2leftPanel"] = gtk.VBox()
            self.GUI["2leftPanel"].set_size_request( 137, -1 )
            if 1: # + instrument panel
                self.GUI["2instrumentPanel"] = gtk.VBox()
                # + + instrument 1 box
                self.GUI["2instrument1Box"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2instrument1Box"].set_size_request( -1, 137 )
                self.GUI["2instrument1volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                self.GUI["2instrument1volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 0 )
                self.GUI["2instrument1volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst1.png", self.GUI["2instrument1volumeAdjustment"], 6 )
                self.GUI["2instrument1volumeSlider"].set_inverted(True)
                self.GUI["2instrument1volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument1volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 0 )
                self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1volumeSlider"], False, False, 0 )
                #self.GUI["2instrument1Button"] = gtk.Button("Inst 1")
                #self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1Button"] )
                self.GUI["2instrument1Box"].pack_start( track_menu(0,'?') )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument1Box"] )
                # + + instrument 2 box
                self.GUI["2instrument2Box"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2instrument2Box"].set_size_request( -1, 137 )
                self.GUI["2instrument2volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][1], 0, 100, 1, 1, 0 )
                self.GUI["2instrument2volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 1 )
                self.GUI["2instrument2volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst2.png", self.GUI["2instrument2volumeAdjustment"], 6 )
                self.GUI["2instrument2volumeSlider"].set_inverted(True)
                self.GUI["2instrument2volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument2volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 1 )
                self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2volumeSlider"], False, False, 0 )
                #self.GUI["2instrument2Button"] = gtk.Button("Inst 2")
                #self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2Button"] )
                self.GUI["2instrument2Box"].pack_start( track_menu(1,'?') )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument2Box"] )
                # + + instrument 3 box
                self.GUI["2instrument3Box"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2instrument3Box"].set_size_request( -1, 137 )
                self.GUI["2instrument3volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][2], 0, 100, 1, 1, 0 )
                self.GUI["2instrument3volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 2 )
                self.GUI["2instrument3volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst3.png", self.GUI["2instrument3volumeAdjustment"], 6 )
                self.GUI["2instrument3volumeSlider"].set_inverted(True)
                self.GUI["2instrument3volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument3volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 2 )
                self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3volumeSlider"], False, False, 0 )
                #self.GUI["2instrument3Button"] = gtk.Button("Inst 3")
                #self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3Button"] )
                self.GUI["2instrument3Box"].pack_start( track_menu(2,'?') )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument3Box"] )
                # + + instrument 4 box
                self.GUI["2instrument4Box"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2instrument4Box"].set_size_request( -1, 137 )
                self.GUI["2instrument4volumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][3], 0, 100, 1, 1, 0 )
                self.GUI["2instrument4volumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 3 )
                self.GUI["2instrument4volumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderInst4.png", self.GUI["2instrument4volumeAdjustment"], 6 )
                self.GUI["2instrument4volumeSlider"].set_inverted(True)
                self.GUI["2instrument4volumeSlider"].set_size_request( 30, -1 )
                self.GUI["2instrument4volumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 3 )
                self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4volumeSlider"], False, False, 0 )
                #self.GUI["2instrument4Button"] = gtk.Button("Inst 4")
                #self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4Button"] )
                self.GUI["2instrument4Box"].pack_start( track_menu(3,'?') )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument4Box"] )
                # + + drum box
                self.GUI["2drumBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2drumBox"].set_size_request( -1, 165 )
                self.GUI["2drumvolumeAdjustment"] = gtk.Adjustment( self._data["track_volume"][4], 0, 100, 1, 1, 0 )
                self.GUI["2drumvolumeAdjustment"].connect( "value_changed", self.onTrackVolumeChanged, 4 )
                self.GUI["2drumvolumeSlider"] = ImageVScale( Config.IMAGE_ROOT+"sliderDrum.png", self.GUI["2drumvolumeAdjustment"], 6 )
                self.GUI["2drumvolumeSlider"].set_inverted(True)
                self.GUI["2drumvolumeSlider"].set_size_request( 30, -1 )
                self.GUI["2drumvolumeAdjustment"].connect( "value-changed", self.handleTrackVolume, 4 )
                self.GUI["2drumBox"].pack_start( self.GUI["2drumvolumeSlider"], False, False, 0 )
                self.GUI["2drumButton"] = gtk.Button("?")
                self.GUI["2drumBox"].pack_start( self.GUI["2drumButton"] )
                #self.GUI["2instrument1Box"].pack_start( track_menu(4,'?') )
                self.GUI["2instrumentPanel"].pack_start( self.GUI["2drumBox"] )
                self.GUI["2leftPanel"].pack_start( self.GUI["2instrumentPanel"], False )
                # + volume panel
                self.GUI["2volumePanel"] = formatRoundBox( RoundHBox(), "#6C9790" )
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

            #------------------------------------------------------------------------
            # right panel
            self.GUI["2rightPanel"] = gtk.VBox()
            if 1: # + track interface
                self.trackInterface = TrackInterface( self.noteDB, self )
                self.noteDB.addListener( self.trackInterface, TrackInterfaceParasite )
                self.trackInterface.set_size_request( -1, 713 )
                self.GUI["2rightPanel"].pack_start( self.trackInterface, False, False, 0 )
                # + tool panel
                self.GUI["2toolPanel"] = gtk.HBox()
                self.GUI["2toolPanel"].set_size_request( -1, 75 )
                # + + tool box
                self.GUI["2toolBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2toolBox"].set_size_request( 146, -1 )
                self.GUI["2toolPointerButton"] = ImageRadioButton( None, Config.IMAGE_ROOT+"pointer.png", Config.IMAGE_ROOT+"pointerDown.png", backgroundFill = "#6C9790" )
                self.GUI["2toolPointerButton"].connect( "clicked", self.handleToolClick , "Default" )
                self.GUI["2toolBox"].pack_start( self.GUI["2toolPointerButton"] )
                self.GUI["2toolPencilButton"] = ImageRadioButton( self.GUI["2toolPointerButton"], Config.IMAGE_ROOT+"pencil.png", Config.IMAGE_ROOT+"pencilDown.png", backgroundFill = "#6C9790" )
                self.GUI["2toolPencilButton"].connect( "clicked", self.handleToolClick , "Draw" )
                self.GUI["2toolBox"].pack_start( self.GUI["2toolPencilButton"] )
                self.GUI["2toolPanel"].pack_start( self.GUI["2toolBox"], False, False )
                self.GUI["2rightPanel"].pack_start( self.GUI["2toolPanel"], False )
                # + + context box (for context sensitive buttons, nothing to do with CAIRO)
                contextWidth = 592
                self.GUI["2contextBox"] = formatRoundBox( RoundFixed(), "#6C9790" )
                self.GUI["2contextBox"].set_size_request( contextWidth, -1 )
                self.GUI["2contextPrevButton"] = gtk.Button("<")
                self.GUI["2contextPrevButton"].connect( "clicked", lambda a1:self.prevContext() )
                self.GUI["2contextBox"].put( self.GUI["2contextPrevButton"], 0, 0 )
                self.GUI["2contextNextButton"] = gtk.Button(">")
                self.GUI["2contextNextButton"].connect( "clicked", lambda a1:self.nextContext() )
                self.GUI["2contextBox"].put( self.GUI["2contextNextButton"], contextWidth-25, 0 )
                # + + + page box
                self.GUI["2pageBox"] = gtk.HBox()
                self.GUI["2pageBox"].set_size_request( contextWidth-50, -1 )
                self.GUI["2pageDeleteButton"] = gtk.Button("Delete")
                self.GUI["2pageDeleteButton"].connect( "clicked", lambda a1:self.removePages() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageDeleteButton"] )
                self.GUI["2pageNewButton"] = gtk.Button("New")
                self.GUI["2pageNewButton"].connect( "clicked", lambda a1:self.addPage() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageNewButton"] )
                self.GUI["2pageDuplicateButton"] = gtk.Button("Duplicate")
                self.GUI["2pageDuplicateButton"].connect( "clicked", lambda a1:self.duplicatePages() )
                self.GUI["2pageBox"].pack_start( self.GUI["2pageDuplicateButton"] )
                self.GUI["2contextBox"].put( self.GUI["2pageBox"], 25, 0 )
                # + + + track box
                self.GUI["2trackBox"] = gtk.HBox()
                self.GUI["2trackBox"].set_size_request( contextWidth-50, -1 )
                self.GUI["2trackDeleteButton"] = gtk.Button("tDelete")
                self.GUI["2trackDeleteButton"].connect( "clicked", lambda a1:self.removePages() )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackDeleteButton"] )
                self.GUI["2trackNewButton"] = gtk.Button("tNew")
                self.GUI["2trackNewButton"].connect( "clicked", lambda a1:self.addPage() )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackNewButton"] )
                self.GUI["2trackDuplicateButton"] = gtk.Button("tDuplicate")
                self.GUI["2trackDuplicateButton"].connect( "clicked", lambda a1:self.duplicatePages() )
                self.GUI["2trackBox"].pack_start( self.GUI["2trackDuplicateButton"] )
                self.GUI["2contextBox"].put( self.GUI["2trackBox"], 25, 0 )
                # + + + note box
                self.GUI["2noteBox"] = gtk.HBox()
                self.GUI["2noteBox"].set_size_request( contextWidth-50, -1 )
                self.GUI["2noteDeleteButton"] = gtk.Button("nDelete")
                self.GUI["2noteDeleteButton"].connect( "clicked", lambda a1:self.removePages() )
                self.GUI["2noteBox"].pack_start( self.GUI["2noteDeleteButton"] )
                self.GUI["2noteNewButton"] = gtk.Button("nNew")
                self.GUI["2noteNewButton"].connect( "clicked", lambda a1:self.addPage() )
                self.GUI["2noteBox"].pack_start( self.GUI["2noteNewButton"] )
                self.GUI["2noteDuplicateButton"] = gtk.Button("nDuplicate")
                self.GUI["2noteDuplicateButton"].connect( "clicked", lambda a1:self.duplicatePages() )
                self.GUI["2noteBox"].pack_start( self.GUI["2noteDuplicateButton"] )
                self.GUI["2contextBox"].put( self.GUI["2noteBox"], 25, 0 )
                self.GUI["2toolPanel"].pack_start( self.GUI["2contextBox"], False )
                # + + transport box
                self.GUI["2transportBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
                self.GUI["2generateButton"] = gtk.Button("G")
                self.GUI["2generateButton"].connect( "clicked", self.handleGenerate, None )
                self.GUI["2transportBox"].pack_start( self.GUI["2generateButton"] )
                self.GUI["2recordButton"] = gtk.ToggleButton("R")
                self.GUI["2transportBox"].pack_start( self.GUI["2recordButton"] )
                self.GUI["2playButton"] = gtk.ToggleButton("P")
                self.GUI["2playButton"].connect( "toggled", self.handlePlay, "Page Play" )
                self.GUI["2transportBox"].pack_start( self.GUI["2playButton"] )
                self.GUI["2loopButton"] = gtk.Button("L")
                self.GUI["2transportBox"].pack_start( self.GUI["2loopButton"] )
                self.GUI["2toolPanel"].pack_start( self.GUI["2transportBox"] )
                # + tune box
                self.GUI["2tuneBox"] = formatRoundBox( RoundVBox(), "#6C9790" )
                self.GUI["2tuneScrolledWindow"] = gtk.ScrolledWindow()
                self.GUI["2tuneScrolledWindow"].set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_NEVER )
                self.GUI["2tuneScrolledWindow"].set_shadow_type(gtk.SHADOW_NONE)
                self.tuneInterface = TuneInterface( self.noteDB, self )
                self.noteDB.addListener( self.tuneInterface, TuneInterfaceParasite, True )
                self.GUI["2tuneScrolledWindow"].add_with_viewport( self.tuneInterface )
                self.GUI["2tuneBox"].pack_start( self.GUI["2tuneScrolledWindow"] )
                self.GUI["2rightPanel"].pack_start( self.GUI["2tuneBox"] )
                self.GUI["2main"].pack_start( self.GUI["2rightPanel"] )

            self.add( self.GUI["2main"] )

            self.generationParametersWindow = GenerationParametersWindow( self.generate, self.variate, self.handleCloseGenerationParametersWindow )

        #===================================================
        # begin initialization
        gtk.EventBox.__init__( self )

        # keyboard variables
        self.kb_active = False
        self.kb_record = False
        self.kb_mono = False
        self.kb_keydict = {}

        # playback params
        self.playing = False
        self.playSource = 'Page'
        self.currentpageId = 0
        self.playingTuneIdx = 0

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
		
        self.csnd.setMasterVolume( self.getVolume() )

        for tid in range(Config.NUMBER_OF_TRACKS):
            self.handleInstrumentChanged( ( tid, self._data['track_inst'][tid] ) )

        first = self.noteDB.addPage( 4 )
        self.displayPage( first )

        self.show_all()  #gtk command

        #self.GUI["2pageBox"].hide()
        self.GUI["2trackBox"].hide()
        self.GUI["2noteBox"].hide()
        self.setContext( CONTEXT.PAGE )

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
            print fps
            self.fpsTotalTime = 0
            self.fpsFrameCount = 0

    #-----------------------------------
    # playback functions
    #-----------------------------------
    def handlePlay( self, widget, data ):

        if widget.get_active():  #play

            #TODO: check for track activation, to not take all
            self.pages_playing = self.tuneInterface.getSelectedIds()

            trackset = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ] )

            notes = []
            if len(trackset) == 0 or len(trackset) == Config.NUMBER_OF_TRACKS:
                for page in self.pages_playing:
                    notes += self.noteDB.getCSNotesByPage( page )
            else:
                for page in self.pages_playing:
                    for track in trackset:
                        notes += self.noteDB.getCSNotesByTrack( page, track )

            self.playing = True
            self.playbackTimeout = gobject.timeout_add( 100, self.onTimeout )

            if len(self.pages_playing) > 1:
                self.displayPage( self.pages_playing[0], self.pages_playing[1] )
            else:
                self.displayPage( self.pages_playing[0] )

            numticks = sum([self.noteDB.getPage(id).ticks for id in self.pages_playing ] )
            print 'play!'
            print 'pages : ', self.pages_playing
            print 'trackset : ', trackset
            print 'numticks : ', numticks
            print 'notes : ', len(notes), 'notes'
            self.csnd.loopClear()
            self.csnd.loopAdd(notes)
            self.csnd.loopSetTick(0)
            self.csnd.loopSetNumTicks( numticks )
            self.csnd.loopSetTempo(self._data['tempo'])
            self.csnd.loopStart()

        else:                    #stop
            gobject.source_remove( self.playbackTimeout )
            if False:
                #This is causing csound to stop working...
                # reimplement this with real CSoundNotes and it should be ok.
                # I suspect the problem is related to the different in the way
                # tracks are handled in this old message, and in CSoundNote.CSound_playNote()
                for track in range( Config.NUMBER_OF_TRACKS ):
                    for i in [Config.INST_TIED, Config.INST_PERC, Config.INST_SIMP]:
                        self.csnd.inputMessage(Config.CSOUND_NOTE_OFF % (i,track))
            self.csnd.loopStop()
            self.playing = False


        self.kb_record = self.GUI["2playButton"].get_active() and self.GUI["2recordButton"].get_active()

    def onTimeout(self):
        self.updateFPS()

        curtick = self.csnd.loopGetTick()
        curIdx =  curtick / ( 4 * Config.TICKS_PER_BEAT) #TODO handle each pages_playing length
        if curIdx + 1 < len(self.pages_playing): predraw = self.pages_playing[curIdx+1]
        else: predraw = self.pages_playing[0]
        self.displayPage( self.pages_playing[curIdx], predraw )

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
        (id, instrumentName) = data
        self._data['track_inst'][id] = instrumentName
        print id, instrumentName
        #self.noteLooper.setInstrument(id, instrumentName)

        #recordButton = self.instrumentRecordButtons[ id ]
        #if instrumentName in Config.RECORDABLE_INSTRUMENTS:
        #    recordButton.show()
        #    recordButton.connect( "clicked",
        #                          self.handleMicRecord,
        #                          Config.RECORDABLE_INSTRUMENT_CSOUND_IDS[ instrumentName ] )
        #else:
        #    recordButton.hide()

    def handleVolume( self, widget ):
    	self._data["volume"] = round( widget.get_value() )
    	self.csnd.setMasterVolume(self._data["volume"])
        img = min(3,int(4*self._data["volume"]/100)) # volume 0-3
        self.GUI["2volumeImage"].set_from_file( Config.IMAGE_ROOT+"volume"+str(img)+".png" )

    def handleTrackVolume( self, widget, track ):
    	self._data["track_volume"][track] = round( widget.get_value() )

    def handleTempo( self, widget ):
        self._data['tempo'] = round( widget.get_value() )
        img = min(7,int(8*(self._data["tempo"]-widget.lower)/(widget.upper-widget.lower)))+1# tempo 1-8
        self.GUI["2tempoImage"].set_from_file( Config.IMAGE_ROOT+"tempo"+str(img)+".png" )


    def handleToolClick( self, widget, mode ):
        if widget.get_active(): self.trackInterface.setInterfaceMode( mode )

    def onKeyboardButton( self, widget, data ):
        self.kb_active = widget.get_active()

    def onKeyboardRecordButton( self, widget, data ):

        self.kb_record = self.GUI["playButton"].get_active() and self.GUI["2recordButton"].get_active()

    #-----------------------------------
    # generation functions
    #-----------------------------------
    def handleGenerate( self, widget, data ):
        #if widget.get_active():
            self.generationParametersWindow.show_all()
        #else:
        #    self.handleCloseGeneratonParametersWindow()

    def handleCloseGenerationParametersWindow( self, widget = None, data = None ):
        self.generationParametersWindow.hide_all()
        #self.generateButton.set_active( False )

    def recompose( self, algo, params):

        # this seems excessive!?
        dict = {}
        for t in range(Config.NUMBER_OF_TRACKS):
            dict[t] = {}
            for p in range(Config.NUMBER_OF_PAGES):
                dict[t][p] = []

        if self.trackSelected == [ 0 for i in range(Config.NUMBER_OF_TRACKS) ]:
            newtracks = set(range(Config.NUMBER_OF_TRACKS))
        else:
            newtracks = set( [ i for i in range(Config.NUMBER_OF_TRACKS) if self.trackSelected[i] ] )
        newpages  = self.tuneInterface.getSelectedIds()

        algo(
                params,
                self._data['track_volume'][:],
                self._data['track_inst'][:],
                self._data['tempo'],
                4,  #beats per page TODO: talk to olivier about handling pages of different sizes
                newtracks,
                newpages,
                dict)

        # filter & fix input
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    intdur = int(note.duration)
                    note.duration = intdur
                    note.pageId = page
                    note.trackId = track

        # add the new notes
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += dict[tid][pid]

        # delete the notes and add the new
        stream = []
        for page in newpages:
            for track in newtracks:
                self.noteDB.deleteNotesByTrack( page, track )
                stream += [ page, track, len(dict[track][page]) ]
                stream += dict[track][page]
        stream += [-1]
        self.noteDB.addNotes( stream )

        self.handleCloseGenerationParametersWindow( None, None )

    def generate( self, params ):
        self.recompose( generator1, params)

    def variate( self, params ):
        self.recompose( variate, params)

    #=======================================================
    # Track Functions

    def toggleTrack( self, trackN, exclusive ):
        if exclusive:
            for i in range(Config.NUMBER_OF_TRACKS):
                self.trackSelected[i] = False
            self.trackSelected[trackN] = True
            self.trackInterface.trackToggled() # invalidate whole page
            self.setContextState( CONTEXT.TRACK, True )
            self.setContext( CONTEXT.TRACK )
        else:
            self.trackSelected[trackN] = not self.trackSelected[trackN]
            self.trackInterface.trackToggled( trackN )
            for i in range(Config.NUMBER_OF_TRACKS):
                if self.trackSelected[i]:
                    self.setContextState( CONTEXT.TRACK, True )
                    self.setContext( CONTEXT.TRACK )
                    return
            self.setContextState( CONTEXT.TRACK, False )

    def getTrackSelected( self, trackN ):
        return self.trackSelected[trackN]

    #=======================================================
    # NoteDB notifications

    def notifyPageAdd( self, id, at ):
        self.displayPage( id )

    def notifyPageDelete( self, which, safe ):
        if self.displayedPage in which:
            self.displayPage( safe )

    def notifyPageDuplicate( self, new, at ):
        self.displayPage( new[self.displayedPage] )

    def notifyPageMove( self, which, low, high ):
        return

    #-----------------------------------
    # tune functions
    #-----------------------------------

    def scrollTune( self, scroll ):
        adj = self.GUI["2tuneScrolledWindow"].get_hadjustment()
        adj.set_value( scroll )

    def displayPage( self, pageId, nextId = -1 ):

        self.displayedPage = pageId

        adj = self.GUI["2tuneScrolledWindow"].get_hadjustment()
        scroll = self.tuneInterface.displayPage( pageId, adj.get_value() )
        if scroll >= 0: adj.set_value(scroll)

        self.trackInterface.displayPage( pageId, nextId )

    def addPage( self, after = -1, beats = False ):

        if after == -1: after = self.tuneInterface.getLastSelected()
        if not beats: beats = self.noteDB.getPage( self.displayedPage ).beats

        self.noteDB.addPage( beats, after )

    def duplicatePages( self, after = -1, pageIds = False ):

        if after == -1: after = self.tuneInterface.getLastSelected()
        if not pageIds: pageIds = self.tuneInterface.getSelectedIds()

        self.noteDB.duplicatePages( pageIds, after )

    def removePages( self, pageIds = -1 ):

        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()

        self.noteDB.deletePages( pageIds )

    #-----------------------------------
    # load and save functions
    #-----------------------------------
    def handleSave(self, widget, data):
        pass

        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_SAVE, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try:
                print 'INFO: serialize to file %s' % chooser.get_filename()
                f = open( chooser.get_filename(), 'w')
                pickle.dump( self._data, f )
                f.close()
            except IOError:
                print 'ERROR: failed to serialize to file %s' % chooser.get_filename()

        chooser.destroy()

    def handleLoad(self, widget, data):
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try:
                print 'INFO: unserialize from file %s' % chooser.get_filename()
                f = open( chooser.get_filename(), 'r')
                self._data = pickle.load( f )
            except IOError:
                print 'ERROR: failed to unserialize from file %s' % chooser.get_filename()

        chooser.destroy()
        print 'ERROR: MainWindow::handleLoad() not implemented'

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
    def onKeyPress(self,widget,event):

        Config.ModKeys.keyPress( event.hardware_keycode )

        key = event.hardware_keycode

        if key == 53 and Config.ModKeys.ctrlDown: # q == 53
            self.destroy( self )

        if not self.kb_active:
            return
        if self.kb_record:
            self.kb_mono = False

        # If the key is already in the dictionnary, exit function (to avoir key repeats)
        if self.kb_keydict.has_key(key):
                return
        # Assign on which track the note will be created according to the number of keys pressed
        track = len(self.kb_keydict)+10
        if self.kb_mono:
            track = 10
        # If the pressed key is in the keymap
        if KEY_MAP.has_key(key):
            # CsoundNote parameters
            onset = self.getCurrentTick()
            pitch = KEY_MAP[key]
            duration = -1
            instrument = self._data['track_inst'][0]
            # get instrument from top selected track if a track is selected
            if self.getSelectedtrackIds():
                instrument = self._data['track_inst'][min(self.getSelectedtrackIds())]

            if instrument == 'drum1kit':
                if GenerationConfig.DRUMPITCH.has_key( pitch ):
                    instrument = Config.DRUM1INSTRUMENTS[ GenerationConfig.DRUMPITCH[ pitch ] ]
                else:
                    instrument = Config.DRUM1INSTRUMENTS[ pitch ]
                pitch = 36
                duration = 100

            if Config.INSTRUMENTS[instrument].csoundInstrumentID == 102:
                duration = 100

            # Create and play the note
            self.kb_keydict[key] = CSoundNote(onset = 0,
                                            pitch = pitch,
                                            amplitude = 1,
                                            pan = 0.5,
                                            duration = duration,
                                            trackId = track,
                                            fullDuration = False,
                                            instrument = instrument,
                                            instrumentFlag = instrument)
            self.kb_keydict[key].playNow()

    def onKeyRelease(self,widget,event):

        Config.ModKeys.keyRelease( event.hardware_keycode )

        if not self.kb_active:
            return
        key = event.hardware_keycode

        if KEY_MAP.has_key(key):
            self.kb_keydict[key].duration = 0
            self.kb_keydict[key].amplitude = 0
            self.kb_keydict[key].nchanges += 1
            self.kb_keydict[key].playNow()
            if self.kb_record and len( self.getSelectedtrackIds() ) != 0:
                print "ERROR: discarding recorded note "
                if False:
                    curtick = something
                    self.kb_keydict[key].duration = curtick - self.kb_keydict[key].onset
                    self.kb_keydict[key].amplitude = 1.0
                    self.kb_keydict[key].nchanges += 1
            del self.kb_keydict[key]

    def delete_event( self, widget, event, data = None ):
        return False

    def destroy( self, widget ):

        if Config.DEBUG:
            print TP.PrintAll()

        gtk.main_quit()

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

    def setContext( self, context ):

        if self.context == context: return

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
