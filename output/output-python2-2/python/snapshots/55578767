import pygtk
pygtk.require( '2.0' )
import gtk 

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
    
    def setupGUI( self ):
        # Init mixing board
        self.mixerWindow = MixerWindow()
        self.micRecordingWindow = MicRecordingWindow()

        self.pagePlayer = PagePlayer( self.getTempo, 
                                      self.getBeatsPerPage,
                                      self.updatePositionIndicator,
                                      self.updatePage,
                                      self.mixerWindow.getVolumeFunctions(),
                                      set( range( Constants.NUMBER_OF_TRACKS ) ) )
        
        self.generator = Generator( self.mixerWindow.getVolumeFunctions(),
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
        self.pageNewPageButton.connect( "clicked", self.addPageCallback, None )
                
        self.mainWindowBox = gtk.HBox( False, 5 )
        self.mainWindowBox.pack_start( self.globalControlsFrame, False )
        self.mainWindowBox.pack_start( self.pageControlsFrame, False )
        self.mainWindowBox.pack_start( self.trackControlsBoxes, False )
        
        self.trackPagesBox = gtk.VBox( False )
        self.trackPagesBox.pack_start( self.mainView, True )
        self.trackPagesBox.pack_start( self.tuneView, False )
        self.trackPagesBox.pack_start( self.pageBankView, False, True, 5 )
        
        self.mainWindowBox.pack_start( self.trackPagesBox )
        self.add( self.mainWindowBox )

        #to update mainView's contents when window gets resized
        #TODO: is this the right way to do this?
        self.connect( "configure-event", self.handleConfigureEvent )
        
        self.keyboardInput = KeyboardInput( self.pagePlayer.getCurrentTick , self.pagePlayer.trackInstruments , self.pagePlayer.trackDictionary , self.pagePlayer.selectedTrackIDs , self.updatePage , self.pagePlayer.update , self.pagePlayer.getCurrentPageID )
        self.connect( "key-press-event", self.keyboardInput.onKeyPress )
        self.connect( "key-release-event", self.keyboardInput.onKeyRelease )
        
        self.handleConfigureEvent( None, None )
        
        self.show_all()

    def initialize( self ):
        # Volume initialisation for Csound.
        CSoundClient.setMasterVolume( self.getVolume() )
        
        for pageIndex in range( GUIConstants.NUMBER_OF_PAGE_BANK_ROWS * 
                                GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS ):
            self.addPage()
            
        
    
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
        self.mainSlidersBox.pack_start( self.barsSlider )

        self.globalControlsBox.pack_start( self.mainSlidersBox )
        self.updateWindowTitle( None, None )

        self.olpcLabel = gtk.Label( "OLPC" )
        self.globalControlsBox.pack_start( self.olpcLabel )
        self.globalControlsFrame.add( self.globalControlsBox )

    def setupPageControls( self ):
        self.pageControlsFrame = gtk.Frame()
        self.pageControlsFrame.set_shadow_type( gtk.SHADOW_ETCHED_OUT )
        self.pageControlsAlignment = gtk.Alignment( 1.0, 0.5, 0.0, 0.0 )
        self.pageControlsBox = gtk.VBox( False )

        self.pagePlayButton = gtk.ToggleButton( "Play" )
        self.pageRecordButton = gtk.ToggleButton( "Record" )
        self.pageKeyboardButton = gtk.ToggleButton( "Keyboard" )
        self.pageGenerateButton = gtk.Button( "Generate" )
        self.pageMixerButton = gtk.Button("Mixer")
        self.pageNewPageButton = gtk.Button( "New Page" )
        self.pageMicRecordingButton = gtk.Button( "Mic Recording" )

        self.pageControlsBox.pack_start( self.pagePlayButton, False )
        self.pageControlsBox.pack_start( self.pageRecordButton, False )
        self.pageControlsBox.pack_start( self.pageKeyboardButton, False )
        self.pageControlsBox.pack_start( self.pageGenerateButton, False )
        self.pageControlsBox.pack_start(self.pageMixerButton, False)
        #self.pageControlsBox.pack_start( self.pageNewPageButton, False )
        self.pageControlsBox.pack_start( self.pageMicRecordingButton, False )

        self.pageControlsAlignment.add( self.pageControlsBox )
        self.pageControlsFrame.add( self.pageControlsAlignment )

        self.pagePlayButton.connect( "toggled", self.handlePlay, "Page Play" )
        self.pageRecordButton.connect( "toggled", self.handleRecord, None )
        self.pageKeyboardButton.connect( "toggled", self.handleKeyboard, None )
        self.pageGenerateButton.connect( "clicked", self.showAlgorithmWindow, None )
        self.pageMixerButton.connect("clicked", self.showMixerWindow, None)
        self.pageMicRecordingButton.connect( "clicked", self.showMicRecordingWindow, None )
        
    def setupTrackControls( self ):
        self.trackControlsBoxes = gtk.VBox()
        for trackID in self.pagePlayer.trackIDs:
            trackControlsBox = gtk.VBox()
        
            playbackControlsBox = gtk.HBox()
            muteButton = gtk.ToggleButton( "Mute" )
            playbackControlsBox.pack_start( muteButton )
            trackControlsBox.pack_start( playbackControlsBox )

            instrumentMenu = gtk.Menu()
            instrumentMenuItem = gtk.MenuItem( "Instrument" )
            instrumentMenuItem.set_submenu( instrumentMenu )
            
            instrumentNames = CSoundConstants.INSTRUMENTS.keys()
            instrumentNames.sort()
            for instrumentName in instrumentNames:
                menuItem = gtk.MenuItem( instrumentName )
                menuItem.connect_object( "activate", self.pagePlayer.setInstrument, ( trackID, instrumentName ) )
                #menuItem.connect_object( "activate", instrumentMenuItem.set_label, instrumentName )
                instrumentMenu.append( menuItem )
        
            instrumentMenuBar = gtk.MenuBar()
            instrumentMenuBar.append( instrumentMenuItem )
            trackControlsBox.pack_start( instrumentMenuBar )

            trackName = "Track %i" % trackID
            muteButton.connect( "toggled", self.handleMuteTrack, trackID )

            self.trackControlsBoxes.pack_start( trackControlsBox )        

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

        self.trackViewsContainers = {} #[ pageID : gtk.Fixed() ]
        self.trackViews = {} # [ trackID : [ pageID : pageView ] ]
        
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
            
        self.keyboardInput.record = self.pagePlayButton.get_active() and self.pageRecordButton.get_active() and self.pageKeyboardButton.get_active()

    def handleKeyboard( self, widget, data ):
        self.keyboardInput.active = widget.get_active()
        
    def handleRecord( self, widget, data ):
        if not self.pageKeyboardButton.get_active():
            self.pageKeyboardButton.set_active( True )
            
        self.keyboardInput.record = self.pagePlayButton.get_active() and self.pageRecordButton.get_active()

    def handleMuteTrack( self, widget, trackID ):
        self.pagePlayer.toggleMuteTrack( trackID )
        self.positionIndicator.queue_draw()
    
    def handleVolumeChanged( self, widget, data ):
    	CSoundClient.setMasterVolume(self.getVolume())
        self.updateWindowTitle()
       
    def handleTempoChanged( self, widget, data ):
        self.pagePlayer.setTempo( self.getTempo() )
        
        if self.pagePlayer.playing():
            self.pagePlayer.stopPlayback()            
            self.pagePlayer.startPlayback()

        self.updateWindowTitle()

    def updatePositionIndicator( self, currentTick ):
        if ( currentTick % 2 ) == 0:
            pixelsPerTick = self.mainView.get_allocation().width / self.getBeatsPerPage() / Constants.TICKS_PER_BEAT
            self.mainView.move( self.positionIndicator, currentTick * pixelsPerTick, 0 )

    #-----------------------------------
    # generation functions
    #-----------------------------------
    def showAlgorithmWindow( self, widget, data ):
        parametersWindow = GenerationParametersWindow( self.generate )
        parametersWindow.show_all()
        
    def generate( self, generationParameters ):
        self.generator.generate( generationParameters )

        self.pagePlayer.update()
        self.updateTrackViews()
        
        self.handleConfigureEvent( None, None )
        
    def updateTrackViews( self ):
        for pageID in self.pagePlayer.selectedPageIDs:
            for trackID in self.pagePlayer.getActiveTrackIDs():
                self.trackViews[ pageID ][ trackID ].setNotes( self.pagePlayer.trackDictionary[ trackID ][ pageID ] )

    #-----------------------------------
    # Mixer functions
    #-----------------------------------
    def showMixerWindow( self, widget, data ):
        self.mixerWindow.show_all()

    def showMicRecordingWindow( self, widget, data ):
        self.micRecordingWindow.show_all()

    #-----------------------------------
    # callback functions
    #-----------------------------------
    def delete_event( self, widget, event, data = None ):
        return False

    def destroy( self, widget ):
        gtk.main_quit()

    def updateWindowTitle( self, widget = None, data = None ):
        self.set_title( self.getWindowTitle() )
    
    def updateNumberOfBars( self, widget = None, data = None ):
        self.updateWindowTitle()
        self.backgroundView.queue_draw()

    def updateSelection( self ):
        self.positionIndicator.queue_draw()
        self.pagePlayer.update()

    def updatePage( self ):
        currentPageID = self.pagePlayer.getCurrentPageID()
        
        for pageID in self.pagePlayer.pageDictionary.keys():
            if pageID == currentPageID:
                self.trackViewsContainers[ pageID ].show()
            else:
                self.trackViewsContainers[ pageID ].hide()
        
        if self.pagePlayer.playingTune:
            self.tuneView.selectPage( self.pagePlayer.currentPageIndex, False )
            self.pageBankView.deselectAll()
        else:
            self.tuneView.deselectAll()
            
        self.handleConfigureEvent( None, None )
        
    def addPage( self ):
        pageID = len( self.pagePlayer.pageDictionary.keys() )

        #setup track views for pageID
        trackViewsContainer = gtk.Fixed()
        self.trackViewsContainers[ pageID ] = trackViewsContainer
        self.trackViews[ pageID ] = {}
        for trackID in self.pagePlayer.trackIDs:
            trackView = TrackView( self.beatsPerPageAdjustment )
            self.trackViews[ pageID ][ trackID ] = trackView
            trackViewsContainer.put( trackView, 0, 0 )
        self.mainView.put( trackViewsContainer, 0, 0 )
        trackViewsContainer.show_all()
        
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
        self.positionIndicator.set_size_request( 3, mainViewRect.height - 4 )
        
        self.backgroundView.set_size_request( mainViewRect.width, mainViewRect.height )
        
        currentPageID = self.pagePlayer.getCurrentPageID()
        if self.trackViewsContainers.has_key( currentPageID ):
            trackIndex = 0

            trackViewContainer = self.trackViewsContainers[ currentPageID ]
            for trackView in trackViewContainer.get_children():
                currentTrackRect = trackView.get_allocation()
                newTrackRect = self.backgroundView.getTrackRect( trackIndex )
            
                if currentTrackRect.x != newTrackRect.x or currentTrackRect.y != newTrackRect.y:
                    trackViewContainer.move( trackView, newTrackRect.x, newTrackRect.y )
            
                if newTrackRect.width > 0 and newTrackRect.height > 0:
                    trackView.set_size_request( newTrackRect.width, newTrackRect.height )
            
                trackView.queue_draw()
                trackIndex += 1

    #-----------------------------------
    # access functions (not sure if this is the best way to go about doing this)
    #-----------------------------------
    def getVolume( self ):
        return round( self.volumeAdjustment.value, 0 )

    def getTempo( self ):
        return round( self.tempoAdjustment.value, 0 )

    def getBeatsPerPage( self ):
        return round( self.beatsPerPageAdjustment.value, 0 )

    def getWindowTitle( self ):
        return "Tam-Tam [Volume %i, Tempo %i, Beats/Page %i]" % ( self.volumeAdjustment.value, self.getTempo(), self.getBeatsPerPage() )
