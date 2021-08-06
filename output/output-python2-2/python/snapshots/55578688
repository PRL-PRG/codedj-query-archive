import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants
from GUI.Core.NoteParametersWindow import NoteParametersWindow

#-------------------------------------------------------------
# This is a TEMPORARY implementaion of the BackgroundView,
# it was written quickly to get track selections working
#-------------------------------------------------------------

# TODO: Do I really have to subclass gtk.EventBox to get the button-press-event?
# (I wasn't getting it subclassing directly from DrawingArea)
class BackgroundView( gtk.EventBox ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, trackIDs, selectedTrackIDs, selectionChangedCallback, mutedTrackIDs, beatsPerPageAdjustment, trackDictionary, selectedPageIDs, updatePageCallback ):
        gtk.EventBox.__init__( self )
        
        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )
        
        self.trackIDs = trackIDs
        self.selectedTrackIDs = selectedTrackIDs
        self.selectionChangedCallback = selectionChangedCallback
        self.mutedTrackIDs = mutedTrackIDs
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.trackDictionary = trackDictionary
        self.selectedPageIDs = selectedPageIDs
        self.updatePageCallback = updatePageCallback
        
        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        
    #-----------------------------------
    # access methods
    #-----------------------------------
    def getTrackRect( self, trackID ):
        return gtk.gdk.Rectangle( GUIConstants.BORDER_SIZE,
                                  self.getTrackYLocation( trackID ), 
                                  self.getTrackWidth(), 
                                  self.getTrackHeight() )
        
    def getTrackWidth( self ):
        return self.get_allocation().width - 2 * ( GUIConstants.BORDER_SIZE + 2 )

    def getFullHeight( self ):
        return int( floor( self.get_allocation().height / len( self.trackIDs ) ) )

    def getTrackHeight( self ):
        return int( self.getFullHeight() - 2 * self.getTrackSpacing() )
    
    #TODO-> trackIDs should probably be ordered!
    # we're just using trackID as an index here (this will only work until you can remove tracks)
    def getTrackYLocation( self, trackID ):
        if self.getTrackHeight() < 0:
            return 0
        else:
            trackIndex = trackID
            
            trackHeight = int( floor( self.get_allocation().height / len( self.trackIDs ) ) )
            trackBackgroundYValue = trackHeight * trackIndex
            return trackBackgroundYValue + GUIConstants.BORDER_SIZE

    def getTrackSpacing( self ):
        #TODO handle this, or make it a constant
        return GUIConstants.TRACK_SPACING
    
    #-----------------------------------
    # callback methods
    #-----------------------------------
    def set_size_request( self, width, height ):
        self.drawingArea.set_size_request( width, height )

    def getNoteParameters( self ):
        for trackID in self.selectedTrackIDs:
            for pageID in self.selectedPageIDs:
                for note in self.trackDictionary[ trackID ][ pageID ]:
                    newPitch = note.pitch + self.noteParameters.pitchAdjust.value
                    newAmplitude = note.amplitude *  self.noteParameters.amplitudeAdjust.value
                    newPan = self.noteParameters.panAdjust.value
                    newReverbSend = note.reverbSend * self.noteParameters.reverbSendAdjust.value

                    if newPitch != note.pitch:
                        if newPitch >= Constants.MINIMUM_PITCH and newPitch <= Constants.MAXIMUM_PITCH:
                            note.pitch = newPitch
                        elif newPitch < Constants.MINIMUM_PITCH:
                            note.pitch = Constants.MINIMUM_PITCH
                        elif newPitch > Constants.MAXIMUM_PITCH:
                            note.pitch = Constants.MAXIMUM_PITCH

                    if newAmplitude != note.amplitude:
                        if newAmplitude >= Constants.MINIMUM_AMPLITUDE and newAmplitude <= Constants.MAXIMUM_AMPLITUDE:
                            note.amplitude = newAmplitude
                        elif newAmplitude < Constants.MINIMUM_AMPLITUDE:
                            note.amplitude = Constants.MINIMUM_AMPLITUDE
                        elif newAmplitude > Constants.MAXIMUM_AMPLITUDE:
                            note.amplitude = Constants.MAXIMUM_AMPLITUDE

                    if newPan != note.pan:
                        note.pan = newPan

                    if newReverbSend != note.reverbSend:
                        if newReverbSend >= Constants.MINIMUM_AMPLITUDE and newReverbSend <= Constants.MAXIMUM_AMPLITUDE:
                            note.reverbSend = newReverbSend
                        elif newReverbSend < Constants.MINIMUM_AMPLITUDE:
                            note.reverbSend = Constants.MINIMUM_AMPLITUDE
                        elif newReverbSend > Constants.MAXIMUM_AMPLITUDE:
                            note.reverbSend = Constants.MAXIMUM_AMPLITUDE

        self.updatePageCallback()

    def handleButtonPress( self, drawingArea, event ):
        #TODO change this to accomodate the space between tracks 
        trackHeight = ( drawingArea.get_allocation().height - 1 ) / len( self.trackIDs )
        trackID = int( floor( event.y / trackHeight ) )
        
        if event.type == gtk.gdk.BUTTON_PRESS:
            #single click toggles track selection
            if trackID in self.selectedTrackIDs:
                self.selectedTrackIDs.discard( trackID )
            else:
                self.selectedTrackIDs.add( trackID )
        elif event.type == gtk.gdk._2BUTTON_PRESS:
            #double click selects a single track
            self.selectedTrackIDs.clear()
            self.selectedTrackIDs.add( trackID )
            
        self.drawingArea.queue_draw()
        self.selectionChangedCallback()
        if event.button == 3:
            self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )
            
    #-----------------------------------
    # drawing methods
    #-----------------------------------
    def draw( self, drawingArea, event ):
        context = drawingArea.window.cairo_create()
        parentRect = self.get_allocation()
        
        self.drawBorders( parentRect, context )
        self.drawBeatLines( parentRect, context )        
        
    #TODO this is just a temporary background
    def drawBorders( self, parentRect, context ):
        trackHeight = int( floor( parentRect.height / len( self.trackIDs ) ) )
        trackWidth = parentRect.width - 2
        trackSpacing = self.getTrackSpacing()
        
        trackIndex = 0
        for trackID in self.trackIDs:
            if trackID in self.selectedTrackIDs:
                context.set_line_width( GUIConstants.SELECTED_BORDER_SIZE )
            else:
                context.set_line_width( GUIConstants.BORDER_SIZE )

            trackBackgroundYValue = trackHeight * trackIndex + ( context.get_line_width() / 2.0 )

            context.move_to( context.get_line_width() / 2.0, trackBackgroundYValue )
            context.rel_line_to( trackWidth - ( context.get_line_width() / 2.0 ), 0 )
            context.rel_line_to( 0, trackHeight - trackSpacing )
            context.rel_line_to( -trackWidth + ( context.get_line_width() / 2.0 ), 0 )
            context.close_path()
            
            #grey background
            context.set_source_rgb( 0.75, 0.75, 0.75 )
            context.fill_preserve()
            
            #black border
            context.set_source_rgb( 0, 0, 0 )
            context.stroke()

            trackIndex += 1

    def drawBeatLines( self, parentRect, context ):
        numberOfBeats = int(round( self.beatsPerPageAdjustment.value, 0 ))
        distanceBetweenBeats = parentRect.width / numberOfBeats

        context.set_line_width( GUIConstants.BEAT_LINE_SIZE )
        for beatIndex in range( 1, numberOfBeats ):
            context.move_to( beatIndex * distanceBetweenBeats, 0 )
            context.rel_line_to( 0, parentRect.height - 4 )
            context.stroke()
