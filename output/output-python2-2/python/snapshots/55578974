import pygtk
pygtk.require( '2.0' )
import gtk

#----------------------------------------------------------------------
# A verical bar used to show the current point in time on a page
# TODO: modify this class to change the current point in time
#       on click and drag
#----------------------------------------------------------------------
class PositionIndicator( gtk.DrawingArea ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, trackIDs, selectedTrackIDs, mutedTrackIDs ):
        gtk.DrawingArea.__init__( self )
        
        self.trackIDs = trackIDs
        self.selectedTrackIDs = selectedTrackIDs
        self.mutedTrackIDs = mutedTrackIDs

        self.connect( "expose-event", self.redraw )

    #-----------------------------------
    # drawing - temporary, just to get the position indicator in place
    #-----------------------------------
    def redraw( self, drawingArea, event ):
        self.gc = self.get_style().fg_gc[ gtk.STATE_NORMAL ]

        indicatorSize = self.get_allocation()
        trackHeight = indicatorSize.height / len( self.trackIDs )

        trackIndex = 0
        for trackID in self.trackIDs:
            height = trackIndex * trackHeight
 
            playingSelected = len( self.selectedTrackIDs ) > 0
            
            if ( playingSelected and trackID in self.selectedTrackIDs ):
                self.gc.foreground = self.get_colormap().alloc_color( "black" )
            elif ( not playingSelected and trackID not in self.mutedTrackIDs ):
                self.gc.foreground = self.get_colormap().alloc_color( "black" )
            else:
                self.gc.foreground = self.get_colormap().alloc_color( "gray" )
            
            self.window.draw_rectangle( self.gc, True, 0, height,
                                        indicatorSize.width, height + trackHeight )
            trackIndex += 1

        # TODO needed to reset the foreground colour to black, 
        # otherwise all other controls were gray
        self.gc.foreground = self.get_colormap().alloc_color( "black" )
        self.window.draw_rectangle( self.gc, False, 0, 0,
                                    indicatorSize.width - 1, indicatorSize.height - 1 )