import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject
from GUI.Core.ThemeWidgets import *

import time

from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

from GUI.GUIConstants import GUIConstants
from GUI.GUIConstants import ModKeys
from GUI.Core.MixerWindow import MixerWindow
from GUI.Core.MicRecordingWindow import MicRecordingWindow
from GUI.Core.PageView import PageView
from GUI.Core.TuneView import TuneView
from GUI.Core.PageBankView import PageBankView
from GUI.Generation.GenerationParametersWindow import GenerationParametersWindow
from TrackInterface import TrackInterface
from TrackView import TrackView
from PositionIndicator import PositionIndicator
#from KeyboardInput import KeyboardInput   #TODO: put functionality back in there

from Framework.Core.Profiler import TP

from Framework.Generation.Generator import generator1, variate

#from Framework.Note import *
#from Framework.Music import *
from Framework.NoteLooper import *

def note_from_CSoundNote( csnote ):
    note = {}
    note['onset'] = csnote.onset
    note['pitch'] = csnote.pitch
    note['amplitude'] = csnote.amplitude
    note['pan'] = csnote.pan
    note['duration'] = csnote.duration
    note['noteID'] = csnote.noteID
    note['trackID'] = csnote.trackID
    note['pageID'] = csnote.pageID
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

INIT_INST = [
        CSoundConstants.FLUTE, 
        CSoundConstants.KOTO, 
        CSoundConstants.GAM,
        CSoundConstants.GAM,
        CSoundConstants.GUIT,
        CSoundConstants.DRUM1KIT,
        CSoundConstants.DRUM1KIT ]
