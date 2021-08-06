import pygtk
pygtk.require( '2.0' )
import gtk 

import gobject
from Util.ThemeWidgets import *

import time

import Config

from Edit.MixerWindow import MixerWindow
from Generation.GenerationConstants import GenerationConstants
from Generation.GenerationParametersWindow import GenerationParametersWindow
from Edit.TrackInterface import TrackInterface
from Edit.TuneInterface import TuneInterface

from Util.Profiler import TP

from Generation.Generator import generator1, variate

from Util.NoteLooper import *

def note_from_CSoundNote( csnote ):
    note = {}
    note['onset'] = csnote.onset
    note['pitch'] = csnote.pitch
    note['amplitude'] = csnote.amplitude
    note['pan'] = csnote.pan
    note['duration'] = csnote.duration
    note['noteId'] = csnote.noteId
    note['trackId'] = csnote.trackId
    note['pageId'] = csnote.pageId
    note['fullDuration'] = csnote.fullDuration
    note['attack'] = csnote.attack
    note['decay'] = csnote.decay
    note['reverbSend'] = csnote.reverbSend
    note['filterType'] = csnote.filterType
    note['filterCutoff'] = csnote.filterCutoff
    note['tied'] = csnote.tied
    note['overlap'] = csnote.overlap
    note['instrumentFlag'] = csnote.instrumentFlag

    return note
