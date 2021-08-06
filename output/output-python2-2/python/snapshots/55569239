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

        self.connect( "expose-event", self.draw )

    def draw( self, drawingArea, event ):
        indicatorSize = self.get_allocation()
        trackHeight = indicatorSize.height / len( self.trackIDs )
        
        context = drawingArea.window.cairo_create()

        trackIndex = 0
        for trackID in self.trackIDs:
            height = trackIndex * trackHeight
 
            context.move_to( 0, height )
            context.rel_line_to( indicatorSize.width, 0 )
            context.rel_line_to( 0, height + trackHeight )
            context.rel_line_to( -indicatorSize.width, 0 )
            context.close_path()

            if trackID not in self.mutedTrackIDs:
                context.set_source_rgb( 0, 0, 0 ) #black
            else:
                context.set_source_rgb( 0.6, 0.6, 0.6 ) #grey
            
            context.fill_preserve()
            context.stroke()
            
            trackIndex += 1