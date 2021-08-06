import pygtk
pygtk.require( '2.0' )
import gtk 

from GUI.GUIConstants import GUIConstants

from NoteView import NoteView

#----------------------------------------------------------------------
# This view class is used to show the contents of a NoteTrack
# i.e. a Collection of Note objects
#----------------------------------------------------------------------
class TrackView:
    #-----------------------------------
    # initialization functions
    #-----------------------------------
    def __init__( self, beatsPerPageAdjustment ):
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.noteViews = []

    #-----------------------------------
    # modification methods
    #-----------------------------------
    def setNotes( self, notes ):
        self.clearNotes()
       
        for note in notes:
            noteView = NoteView( note, self, self.beatsPerPageAdjustment )
            self.noteViews.append( noteView )
        
        self.updateNoteTransforms()

    def clearNotes( self ):
        del self.noteViews
        self.noteViews = []

    #-----------------------------------
    # drawing methods
    #-----------------------------------
    
    def getBorderWidth( self ):             #should return a constant value, otherwise we have to recalculate sizing and positioning everyframe!
        return GUIConstants.BORDER_SIZE   

    def getBeatLineWidth( self ):
        return GUIConstants.BEAT_LINE_SIZE  #should return a constant value, otherwise we have to recalculate sizing and positioning everyframe!

    def draw( self, context, offset, beatCount, selected ):
        #if selected: lineW = GUIConstants.SELECTED_BORDER_SIZE
        #else:        lineW = GUIConstants.BORDER_SIZE
        lineW = self.getBorderWidth()
        context.set_line_width( GUIConstants.BORDER_SIZE )    
        lineWDIV2 = GUIConstants.BORDER_SIZE/2.0    

        context.move_to( offset[0] + lineWDIV2, offset[1] + lineWDIV2 )
        context.rel_line_to( self.width - lineW, 0 )
        context.rel_line_to( 0, self.height - lineW )
        context.rel_line_to( -self.width + lineW, 0 )
        context.close_path()

        #draw the background
        context.set_source_rgb( 0.75, 0.75, 0.75 )
        context.fill_preserve()
            
        #draw the border
        if selected: context.set_source_rgb( 1, 1, 1 )
        else:        context.set_source_rgb( 0, 0, 0 )
        context.stroke()    
       
        #draw the beat lines
        beatLineWidth = self.getBeatLineWidth()
        beatWidth = (self.width - 2*lineW + beatLineWidth)/beatCount
        beatStart = offset[0] + lineW - beatLineWidth/2.0
        context.set_source_rgb( 0, 0, 0 )
        for i in range(1,beatCount):
            context.move_to( beatStart + i*beatWidth, offset[1] + lineW )
            context.rel_line_to( 0, self.height - 2*lineW )
            context.stroke()

        #draw the notes
        for note in self.noteViews:
            note.draw( context, ( offset[0]+lineW, offset[1]+lineW ) )

    #-----------------------------------
    # sizing methods
    #-----------------------------------

    def updateNoteTransforms( self ):
        width = self.width - 2*self.getBorderWidth() + self.getBeatLineWidth() # add this so that the last note butts against the border
        height = self.height - 2*self.getBorderWidth() # adjust for actual note drawing area
        for noteView in self.noteViews:
            noteView.updateTransform( (width, height) )

    def set_size_request( self, width, height ):
        self.width = width
        self.height = height
        self.updateNoteTransforms()
        
        
#unused for now...
class NoteViewPool:
    def __init__( self, parentContainer, beatsPerPageAdjustment ):
        self.parentContainer = parentContainer
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.pool = []

    def addNoteView( self, noteView ):
        #noteView.hide()
        self.pool.append( noteView )
    
    def addNoteViews( self, noteViews ):
        for noteView in noteViews:
            self.addNoteView( noteView )
        
    def getNoteView( self ):
        poolSize = len( pool )
        if poolSize != 0:
            return pool.pop( poolSize )
        
        return NoteView( None, self.parentContainer, self.beatsPerPageAdjustment  )
