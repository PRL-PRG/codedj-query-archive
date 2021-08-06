import pygtk
pygtk.require( '2.0' )
import gtk 

from Framework.Constants import Constants
from Framework.Core.TrackPlayer import TrackPlayer
from Framework.Generation.Generator import GenerationParameters

from GUI.Generation.GenerationParametersWindow import GenerationParametersWindow
from BackgroundView import BackgroundView
from TrackView import TrackView
from PositionIndicator import PositionIndicator

#-----------------------------------
# The main TamTam window
#-----------------------------------
class MainWindow( gtk.Window ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        
        self.trackPlayer = TrackPlayer( self.getTempo, 
                                        self.getBeatsPerPage,
                                        self.updatePositionIndicator, 
                                        Constants.NUMBER_OF_TRACKS )
        
        self.setupWindow()
        self.setupGlobalControls()
        self.setupPageControls()
        self.setupTrackControls()
        self.setupMainView()

        self.mainWindowBox = gtk.HBox( False, 5 )
        self.mainWindowBox.pack_start( self.globalControlsFrame, False )
        self.mainWindowBox.pack_start( self.pageControlsFrame, False )
        self.mainWindowBox.pack_start( self.trackControlsBoxes, False )
        self.mainWindowBox.pack_start( self.mainView )
        self.add( self.mainWindowBox )

        #to update mainView's contents when window gets resized
        #TODO: is this the right way to do this?
        self.connect( "configure-event", self.handleConfigureEvent )
        self.show_all()
    
    #-----------------------------------
    # GUI setup functions
    #-----------------------------------
    def setupWindow( self ):
        self.connect( "delete_event", self.delete_event )
        self.connect( "destroy", self.destroy )
        
        numberOfTracks = len( self.trackPlayer.trackIDs )
        self.set_border_width( 10 )
        self.set_geometry_hints( None, 855, numberOfTracks * 100, 900, numberOfTracks * 300 )
    
    # contains TAM-TAM and OLPC labels, as well as the volume and tempo sliders
    def setupGlobalControls( self ):
        self.globalControlsFrame = gtk.Frame()
        self.globalControlsFrame.set_shadow_type( gtk.SHADOW_ETCHED_OUT )
        
        self.globalControlsBox = gtk.VBox()
        
        self.tamTamLabel = gtk.Label( "     TAM - TAM     " )
        self.globalControlsBox.pack_start( self.tamTamLabel )
        
        self.mainSlidersBox = gtk.HBox()
        self.volumeAdjustment = gtk.Adjustment( 50, 0, 100, 1, 0, 0 )
        self.volumeAdjustment.connect( "value_changed", self.updateWindowTitle, None )
        self.volumeSlider = gtk.VScale( self.volumeAdjustment )
        self.volumeSlider.set_draw_value( False )
        self.volumeSlider.set_digits( 0 )
        self.volumeSlider.set_inverted( True )
        self.mainSlidersBox.pack_start( self.volumeSlider, True, True, 4 )
        
        self.tempoAdjustment = gtk.Adjustment( 100, 60, 180, 1, 0, 0 )
        self.tempoAdjustment.connect( "value_changed", self.handleTempoChanged, None )
        self.tempoSlider = gtk.VScale( self.tempoAdjustment )
        self.tempoSlider.set_draw_value( False )
        self.tempoSlider.set_digits( 0 )
        self.tempoSlider.set_inverted( True )
        self.mainSlidersBox.pack_start( self.tempoSlider )

        self.beatsPerPageAdjustment = gtk.Adjustment( 4, 1, 8, 1, 0, 0 )
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
        self.pageGenerateButton = gtk.Button( "Generate" )
        self.pageInstrumentButton = gtk.Button( "Instrument" )

        self.pageControlsBox.pack_start( self.pagePlayButton, False )
        self.pageControlsBox.pack_start( self.pageGenerateButton, False )
        self.pageControlsBox.pack_start( self.pageInstrumentButton, False )
        
        self.pageControlsAlignment.add( self.pageControlsBox )
        self.pageControlsFrame.add( self.pageControlsAlignment )

        self.pagePlayButton.connect( "toggled", self.handlePlay, "Page Play" )
        self.pageGenerateButton.connect( "clicked", self.showAlgorithmWindow, None )
        #self.pageInstrumentButton.connect( "clicked", self.handleButtonClicked, "Page Instrument" )
        
    def setupTrackControls( self ):
        self.trackControlsBoxes = gtk.VBox()
        for trackID in self.trackPlayer.trackIDs:
            trackControlsBox = gtk.VBox()
        
            playbackControlsBox = gtk.HBox()
            muteButton = gtk.ToggleButton( "Mute" )
            playbackControlsBox.pack_start( muteButton )
            trackControlsBox.pack_start( playbackControlsBox )

            instrumentButton = gtk.Button( "Insrument" )    
            trackControlsBox.pack_start( instrumentButton )
        
            trackName = "Track %i" % trackID
            muteButton.connect( "toggled", self.handleMuteTrack, trackID )
            #instrumentButton.connect( "clicked", self.handleButtonClicked, "%s Instrument" % trackName )

            self.trackControlsBoxes.pack_start( trackControlsBox )        

    def setupMainView( self ):
        self.mainView = gtk.Fixed()

        self.backgroundView = BackgroundView( self.trackPlayer.trackIDs, 
                                              self.trackPlayer.selectedTrackIDs,
                                              self.updateSelection,
                                              self.trackPlayer.mutedTrackIDs,
                                              self.beatsPerPageAdjustment )
        self.mainView.put( self.backgroundView, 0, 0 )

        self.trackViews = {}
        self.trackViewsContainer = gtk.Fixed()
        for trackID in self.trackPlayer.trackIDs:
            trackView = TrackView( self.beatsPerPageAdjustment )
            self.trackViews[ trackID ] = trackView
            self.trackViewsContainer.put( trackView, 0, 0 )
            
        self.mainView.put( self.trackViewsContainer, 0, 0 )
        
        self.positionIndicator = PositionIndicator( self.trackPlayer.trackIDs, 
                                                    self.trackPlayer.selectedTrackIDs, 
                                                    self.trackPlayer.mutedTrackIDs )
        self.mainView.put( self.positionIndicator, 0, 1 )

    #-----------------------------------
    # playback functions
    #-----------------------------------
    def handlePlay( self, widget, data ):
        if widget.get_active():
            self.trackPlayer.startPlayback()
        else:
            self.trackPlayer.stopPlayback()

    def handleMuteTrack( self, widget, trackID ):
        self.trackPlayer.toggleMuteTrack( trackID )
        self.positionIndicator.queue_draw()
    
    def handleTempoChanged( self, widget, data ):
        if self.trackPlayer.playing():
            self.trackPlayer.stopPlayback()            
            self.trackPlayer.startPlayback()
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
        self.trackPlayer.generate( generationParameters )
    
        if len( self.trackPlayer.selectedTrackIDs ) == 0:
            for trackID in self.trackPlayer.trackIDs:
                self.trackViews[ trackID ].setNotes( self.trackPlayer.getEvents( trackID ) )
        else:
            for trackID in self.trackPlayer.selectedTrackIDs:
                self.trackViews[ trackID ].setNotes( self.trackPlayer.getEvents( trackID ) )
            
        self.handleConfigureEvent( None, None )

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
        for trackView in self.trackViews.values():
            trackView.queue_draw()

    def updateSelection( self ):
        self.positionIndicator.queue_draw()
        self.trackPlayer.update()

    # handle resize (TODO: this could probably be done more efficiently)
    def handleConfigureEvent( self, widget, event ):
        mainViewRect = self.mainView.get_allocation()
        
        self.backgroundView.set_size_request( mainViewRect.width, mainViewRect.height )
        
        trackIndex = 0
        for trackView in self.trackViews.values():
            currentTrackRect = trackView.get_allocation()
            newTrackRect = self.backgroundView.getTrackRect( trackIndex )
            
            if currentTrackRect.x != newTrackRect.x or currentTrackRect.y != newTrackRect.y:
                self.trackViewsContainer.move( trackView, newTrackRect.x, newTrackRect.y )
            
            if  newTrackRect.width > 0 and newTrackRect.height > 0:
                trackView.set_size_request( newTrackRect.width, newTrackRect.height )
            
            trackView.queue_draw()
            trackIndex += 1
    
        #TODO: why do we specify mainViewRect.height - 4?  should this be a constant?
        # this logic (width/height realtive to parent) should probably be inside PositionIndicator
        self.positionIndicator.set_size_request( 3, mainViewRect.height - 4 )

    #-----------------------------------
    # access functions (not sure if this is the best way to go about doing this)
    #-----------------------------------
    def getTempo( self ):
        return round( self.tempoAdjustment.value, 0 )

    def getBeatsPerPage( self ):
        return round( self.beatsPerPageAdjustment.value, 0 )

    def getWindowTitle( self ):
        return "Tam-Tam [Volume %i, Tempo %i, Beats/Page %i]" % ( self.volumeAdjustment.value, self.getTempo(), self.getBeatsPerPage() )
