import pygtk
pygtk.require( '2.0' )
import gtk

import Config

class NoteInterface:

    def __init__( self, parent, page, track, note, pitch, onset, duration, amplitude, image, imageSelected, colors ):
        self.parent = parent
        self.page = page
        self.track = track
        self.note = note # note id, not csnote!

        self.x = 0
        self.y = 0
        self.width = 1
        self.height = Config.NOTE_HEIGHT
        self.imgX = 0
        self.imgY = 0
        self.imgWidth = 1
        self.imgHeight = self.height + Config.NOTE_IMAGE_PADDING_MUL2

        self.selected = False
        self.potentialDeselect = False
            
        self.lastDragO = 0
        self.lastDragP = 0
        self.lastDragD = 0
        
        self.image = image
        self.imageSelected = imageSelected
        self.baseColors = colors

        self.updateParams( pitch, onset, duration, amplitude )

    def destroy( self ):
        # nothing to do?
        return

    def updateParams( self, pitch, onset, duration, amplitude):
        self.pitch = pitch
        self.onset = onset
        self.duration = duration
        self.end = onset + duration

        self.amplitude = amplitude
        r = self.baseColors[0][0] + int(self.baseColors[1][0]*amplitude)
        g = self.baseColors[0][1] + int(self.baseColors[1][1]*amplitude)
        b = self.baseColors[0][2] + int(self.baseColors[1][2]*amplitude)
        self.color = self.parent.drawingArea.get_colormap().alloc_color( r, g, b, True, True )

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
            oldX = self.imgX
            oldY = self.imgY
            oldEndX = self.imgX + self.imgWidth
 
        origin = self.parent.getTrackOrigin( self.track )
        self.x = self.parent.ticksToPixels( self.onset )
        self.width = self.parent.ticksToPixels( self.end ) - self.x
        self.imgWidth = self.width + Config.NOTE_IMAGE_PADDING_MUL2
        self.x += origin[0]
        self.imgX = self.x - Config.NOTE_IMAGE_PADDING
        if not onlyX: 
            self.y = self.parent.pitchToPixels( self.pitch ) + origin[1]
            self.imgY = self.y - Config.NOTE_IMAGE_PADDING
            
        if self.page == self.parent.curPage:
            x = min( self.imgX, oldX )
            y = min( self.imgY, oldY )
            endx = max( self.imgX + self.imgWidth, oldEndX )
            endy = max( self.imgY, oldY ) + self.imgHeight
            self.parent.invalidate_rect( x, y, endx-x, endy-y, self.page )

    def updateDragLimits( self, dragLimits, leftBound, rightBound, widthBound ):
        left = leftBound - self.onset
        right = rightBound - self.duration - self.onset
        up = Config.MAXIMUM_PITCH - self.pitch
        down = Config.MINIMUM_PITCH - self.pitch
        short = Config.MINIMUM_NOTE_DURATION - self.duration
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
            check = self.onset - Config.TICKS_PER_BEAT
            while start <= check: start += Config.TICKS_PER_BEAT
            stop = start + Config.TICKS_PER_BEAT
            check += self.duration
            while stop < check: stop += Config.TICKS_PER_BEAT
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
                self.parent.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.page )
            return True # state changed
        return False    # state is the same

    def getSelected( self ):
        return self.selected

    #=======================================================
    #  Selection

    def draw( self, win, gc, startX, stopX ):
        if stopX < self.imgX: return False                  # we don't need to draw and no one after us will draw
        if startX > self.imgX + self.imgWidth: return True  # we don't need to draw, but maybe a later note does

        gc.foreground = self.color
        win.draw_rectangle( gc, True, self.x+1, self.y+1, self.width-2, self.height-2 )

        if self.selected: img = self.imageSelected
        else:             img = self.image
        win.draw_pixbuf( gc, img, 0, 0, self.imgX, self.imgY, self.imgWidth-Config.NOTE_IMAGE_ENDLENGTH, self.imgHeight, gtk.gdk.RGB_DITHER_NONE )
        win.draw_pixbuf( gc, img, Config.NOTE_IMAGE_TAIL, 0, self.imgX+self.imgWidth-Config.NOTE_IMAGE_ENDLENGTH, self.imgY, Config.NOTE_IMAGE_ENDLENGTH, self.imgHeight, gtk.gdk.RGB_DITHER_NONE )

        return True # we drew something
        
