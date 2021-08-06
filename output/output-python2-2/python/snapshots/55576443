import pygtk
pygtk.require( '2.0' )
import gtk

from Edit.NoteInterface import NoteInterface
import Config

class HitInterface( NoteInterface ):

    def __init__( self, parent, page, track, note, pitch, onset, duration, amplitude, image, imageSelected, colors ):
        NoteInterface.__init__( self, parent, page, track, note, pitch, onset, duration, amplitude, image, imageSelected, colors )

        self.width = self.height = Config.HIT_HEIGHT 
        self.imgWidth = self.imgHeight = Config.HIT_HEIGHT + Config.HIT_IMAGE_PADDING_MUL2

        self.firstTransform = True
        self.updateTransform()

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

        self.updateTransform()  
        
    def getId( self ):
        return self.note  

    def getStartTick( self ):
        return self.onset

    def getEndTick( self ):
        return self.end  

    def testOnset( self, start, stop ):
        return self.onset >= start and self.onset < stop

    def updateTransform( self ):
        if self.page == self.parent.curPage and not self.firstTransform:
            oldX = self.imgX
            oldY = self.imgY
            oldEndX = self.imgX + self.imgWidth

        if self.onset != self.oldOnset:
            self.x = self.parent.ticksToPixels( self.onset )
            self.x += self.origin[0]
            self.imgX = self.x - Config.NOTE_IMAGE_PADDING
            self.oldOnset = self.onset
        if self.pitch != self.oldPitch:
            self.y = self.parent.pitchToPixelsDrum( self.pitch ) + self.origin[1]
            self.imgY = self.y - Config.NOTE_IMAGE_PADDING
            self.oldPitch = self.pitch
            
        if self.page == self.parent.curPage:
            if self.firstTransform:
                self.parent.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.page )
                self.firstTransform = False
            else:
                x = min( self.imgX, oldX )
                y = min( self.imgY, oldY )
                endx = max( self.imgX + self.imgWidth, oldEndX )
                endy = max( self.imgY, oldY ) + self.imgHeight
                self.parent.invalidate_rect( x, y, endx-x, endy-y, self.page )

    def updateDragLimits( self, dragLimits, leftBound, rightBound, widthBound, maxRightBound ):
        left = 0 - self.onset
        right = maxRightBound - self.duration - self.onset
        up = Config.MAXIMUM_PITCH_DRUM - self.pitch
        down = Config.MINIMUM_PITCH_DRUM - self.pitch
        
        if dragLimits[0][0] < left:  dragLimits[0][0] = left
        if dragLimits[0][1] > right: dragLimits[0][1] = right
        if dragLimits[1][0] < down:  dragLimits[1][0] = down
        if dragLimits[1][1] > up:    dragLimits[1][1] = up
        
        # store the current loc as a reference point
        self.baseOnset = self.onset
        self.basePitch = self.pitch

    #=======================================================
    #  Events

    # handleButtonPress returns:
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
            return 0 # not a hit
    
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
            if percent < 0.5:   emitter.setCurrentAction( "note-drag-onset", self )
            else:               emitter.setCurrentAction( "note-drag-pitch-drum", self )
                
        return 1

    def noteDrag( self, emitter, do, dp, dd ):
        self.potentialDeselect = False
        changed = False

        if do != self.lastDragO:
            self.lastDragO = do
            self.onset = self.baseOnset + do
            self.end = self.onset + self.duration
            changed = True

        if dp != self.lastDragP and not dp%2:
            self.lastDragP = dp
            newPitch = self.basePitch + dp
            self.pitch = newPitch
            self.updateSampleNote( newPitch )
            changed = True

        self.updateTransform()

        if changed: return (self.note, self.pitch, self.onset, self.duration )
        else: return False

    # updateTooltip returns:
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
            return 0 # not a hit
            
        percent = eX/self.width
        if percent < 0.5:   emitter.setCursor("drag-onset")
        else:               emitter.setCursor("drag-pitch")
        
        return 1 # we handled it

    #=======================================================
    #  Draw

    def draw( self, win, gc, startX, stopX ):
        if stopX < self.imgX: return False                  # we don't need to draw and no one after us will draw
        if startX > self.imgX + self.imgWidth: return True  # we don't need to draw, but maybe a later note does

        gc.foreground = self.color
        win.draw_rectangle( gc, True, self.x+2, self.y+2, self.width-4, self.height-4 )

        if self.selected: img = self.imageSelected
        else:             img = self.image
        win.draw_pixbuf( gc, img, 0, 0, self.imgX, self.imgY, self.imgWidth, self.imgHeight, gtk.gdk.RGB_DITHER_NONE )

        return True # we drew something
        
