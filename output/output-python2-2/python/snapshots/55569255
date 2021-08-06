import pygtk
pygtk.require( '2.0' )
import gtk 

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants

from BackgroundView import SELECTNOTES
from NoteView import NoteView


#----------------------------------------------------------------------
# This view class is used to show the contents of a NoteTrack
# i.e. a Collection of Note objects
#----------------------------------------------------------------------
class TrackView:
    #-----------------------------------
    # initialization functions
    #-----------------------------------
    def __init__( self, trackID, beatsPerPageAdjustment ):
        self.trackID = trackID
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.noteViews = []
        self.posOffset = (0,0)
        self.selectedNotes = []

    def getID( self ):
        return self.trackID

    #-----------------------------------
    # modification methods
    #-----------------------------------
    def setNotes( self, notes ):
        self.clearNotes()
       
        lineW = self.getBorderWidth()

        for note in notes:
            noteView = NoteView( note, self, self.beatsPerPageAdjustment )
            self.noteViews.append( noteView )
            noteView.setPositionOffset( (self.posOffset[0]+lineW, self.posOffset[1]+lineW ) )
        
        self.updateNoteTransforms()

    def clearNotes( self ):
        del self.noteViews
        self.noteViews = []
        self.selectedNotes = []

    def selectNotes( self, mode, which ):
        if mode == SELECTNOTES.ALL:
            for note in self.noteViews: note.setSelected( True )
            self.selectedNotes = self.noteViews[:]
        elif mode == SELECTNOTES.NONE:
            for note in self.noteViews: note.setSelected( False )
            self.selectedNotes = []
        elif mode == SELECTNOTES.ADD:
            for note in which: 
                if note.setSelected( True ): 
                    self.selectedNotes.insert( 0, note )
        elif mode == SELECTNOTES.REMOVE:
            for note in which: 
                if note.setSelected( False ):
                    self.selectedNotes.remove( note )
        elif mode == SELECTNOTES.EXCLUSIVE:
            for note in self.noteViews:
                if note in which: 
                    if note.setSelected( True ):
                        self.selectedNotes.insert( 0, note )
                else: 
                    if note.setSelected( False ):
                        self.selectedNotes.remove( note )

    def updateDragLimits( self, dragLimits ):
        if not len(self.selectedNotes): return  # no selected notes here

        leftBound = 0
        maxRightBound = round( self.beatsPerPageAdjustment.value, 0 ) * Constants.TICKS_PER_BEAT
        last = len(self.noteViews)-1
        for i in range(0,last):
            if not self.noteViews[i].getSelected(): 
                leftBound = self.noteViews[i].getEndTick()
            else:
                if not self.noteViews[i+1].getSelected(): 
                    rightBound = min( self.noteViews[i+1].getStartTick(), maxRightBound )
                    widthBound = rightBound
                else:
                    rightBound = maxRightBound
                    widthBound = min( self.noteViews[i+1].getStartTick(), maxRightBound )
                self.noteViews[i].updateDragLimits( dragLimits, leftBound, rightBound, widthBound )
        if self.noteViews[last].getSelected(): 
            self.noteViews[last].updateDragLimits( dragLimits, leftBound, maxRightBound, maxRightBound )

    def getNotesByBar( self, beatCount, startX, stopX ):
        beatWidth = self.getBeatLineSpacing( beatCount )
        beatStart = self.getBeatLineStart()
        while beatStart+beatWidth <= startX:
            beatStart += beatWidth
        beatStop = beatStart + beatWidth
        while beatStop+beatWidth < stopX:
            beatStop += beatWidth

        notes = []
        for note in self.noteViews:
            if note.checkX( beatStart, beatStop ):
                notes.insert(0,note)
        return notes

    #-----------------------------------
    # event methods
    #-----------------------------------

    def handleButtonPress( self, emitter, event ):
        eX = event.x - self.posOffset[0]
        eY = event.y - self.posOffset[1] 
        if eX < 0 or eX > self.width or eY < 0 or eY > self.height: 
            return False

        for note in self.noteViews:
            handled = note.handleButtonPress( emitter, event )
            if handled: return handled

        return False
    
    def handleButtonRelease( self, emitter, event, buttonPressCount ):
        eX = event.x - self.posOffset[0]
        eY = event.y - self.posOffset[1] 

        if eX < 0 or eX > self.width or eY < 0 or eY > self.height: 
           return False

        if event.button == 1:
            if buttonPressCount == 1: emitter.toggleTrack( self.trackID, False )
            else:                     emitter.toggleTrack( self.trackID, True )

        return True

    def handleMarqueeSelect( self, emitter, start, stop ):
        intersectionY = [ max(start[1],self.posOffset[1]), min(stop[1],self.posOffset[1]+self.height) ]
        if intersectionY[0] > intersectionY[1]:
            return False

        intersectionX = [ max(start[0],self.posOffset[0]), min(stop[0],self.posOffset[0]+self.width) ]
        if intersectionX[0] > intersectionX[1]:
           return False


        hits = []
        for note in self.noteViews:
            hit = note.handleMarqueeSelect( emitter, 
                                      [ intersectionX[0], intersectionY[0] ], \
                                      [ intersectionX[1], intersectionY[1] ] )           
            if hit: hits.insert(0,note)

        if len(hits): return hits
        
        return False

    def noteDrag( self, emitter, dx, dy, dw ):
        for note in self.selectedNotes:
            note.noteDrag( emitter, dx, dy, dw )

    def doneNoteDrag( self, emitter ):
        for note in self.selectedNotes:
            note.doneNoteDrag( emitter )

    #-----------------------------------
    # drawing methods
    #-----------------------------------
    
    def getBorderWidth( self ):             #should return a constant value, otherwise we have to recalculate sizing and positioning everyframe!
        return GUIConstants.BORDER_SIZE   

    def getBeatLineWidth( self ):
        return GUIConstants.BEAT_LINE_SIZE  #should return a constant value, otherwise we have to recalculate sizing and positioning everyframe!

    def getBeatLineSpacing( self, beatCount ):
        return (self.width - 2*self.getBorderWidth() + self.getBeatLineWidth())/beatCount
    
    def getBeatLineStart( self ):
        return self.posOffset[0] + self.getBorderWidth() - self.getBeatLineWidth()/2.0

    def setPositionOffset( self, offset ):
        self.posOffset = offset

        lineW = self.getBorderWidth()
        for note in self.noteViews:
            note.setPositionOffset( ( self.posOffset[0]+lineW, self.posOffset[1]+lineW ) )

    def draw( self, context, beatCount, selected ):
        #if selected: lineW = GUIConstants.SELECTED_BORDER_SIZE
        #else:        lineW = GUIConstants.BORDER_SIZE
        lineW = self.getBorderWidth()
        context.set_line_width( lineW )    
        lineWDIV2 = lineW/2.0    

        context.move_to( self.posOffset[0] + lineWDIV2, self.posOffset[1] + lineWDIV2 )
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
        context.set_line_width( beatLineWidth )
        beatWidth = self.getBeatLineSpacing( beatCount )
        beatStart = self.getBeatLineStart()
        context.set_source_rgb( 0, 0, 0 )
        for i in range(1,beatCount):
            context.move_to( beatStart + i*beatWidth, self.posOffset[1] + lineW )
            context.rel_line_to( 0, self.height - 2*lineW )
            context.stroke()

        #draw the notes
        for note in self.noteViews:
            note.draw( context )

    #-----------------------------------
    # sizing methods
    #-----------------------------------

    def updateNoteTransforms( self ):
        width = self.width - 2*self.getBorderWidth()
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