#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.EventBox ):
        
    def __init__( self, CSoundClient ):
        self.csnd = CSoundClient
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
            # + instrument panel
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
            # + track interface
            self.trackInterface = TrackInterface( self.onTrackInterfaceNoteDrag )
            self.trackInterface.set_size_request( -1, 713 )
            self.GUI["2rightPanel"].pack_start( self.trackInterface, False, False, 0 )
            # + tool panel
            self.GUI["2toolPanel"] = gtk.HBox()
            self.GUI["2toolPanel"].set_size_request( -1, 75 )
            # + + page box
            self.GUI["2pageBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
            self.GUI["2pageDeleteButton"] = gtk.Button("Delete")
            self.GUI["2pageDeleteButton"].connect( "clicked", lambda a1:self.removePages() )
            self.GUI["2pageBox"].pack_start( self.GUI["2pageDeleteButton"] )
            self.GUI["2pageNewButton"] = gtk.Button("New")
            self.GUI["2pageNewButton"].connect( "clicked", lambda a1:self.addPage() )
            self.GUI["2pageBox"].pack_start( self.GUI["2pageNewButton"] )
            self.GUI["2pageDuplicateButton"] = gtk.Button("Duplicate")
            self.GUI["2pageDuplicateButton"].connect( "clicked", lambda a1:self.duplicatePages() )
            self.GUI["2pageBox"].pack_start( self.GUI["2pageDuplicateButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2pageBox"], False )
            # + + transport box
            self.GUI["2transportBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
            #self.GUI["2pageBox"].set_size_request( 120, -1 )
            self.GUI["2generateButton"] = gtk.Button("Gen")
            self.GUI["2generateButton"].connect( "clicked", self.handleGenerate, None )
            self.GUI["2transportBox"].pack_start( self.GUI["2generateButton"] )
            self.GUI["2keyboardButton"] = gtk.Button("KB")
            self.GUI["2transportBox"].pack_start( self.GUI["2keyboardButton"] )
            self.GUI["2recordButton"] = gtk.Button("Rec")
            self.GUI["2transportBox"].pack_start( self.GUI["2recordButton"] )
            self.GUI["2playButton"] = gtk.ToggleButton("Play")
            self.GUI["2playButton"].connect( "toggled", self.handlePlay, "Page Play" )
            self.GUI["2transportBox"].pack_start( self.GUI["2playButton"] )
            self.GUI["2loopButton"] = gtk.Button("Loop")
            self.GUI["2transportBox"].pack_start( self.GUI["2loopButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2transportBox"] )
            # + + tool box
            self.GUI["2toolBox"] = formatRoundBox( RoundHBox(), "#6C9790" )
            self.GUI["2toolPencilButton"] = gtk.Button("Pencil")
            self.GUI["2toolBox"].pack_start( self.GUI["2toolPencilButton"] )
            self.GUI["2toolPointerButton"] = gtk.Button("Pointer")
            self.GUI["2toolBox"].pack_start( self.GUI["2toolPointerButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2toolBox"] )
            self.GUI["2rightPanel"].pack_start( self.GUI["2toolPanel"], False )
            # + tune box
            self.GUI["2tuneBox"] = formatRoundBox( RoundVBox(), "#6C9790" )
            self.GUI["2tuneScrolledWindow"] = gtk.ScrolledWindow()
            self.GUI["2tuneScrolledWindow"].set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_NEVER )
            self.GUI["2tuneScrolledWindow"].set_shadow_type(gtk.SHADOW_NONE)
            self.tuneInterface = TuneInterface( self )
            self.GUI["2tuneScrolledWindow"].add_with_viewport( self.tuneInterface )
            self.GUI["2tuneBox"].pack_start( self.GUI["2tuneScrolledWindow"] )
            self.GUI["2rightPanel"].pack_start( self.GUI["2tuneBox"] )
            self.GUI["2main"].pack_start( self.GUI["2rightPanel"] )
            
            self.add( self.GUI["2main"] )
            
            self.addPage( 0, 4 ) # yeah! a page!
            
            
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
            self._data['ticks_per_sec'] = self._data['tempo'] * 0.2 # 12 BPM / 60 SPM
            self._data['tune'] = []
            self._data['notebin'] = []
            self._noteId = {}
            self._noteIdBase = 0
            
            self._data["pages"] = []
            self._pageIdBase = 0

        # these helper functions do not 
        # run in any particular order.... 
        # TODO: give these functions better names, put them in execution order, cut hierarchy

        def setupGUI( ):
            
            self.volumeFunctions = {}

            self.generateParametersWindow = GenerationParametersWindow( self.generate, self.variate, self.handleCloseGenerateWindow )
            
            setupGlobalControls()
            setupPageControls()
            setupTrackControls()
            #setupMainView()

            #self.tuneView = TuneView( self.onTuneViewSelect )
            #self.pageBankView = PageBankView( self.onPageBankSelect, self.onPageBankDrop )
                    
            self.mainWindowBox = gtk.HBox( False, 5 )

            self.globalControlsBox = gtk.VBox( False )
     
            self.fpsText = gtk.Label( "" )
            self.globalControlsBox.pack_start( self.fpsText, False )
            self.globalControlsBox.pack_start( self.globalControlsFrame, True )

            self.mainWindowBox.pack_start( self.globalControlsBox, False )

            
            controlsBox = gtk.VBox( False, 5 )
            controlsBox.pack_start( self.trackControlsBox, False )
            #TODO: this Label is temporary!!
            controlsBox.pack_start( gtk.Label( "" ), True )
            controlsBox.pack_start( self.pageControlsBox, False )
            self.mainWindowBox.pack_start( controlsBox, False )
            
            self.trackPagesBox = gtk.VBox( False )
            #self.trackPagesBox.pack_start( self.mainView, True )
            #self.trackPagesBox.pack_start( self.tuneView, False )
            #self.trackPagesBox.pack_start( self.pageBankView, False, True, 5 )
            
            self.mainWindowBox.pack_start( self.trackPagesBox )
            
            #self.add( self.mainWindowBox )

        # contains TAM-TAM and OLPC labels, as well as the volume and tempo sliders
        def setupGlobalControls( ):
            self.globalControlsFrame = gtk.Frame()
            self.globalControlsFrame.set_shadow_type( gtk.SHADOW_ETCHED_OUT )
            
            self.globalControlsBox = gtk.VBox()
            
            self.tamTamLabel = gtk.Label( "     TAM - TAM     " )
            self.globalControlsBox.pack_start( self.tamTamLabel )
            

            self.beatsPerPageAdjustment = gtk.Adjustment( 4, 1, 8, 1, 1, 0 )
            self.beatsPerPageAdjustment.connect( "value_changed", self.updateNumberOfBars, None )
            self.barsSlider = gtk.VScale( self.beatsPerPageAdjustment )
            self.barsSlider.set_draw_value( False )
            self.barsSlider.set_digits( 0 )
            self.barsSlider.set_inverted( True )
            self.barsSlider.set_increments( 1, 1 )
            self.barsSlider.set_update_policy( gtk.UPDATE_DELAYED )
            #self.mainSlidersBox.pack_start( self.barsSlider )

            #self.globalControlsBox.pack_start( self.mainSlidersBox )

            self.olpcLabel = gtk.Label( "OLPC" )
            self.globalControlsBox.pack_start( self.olpcLabel )
            
            self.saveButton = gtk.Button("Save")
            self.loadButton = gtk.Button("Open")
            
            fileBox = gtk.HBox()
            fileBox.pack_start( self.saveButton, True )
            fileBox.pack_start( self.loadButton, True )
            self.globalControlsBox.pack_start( fileBox, False )
            self.saveButton.connect("clicked", self.handleSave, None )
            self.loadButton.connect("clicked", self.handleLoad, None )
            self.globalControlsFrame.add( self.globalControlsBox )

        def setupPageControls( ):
            self.pageControlsBox = gtk.VBox( False )

            self.generateButton = gtk.ToggleButton( "Generate" )
            self.playButton = gtk.ToggleButton( "Play" )
            self.keyboardButton = gtk.ToggleButton( "K" )
            self.keyboardRecordButton = gtk.ToggleButton( "Record" )
            
            self.pageControlsBox.pack_start( self.generateButton, False )
            self.pageControlsBox.pack_start( self.playButton, False )
            
            keyboardBox = gtk.HBox()
            keyboardBox.pack_start( self.keyboardButton, False )
            keyboardBox.pack_start( self.keyboardRecordButton )
            self.pageControlsBox.pack_start( keyboardBox, False )
            
            self.generateButton.connect( "toggled", self.handleGenerate, None )
            self.playButton.connect( "toggled", self.handlePlay, "Page Play" )
            self.keyboardButton.connect( "toggled", self.onKeyboardButton, None )
            self.keyboardRecordButton.connect( "toggled", self.onKeyboardRecordButton, None )
            
        def setupTrackControls( ):
            self.trackControlsBox = gtk.VBox()
            self.instrumentRecordButtons = {}
            for trackId in range( Config.NUMBER_OF_TRACKS):
                trackControlsBox = gtk.HBox()

                #setup instrument controls
                instrumentControlsBox = gtk.VBox()
                
                instrumentMenu = gtk.Menu()
                instrumentMenuItem = gtk.MenuItem( "Instrument" )
                instrumentMenuItem.set_submenu( instrumentMenu )
                
                instrumentNames = []
                instrumentFolderNames = Config.INSTRUMENTS.keys()
                for instrumentName in instrumentFolderNames:
                    if not instrumentName[0: 4] == 'drum':
                       instrumentNames.append( instrumentName )

                instrumentNames.append( 'drum1kit' )
                instrumentNames.sort()
                for instrumentName in instrumentNames:
                    menuItem = gtk.MenuItem( instrumentName )
                    menuItem.connect_object( "activate", self.handleInstrumentChanged, ( trackId, instrumentName ) )
                    instrumentMenu.append( menuItem )
                    
                instrumentMenuBar = gtk.MenuBar()
                instrumentMenuBar.append( instrumentMenuItem )
                instrumentControlsBox.pack_start( instrumentMenuBar )
                
                recordButton = gtk.Button()
                recordButton.set_size_request( 15, 15 )
                self.instrumentRecordButtons[ trackId ] = recordButton
                instrumentControlsBox.pack_start( recordButton, False )
                
                trackControlsBox.pack_start( instrumentControlsBox )

                #setup playback controls
                playbackControlsBox = gtk.VBox()
                
                muteButton = gtk.ToggleButton()
                muteButton.set_size_request( 15, 15 )
                playbackControlsBox.pack_start( muteButton, False )
                
                volumeAdjustment = gtk.Adjustment( 0.8, 0, 1, 0.01, 0.01, 0 )
                volumeAdjustment.connect( "value_changed", self.onTrackVolumeChanged, trackId )
                self.volumeFunctions[ trackId ] = volumeAdjustment.get_value
                volumeSlider = gtk.VScale( volumeAdjustment )
                volumeSlider.set_update_policy( 0 )
                volumeSlider.set_digits( 2 )
                volumeSlider.set_draw_value( False )
                volumeSlider.set_digits( 0 )
                volumeSlider.set_inverted( True )
                playbackControlsBox.pack_start( volumeSlider, True )
                            
                trackControlsBox.pack_start( playbackControlsBox )

                trackName = "Track %i" % trackId
                muteButton.connect( "toggled", self.onMuteTrack, trackId )

                self.trackControlsBox.pack_start( trackControlsBox )
    
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
        
        init_data()   #above
        setupGUI()    #above #TEMP
        init_GUI()    #above
		
        self.noteLooper = NoteLooper( 
                0.2,
                Config.PLAYER_TEMPO * 0.2   #0.2 currently converts beats per second to seconds_per_tick
                )
        self.csnd.startTime()
        self.noteLooper.startTime()
        time.sleep(0.001)

        self.csnd.setMasterVolume( self.getVolume() )
        
        #for pageId in range( GUIConfig.NUMBER_OF_PAGE_BANK_ROWS * GUIConfig.NUMBER_OF_PAGE_BANK_COLUMNS ):
        #    self.pageBankView.addPage( pageId, False )
        
        for tid in range(Config.NUMBER_OF_TRACKS):
            self.handleInstrumentChanged( ( tid, self._data['track_inst'][tid] ) )

        #self.handleConfigureEvent( None, None ) # needs to come after pages have been added in initialize()
        
        self.show_all()  #gtk command
    
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

        def shutOffCSound():
            for track in range( Config.NUMBER_OF_TRACKS ):
                for i in range( 3 ):
                    csoundInstrument = i + 5001
                    self.csnd.sendText( Config.PLAY_NOTE_OFF_COMMAND % ( csoundInstrument, track ) )

        if widget.get_active():  #play

            #TODO: check for track activation, to not take all
            trackset = set(range(Config.NUMBER_OF_TRACKS))

            self.noteLooper.clear()       #erase all loaded notes
            notes = []

            pagedelay = 0
            self.pages_playing = self.tuneInterface.getSelectedIds()
            for p in self.pages_playing:
                notes += [(n['onset'] + pagedelay, n) for n in self._data['notebin'] \
                        if n['pageId']==p and n['trackId'] in trackset ]
                pagedelay += self._data['page_beats'][p] * Config.TICKS_PER_BEAT

            self.noteLooper.insert(notes)
            self.noteLooper.setDuration( pagedelay )
            self.noteLooper.setTick(0)    #TODO: get playback head position
            self.noteLooper.setRate( self._data['ticks_per_sec'] )

            cmds = self.noteLooper.next()
            for c in cmds: self.csnd.sendText( c )
            time.sleep(0.001)
            self.playbackTimeout = gobject.timeout_add( 100, self.onTimeout )
            self.playing = True

        else:                    #stop
            gobject.source_remove( self.playbackTimeout )
            shutOffCSound()
            self.playing = False


        self.kb_record = self.playButton.get_active() and self.keyboardRecordButton.get_active() and self.keyboardButton.get_active()

    def onTimeout(self):

        cmds = self.noteLooper.next()
        for c in cmds: self.csnd.sendText( c )

        self.updateFPS()

        curtick = self.noteLooper.getCurrentTick(0,True, time.time())
        curIdx =  curtick / ( 4 * Config.TICKS_PER_BEAT) #TODO handle each pages_playing length
        self.tuneInterface.displayPage( self.pages_playing[curIdx], 0 )
        self.trackInterface.displayPage(self.pages_playing[curIdx], 4 )  #TODO: use page_beats

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

        recordButton = self.instrumentRecordButtons[ id ]
        if instrumentName in Config.RECORDABLE_INSTRUMENTS:
            recordButton.show()
            recordButton.connect( "clicked", 
                                  self.handleMicRecord,
                                  Config.RECORDABLE_INSTRUMENT_CSOUND_IDS[ instrumentName ] )
        else:
            recordButton.hide()

    def handleVolume( self, widget ):
    	self._data["volume"] = round( widget.get_value() )
    	self.csnd.setMasterVolume(self._data["volume"])
        img = min(3,int(4*self._data["volume"]/100)) # volume 0-3
        self.GUI["2volumeImage"].set_from_file( Config.IMAGE_ROOT+"volume"+str(img)+".png" )
    
    def handleTrackVolume( self, widget, track ):
    	self._data["track_volume"][track] = round( widget.get_value() )
        
    def handleTempo( self, widget ):
        self._data['tempo'] = round( widget.get_value() )
        self._data['ticks_per_sec'] = self._data['tempo'] * 0.2 # 12 BPM / 60 SPM
        self.noteLooper.setRate(self._data['ticks_per_sec'])
        img = min(7,int(8*(self._data["tempo"]-widget.lower)/(widget.upper-widget.lower)))+1# tempo 1-8
        self.GUI["2tempoImage"].set_from_file( Config.IMAGE_ROOT+"tempo"+str(img)+".png" )
        

    def onKeyboardButton( self, widget, data ):
        self.kb_active = widget.get_active()
        
    def onKeyboardRecordButton( self, widget, data ):
        if not self.kb_active:
            self.keyboardButton.set_active( True )
            
        self.kb_record = self.playButton.get_active() and self.keyboardRecordButton.get_active()

    def onScoreChange( self, action, noteList ):
        pass

    def onTrackInterfaceNoteDrag( self, dragList ):
        for (id, pitch, onset, duration) in dragList:
            n = self._noteId[id]
            n['pitch'] = pitch
            n['onset'] = onset
            n['duration'] = duration

    #-----------------------------------
    # generation functions
    #-----------------------------------
    def handleGenerate( self, widget, data ):
        #if widget.get_active():
            self.generateParametersWindow.show_all()
        #else:
        #    self.handleCloseGenerateWindow()
            
    def handleCloseGenerateWindow( self, widget = None, data = None ):
        self.generateParametersWindow.hide_all()
        #self.generateButton.set_active( False )
    
    def addNotesToTrackInterface( self, notes ):
        pageList = []
        trackList = []
        noteList = []
        csnoteList = []
        beatList = []

        for n in notes:
            pageList.append( n["pageId"] )
            trackList.append( n["trackId"] )
            noteList.append( n["noteId"] )
            csnoteList.append( n )
            beatList.append( p["beats"] for p in self._data["pages"] if p["pageId"] == n["pageId"] )
         
        self.trackInterface.addNotes( 
                {   "page":pageList,
                    "track":trackList,
                    "note":noteList,
                    "csnote":csnoteList,
                    "beatCount":beatList},
                len(notes) )
                
    def removeNotesFromTrackInterface( self, notes ):
        pageList = []
        trackList = []
        noteList = []
        
        for n in notes:
            pageList.append( n["pageId"] )
            trackList.append( n["trackId"] )
            noteList.append( n["noteId"] )
            
        self.trackInterface.deleteNotes( 
                {   "page":pageList,
                    "track":trackList,
                    "note":noteList },
                len(notes) )
                
    def recompose( self, algo, params):
        def none_to_all(tracks):
            print 'tracks = ',tracks
            if tracks == []: return set(range(0,Config.NUMBER_OF_TRACKS))
            else:            return set(tracks)

        dict = {}
        for t in range(Config.NUMBER_OF_TRACKS):
            dict[t] = {}
            for p in range(Config.NUMBER_OF_PAGES):
                dict[t][p] = []

        newtracks = none_to_all( self.trackInterface.getSelectedTracks())
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

        # filter for invalid input
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    intdur = int(note.duration)
                    if intdur != note.duration:
                        print "Invalid note duration!"
                    note.duration = intdur
                    note.pageId = page
                    note.trackId = track
                    self._noteIdBase = self._noteIdBase + 1
                    while self._noteIdBase in self._noteId: self._noteIdBase = self._noteIdBase + 1
                    note.noteId = self._noteIdBase

        #add notes to self._data
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += [note_from_CSoundNote(n) for n in dict[tid][pid]]

        for n in newnotes:
            self._noteId[n['noteId']] = n

        #delete the old pages & tracks!
        togo = [n for n in self._data['notebin'] if (n['trackId'] in newtracks and n['pageId'] in newpages)  ]
        self.trackInterface.deleteNotes( 
                {   "page": [n['pageId'] for n in togo],
                    "track":[n['trackId'] for n in togo] ,
                    "note": [n['noteId'] for n in togo]},
                len(togo))
        for n in togo:
            del self._noteId[n['noteId']]

        self._data['notebin'] = \
                [n for n in self._data['notebin'] if not (n['trackId'] in newtracks and n['pageId'] in newpages)  ] \
                + newnotes

        self.addNotesToTrackInterface( newnotes )

        self.handleCloseGenerateWindow( None, None )
        #self.handleConfigureEvent( None, None )

    def generate( self, params ):
        self.recompose( generator1, params)

    def variate( self, params ):
        self.recompose( variate, params)
        
    #-----------------------------------
    # tune functions
    #-----------------------------------    
    
    def scrollTune( self, scroll ):
        adj = self.GUI["2tuneScrolledWindow"].get_hadjustment()
        adj.set_value( scroll )
    
    def displayPage( self, pageId, beats = -1 ):
        
        if beats == -1:
            for page in self._data["pages"]:
                if pageId == page["pageId"]: break
            beats = page["beats"]
            
        self.displayedPage = pageId
        self.displayedBeats = beats
        
        adj = self.GUI["2tuneScrolledWindow"].get_hadjustment()
        scroll = self.tuneInterface.displayPage( pageId, adj.get_value() )
        if scroll >= 0: adj.set_value(scroll)
                
        self.trackInterface.displayPage( pageId, beats )    
    
    def addPage( self, insert = -1, beats = -1 ):
    
        if insert == -1: insert = self.tuneInterface.getLastSelected() + 1
        if beats == -1: beats = self.displayedBeats
        
        pageId = self._pageIdBase
        self._pageIdBase += 1
        while pageId in [ x["pageId"] for x in self._data["pages"] ]:
            pageId = self._pageIdBase
            self._pageIdBase += 1
        
        newpage = { "pageId": pageId, "beats": beats }
        self._data["pages"].insert( insert, newpage )
        
        self.tuneInterface.clearSelection()
        self.tuneInterface.insertPage( pageId, insert )
        
        self.displayPage( pageId, beats )
    
    def duplicatePages( self, insert = -1, pageIds = -1 ):
        
        if insert == -1: insert = self.tuneInterface.getLastSelected() + 1
        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()
        
        nextDisplay = -1
        nextBeats = -1
        newpages = []
        for id in pageIds:
            for page in self._data["pages"]:
                if id == page["pageId"]: break
            
            pageId = self._pageIdBase
            self._pageIdBase += 1
            while pageId in [ x["pageId"] for x in self._data["pages"] ]:
                pageId = self._pageIdBase
                self._pageIdBase += 1
            
            if self.displayedPage == id:
                nextDisplay = pageId
                nextBeats = page["beats"]
            
            newnotes = [ n.copy() for n in self._data["notebin"] if n["pageId"] == id ]
            for n in newnotes: n["pageId"] = pageId
            
            self._data["notebin"].extend( newnotes )
            self.addNotesToTrackInterface( newnotes )
            
            newpages.append( { "pageId": pageId, "beats": page["beats"] } )
            
        self.tuneInterface.insertPages( [ p["pageId"] for p in newpages ], insert, True, True )
       
        for page in newpages:
            self._data["pages"].insert( insert, page )
            insert += 1
        
        self.displayPage( nextDisplay, nextBeats )
        
    def removePages( self, pageIds = -1 ):
        
        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()
        
        next = self.tuneInterface.getLastSelected() + 1
        if next == len(self._data["pages"]):
            next -= 2
            while next >= 0 and self._data["pages"][next]["pageId"] in pageIds:
                next -= 1
        
        if next == -1:
            self.addPage()
        else:
            self.tuneInterface.selectPage( self._data["pages"][next]["pageId"], True ) # exclusive select
            self.displayPage( self._data["pages"][next]["pageId"], self._data["pages"][next]["beats"] )
    
        self.tuneInterface.removePages( pageIds )
        self.tuneInterface.selectPage( self._data["pages"][next]["pageId"] )
    
        for id in pageIds:
            for page in self._data["pages"]:
                if id == page["pageId"]: break
    
            self._data["pages"].remove(page)
            
            notes = [ n for n in self._data["notebin"] if n["pageId"] == id ]
            self.removeNotesFromTrackInterface( notes )
            self._data["notebin"] = [ n for n in self._data["notebin"] if n["pageId"] != id ]
            
    def movePages( self, insert = -1, pageIds = -1 ):
        
        if pageIds == -1: pageIds = self.tuneInterface.getSelectedIds()
        if insert == -1: insert = self.tuneInterface.getLastSelected() + 1 - len(pageIds)
        
        for id in pageIds:
            remove = 0
            for page in self._data["pages"]:
        	    if id == page["pageId"]: break
        	    remove += 1
            
            self.tuneInterface.movePage( remove, insert )
            
            if remove == insert: 
                insert += 1
                continue
            elif remove < insert:
                if remove == insert-1: continue
                insert -= 1
	
            self._data["pages"].pop(remove)
            self._data["pages"].insert( insert, page )

            insert += 1

    #-----------------------------------
    # load and save functions
    #-----------------------------------
    def handleSave(self, widget, data):
        #gtk.main_quit()
        self.csnd.initialize(False)
        return

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
        #gtk.main_quit()
        self.csnd.initialize(True)
        return
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: unserialize from file %s' % chooser.get_filename()
                f = open( chooser.get_filename(), 'r')
                self._data = pickle.load( f )
            except IOError: 
                print 'ERROR: failed to unserialize from file %s' % chooser.get_filename()

        chooser.destroy()
        print 'TODO: update misc. program state to new music'

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
            self.kb_keydict[key] = Note.note_new(onset = 0, 
                                            pitch = pitch, 
                                            amplitude = 1, 
                                            pan = 0.5, 
                                            duration = duration, 
                                            trackId = track, 
                                            fullDuration = False, 
                                            instrument = instrument, 
                                            instrumentFlag = instrument)
            Note.note_play(self.kb_keydict[key])
                
    def onKeyRelease(self,widget,event):

        Config.ModKeys.keyRelease( event.hardware_keycode )

        if not self.kb_active:
            return
        key = event.hardware_keycode 
        
        if KEY_MAP.has_key(key):
            self.kb_keydict[key]['duration'] = 0
            self.kb_keydict[key]['amplitude'] = 0
            self.kb_keydict[key]['dirty'] = True
            Note.note_play(self.kb_keydict[key])
            self.kb_keydict[key]['duration'] = self.getCurrentTick() - self.kb_keydict[key]['onset']
            #print "onset",self.kb_keydict[key].onset
            #print "dur",self.kb_keydict[key].duration
            if self.kb_record and len( self.getSelectedtrackIds() ) != 0:
                self.kb_keydict[key]['amplitude'] = 1
                self.getTrackDictionary()[min(self.getSelectedtrackIds())][self.getCurrentpageIdCallback()].append(self.kb_keydict[key])
                print 'ERROR: keyboard release... callbacks?'
            del self.kb_keydict[key]

    def delete_event( self, widget, event, data = None ):
        return False

    def destroy( self, widget ):

        if Config.DEBUG:
            print TP.PrintAll()

        gtk.main_quit()
    
    def updateNumberOfBars( self, widget = None, data = None ):
        self.trackInterface.updateBeatCount( int(round( self.beatsPerPageAdjustment.value)) )
        
    def updateSelection( self ):
        print 'WARNING: wtf is this?'

    def updatePage( self ):
        TP.ProfileBegin( "updatePage" )

        if self.playingTune:
            self.tuneView.selectPage( self.currentpageId, False )
            self.pageBankView.selectPage(self.pageBankView.NO_PAGE,False)
        else:
            self.tuneView.deselectAll()
            self.tuneView.selectPage(self.tuneView.NO_PAGE,False)

        # temp        
        self.trackInterface.displayPage(0,int(round( self.beatsPerPageAdjustment.value)))

        self.handleConfigureEvent( None, None )

        print TP.ProfileEndAndPrint( "updatePage" )
        
        
    # handle resize (TODO: this could probably be done more efficiently)
    #def handleConfigureEvent( self, widget, event ):
    #    mainBoxRect = self.trackPagesBox.get_allocation()
        
        #self.tuneView.set_size_request( mainBoxRect.width, GUIConfig.PAGE_HEIGHT + 
        #                                                   self.tuneView.get_hscrollbar().get_allocation().height + 10 )
        #self.tuneView.show_all()
    
        #self.pageBankView.set_size_request( mainBoxRect.width, GUIConfig.PAGE_HEIGHT * GUIConfig.NUMBER_OF_PAGE_BANK_ROWS )
        #self.pageBankView.show_all()
        
        #mainViewRect = self.mainView.get_allocation()
        
        #self.trackInterface.set_size_request( mainViewRect.width, mainViewRect.height )
        
        #self.trackControlsBox.set_size_request( 100, mainViewRect.height )

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
