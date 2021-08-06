import pygtk
pygtk.require( '2.0' )
import gtk

from common.Util.NoteDB import PARAMETER
from Edit.NoteInterface import NoteInterface
import common.Config as Config

class HitInterface( NoteInterface ):

    def __init__( self, noteDB, owner, note ):
        NoteInterface.__init__( self, noteDB, owner, note )

        self.width = self.height = Config.HIT_HEIGHT
        self.imgWidth = self.imgHeight = Config.HIT_HEIGHT + Config.HIT_IMAGE_PADDING_MUL2

        self.firstTransform = True
        self.updateTransform()

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
            self.oldBeats = beats
        if self.note.cs.pitch != self.oldPitch:
            self.y = self.owner.pitchToPixelsDrum( self.note.cs.pitch ) + self.origin[1]
            self.imgY = self.y - Config.NOTE_IMAGE_PADDING
            self.oldPitch = self.note.cs.pitch

        if dirty:
            if self.firstTransform:
                self.owner.invalidate_rect( self.imgX, self.imgY, self.imgWidth, self.imgHeight, self.note.page )
                self.firstTransform = False
            else:
                x = min( self.imgX, oldX )
                y = min( self.imgY, oldY )
                endx = max( self.imgX + self.imgWidth, oldEndX )
                endy = max( self.imgY, oldY ) + self.imgHeight
                self.owner.invalidate_rect( x, y, endx-x, endy-y, self.note.page )

        self.firstTransform = False

    def updateDragLimits( self, dragLimits, leftBound, rightBound, widthBound, maxRightBound ):
        left = 0 - self.note.cs.onset
        right = maxRightBound - self.note.cs.duration - self.note.cs.onset
        up = Config.MAXIMUM_PITCH_DRUM - self.note.cs.pitch
        down = Config.MINIMUM_PITCH_DRUM - self.note.cs.pitch

        if dragLimits[0][0] < left:  dragLimits[0][0] = left
        if dragLimits[0][1] > right: dragLimits[0][1] = right
        if dragLimits[1][0] < down:  dragLimits[1][0] = down
        if dragLimits[1][1] > up:    dragLimits[1][1] = up

        # store the current loc as a reference point
        self.baseOnset = self.note.cs.onset
        self.basePitch = self.note.cs.pitch

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

        playSample = False

        if event.type == gtk.gdk._2BUTTON_PRESS:     # select bar
            self.potentialDeselect = False
            start = 0
            check = self.note.cs.onset - Config.TICKS_PER_BEAT
            while start <= check: start += Config.TICKS_PER_BEAT
            stop = start + Config.TICKS_PER_BEAT
            check += 1
            while stop < check: stop += Config.TICKS_PER_BEAT
            emitter.selectNotesByBar( self.note.track, start, stop )
        elif event.type == gtk.gdk._3BUTTON_PRESS:   # select track
            self.potentialDeselect = False
            emitter.selectNotesByTrack( self.note.track )
        else:
            if self.getSelected():                       # we already selected, might want to delected
                self.potentialDeselect = True
            else:
                emitter.selectNotes( { self.note.track: [ self ] } )
                playSample = True

            percent = eX/self.width
            if percent < 0.5:   emitter.setCurrentAction( "note-drag-onset", self )
            else:               
                emitter.setCurrentAction( "note-drag-pitch-drum", self )
                if playSample: self.playSampleNote()

        return 1

    def noteDragPitch( self, dp, stream ):
        self.potentialDeselect = False
        if dp != self.lastDragP and not dp%2:
            self.lastDragP = dp
            stream += [ self.note.id, self.basePitch + dp ]

    def noteDragDuration( self, dd, stream ):
        return

    def noteDecOnset( self, step, leftBound, stream ):
        if self.selected:
            if leftBound < self.note.cs.onset:
                onset = max( self.note.cs.onset+step, leftBound )
                stream += [ self.note.id, onset ]
        return leftBound

    def noteIncOnset( self, step, rightBound, stream ):
        if self.selected:
            if rightBound > self.end:
                onset = min( self.end+step, rightBound ) - self.note.cs.duration
                stream += [ self.note.id, onset ]
        return rightBound

    def noteDecPitch( self, step, stream ):
        if self.note.cs.pitch > Config.MINIMUM_PITCH_DRUM:
            stream += [ self.note.id, max( self.note.cs.pitch+2*step, Config.MINIMUM_PITCH_DRUM ) ]

    def noteIncPitch( self, step, stream ):
        if self.note.cs.pitch < Config.MAXIMUM_PITCH_DRUM:
            stream += [ self.note.id, min( self.note.cs.pitch+2*step, Config.MAXIMUM_PITCH_DRUM ) ]

    def noteDecDuration( self, step, stream ):
        return

    def noteIncDuration( self, step, rightBound, stream ):
        return

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

