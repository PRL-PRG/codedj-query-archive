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
class NoteView( gtk.EventBox ):
    #-----------------------------------
    # initialization
    # TODO: not sure if passing in beatsPerPageAdjustment is the best way to go about it
    #-----------------------------------
    def __init__( self, note, parentContainer, beatsPerPageAdjustment ):
        gtk.EventBox.__init__( self )
        
        self.note = note
        self.parentContainer = parentContainer
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        
        self.drawingArea = gtk.DrawingArea()
        self.drawingArea.connect( "expose-event", self.handleExposeEvent )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion_notify_event", self.handleMotion )
        self.add( self.drawingArea )
        self.show_all()
        
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

        self.parent.move( self, self.getXPosition(), self.getYPosition() )
        self.queue_draw()

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
            
    def handleButtonRelease( self, eventBox, event ):
        self.sampleNote.duration = 0
        self.sampleNote.play()
        del self.sampleNote
        self.sampleNote = None

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

            self.parent.move( self, self.getXPosition(), self.getYPosition() )

    # TODO: this is a TEMPORARY implementation to get notes displayed
    def handleExposeEvent( self, drawingArea, event ):
        size = drawingArea.get_allocation()
        context = drawingArea.window.cairo_create()
        
        context.set_line_width( GUIConstants.BORDER_SIZE )
        context.move_to( 0, 0 )
        context.rel_line_to( size.width, 0 )
        context.rel_line_to( 0, size.height )
        context.rel_line_to( -size.width, 0 )
        context.close_path()
            
        #blue background
        colour = 1 - ( ( self.note.amplitude * 0.7 ) + 0.3 )
        context.set_source_rgb( colour, colour, colour )
        context.fill_preserve()
            
        #black border
        context.set_source_rgb( 0, 0, 0 )
        context.stroke()

    #-----------------------------------
    # update
    #-----------------------------------

    def updateSize( self ):
        width = self.getWidth()
        height = self.getHeight()
        self.set_size_request( width, height )
        self.drawingArea.set_size_request( width, height )
        self.drawingArea.queue_draw()

    #-----------------------------------
    # get size functions
    #-----------------------------------    
    def getWidth( self ):
        return int( self.getParentWidth() / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT * self.note.duration )

    def getHeight( self ):
        return int( max( GUIConstants.MINIMUM_NOTE_HEIGHT, self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES ) )

    def getXPosition( self ):
        return int( self.note.onset * self.getParentWidth() / round( self.beatsPerPageAdjustment.value, 0 ) / Constants.TICKS_PER_BEAT )

    def getYPosition( self ):
        return int( ( Constants.MAXIMUM_PITCH - self.note.pitch ) * self.getParentHeight() / Constants.NUMBER_OF_POSSIBLE_PITCHES )

    def getParentWidth( self ):
        return self.parentContainer.get_allocation().width

    def getParentHeight( self ):
        return self.parentContainer.get_allocation().height
