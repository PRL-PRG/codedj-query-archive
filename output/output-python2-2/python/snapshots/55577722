import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants

class NoteInterface:

    def __init__( self, parent, page, track, note, pitch, onset, duration, amplitude):
        self.parent = parent
        self.page = page
        self.track = track
        self.note = note # note id, not csnote!

        self.x = 0
        self.y = 0
        self.width = 1
        self.height = GUIConstants.NOTE_HEIGHT

        self.updateParams( pitch, onset, duration, amplitude )

        self.selected = False
        self.potentialDeselect = False
            
        self.lastDragO = 0
        self.lastDragP = 0
        self.lastDragD = 0

    def destroy( self ):
        # nothing to do?
        return

    def updateParams( self, pitch, onset, duration, amplitude):
        self.pitch = pitch
        self.onset = onset
        self.duration = duration
        self.end = onset + duration

        self.amplitude = amplitude
        self.bgColour = 1 - ( ( self.amplitude * 0.7 ) + 0.3 )

        self.updateTransform( False )  
        
    def getId( self ):
        return self.note  

    def getStartTick( self ):
        return self.onset

    def getEndTick( self ):
        return self.end  

    def testOnset( self, start, stop ):
        return self.onset >= start and self.onset < stop

    def updateTransform( self, onlyX ):
        if self.page == self.parent.curPage:
            oldX = self.x
            oldY = self.y
            oldEndX = self.x + self.width
 
        origin = self.parent.getTrackOrigin( self.track )
        self.x = self.parent.ticksToPixels( self.onset )
        self.width = self.parent.ticksToPixels( self.end ) - self.x
        self.x += origin[0]
        if not onlyX: 
            self.y = self.parent.pitchToPixels( self.pitch ) + origin[1]

        if self.page == self.parent.curPage:
            x = min( self.x, oldX )
            y = min( self.y, oldY )
            endx = max( self.x + self.width, oldEndX )
            endy = max( self.y, oldY ) + self.height
            self.parent.invalidate_rect( x, y, endx-x, endy-y )

    def updateDragLimits( self, dragLimits, leftBound, rightBound, widthBound ):
        left = leftBound - self.onset
        right = rightBound - self.duration - self.onset
        up = Constants.MAXIMUM_PITCH - self.pitch
        down = Constants.MINIMUM_PITCH - self.pitch
        short = Constants.MINIMUM_NOTE_DURATION - self.duration
        long = widthBound - self.duration - self.onset

        if dragLimits[0][0] < left:  dragLimits[0][0] = left
        if dragLimits[0][1] > right: dragLimits[0][1] = right
        if dragLimits[1][0] < down:  dragLimits[1][0] = down
        if dragLimits[1][1] > up:    dragLimits[1][1] = up
        if dragLimits[2][0] < short: dragLimits[2][0] = short
        if dragLimits[2][1] > long:  dragLimits[2][1] = long

        # store the current loc as a reference point
        self.baseOnset = self.onset
        self.basePitch = self.pitch
        self.baseDuration = self.duration

    def updateSampleNote( self, pitch ):
        return

    def clearSampleNote( self ):
        return

    #=======================================================
    #  Events

    # handleButtonPress returns:
    # -2, not a hit but there was X overlap
    # -1, event occurs before us so don't bother checking any later notes
    #  0, event didn't hit
    #  1, event was handled
    def handleButtonPress( self, emitter, event ):
        eX = event.x - self.x
        if eX < 0:
            return -1 # event occurs before us, no point in checking further
        if eX > self.width:
            return 0 # no X overlap
            
        eY = event.y - self.y
        if eY < 0 or eY > self.height:
            return -2 # not a hit, but it was in our X range
    
        if event.button == 3:
            print "Show some note parameters!?!"            
            #self.noteParameters = NoteParametersWindow( self.note, self.getNoteParameters ) 
            return 1 # handled

        if event.type == gtk.gdk._2BUTTON_PRESS:     # select bar
            self.potentialDeselect = False
            start = 0
            check = self.onset - Constants.TICKS_PER_BEAT
            while start <= check: start += Constants.TICKS_PER_BEAT
            stop = start + Constants.TICKS_PER_BEAT
            check += self.duration
            while stop < check: stop += Constants.TICKS_PER_BEAT
            emitter.selectNotesByBar( self.track, start, stop )
        elif event.type == gtk.gdk._3BUTTON_PRESS:   # select track
            self.potentialDeselect = False
            emitter.selectNotesByTrack( self.track )
        else:
            if self.getSelected():                       # we already selected, might want to delected
                self.potentialDeselect = True
            else:
                emitter.selectNotes( { self.track: [ self ] } )
            self.updateSampleNote( self.pitch )
            
            percent = eX/self.width
            if percent < 0.3:   emitter.setCurrentAction( "note-drag-onset", self )
            elif percent > 0.7: emitter.setCurrentAction( "note-drag-duration", self )
            else:               emitter.setCurrentAction( "note-drag-pitch", self )
                
        return 1

    def handleButtonRelease( self, emitter, event, buttonPressCount ):

        if self.potentialDeselect:
            self.potentialDeselect = False
            emitter.deselectNotes( { self.track: [ self ] } )

        self.clearSampleNote()
        
        emitter.doneCurrentAction()

        return True

    def noteDrag( self, emitter, do, dp, dd ):
        self.potentialDeselect = False

        if do != self.lastDragO:
            self.lastDragO = do
            self.onset = self.baseOnset + do
            self.end = self.onset + self.duration

        if dp != self.lastDragP:
            self.lastDragP = dp
            newPitch = self.basePitch + dp
            self.pitch = newPitch
            self.updateSampleNote( newPitch )

        if dd != self.lastDragD:
            self.lastDragD = dd
            self.duration = self.baseDuration + dd
            self.end = self.onset + self.duration

        self.updateTransform( False )

        return (self.note, self.pitch, self.onset, self.duration)

    def doneNoteDrag( self, emitter ):
        self.baseOnset = self.onset
        self.basePitch = self.pitch
        self.baseDuration = self.duration
    
        self.lastDragO = 0
        self.lastDragP = 0
        self.lastDragD = 0

        self.clearSampleNote()
        

    def handleMarqueeSelect( self, emitter, start, stop ):
        intersectionY = [ max(start[1],self.y), min(stop[1],self.y+self.height) ]
        if intersectionY[0] > intersectionY[1]:
            return False

        intersectionX = [ max(start[0],self.x), min(stop[0],self.x+self.width) ]
        if intersectionX[0] > intersectionX[1]:
           return False

        return True
        
    # updateTooltip returns:
    # -2, not a hit but there was X overlap
    # -1, event occurs before us so don't bother checking any later notes
    #  0, event didn't hit
    #  1, event was handled
    def updateTooltip( self, emitter, event ):
        eX = event.x - self.x
        if eX < 0:
            return -1 # event occurs before us, no point in checking further
        if eX > self.width:
            return 0 # no X overlap
            
        eY = event.y - self.y
        if eY < 0 or eY > self.height:
            return -2 # not a hit, but it was in our X range
            
        percent = eX/self.width
        if percent < 0.3:   emitter.setCursor("drag-onset")
        elif percent > 0.7: emitter.setCursor("drag-duration")
        else:               emitter.setCursor("drag-pitch")
        
        return 1 # we handled it

    #=======================================================
    #  Selection
    
    def setSelected( self, state ):
        if self.selected != state:
            self.selected = state
            if self.page == self.parent.curPage:
                self.parent.invalidate_rect( self.x, self.y, self.width, self.height )
            return True # state changed
        return False    # state is the same

    def getSelected( self ):
        return self.selected

    #=======================================================
    #  Selection

    def draw( self, context, startX, stopX ):
        if stopX < self.x: return False               # we don't need to draw and no one after us will draw
        if startX > self.x + self.width: return True  # we don't need to draw, but maybe a later note does
       
        context.set_line_width( GUIConstants.NOTE_BORDER_SIZE )

        context.move_to( self.x + GUIConstants.NOTE_BORDER_SIZE_DIV2, self.y + GUIConstants.NOTE_BORDER_SIZE_DIV2 )
        context.rel_line_to( self.width - GUIConstants.NOTE_BORDER_SIZE, 0 )
        context.rel_line_to( 0, self.height - GUIConstants.NOTE_BORDER_SIZE )
        context.rel_line_to( -self.width + GUIConstants.NOTE_BORDER_SIZE, 0 )
        context.close_path()
            
        #background
        context.set_source_rgb( self.bgColour, self.bgColour, self.bgColour )
        context.fill_preserve()
            
        #border
        if self.selected: context.set_source_rgb( 1, 1, 1 )
        else:             context.set_source_rgb( 0, 0, 0 )
        context.stroke()

        return True # we drew something
        
