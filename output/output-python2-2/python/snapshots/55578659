import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from GUI.GUIConstants import GUIConstants
from GUI.Core.NoteParametersWindow import NoteParametersWindow

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
    def __init__( self, note, parentContainer, beatsPerPageAdjustment ):
        self.note = note
        self.parentContainer = parentContainer
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        
        self.sampleNote = None

    def getNoteParameters( self ):
        self.note.pitch = self.noteParameters.pitchAdjust.value
        self.note.amplitude = self.noteParameters.amplitudeAdjust.value
        self.note.pan = self.noteParameters.panAdjust.value
        self.note.attack = self.noteParameters.attackAdjust.value
        self.note.decay = self.noteParameters.decayAdjust.value
        self.note.reverbSend = self.noteParameters.reverbSendAdjust.value
        self.note.filterType = self.noteParameters.filterType
        self.note.filterCutoff = self.noteParameters.filterCutoff

    # depricated since we're no longer an EventBox, needs to be replaced
    def handleButtonPress( self, eventBox, event ):
        self.sampleNote = self.note.clone()
        #TODO clean this up:
        if CSoundConstants.INSTRUMENTS[ self.sampleNote.instrument ].csoundInstrumentID == 103:
            self.sampleNote.duration = 100
        else:
            self.sampleNote.duration = -1
        
        if event.button == 1:
            self.sampleNote.play()
            self.buttonPressYLocation = event.y
        elif event.button == 3:
            self.noteParameters = NoteParametersWindow( self.note, self.getNoteParameters )
      
    # depricated since we're no longer an EventBox, needs to be replaced          
    def handleButtonRelease( self, eventBox, event ):
        self.sampleNote.duration = 0
        self.sampleNote.play()
        del self.sampleNote
        self.sampleNote = None

    # depricated since we're no longer an EventBox, needs to be replaced
    def handleMotion( self, eventBox, event ):
        transposeAmount = round( ( self.buttonPressYLocation - event.y ) / self.getHeight() )
        newPitch = self.note.pitch + transposeAmount
        
        if transposeAmount != 0:
            if newPitch >= Constants.MINIMUM_PITCH and newPitch <= Constants.MAXIMUM_PITCH:
                self.note.adjustPitch( transposeAmount )
                self.sampleNote.adjustPitch( transposeAmount )
                self.sampleNote.play()
            elif newPitch < Constants.MINIMUM_PITCH and self.note.pitch != Constants.MINIMUM_PITCH:
                self.note.pitch = Constants.MINIMUM_PITCH
            elif newPitch > Constants.MAXIMUM_PITCH and self.note.pitch != Constants.MAXIMUM_PITCH:
                self.note.pitch = Constants.MAXIMUM_PITCH

            self.updateTransform()

    #-----------------------------------
    # draw
    #-----------------------------------

    def draw( self, context, offset ):
        lineW = GUIConstants.BORDER_SIZE
        lineWDIV2 = lineW/2.0
        context.set_line_width( lineW )

        context.move_to( offset[0] + self.x + lineWDIV2, offset[1] + self.y + lineWDIV2 )
        context.rel_line_to( self.width - lineW, 0 )
        context.rel_line_to( 0, self.height - lineW )
        context.rel_line_to( -self.width + lineW, 0 )
        context.close_path()
            
        #background
        colour = 1 - ( ( self.note.amplitude * 0.7 ) + 0.3 )
        context.set_source_rgb( colour, colour, colour )
        context.fill_preserve()
            
        #border
        context.set_source_rgb( 0, 0, 0 )
        context.stroke()

    #-----------------------------------
    # update
    #-----------------------------------

    def updateTransform( self, parentSize ):
        self.width = int( parentSize[0] / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT * self.note.duration )
        self.height = int( max( GUIConstants.MINIMUM_NOTE_HEIGHT, parentSize[1] / Constants.NUMBER_OF_POSSIBLE_PITCHES ) )
        self.x = int( self.note.onset * parentSize[0] / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT )
        self.y = int( ( Constants.MAXIMUM_PITCH - self.note.pitch ) * parentSize[1] / Constants.NUMBER_OF_POSSIBLE_PITCHES )
 
