import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants

#----------------------------------------------------------------------
# TODO: currently we are only using CSoundNotes, 
#     - this should updated to handle generic Note objects
#----------------------------------------------------------------------

#----------------------------------------------------------------------
# A view for the (CSound)Note class
#----------------------------------------------------------------------
class NoteView( gtk.EventBox ):
    #-----------------------------------
    # initialization
    # TODO: not sure if passing in beatsPerPageAdjustment is the best way to go about it
    #-----------------------------------
    def __init__( self, note, parentContainer, beatsPerPageAdjustment ):
        gtk.EventBox.__init__( self )
        
        self.note = note
        self.parentContainer = parentContainer
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        
        self.drawingArea = gtk.DrawingArea()
        self.drawingArea.connect( "expose-event", self.handleExposeEvent )
        self.connect( "button-press-event", self.handleButtonPress )
        self.add( self.drawingArea )
        self.show_all()

    def handleButtonPress( self, eventBox, event ):
        print "clicked a note!"

    # TODO: this is a TEMPORARY implementation to get notes displayed
    def handleExposeEvent( self, drawingArea, event ):
        size = drawingArea.get_allocation()
        context = drawingArea.window.cairo_create()
        
        context.set_line_width( GUIConstants.BORDER_SIZE )
        context.move_to( 0, 0 )
        context.rel_line_to( size.width, 0 )
        context.rel_line_to( 0, size.height )
        context.rel_line_to( -size.width, 0 )
        context.close_path()
            
        #blue background
        context.set_source_rgb( 0, 0, 1 )
        context.fill_preserve()
            
        #black border
        context.set_source_rgb( 0, 0, 0 )
        context.stroke()

    #-----------------------------------
    # update
    #-----------------------------------
    def updateSize( self ):
        width = self.getWidth()
        height = self.getHeight()
        self.set_size_request( width, height )
        self.drawingArea.set_size_request( width, height )
        self.drawingArea.queue_draw()

    #-----------------------------------
    # get size functions
    #-----------------------------------    
    def getWidth( self ):
        return int( self.getParentWidth() / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT * self.note.duration )

    def getHeight( self ):
        return int( max( GUIConstants.MINIMUM_NOTE_HEIGHT, self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES ) )

    def getXPosition( self ):
        return int( self.note.onset * self.getParentWidth() / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT )

    def getYPosition( self ):
        return int( ( 24 - ( self.note.pitch - 24 ) ) * self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES )

    def getParentWidth( self ):
        return self.parentContainer.get_allocation().width

    def getParentHeight( self ):
        return self.parentContainer.get_allocation().height
