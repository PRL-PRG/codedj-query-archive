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
    def handleExposeEvent( self, area, event ):
        self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "blue" ) )
        gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
        
        self.drawingArea.window.draw_rectangle( gc, True, 0, 0, self.getWidth(), self.getHeight() )
        
        self.drawingArea.modify_fg( gtk.STATE_NORMAL, self.drawingArea.get_colormap().alloc_color( "black" ) )
        gc = self.drawingArea.get_style().fg_gc[ gtk.STATE_NORMAL ]
        self.drawingArea.window.draw_rectangle( gc, False, 0, 0, self.getWidth() - 1, self.getHeight() - 1 )

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
        #return self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES #+ 5

    def getXPosition( self ):
        return int( self.note.onset * self.getParentWidth() / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT )

    def getYPosition( self ):
        return int( ( 24 - ( self.note.pitch - 24 ) ) * self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES )

    def getParentWidth( self ):
        return self.parentContainer.get_allocation().width

    def getParentHeight( self ):
        return self.parentContainer.get_allocation().height