INIT_VOL =  [ Constants.DEFAULT_VOLUME for i in INIT_INST ]
INIT_MUTE = [ 1.0 for i in INIT_INST ]
#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.EventBox ):


    def __init__( self ):
    

        def formatRoundBox( box, fillcolor ):
            box.set_radius( 10 )
            box.set_border_width( 1 )
            box.set_fill_color( fillcolor )
            box.set_border_color( "#EEE" )
            return box

        def init_GUI():
            self.GUI = {}
            self.GUI["2main"] = gtk.HBox()
            
            #------------------------------------------------------------------------
            # left panel
            self.GUI["2leftPanel"] = gtk.VBox()
            self.GUI["2leftPanel"].set_size_request( 80, -1 )
            # + mode panel
            self.GUI["2modePanel"] = gtk.HBox()
            self.GUI["2modePanel"].set_size_request( -1, 30 )
            # + + mode 1 box
            self.GUI["2mode1Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2mode1Button"] = gtk.Button("I")
            self.GUI["2mode1Box"].pack_start( self.GUI["2mode1Button"] )
            self.GUI["2modePanel"].pack_start( self.GUI["2mode1Box"] )
            # + + mode 3 box
            self.GUI["2mode3Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2mode3Button"] = gtk.Button("III")
            self.GUI["2mode3Box"].pack_start( self.GUI["2mode3Button"] )
            self.GUI["2modePanel"].pack_start( self.GUI["2mode3Box"] )
            self.GUI["2leftPanel"].pack_start( self.GUI["2modePanel"], False )
            # + title box
            self.GUI["2titleBox"] = formatRoundBox( RoundVBox(), "#9FB" )
            self.GUI["2titleImage"] = gtk.Image()
            self.GUI["2titleImage"].set_from_file( "Resources/Images/add.png" )
            self.GUI["2titleBox"].pack_start( self.GUI["2titleImage"] )
            self.GUI["2leftPanel"].pack_start( self.GUI["2titleBox"] )
            # + save box
            self.GUI["2saveBox"] = formatRoundBox( RoundVBox(), "#9FB" )
            self.GUI["2saveBox"].set_size_request( -1, 120 )
            self.GUI["2saveButton"] = gtk.Button("Save")
            self.GUI["2saveBox"].pack_start( self.GUI["2saveButton"] )
            self.GUI["2meshButton"] = gtk.Button("Mesh")
            self.GUI["2saveBox"].pack_start( self.GUI["2meshButton"] )
            self.GUI["2leftPanel"].pack_start( self.GUI["2saveBox"], False )
            # + volume box
            self.GUI["2volumeBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2volumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2volumeSlider"] = ImageVScale( "Resources/Images/RedHead.png", self.GUI["2volumeAdjustment"], 22 )
            self.GUI["2volumeBox"].pack_start( self.GUI["2volumeSlider"] )
            self.GUI["2tempoAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2tempoSlider"] = ImageVScale( "Resources/Images/BlueHead.png", self.GUI["2tempoAdjustment"], 22 )
            self.GUI["2volumeBox"].pack_start( self.GUI["2tempoSlider"] )
            self.GUI["2leftPanel"].pack_start( self.GUI["2volumeBox"] )
            self.GUI["2main"].pack_start( self.GUI["2leftPanel"], False )
            
            #-------------------------------------------------------------------------
            # right panel
            self.GUI["2rightPanel"] = gtk.VBox()
            # + edit panel
            self.GUI["2editPanel"] = gtk.HBox()
            # + + instrument panel
            self.GUI["2instrumentPanel"] = gtk.VBox()
            self.GUI["2instrumentPanel"].set_size_request( 80, -1 )
            # + + + instrument 1 box
            self.GUI["2instrument1Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2instrument1volumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2instrument1volumeSlider"] = gtk.VScale( self.GUI["2instrument1volumeAdjustment"] )
            self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1volumeSlider"] )
            self.GUI["2instrument1Button"] = gtk.Button("Inst 1")
            self.GUI["2instrument1Box"].pack_start( self.GUI["2instrument1Button"] )
            self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument1Box"] )
            # + + + instrument 2 box
            self.GUI["2instrument2Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2instrument2volumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2instrument2volumeSlider"] = gtk.VScale( self.GUI["2instrument2volumeAdjustment"] )
            self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2volumeSlider"] )
            self.GUI["2instrument2Button"] = gtk.Button("Inst 2")
            self.GUI["2instrument2Box"].pack_start( self.GUI["2instrument2Button"] )
            self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument2Box"] )
            # + + + instrument 3 box
            self.GUI["2instrument3Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2instrument3volumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2instrument3volumeSlider"] = gtk.VScale( self.GUI["2instrument3volumeAdjustment"] )
            self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3volumeSlider"] )
            self.GUI["2instrument3Button"] = gtk.Button("Inst 3")
            self.GUI["2instrument3Box"].pack_start( self.GUI["2instrument3Button"] )
            self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument3Box"] )
            # + + + instrument 4 box
            self.GUI["2instrument4Box"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2instrument4volumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2instrument4volumeSlider"] = gtk.VScale( self.GUI["2instrument4volumeAdjustment"] )
            self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4volumeSlider"] )
            self.GUI["2instrument4Button"] = gtk.Button("Inst 4")
            self.GUI["2instrument4Box"].pack_start( self.GUI["2instrument4Button"] )
            self.GUI["2instrumentPanel"].pack_start( self.GUI["2instrument4Box"] )
            # + + + drum box
            self.GUI["2drumBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2drumvolumeAdjustment"] = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.GUI["2drumvolumeSlider"] = gtk.VScale( self.GUI["2drumvolumeAdjustment"] )
            self.GUI["2drumBox"].pack_start( self.GUI["2drumvolumeSlider"] )
            self.GUI["2drumButton"] = gtk.Button("Drum")
            self.GUI["2drumBox"].pack_start( self.GUI["2drumButton"] )
            self.GUI["2instrumentPanel"].pack_start( self.GUI["2drumBox"] )
            self.GUI["2editPanel"].pack_start( self.GUI["2instrumentPanel"], False )
            # + + track interface
            self.GUI["2trackinterface"] = formatRoundBox( RoundHBox(), "#9BF" ) # temp
            self.GUI["2editPanel"].pack_start( self.GUI["2trackinterface"] )
            self.GUI["2rightPanel"].pack_start( self.GUI["2editPanel"] )
            # + tune box
            self.GUI["2tuneBox"] = formatRoundBox( RoundVBox(), "#9FB" )
            self.GUI["2tuneBox"].set_size_request( -1, 80 )
            self.GUI["2tuneScrolledWindow"] = gtk.ScrolledWindow()
            self.GUI["2tuneWindow"] = gtk.HBox()
            self.GUI["2tuneScrolledWindow"].add_with_viewport( self.GUI["2tuneWindow"] )
            self.GUI["2tuneBox"].pack_start( self.GUI["2tuneScrolledWindow"] )
            self.GUI["2rightPanel"].pack_start( self.GUI["2tuneBox"], False )
            # + tool panel
            self.GUI["2toolPanel"] = gtk.HBox()
            self.GUI["2toolPanel"].set_size_request( -1, 80 )
            # + + page box
            self.GUI["2pageBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2pageBox"].set_size_request( 80, -1 )
            self.GUI["2pageNewButton"] = gtk.Button("New Page")
            self.GUI["2pageBox"].pack_start( self.GUI["2pageNewButton"] )
            self.GUI["2pageDuplicateButton"] = gtk.Button("Duplicate Page")
            self.GUI["2pageBox"].pack_start( self.GUI["2pageDuplicateButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2pageBox"], False )
            # + + context box
            self.GUI["2contextBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2toolPanel"].pack_start( self.GUI["2contextBox"] )
            # + + transport box
            self.GUI["2transportBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2pageBox"].set_size_request( 120, -1 )
            self.GUI["2keyboardButton"] = gtk.Button("KB")
            self.GUI["2transportBox"].pack_start( self.GUI["2keyboardButton"] )
            self.GUI["2recordButton"] = gtk.Button("Rec")
            self.GUI["2transportBox"].pack_start( self.GUI["2recordButton"] )
            self.GUI["2playButton"] = gtk.Button("Play")
            self.GUI["2transportBox"].pack_start( self.GUI["2playButton"] )
            self.GUI["2loopButton"] = gtk.Button("Loop")
            self.GUI["2transportBox"].pack_start( self.GUI["2loopButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2transportBox"] )
            # + + tool box
            self.GUI["2toolBox"] = formatRoundBox( RoundHBox(), "#9FB" )
            self.GUI["2toolPencilButton"] = gtk.Button("Pencil")
            self.GUI["2toolBox"].pack_start( self.GUI["2toolPencilButton"] )
            self.GUI["2toolPointerButton"] = gtk.Button("Pointer")
            self.GUI["2toolBox"].pack_start( self.GUI["2toolPointerButton"] )
            self.GUI["2toolPanel"].pack_start( self.GUI["2toolBox"] )
            self.GUI["2rightPanel"].pack_start( self.GUI["2toolPanel"], False )
            self.GUI["2main"].pack_start( self.GUI["2rightPanel"] )
        
            self.add( self.GUI["2main"] )
            
        def init_data( ):
            self._data = {}

            #[ volume, ... ]
            self._data['track_volume'] = [ 0.8 ] * Constants.NUMBER_OF_TRACKS
            self._data['track_mute']   = [False] * Constants.NUMBER_OF_TRACKS

            #[ instrument index, ... ]
            track_inst = [ CSoundConstants.FLUTE, CSoundConstants.KOTO, CSoundConstants.GAM, CSoundConstants.GAM,
                           CSoundConstants.GUIT, CSoundConstants.DRUM1KIT, CSoundConstants.DRUM1KIT ]

            self._data['track_inst'] = track_inst + [CSoundConstants.FLUTE] * (Constants.NUMBER_OF_TRACKS - len( track_inst) )
            #{ pageId: { [track 0 = note list], [track 2 = note list], ... ] }
            npages = 40
            nbeats = 4

            self._data['page_beats'] = [nbeats  for p in range(npages)]
            self._data['tempo'] = Constants.DEFAULT_TEMPO
            self._data['tune'] = []
            self._data['notebin'] = []
            self._noteId = {}
            self._noteIdBase = 0

        # these helper functions do not 
        # run in any particular order.... 
        # TODO: give these functions better names, put them in execution order, cut hierarchy

        def setupGUI( ):
            self.volumeFunctions = {}

            self.generateParametersWindow = GenerationParametersWindow( self.generate, self.variate, self.handleCloseGenerateWindow )
            
            setupGlobalControls()
            setupPageControls()
            setupTrackControls()
            setupMainView()

            self.tuneView = TuneView( self.onTuneViewSelect )
            self.pageBankView = PageBankView( self.onPageBankSelect, self.onPageBankDrop )
                    
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
            self.trackPagesBox.pack_start( self.mainView, True )
            self.trackPagesBox.pack_start( self.tuneView, False )
            self.trackPagesBox.pack_start( self.pageBankView, False, True, 5 )
            
            self.mainWindowBox.pack_start( self.trackPagesBox )
            
            self.add( self.mainWindowBox )

        # contains TAM-TAM and OLPC labels, as well as the volume and tempo sliders
        def setupGlobalControls( ):
            self.globalControlsFrame = gtk.Frame()
            self.globalControlsFrame.set_shadow_type( gtk.SHADOW_ETCHED_OUT )
            
            self.globalControlsBox = gtk.VBox()
            
            self.tamTamLabel = gtk.Label( "     TAM - TAM     " )
            self.globalControlsBox.pack_start( self.tamTamLabel )
            
            self.mainSlidersBox = gtk.HBox()
            self.volumeAdjustment = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
            self.volumeAdjustment.connect( "value_changed", self.onVolumeChanged, None )
            self.volumeSlider = gtk.VScale( self.volumeAdjustment )
            self.volumeSlider.set_draw_value( False )
            self.volumeSlider.set_digits( 0 )
            self.volumeSlider.set_inverted( True )
            self.mainSlidersBox.pack_start( self.volumeSlider, True, True, 4 )
            
            self.tempoAdjustment = gtk.Adjustment( 100, 60, 180, 1, 1, 0 )
            self.tempoAdjustment.connect( "value_changed", self.onTempoChanged, None )
            self.tempoSlider = gtk.VScale( self.tempoAdjustment )
            self.tempoSlider.set_draw_value( False )
            self.tempoSlider.set_digits( 0 )
            self.tempoSlider.set_inverted( True )
            self.mainSlidersBox.pack_start( self.tempoSlider )

            self.beatsPerPageAdjustment = gtk.Adjustment( 4, 1, 8, 1, 1, 0 )
            self.beatsPerPageAdjustment.connect( "value_changed", self.updateNumberOfBars, None )
            self.barsSlider = gtk.VScale( self.beatsPerPageAdjustment )
            self.barsSlider.set_draw_value( False )
            self.barsSlider.set_digits( 0 )
            self.barsSlider.set_inverted( True )
            self.barsSlider.set_increments( 1, 1 )
            self.barsSlider.set_update_policy( gtk.UPDATE_DELAYED )
            self.mainSlidersBox.pack_start( self.barsSlider )

            self.globalControlsBox.pack_start( self.mainSlidersBox )

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
            for trackID in range( Constants.NUMBER_OF_TRACKS):
                trackControlsBox = gtk.HBox()

                #setup instrument controls
                instrumentControlsBox = gtk.VBox()
                
                instrumentMenu = gtk.Menu()
                instrumentMenuItem = gtk.MenuItem( "Instrument" )
                instrumentMenuItem.set_submenu( instrumentMenu )
                
                instrumentNames = []
                instrumentFolderNames = CSoundConstants.INSTRUMENTS.keys()
                for instrumentName in instrumentFolderNames:
                    if not instrumentName[0: 4] == 'drum':
                       instrumentNames.append( instrumentName )
                                    
                instrumentNames.append( 'drum1kit' )
                instrumentNames.sort()
                for instrumentName in instrumentNames:
                    menuItem = gtk.MenuItem( instrumentName )
                    menuItem.connect_object( "activate", self.handleInstrumentChanged, ( trackID, instrumentName ) )
                    instrumentMenu.append( menuItem )
                    
                instrumentMenuBar = gtk.MenuBar()
                instrumentMenuBar.append( instrumentMenuItem )
                instrumentControlsBox.pack_start( instrumentMenuBar )
                
                recordButton = gtk.Button()
                recordButton.set_size_request( 15, 15 )
                self.instrumentRecordButtons[ trackID ] = recordButton
                instrumentControlsBox.pack_start( recordButton, False )
                
                trackControlsBox.pack_start( instrumentControlsBox )

                #setup playback controls
                playbackControlsBox = gtk.VBox()
                
                muteButton = gtk.ToggleButton()
                muteButton.set_size_request( 15, 15 )
                playbackControlsBox.pack_start( muteButton, False )
                
                volumeAdjustment = gtk.Adjustment( 0.8, 0, 1, 0.01, 0.01, 0 )
                volumeAdjustment.connect( "value_changed", self.handleTrackVolumeChanged, trackID )
                self.volumeFunctions[ trackID ] = volumeAdjustment.get_value
                volumeSlider = gtk.VScale( volumeAdjustment )
                volumeSlider.set_update_policy( 0 )
                volumeSlider.set_digits( 2 )
                volumeSlider.set_draw_value( False )
                volumeSlider.set_digits( 0 )
                volumeSlider.set_inverted( True )
                playbackControlsBox.pack_start( volumeSlider, True )
                            
                trackControlsBox.pack_start( playbackControlsBox )

                trackName = "Track %i" % trackID
                muteButton.connect( "toggled", self.onMuteTrack, trackID )

                self.trackControlsBox.pack_start( trackControlsBox )

        def setupMainView( ):
            self.mainView = gtk.Fixed()

            self.trackInterface = TrackInterface( self.onTrackInterfaceNoteDrag )
            self.trackInterface.displayPage(0,int(round( self.beatsPerPageAdjustment.value)))
            self.mainView.put( self.trackInterface, 0, 0 )

            self.trackViews = {} # [ pageID : [ trackID : TrackView ] ]
            
        gtk.EventBox.__init__( self )
            
        # keyboard variables
        self.kb_active = False
        self.kb_record = False
        self.kb_mono = False
        self.kb_keydict = {}

        # playback params
        self.playing = False
        self.playSource = 'Page'
        self.currentPageId = 0
        self.playingTuneIdx = 0

        # FPS stuff
        self.fpsTotalTime = 0
        self.fpsFrameCount = 0
        self.fpsN = 100 # how many frames to average FPS over
        self.fpsLastTime = time.time() # fps will be borked for the first few frames but who cares?
        
        init_data()   #above
        setupGUI()    #above

        self.noteLooper = NoteLooper( 0.2, Constants.DEFAULT_TEMPO * 0.2, INIT_INST, INIT_VOL, INIT_MUTE)

        CSoundClient.setMasterVolume( self.getVolume() )
        
        for pageId in range( GUIConstants.NUMBER_OF_PAGE_BANK_ROWS * GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS ):
            self.pageBankView.addPage( pageId, False )
        
        for tid in range(Constants.NUMBER_OF_TRACKS):
            self.handleInstrumentChanged( ( tid, self._data['track_inst'][tid] ) )

        self.handleConfigureEvent( None, None ) # needs to come after pages have been added in initialize()
        
        self.show_all()  #gtk command

        CSoundClient.sendText( "perf.destroy()" )
    
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
            for track in range( Constants.NUMBER_OF_TRACKS ):
                for i in range( 3 ):
                    csoundInstrument = i + 5001
                    CSoundClient.sendText( CSoundConstants.PLAY_NOTE_OFF_COMMAND % ( csoundInstrument, track ) )

        if widget.get_active():  #play

            #TODO: check for track activation, to not take all
            trackset = set(range(Constants.NUMBER_OF_TRACKS))

            self.noteLooper.clear()       #erase all loaded notes
            notes = []

            if self.playSource == 'Tune':
                pagedelay = 0
                for p in self.tuneView.getTune():
                    notes += [(n['onset'] + pagedelay, n) for n in self._data['notebin'] \
                            if n['pageID']==p and n['trackID'] in trackset ]
                    pagedelay += self._data['page_beats'][p] * Constants.TICKS_PER_BEAT
            elif self.playSource == 'Page':
                notes = [(n['onset'], n) for n in self._data['notebin'] \
                        if n['pageID']== self.currentPageId and n['trackID'] in trackset ]
                pagedelay = self._data['page_beats'][self.currentPageId] * Constants.TICKS_PER_BEAT
            else: 
                print 'ERROR: handlePlay() invalid playSource', self.playSource
                return

            self.noteLooper.insert(notes)
            self.noteLooper.setDuration( pagedelay )
            self.noteLooper.setTick(0)    #TODO: get playback head position
            self.noteLooper.setRate( round( self.tempoAdjustment.value, 0 ) * 0.2 )

            CSoundClient.sendText( "\n".join(self.noteLooper.next()) ) 
            self.playbackTimeout = gobject.timeout_add( 50, self.onTimeout )
            self.playing = True

        else:                    #stop
            gobject.source_remove( self.playbackTimeout )
            shutOffCSound()
            self.playing = False


        self.kb_record = self.playButton.get_active() and self.keyboardRecordButton.get_active() and self.keyboardButton.get_active()

    def onTimeout(self):
        pref="MainWindow::onTimeout "

        TP.ProfileBegin( pref+"send" )
        CSoundClient.sendText( "\n".join(self.noteLooper.next()) ) 
        TP.ProfileEnd( pref+"send" )

        TP.ProfileBegin( pref+"update" )
        self.updateFPS()
        TP.ProfileEnd( pref+"update" )

        TP.ProfileBegin( pref+"tune" )
        if self.playSource == 'Tune':
            curtick = self.noteLooper.getCurrentTick(0,True, time.time())
            curIdx =  curtick / ( 4 * Constants.TICKS_PER_BEAT) #TODO
            if curIdx != self.tuneView.selectedPageIndex:
                self.tuneView.selectPage( curIdx )
        TP.ProfileEnd( pref+"tune" )

        return True

    def onMuteTrack( self, widget, trackID ):
        self._data['track_mute'][trackID] = not self._data['track_mute'][trackID] 
        if self._data['track_mute'][trackID]:
            self.noteLooper.setMute( trackID, 0.0 )
        else:
            self.noteLooper.setMute( trackID, 1.0 )

    def handleTrackVolumeChanged( self, widget, trackID ):
        v =  widget.get_value()
        self._data['track_volume'][trackID] = v
        self.noteLooper.setVolume( trackID, v )
        
    # data is tuple ( trackID, instrumentName )
    def handleInstrumentChanged( self, data ):
        (id, instrumentName) = data
        self._data['track_inst'][id] = instrumentName
        self.noteLooper.setInstrument(id, instrumentName)

        recordButton = self.instrumentRecordButtons[ id ]
        if instrumentName in CSoundConstants.RECORDABLE_INSTRUMENTS:
            recordButton.show()
            recordButton.connect( "clicked", 
                                  self.handleMicRecord,
                                  CSoundConstants.RECORDABLE_INSTRUMENT_CSOUND_IDS[ instrumentName ] )
        else:
            recordButton.hide()

    def onVolumeChanged( self, widget, data ):
    	CSoundClient.setMasterVolume(self.getVolume())
       
    def onTempoChanged( self, widget, data ):
        tempo = round( self.tempoAdjustment.value, 0 )
        ticks_per_sec = tempo * 0.2 # 12 BPM / 60 SPM

        self._data['tempo'] = tempo
        self.noteLooper.setRate(ticks_per_sec)

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
        if widget.get_active():
            self.generateParametersWindow.show_all()
        else:
            self.handleCloseGenerateWindow()
            
    def handleCloseGenerateWindow( self, widget = None, data = None ):
        self.generateParametersWindow.hide_all()
        self.generateButton.set_active( False )
                
    def recompose( self, algo, params):
        def none_to_all(tracks):
            print 'tracks = ',tracks
            if tracks == []: return set(range(0,Constants.NUMBER_OF_TRACKS))
            else:            return set(tracks)

        dict = {}
        for t in range(Constants.NUMBER_OF_TRACKS):
            dict[t] = {}
            for p in range(Constants.NUMBER_OF_PAGES):
                dict[t][p] = []

        newtracks = none_to_all( self.trackInterface.getSelectedTracks())
        newpages  = self.pageBankView.getSelectedPageIds()

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
                    note.pageID = page
                    note.trackID = track
                    self._noteIdBase = self._noteIdBase + 1
                    while self._noteIdBase in self._noteId: self._noteIdBase = self._noteIdBase + 1
                    note.noteID = self._noteIdBase

        # TODO: remove notes from the tracks we are replacing
        #add notes to self._data
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += [note_from_CSoundNote(n) for n in dict[tid][pid]]

        for n in newnotes:
            self._noteId[n['noteID']] = n

        #delete the old pages & tracks!
        togo = [n for n in self._data['notebin'] if (n['trackID'] in newtracks and n['pageID'] in newpages)  ]
        self.trackInterface.deleteNotes( 
                {   "page": [n['pageID'] for n in togo],
                    "track":[n['trackID'] for n in togo] ,
                    "note": [n['noteID'] for n in togo]},
                len(togo))
        for n in togo:
            del self._noteId[n['noteID']]

        self._data['notebin'] = \
                [n for n in self._data['notebin'] if not (n['trackID'] in newtracks and n['pageID'] in newpages)  ] \
                + newnotes

        pageList = []
        trackList = []
        noteList = []
        csnoteList = []
        beatList = []

        i = 0
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    pageList.append(page)
                    trackList.append(track)
                    noteList.append(note.noteID)
                    csnoteList.append(note)
                    beatList.append(4)
                    i += 1

        self.trackInterface.addNotes( 
                {   "page":pageList,
                    "track":trackList,
                    "note":noteList,
                    "csnote":csnoteList,
                    "beatCount":beatList},
                i )

        self.handleCloseGenerateWindow( None, None )
        self.handleConfigureEvent( None, None )

    def generate( self, params ):
        self.recompose( generator1, params)

    def variate( self, params ):
        self.recompose( variate, params)
        
    #-----------------------------------
    # load and save functions
    #-----------------------------------
    def handleSave(self, widget, data):
        print TP.PrintAll()
        gtk.main_quit()
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
        CSoundClient.micRecording( data )
            
    def handleCloseMicRecordWindow( self, widget = None, data = None ):
        self.micRecordWindow.destroy()
        self.micRecordButton.set_active( False )

    #-----------------------------------
    # callback functions
    #-----------------------------------
    def selectPage(self, pageId):
        self.trackInterface.displayPage(pageId,int(round( self.beatsPerPageAdjustment.value)))
        self.currentPageId = pageId

    #can be called either by playback switching pages, or mouse switching pages
    def onTuneViewSelect(self, pageId, tuneIdx):
        if pageId != self.tuneView.NO_PAGE:
            self.pageBankView.selectPage( self.pageBankView.NO_PAGE, False, True )  #de-select the pageBank
            self.selectPage(pageId)
            self.playSource = 'Tune'
        else:
            self.playSource = 'Page'

    def onPageBankSelect( self, pageId ):
        self.tuneView.selectPage( self.tuneView.NO_PAGE, False )  #de-select the tuneView
        self.selectPage(pageId)
        self.playSource = 'Page'

    def onPageBankDrop( self, data ):
        if data[0] == 'p':
            pass
        elif data[0] == 't':
            pageIdx = int( data.split()[2] )
            self.tuneView.removePage(pageIdx)
            self.tuneView.selectPage(pageIdx)
        else:
            raise 'ERROR'

    def onKeyPress(self,widget,event):
        
        ModKeys.keyPress( event.hardware_keycode )

        if not self.kb_active:
            return
        if self.kb_record:
            self.kb_mono = False
        
        key = event.hardware_keycode 
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
            if self.getSelectedTrackIDs():
                instrument = self._data['track_inst'][min(self.getSelectedTrackIDs())]
            
            if instrument == 'drum1kit':
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ GenerationConstants.DRUMPITCH[ pitch ] ]
                else:
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                pitch = 36
                duration = 100
            
            if CSoundConstants.INSTRUMENTS[instrument].csoundInstrumentID == 102:
                duration = 100
            
            # Create and play the note
            self.kb_keydict[key] = Note.note_new(onset = 0, 
                                            pitch = pitch, 
                                            amplitude = 1, 
                                            pan = 0.5, 
                                            duration = duration, 
                                            trackID = track, 
                                            fullDuration = False, 
                                            instrument = instrument, 
                                            instrumentFlag = instrument)
            Note.note_play(self.kb_keydict[key])
                
    def onKeyRelease(self,widget,event):

        ModKeys.keyRelease( event.hardware_keycode )

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
            if self.kb_record and len( self.getSelectedTrackIDs() ) != 0:
                self.kb_keydict[key]['amplitude'] = 1
                self.getTrackDictionary()[min(self.getSelectedTrackIDs())][self.getCurrentPageIDCallback()].append(self.kb_keydict[key])
                print 'ERROR: keyboard release... callbacks?'
            del self.kb_keydict[key]

    def delete_event( self, widget, event, data = None ):
        return False

    def destroy( self, widget ):
        print TP.PrintAll()
        gtk.main_quit()
    
    def updateNumberOfBars( self, widget = None, data = None ):
        self.trackInterface.updateBeatCount( int(round( self.beatsPerPageAdjustment.value)) )
        
    def updateSelection( self ):
        print 'WARNING: wtf is this?'

    def updatePage( self ):
        print 'INFO updatePage called'
        TP.ProfileBegin( "updatePage" )

        if self.playingTune:
            self.tuneView.selectPage( self.currentPageId, False )
            self.pageBankView.selectPage(self.pageBankView.NO_PAGE,False)
        else:
            self.tuneView.deselectAll()
            self.tuneView.selectPage(self.tuneView.NO_PAGE,False)

        # temp        
        self.trackInterface.displayPage(0,int(round( self.beatsPerPageAdjustment.value)))

        self.handleConfigureEvent( None, None )

        print TP.ProfileEndAndPrint( "updatePage" )
        
        
    # handle resize (TODO: this could probably be done more efficiently)
    def handleConfigureEvent( self, widget, event ):
        mainBoxRect = self.trackPagesBox.get_allocation()
        
        self.tuneView.set_size_request( mainBoxRect.width, GUIConstants.PAGE_HEIGHT + 
                                                           self.tuneView.get_hscrollbar().get_allocation().height + 10 )
        self.tuneView.show_all()
    
        self.pageBankView.set_size_request( mainBoxRect.width, GUIConstants.PAGE_HEIGHT * GUIConstants.NUMBER_OF_PAGE_BANK_ROWS )
        self.pageBankView.show_all()
        
        mainViewRect = self.mainView.get_allocation()
        
        self.trackInterface.set_size_request( mainViewRect.width, mainViewRect.height )
        
        self.trackControlsBox.set_size_request( 100, mainViewRect.height )

    #-----------------------------------
    # access functions (not sure if this is the best way to go about doing this)
    #-----------------------------------
    def getVolume( self ):
        return round( self.volumeAdjustment.value, 0 )

    def getTempo( self ):
        return round( self.tempoAdjustment.value, 0 )

    def getBeatsPerPage( self ):
        return int(round( self.beatsPerPageAdjustment.value, 0 ))

    def getWindowTitle( self ):
        return "Tam-Tam [Volume %i, Tempo %i, Beats/Page %i]" % ( self.volumeAdjustment.value, self.getTempo(), self.getBeatsPerPage() )
