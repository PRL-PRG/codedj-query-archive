import pygtk
pygtk.require( '2.0' )
import gtk

import common.Config as Config

from common.Util.NoteDB import PARAMETER
from common.Util.CSoundClient import new_csound_client

class NoteInterface:

    def __init__( self, noteDB, owner, note ):
        self.noteDB = noteDB
        self.owner = owner
        self.note = note

        self.origin = self.owner.getTrackOrigin( note.track )
        self.firstTransform = True
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

        self.oldOnset = -1
        self.oldEnd = -1
        self.oldPitch = -1
        self.oldAmplitude = -1
        self.oldBeats = -1
        self.lastDragO = 0
        self.lastDragP = 0
        self.lastDragD = 0

        self.image, self.imageSelected, self.colormap, self.baseColors = self.owner.getDrawingPackage( note.track )

        self.updateParameter( None, None )

    def destroy( self ):
        if self.selected:
            self.owner.deselectNotes( { self.note.track: [self] } )
        else: # if we were deselected above the rect has already been invalidated
            self.owner.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.note.page, True )

    def updateParameter( self, parameter, value ):
        self.end = self.note.cs.onset + self.note.cs.duration

        if self.oldAmplitude != self.note.cs.amplitude:
            r = self.baseColors[0][0] + int(self.baseColors[1][0]*self.note.cs.amplitude)
            g = self.baseColors[0][1] + int(self.baseColors[1][1]*self.note.cs.amplitude)
            b = self.baseColors[0][2] + int(self.baseColors[1][2]*self.note.cs.amplitude)
            self.color = self.colormap.alloc_color( r, g, b, True, True ) # TODO potential memory leak?
            self.oldAmplitude = self.note.cs.amplitude

        self.updateTransform()

    def getId( self ):
        return self.note.id

    def getStartTick( self ):
        return self.note.cs.onset

    def getEndTick( self ):
        return self.end

    def testOnset( self, start, stop ):
        return self.note.cs.onset >= start and self.note.cs.onset < stop

    def getPitch( self ):
        return self.note.cs.pitch

    def updateTransform( self ):
        if self.note.page in self.owner.getActivePages():
            if not self.firstTransform:
                oldX = self.imgX
                oldY = self.imgY
                oldEndX = self.imgX + self.imgWidth
            dirty = True
        else:
            dirty = False

        beats = self.noteDB.getPage( self.note.page ).beats
        if self.note.cs.onset != self.oldOnset or beats != self.oldBeats:
            self.x = self.owner.ticksToPixels( beats, self.note.cs.onset )
            self.x += self.origin[0]
            self.imgX = self.x - Config.NOTE_IMAGE_PADDING
            self.oldOnset = self.note.cs.onset
        if self.end != self.oldEnd or self.note.cs.onset != self.oldOnset or beats != self.oldBeats:
            self.width = self.owner.ticksToPixels( beats, self.end ) - self.x + self.origin[0]
            self.imgWidth = self.width + Config.NOTE_IMAGE_PADDING_MUL2
            self.oldEnd = self.end
        if self.note.cs.pitch != self.oldPitch:
            self.y = self.owner.pitchToPixels( self.note.cs.pitch ) + self.origin[1]
            self.imgY = self.y - Config.NOTE_IMAGE_PADDING
            self.oldPitch = self.note.cs.pitch
        self.oldBeats = beats

        if dirty:
            if self.firstTransform:
                self.owner.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.note.page, True )
            else:
                x = min( self.imgX, oldX )
                y = min( self.imgY, oldY )
                endx = max( self.imgX + self.imgWidth, oldEndX )
                endy = max( self.imgY, oldY ) + self.imgHeight
                self.owner.invalidate_rect( x, y, endx-x, endy-y, self.note.page, True )

        self.firstTransform = False

    def updateDragLimits( self, dragLimits, leftBound, rightBound, widthBound, maxRightBound ):
        left = leftBound - self.note.cs.onset
        right = rightBound - self.note.cs.duration - self.note.cs.onset
        up = Config.MAXIMUM_PITCH - self.note.cs.pitch
        down = Config.MINIMUM_PITCH - self.note.cs.pitch
        short = Config.MINIMUM_NOTE_DURATION - self.note.cs.duration
        long = widthBound - self.note.cs.duration - self.note.cs.onset

        if dragLimits[0][0] < left:  dragLimits[0][0] = left
        if dragLimits[0][1] > right: dragLimits[0][1] = right
        if dragLimits[1][0] < down:  dragLimits[1][0] = down
        if dragLimits[1][1] > up:    dragLimits[1][1] = up
        if dragLimits[2][0] < short: dragLimits[2][0] = short
        if dragLimits[2][1] > long:  dragLimits[2][1] = long

        # store the current loc as a reference point
        self.baseOnset = self.note.cs.onset
        self.basePitch = self.note.cs.pitch
        self.baseDuration = self.note.cs.duration

    def playSampleNote( self, full=True ):
        secs_per_tick = 0.025
        csnd = new_csound_client()

        if full:
            onset = self.note.cs.onset
            self.note.cs.onset = 0
            csnd.play( self.note.cs, 0.024)
            self.note.cs.onset = onset
        else:
            (onset,duration) = ( self.note.cs.onset, self.note.cs.duration)
            ( self.note.cs.onset, self.note.cs.duration) = (0, 10)
            csnd.play( self.note.cs, 0.024)
            ( self.note.cs.onset, self.note.cs.duration) = (onset,duration)

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

        playSample = False

        if event.type == gtk.gdk._2BUTTON_PRESS:     # select bar
            self.potentialDeselect = False
            start = 0
            check = self.note.cs.onset - Config.TICKS_PER_BEAT
            while start <= check: start += Config.TICKS_PER_BEAT
            stop = start + Config.TICKS_PER_BEAT
            check += self.note.cs.duration
            while stop < check: stop += Config.TICKS_PER_BEAT
            emitter.selectNotesByBar( self.note.track, start, stop )
        elif event.type == gtk.gdk._3BUTTON_PRESS:   # select track
            self.potentialDeselect = False
            emitter.selectNotesByTrack( self.note.track )
        else:
            if self.selected:                       # we already selected, might want to delected
                self.potentialDeselect = True
            else:
                emitter.selectNotes( { self.note.track: [ self ] } )
                playSample = True

            percent = eX/self.width
            if percent < 0.3:   emitter.setCurrentAction( "note-drag-onset", self )
            elif percent > 0.7: emitter.setCurrentAction( "note-drag-duration", self )
            else: 
                emitter.setCurrentAction( "note-drag-pitch", self )
                if playSample: self.playSampleNote()

        return 1

    def handleButtonRelease( self, emitter, event, buttonPressCount ):

        if self.potentialDeselect:
            self.potentialDeselect = False
            emitter.deselectNotes( { self.note.track: [ self ] } )

        emitter.doneCurrentAction()

        return True

    def noteDragOnset( self, do, stream ):
        self.potentialDeselect = False
        if do != self.lastDragO:
            self.lastDragO = do
            stream += [ self.note.id, self.baseOnset + do ]

    def noteDragPitch( self, dp, stream ):
        self.potentialDeselect = False
        if dp != self.lastDragP:
            self.lastDragP = dp
            stream += [ self.note.id, self.basePitch + dp ]

    def noteDragDuration( self, dd, stream ):
        self.potentialDeselect = False
        if dd != self.lastDragD:
            self.lastDragD = dd
            stream += [ self.note.id, self.baseDuration + dd ]

    def doneNoteDrag( self, emitter ):
        self.baseOnset = self.note.cs.onset
        self.basePitch = self.note.cs.pitch
        self.baseDuration = self.note.cs.duration

        self.lastDragO = 0
        self.lastDragP = 0
        self.lastDragD = 0

    def noteDecOnset( self, step, leftBound, stream ):
        if self.selected:
            if leftBound < self.note.cs.onset:
                onset = max( self.note.cs.onset+step, leftBound )
                stream += [ self.note.id, onset ]
                return onset + self.note.cs.duration
        return self.end

    def noteIncOnset( self, step, rightBound, stream ):
        if self.selected:
            if rightBound > self.end:
                onset = min( self.end+step, rightBound ) - self.note.cs.duration
                stream += [ self.note.id, onset ]
                return onset
        return self.note.cs.onset

    def noteDecPitch( self, step, stream ):
        if self.note.cs.pitch > Config.MINIMUM_PITCH:
            stream += [ self.note.id, max( self.note.cs.pitch+step, Config.MINIMUM_PITCH ) ]

    def noteIncPitch( self, step, stream ):
        if self.note.cs.pitch < Config.MAXIMUM_PITCH:
            stream += [ self.note.id, min( self.note.cs.pitch+step, Config.MAXIMUM_PITCH ) ]

    def noteDecDuration( self, step, stream ):
        if self.note.cs.duration > Config.MINIMUM_NOTE_DURATION:
            stream += [ self.note.id, max( self.note.cs.duration+step, Config.MINIMUM_NOTE_DURATION ) ]

    def noteIncDuration( self, step, rightBound, stream ):
        if self.selected:
            if self.end < rightBound:
                stream += [ self.note.id, min( self.end+step, rightBound ) - self.note.cs.onset ]

    def noteDecVolume( self, step, stream ):
        if self.note.cs.amplitude > 0:
            stream += [ self.note.id, max( self.note.cs.amplitude+step, 0 ) ]

    def noteIncVolume( self, step, stream ):
        if self.note.cs.amplitude < 1:
            stream += [ self.note.id, min( self.note.cs.amplitude+step, 1 ) ]

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
            self.owner.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.note.page )
            return True # state changed
        return False    # state is the same

    def getSelected( self ):
        return self.selected

    #=======================================================
    #  Draw

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

