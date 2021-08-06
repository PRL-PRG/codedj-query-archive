import pygtk
pygtk.require( '2.0' )
import gtk 

import time

from Framework.Constants import Constants
from Framework.Core.PagePlayer import PagePlayer
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.Generator import Generator

from GUI.GUIConstants import GUIConstants
from GUI.Core.MixerWindow import MixerWindow
from GUI.Core.MicRecordingWindow import MicRecordingWindow
from GUI.Core.PageView import PageView
from GUI.Core.TuneView import TuneView
from GUI.Core.PageBankView import PageBankView
from GUI.Generation.GenerationParametersWindow import GenerationParametersWindow
from BackgroundView import BackgroundView
from TrackView import TrackView
from PositionIndicator import PositionIndicator
from KeyboardInput import KeyboardInput

from Framework.Core.Profiler import TP

#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.Window ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        
        self.setupGUI()
        self.initialize()

        self.handleConfigureEvent( None, None ) # needs to come after pages have been added in initialize()
        
        self.show_all()
    
    def setupGUI( self ):
        self.volumeFunctions = {}

        self.pagePlayer = PagePlayer( set( range( Constants.NUMBER_OF_TRACKS ) ),
                                      self.updatePositionIndicator,
                                      self.updatePage )
        
        self.generator = Generator( self.volumeFunctions,
                                    self.getTempo,
                                    self.pagePlayer.trackInstruments,
                                    self.pagePlayer.trackDictionary,
                                    self.getBeatsPerPage,
                                    self.pagePlayer.getActiveTrackIDs,
                                    self.pagePlayer.selectedPageIDs )
        
        self.setupWindow()
        self.setupGlobalControls()
        self.setupPageControls()
        self.setupTrackControls()
        self.setupMainView()
        self.tuneView = TuneView( self.pagePlayer.setPlayTune, self.pagePlayer.tunePages )
        self.pageBankView = PageBankView( self.pagePlayer.setPlayPage, self.pagePlayer.selectedPageIDs )
                
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

        #to update mainView's contents when window gets resized
        #TODO: is this the right way to do this?
        self.connect( "configure-event", self.handleConfigureEvent )
        
        self.keyboardInput = KeyboardInput( self.pagePlayer.getCurrentTick, self.pagePlayer.trackInstruments,
                                            self.pagePlayer.trackDictionary, self.pagePlayer.selectedTrackIDs, 
                                            self.updateTrackViews, self.pagePlayer.updatePageDictionary, self.pagePlayer.getCurrentPageID )
        self.connect( "key-press-event", self.keyboardInput.onKeyPress )
        self.connect( "key-release-event", self.keyboardInput.onKeyRelease )
       

    def initialize( self ):
        # Volume initialisation for Csound.
        CSoundClient.setMasterVolume( self.getVolume() )
        
        for pageIndex in range( GUIConstants.NUMBER_OF_PAGE_BANK_ROWS * 
                                GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS ):
            self.addPage()
        self.pageBankView.selectPage( 0 )    
        
        for trackID in self.pagePlayer.trackIDs:
            self.handleInstrumentChanged( ( trackID, self.pagePlayer.trackInstruments[ trackID ] ) )

        # FPS stuff
        self.fpsTotalTime = 0
        self.fpsFrameCount = 0
        self.fpsN = 100 # how many frames to average FPS over
        self.fpsLastTime = time.time() # fps will be borked for the first few frames but who cares?

    def updateFPS( self ):
        t = time.time()
        dt = t - self.fpsLastTime
        self.fpsLastTime = t
        self.fpsTotalTime += dt
        self.fpsFrameCount += 1
        if self.fpsFrameCount == self.fpsN:
            fps = self.fpsN/self.fpsTotalTime
            avgMS = 1000/fps
            self.fpsText.set_text("FPS %d ms %.2f" % (fps, avgMS) )
            self.fpsTotalTime = 0
            self.fpsFrameCount = 0

    #-----------------------------------
    # GUI setup functions
    #-----------------------------------
    def setupWindow( self ):
        self.connect( "delete_event", self.delete_event )
        self.connect( "destroy", self.destroy )
        
        numberOfTracks = len( self.pagePlayer.trackIDs )
        self.set_border_width( 10 )
        self.set_geometry_hints( None, 855, numberOfTracks * 100 + 200, 900, numberOfTracks * 300 + 200 )
    
    # contains TAM-TAM and OLPC labels, as well as the volume and tempo sliders
    def setupGlobalControls( self ):
        self.globalControlsFrame = gtk.Frame()
        self.globalControlsFrame.set_shadow_type( gtk.SHADOW_ETCHED_OUT )
        
        self.globalControlsBox = gtk.VBox()
        
        self.tamTamLabel = gtk.Label( "     TAM - TAM     " )
        self.globalControlsBox.pack_start( self.tamTamLabel )
        
        self.mainSlidersBox = gtk.HBox()
        self.volumeAdjustment = gtk.Adjustment( 50, 0, 100, 1, 1, 0 )
        self.volumeAdjustment.connect( "value_changed", self.handleVolumeChanged, None )
        self.volumeSlider = gtk.VScale( self.volumeAdjustment )
        self.volumeSlider.set_draw_value( False )
        self.volumeSlider.set_digits( 0 )
        self.volumeSlider.set_inverted( True )
        self.mainSlidersBox.pack_start( self.volumeSlider, True, True, 4 )
        
        self.tempoAdjustment = gtk.Adjustment( 100, 60, 180, 1, 1, 0 )
        self.tempoAdjustment.connect( "value_changed", self.handleTempoChanged, None )
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
        self.updateWindowTitle( None, None )

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

    def setupPageControls( self ):
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
        self.keyboardButton.connect( "toggled", self.handleKeyboard, None )
        self.keyboardRecordButton.connect( "toggled", self.handleKeyboardRecord, None )
        
    def setupTrackControls( self ):
        self.trackControlsBox = gtk.VBox()
        self.instrumentRecordButtons = {}
        for trackID in self.pagePlayer.trackIDs:
            trackControlsBox = gtk.HBox()

            #setup instrument controls
            instrumentControlsBox = gtk.VBox()
            
            instrumentMenu = gtk.Menu()
            instrumentMenuItem = gtk.MenuItem( "Instrument" )
            instrumentMenuItem.set_submenu( instrumentMenu )
            
            instrumentNames = CSoundConstants.INSTRUMENTS.keys()
            instrumentNames.sort()
            for instrumentName in instrumentNames:
                menuItem = gtk.MenuItem( instrumentName )
                menuItem.connect_object( "activate", self.pagePlayer.setInstrument, ( trackID, instrumentName ) )
                menuItem.connect_object( "activate", self.handleInstrumentChanged, ( trackID, instrumentName ) )
                #menuItem.connect_object( "activate", instrumentMenuItem.set_label, instrumentName )
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
            muteButton.connect( "toggled", self.handleMuteTrack, trackID )

            self.trackControlsBox.pack_start( trackControlsBox )

    def setupMainView( self ):
        self.mainView = gtk.Fixed()

        self.backgroundView = BackgroundView( self.pagePlayer.trackIDs, 
                                              self.pagePlayer.selectedTrackIDs,
                                              self.updateSelection,
                                              self.pagePlayer.mutedTrackIDs,
                                              self.beatsPerPageAdjustment,
                                              self.pagePlayer.trackDictionary,
                                              self.pagePlayer.selectedPageIDs,
                                              self.updatePage )
        self.mainView.put( self.backgroundView, 0, 0 )

        self.trackViews = {} # [ pageID : [ trackID : TrackView ] ]
        
        self.positionIndicator = PositionIndicator( self.pagePlayer.trackIDs, 
                                                    self.pagePlayer.selectedTrackIDs, 
                                                    self.pagePlayer.mutedTrackIDs )
        self.mainView.put( self.positionIndicator, 0, 1 )

    #-----------------------------------
    # playback functions
    #-----------------------------------
    def handlePlay( self, widget, data ):
        if widget.get_active():
            self.pagePlayer.startPlayback()
        else:
            self.pagePlayer.stopPlayback()
            
        self.keyboardInput.record = self.playButton.get_active() and self.keyboardRecordButton.get_active() and self.keyboardButton.get_active()

    def handleKeyboard( self, widget, data ):
        self.keyboardInput.active = widget.get_active()
        
    def handleKeyboardRecord( self, widget, data ):
        if not self.keyboardButton.get_active():
            self.keyboardButton.set_active( True )
            
        self.keyboardInput.record = self.playButton.get_active() and self.keyboardRecordButton.get_active()

    def handleMuteTrack( self, widget, trackID ):
        self.pagePlayer.toggleMuteTrack( trackID )
        self.positionIndicator.queue_draw()
    
    def handleVolumeChanged( self, widget, data ):
    	CSoundClient.setMasterVolume(self.getVolume())
        self.updateWindowTitle()
       
    def handleTempoChanged( self, widget, data ):
        self.pagePlayer.tempo = self.getTempo()
        
        if self.pagePlayer.playing():
            self.pagePlayer.stopPlayback()            
            self.pagePlayer.startPlayback()

        self.updateWindowTitle()

    def updatePositionIndicator( self, fraction ):
        self.updateFPS()
        self.mainView.move( self.positionIndicator, int( fraction * self.mainView.get_allocation().width), 0 )

    #-----------------------------------
    # generation functions
    #-----------------------------------
    def handleGenerate( self, widget, data ):
        if widget.get_active():
            self.generateParametersWindow = GenerationParametersWindow( self.generate, self.handleCloseGenerateWindow )
            self.generateParametersWindow.show_all()
        else:
            self.handleCloseGenerateWindow()
            
    def handleCloseGenerateWindow( self, widget = None, data = None ):
        self.generateParametersWindow.destroy()
        self.generateButton.set_active( False )
                
    def generate( self, generationParameters ):
        self.generator.generate( generationParameters )

        self.pagePlayer.updatePageDictionary()
        self.updateTrackViews()
        
        self.handleCloseGenerateWindow( None, None )
        self.handleConfigureEvent( None, None )
        

    def updatePages( self, pageSet ) :
        for pageID in pageSet:
            for trackID in self.pagePlayer.getActiveTrackIDs():
                self.trackViews[ pageID ][ trackID ].setNotes( self.pagePlayer.trackDictionary[ trackID ][ pageID ] )

        #TODO: find a better place for this expensive call
        self.handleConfigureEvent( False, False )

    def updateTrackViews( self ):
        if len( self.pagePlayer.selectedPageIDs ) == 0:
            pageIDs = [ self.pagePlayer.getCurrentPageID() ]
        else:
            pageIDs = self.pagePlayer.selectedPageIDs
        
        self.updatePages( pageIDs )

        self.backgroundView.redraw()


    #-----------------------------------
    # load and save functions
    #-----------------------------------
    def handleSave(self, widget, data):
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_SAVE, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: serialize to file %s' % chooser.get_filename()
                f = open( chooser.get_filename(), 'w')
                self.pagePlayer.serialize( f )
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
                self.pagePlayer.unserialize( f )
                f.close()
            except IOError: 
                print 'ERROR: failed to unserialize from file %s' % chooser.get_filename()

        chooser.destroy()
        #CAREFUL: if the unserialization failed half-way our whole data-structure is messed.
        self.updatePages( self.pagePlayer.pageDictionary.keys() )
        
    def handleTrackVolumeChanged( self, widget, trackID ):
        self.pagePlayer.trackVolumes[ trackID ] = widget.get_value()
        
    # data is tuple ( trackID, instrumentName )
    def handleInstrumentChanged( self, data ):
        trackID = data[ 0 ]
        instrumentName = data[ 1 ]

        recordButton = self.instrumentRecordButtons[ trackID ]
        if instrumentName in CSoundConstants.RECORDABLE_INSTRUMENTS:
            recordButton.show()
            recordButton.connect( "clicked", 
                                  self.handleMicRecord,
                                  CSoundConstants.RECORDABLE_INSTRUMENT_CSOUND_IDS[ instrumentName ] )
        else:
            recordButton.hide()

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
    def delete_event( self, widget, event, data = None ):
        return False

    def destroy( self, widget ):
        print TP.PrintAll()
        gtk.main_quit()

    def updateWindowTitle( self, widget = None, data = None ):
        self.set_title( self.getWindowTitle() )
    
    def updateNumberOfBars( self, widget = None, data = None ):
        self.updateWindowTitle()
        self.updateTrackViews()
        self.backgroundView.queue_draw()
        
    def updateSelection( self ):
        self.positionIndicator.queue_draw()
        self.pagePlayer.updatePageDictionary()

    def updatePage( self ):
        TP.ProfileBegin( "updatePage" )

        currentPageID = self.pagePlayer.getCurrentPageID()
   
        if self.pagePlayer.playingTune:
            self.tuneView.selectPage( self.pagePlayer.currentPageIndex, False )
            self.pageBankView.deselectAll()
        else:
            self.tuneView.deselectAll()

        self.backgroundView.setCurrentTracks( self.trackViews[currentPageID] )

        self.handleConfigureEvent( None, None )

        print TP.ProfileEndAndPrint( "updatePage" )
        
    def addPage( self ):
        pageID = len( self.pagePlayer.pageDictionary.keys() )

        #setup track views for pageID
        self.trackViews[ pageID ] = {}
        for trackID in self.pagePlayer.trackIDs:
            trackView = TrackView( trackID, self.beatsPerPageAdjustment )
            self.trackViews[ pageID ][ trackID ] = trackView
        
        self.pagePlayer.addPage( pageID )
        self.pagePlayer.setPlayPage( pageID )
        self.pageBankView.addPage( pageID, False )
        
    def addPageCallback( self, widget = None, event = None,  ):
        self.addPage()
        
    # handle resize (TODO: this could probably be done more efficiently)
    def handleConfigureEvent( self, widget, event ):
        mainBoxRect = self.trackPagesBox.get_allocation()
        
        self.tuneView.set_size_request( mainBoxRect.width, GUIConstants.PAGE_HEIGHT + 
                                                           self.tuneView.get_hscrollbar().get_allocation().height + 10 )
        self.tuneView.show_all()
    
        self.pageBankView.set_size_request( mainBoxRect.width, GUIConstants.PAGE_HEIGHT * GUIConstants.NUMBER_OF_PAGE_BANK_ROWS )
        self.pageBankView.show_all()
        
        mainViewRect = self.mainView.get_allocation()
        
        #TODO: why do we specify mainViewRect.height - 4?  should this be a constant?
        # this logic (width/height realtive to parent) should probably be inside PositionIndicator
        self.positionIndicator.set_size_request( 3, max(0, mainViewRect.height - 4) )
        
        self.backgroundView.set_size_request( mainViewRect.width, mainViewRect.height )
        
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
