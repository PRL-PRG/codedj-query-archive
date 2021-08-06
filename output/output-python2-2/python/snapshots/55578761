import pygtk
pygtk.require( '2.0' )
import gtk 

from NoteView import NoteView

#----------------------------------------------------------------------
# This view class is used to show the contents of a NoteTrack
# i.e. a Collection of Note objects
#----------------------------------------------------------------------
class TrackView( gtk.Fixed ):
    #-----------------------------------
    # initialization functions
    #-----------------------------------
    def __init__( self, beatsPerPageAdjustment ):
        gtk.Fixed.__init__( self )

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
            self.put( noteView, noteView.getXPosition(), noteView.getYPosition() )
            
        self.queue_draw()

    def clearNotes( self ):
        for noteView in self.noteViews:
            noteView.hide()
            
        del self.noteViews
        self.noteViews = []

    #-----------------------------------
    # sizing methods
    #-----------------------------------
    def updateNoteViewSizes( self ):
        for noteView in self.noteViews:
            noteView.updateSize()
            self.move( noteView, noteView.getXPosition(), noteView.getYPosition() )

    def set_size_request( self, width, height ):
        gtk.Fixed.set_size_request( self, width, height )
        self.updateNoteViewSizes()
        
#unused for now...
class NoteViewPool:
    def __init__( self, parentContainer, beatsPerPageAdjustment ):
        self.parentContainer = parentContainer
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.pool = []

    def addNoteView( self, noteView ):
        noteView.hide()
        self.pool.append( noteView )
    
    def addNoteViews( self, noteViews ):
        for noteView in noteViews:
            self.addNoteView( noteView )
        
    def getNoteView( self ):
        poolSize = len( pool )
        if poolSize != 0:
            return pool.pop( poolSize )
        
        return NoteView( None, self.parentContainer, self.beatsPerPageAdjustment  )