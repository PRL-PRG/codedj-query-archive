import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants

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
    def __init__( self, trackIDs, selectedTrackIDs, selectionChangedCallback, mutedTrackIDs, beatsPerPageAdjustment ):
        gtk.EventBox.__init__( self )
        
        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )
        
        self.trackIDs = trackIDs
        self.selectedTrackIDs = selectedTrackIDs
        self.selectionChangedCallback = selectionChangedCallback
        self.mutedTrackIDs = mutedTrackIDs
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        
        self.drawingArea.connect( "expose-event", self.redraw )
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

    def handleButtonPress( self, drawingArea, event ):
        #TODO change this to accomodate the space between tracks 
        trackHeight = ( drawingArea.get_allocation().height - 1 ) / len( self.trackIDs )
        trackID = int( floor( event.y / trackHeight ) )
        
        if trackID in self.selectedTrackIDs:
            self.selectedTrackIDs.discard( trackID )
        else:
            self.selectedTrackIDs.add( trackID )
            
        self.drawingArea.queue_draw()
        self.selectionChangedCallback()

    #-----------------------------------
    # drawing methods
    #-----------------------------------
    def redraw( self, drawingArea, event ):
        parentRect = self.get_allocation()
        self.drawTrackBackgrounds( parentRect )
        self.drawBeatLines( parentRect )

    #TODO this is just a temporary background
    def drawTrackBackgrounds( self, parentRect ):
        trackHeight = int( floor( parentRect.height / len( self.trackIDs ) ) )
        trackWidth = parentRect.width - 2
        
        trackSpacing = self.getTrackSpacing()
        
        trackIndex = 0
        for trackID in self.trackIDs:
            if trackID in self.selectedTrackIDs:
                self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
                borderSize = GUIConstants.SELECTED_BORDER_SIZE
            else:
                self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
                borderSize = GUIConstants.BORDER_SIZE

            trackBackgroundYValue = trackHeight * trackIndex
        
            #background
            self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
            self.drawingArea.window.draw_rectangle( self.gc, True,
                                                    0, trackBackgroundYValue,
                                                    trackWidth, trackHeight - trackSpacing )

            #background frame
            self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
            self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
            self.drawingArea.window.draw_rectangle( self.gc, False,
                                                    0, trackBackgroundYValue,
                                                    trackWidth - 1, trackHeight - trackSpacing - 1 )
            
            #foreground
            self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "gray" ) )
            self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
            self.drawingArea.window.draw_rectangle( self.gc, True, 
                                                    borderSize, trackBackgroundYValue + borderSize, 
                                                    trackWidth - 2 * borderSize, 
                                                    trackHeight - trackSpacing - 2 * borderSize)
            
            #foreground frame
            self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
            self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
            self.drawingArea.window.draw_rectangle( self.gc, False, 
                                                    borderSize, trackBackgroundYValue + borderSize, 
                                                    ( trackWidth - 2 * borderSize ) - 1,
                                                    ( trackHeight - trackSpacing - 2 * borderSize ) - 1 )
            
            trackIndex += 1

    def drawBeatLines( self, parentRect ):
        numberOfBeats = round( self.beatsPerPageAdjustment.value, 0 )
        distanceBetweenBeats = parentRect.width / numberOfBeats
        numberOfBeats = int( numberOfBeats )

        self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
        self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
        for beatIndex in range( 1, numberOfBeats ):
            self.drawingArea.window.draw_line( self.gc, int( beatIndex * distanceBetweenBeats ), 0,
                                               int( beatIndex * distanceBetweenBeats ), parentRect.height - 8 )

    def drawRect( self, rect ):
        self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "green" ) )
        self.gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
        self.drawingArea.window.draw_rectangle( self.gc, False,
                                                rect.x, rect.y, rect.width, rect.height )