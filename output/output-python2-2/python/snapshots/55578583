import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from GUI.GUIConstants import GUIConstants
from GUI.Core.NoteParametersWindow import NoteParametersWindow

from BackgroundView import SELECTNOTES

#----------------------------------------------------------------------
# TODO: currently we are only using CSoundNotes, 
#     - this should updated to handle generic Note objects
#----------------------------------------------------------------------

#----------------------------------------------------------------------
# A view for the (CSound)Note class
#----------------------------------------------------------------------
class NoteView:
    #-----------------------------------
    # initialization
    # TODO: not sure if passing in beatsPerPageAdjustment is the best way to go about it
    #-----------------------------------
    def __init__( self, note, track, beatsPerPageAdjustment ):
        self.note = note
        self.track = track
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.posOffset = (0,0)

        self.baseX = self.baseY = self.lastDragX = self.lastDragY = 0 # note dragging properties

        self.sampleNote = None

        self.parentSize = False

        self.selected = False
        self.potentialDeselect = False


    def getNoteParameters( self ):
        self.note.pitch = self.noteParameters.pitchAdjust.value
        self.note.amplitude = self.noteParameters.amplitudeAdjust.value
        self.note.pan = self.noteParameters.panAdjust.value
        self.note.attack = self.noteParameters.attackAdjust.value
        self.note.decay = self.noteParameters.decayAdjust.value
        self.note.reverbSend = self.noteParameters.reverbSendAdjust.value
        self.note.filterType = self.noteParameters.filterType
        self.note.filterCutoff = self.noteParameters.filterCutoff
        self.note.tied = self.noteParameters.tied
        self.note.overlap = self.noteParameters.overlap

    def handleButtonPress( self, emitter, event ):
        eX = event.x - self.x
        eY = event.y - self.y
      
        if         eX < 0 or eX > self.width \
                or eY < 0 or eY > self.height:
            return False
    
        if event.button == 3:
            self.noteParameters = NoteParametersWindow( self.note, self.getNoteParameters ) 
            return True

        if event.type == gtk.gdk._2BUTTON_PRESS:     # select bar
            self.potentialDeselect = False
            emitter.selectNotesByBar( self.track.getID(), self.x, self.x+self.width )
        elif event.type == gtk.gdk._3BUTTON_PRESS:   # select track
            self.potentialDeselect = False
            emitter.selectNotesByTrack( self.track.getID() )
        else:
            if self.getSelected():                       # we already selected, might want to delected
                self.potentialDeselect = True
            else:
                emitter.selectNotes( { self.track.getID(): [ self ] } )
            self.updateSampleNote( self.note.pitch )
            emitter.setCurrentAction( "note-drag", self )

               
        emitter.dirty()
                
        return True

    def handleButtonRelease( self, emitter, event, buttonPressCount ):

        if self.potentialDeselect:
            self.potentialDeselect = False
            emitter.deselectNotes( { self.track.getID(): [ self ] } )

        self.clearSampleNote()
        
        emitter.doneCurrentAction()

        return True

    def noteDrag( self, emitter, dx, dy ):
        self.potentialDeselect = False

        if dx != self.lastDragX:
            self.lastDragX = dx
            eX = float(self.baseX - self.posOffset[0] + dx)
            self.note.onset = round( eX/self.parentSize[0] * (round( self.beatsPerPageAdjustment.value, 0 ) * Constants.TICKS_PER_BEAT), 0 )

        if dy != self.lastDragY:
            self.lastDragY = dy
            eY = float(self.baseY - self.posOffset[1] + dy)
            newPitch = round ( Constants.MAXIMUM_PITCH - (Constants.MAXIMUM_PITCH-Constants.MINIMUM_PITCH)*eY/(self.parentSize[1]-self.height) )
            self.note.pitch = newPitch
            self.updateSampleNote( newPitch )

        self.updateTransform( False )

    def doneNoteDrag( self, emitter ):
        self.baseX = self.x
        self.baseY = self.y    
    
        self.lastDragX = 0
        self.lastDragY = 0

        self.clearSampleNote()
        

    def handleMarqueeSelect( self, emitter, start, stop ):
        intersectionY = [ max(start[1],self.y), min(stop[1],self.y+self.height) ]
        if intersectionY[0] > intersectionY[1]:
            return False

        intersectionX = [ max(start[0],self.x), min(stop[0],self.x+self.width) ]
        if intersectionX[0] > intersectionX[1]:
           return False

        return True

    #-----------------------------------
    # draw
    #-----------------------------------

    def setPositionOffset( self, offset ):
        self.posOffset = offset
        if self.parentSize: self.updateTransform( False )

    def draw( self, context ):
        lineW = GUIConstants.NOTE_BORDER_SIZE
        lineWDIV2 = lineW/2.0
        context.set_line_width( lineW )

        context.move_to( self.x + lineWDIV2, self.y + lineWDIV2 )
        context.rel_line_to( self.width - lineW, 0 )
        context.rel_line_to( 0, self.height - lineW )
        context.rel_line_to( -self.width + lineW, 0 )
        context.close_path()
            
        #background
        colour = 1 - ( ( self.note.amplitude * 0.7 ) + 0.3 )
        context.set_source_rgb( colour, colour, colour )
        context.fill_preserve()
            
        #border
        if self.selected: context.set_source_rgb( 1, 1, 1 )
        else:             context.set_source_rgb( 0, 0, 0 )
        context.stroke()

    #-----------------------------------
    # update
    #-----------------------------------

    def updateTransform( self, parentSize ):
        if parentSize: self.parentSize = parentSize
        self.width = int( self.parentSize[0] * self.note.duration / (round( self.beatsPerPageAdjustment.value, 0 ) * Constants.TICKS_PER_BEAT) )
        self.height = int( max( GUIConstants.MINIMUM_NOTE_HEIGHT, self.parentSize[1] / (Constants.NUMBER_OF_POSSIBLE_PITCHES-1) ) )
        self.x = int( self.parentSize[0] * self.note.onset / (round( self.beatsPerPageAdjustment.value, 0 ) * Constants.TICKS_PER_BEAT) ) \
                 + self.posOffset[0]
        self.y = int(  (self.parentSize[1]-self.height) * ( Constants.MAXIMUM_PITCH - self.note.pitch ) / (Constants.NUMBER_OF_POSSIBLE_PITCHES-1) ) \
                 + self.posOffset[1]

    def checkX( self, startx, stopx ):
        if self.x > startx and self.x < stopx: return True
        else: return False

    def updateDragLimits( self, dragLimits ):
        left = -self.x + self.posOffset[0]
        right = self.parentSize[0] - self.width + left
        up = -self.y + self.posOffset[1]
        down = (self.parentSize[1]-self.height) + up
        #up =  -(self.parentSize[1]-self.height) * ( Constants.MAXIMUM_PITCH - self.note.pitch ) / (Constants.NUMBER_OF_POSSIBLE_PITCHES-1)
        #down = (self.parentSize[1]-self.height) + up
        #left = -self.parentSize[0] * self.note.onset / (round( self.beatsPerPageAdjustment.value, 0 ) * Constants.TICKS_PER_BEAT)
        #right = self.parentSize[0] -  + left
        if dragLimits[0][0] < left: dragLimits[0][0] = left
        if dragLimits[0][1] > right: dragLimits[0][1] = right
        if dragLimits[1][0] < up: dragLimits[1][0] = up
        if dragLimits[1][1] > down: dragLimits[1][1] = down

        # store the current loc as a reference point
        self.baseX = self.x
        self.baseY = self.y

    def updateSampleNote( self, pitch ):
        if self.sampleNote == None:
            self.sampleNote = self.note.clone()
            #TODO clean this up:
            if CSoundConstants.INSTRUMENTS[ self.sampleNote.instrumentFlag ].csoundInstrumentID == 103:
                self.sampleNote.duration = 100
            else:
                self.sampleNote.duration = -1
            self.sampleNote.play()

        elif self.sampleNote.pitch != pitch:
            self.sampleNote.pitch = pitch
            self.sampleNote.play()

    def clearSampleNote( self ):
        if self.sampleNote != None:
            self.sampleNote.duration = 0
            self.sampleNote.play()
            del self.sampleNote
            self.sampleNote = None
    
    #-----------------------------------
    # Selection
    #-----------------------------------
    
    def setSelected( self, state ):
        if self.selected != state:
            self.selected = state
            return True # state changed
        return False    # state is the same

    def getSelected( self ):
        return self.selected
