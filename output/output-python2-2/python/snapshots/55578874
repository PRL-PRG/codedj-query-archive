from Framework.Core.Event import Event
from Framework.Constants import Constants from Framework.CSound.CSoundClient import CSoundClientfrom Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants#----------------------------------------------------------------------# TODO: extend this hierarchy to include a Note base class# 		i.e. Event -> Note -> CSoundNote#		most classes should only deal with Events and Notes, #		and not CSoundNotes#----------------------------------------------------------------------#----------------------------------------------------------------------# An Event subclass that represents a CSound note event#----------------------------------------------------------------------
class CSoundNote( Event ):	#-----------------------------------	# initialization	#-----------------------------------
    def __init__( self, onset, pitch, amplitude, pan, duration, trackID, volumeFunction, getTempoCallback, tied = False, instrument = CSoundConstants.CELLO ):        Event.__init__( self, onset )                self.pitch = pitch        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackID = trackID
        self.volumeFunction = volumeFunction
        self.getTempoCallback = getTempoCallback
        self.instrument = instrument
        self.tied = tied	#-----------------------------------	# playback	#-----------------------------------
    def play( self ):    	CSoundClient.sendText( self.getText() )    # TODO: this needs to be cleaned up... it seems CSoundClient needs to fill in some of this text    # e.g. clientID (3333), duration too probably (since this depends on tempo (120))
    def getText( self ):
        # duration for CSound is in seconds
        newPitch = self.getTranspositionFactor( self.pitch )
        oneTickDuration = (Constants.MS_PER_MINUTE / 1000)  / self.getTempoCallback() / Constants.TICKS_PER_BEAT
        newDuration = oneTickDuration * self.duration
        # condition only on instruments that allow tied notes
        if CSoundConstants.INSTRUMENTS[ self.instrument ][ 1 ]  == 101  and self.tied:
            newDuration = -1

        newAmplitude = self.amplitude * self.volumeFunction()

        return CSoundConstants.PLAY_NOTE_COMMAND % ( CSoundConstants.INSTRUMENTS[ self.instrument ][ 1 ], 
                                                     self.trackID, 													 newDuration, 													 newPitch, 
													 newAmplitude, 													 self.pan,													 CSoundConstants.INSTRUMENT_TABLE_OFFSET + 													 	CSoundConstants.INSTRUMENTS[ self.instrument ][ 0 ] )
    def getTranspositionFactor( self, pitch ):
        return pow( GenerationConstants.TWO_ROOT_TWELVE, pitch - 36 )	#-----------------------------------	# adjustment functions	#-----------------------------------
    def adjustDuration( self, amount ):        self.duration += amount    def adjustAmplitude( self, amount ):        self.amplitude += amount    def transpose( self, amount ):        self.pitch += amount
