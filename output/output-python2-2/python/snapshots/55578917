from Framework.Constants import Constantsfrom Framework.Core.Event import Eventfrom Framework.CSound.CSoundClient import CSoundClient
from Framework.Generation.GenerationConstants import GenerationConstants#----------------------------------------------------------------------# TODO: extend this hierarchy to include a Note base class# 		i.e. Event -> Note -> CSoundNote#		most classes should only deal with Events and Notes, #		and not CSoundNotes#----------------------------------------------------------------------#----------------------------------------------------------------------# An Event subclass that represents a CSound note event#----------------------------------------------------------------------
class CSoundNote( Event ):	#-----------------------------------	# initialization	#-----------------------------------
    def __init__( self, onset, pitch, amplitude, pan, duration, trackID, tied = False, instrument = Constants.FLUTE ):        Event.__init__( self, onset )                self.pitch = pitch        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackID = trackID
        self.instrument = instrument
        self.tied = tied	#-----------------------------------	# playback	#-----------------------------------
    def play( self ):    	CSoundClient.sendText( self.getText() )    # TODO: this needs to be cleaned up... it seems CSoundClient needs to fill in some of this text    # e.g. clientID (3333), duration too probably (since this depends on tempo (120))
    def getText( self ):
        # duration for CSound is in seconds
        newPitch = self.getTranspositionFactor( self.pitch )
        newDuration = self.duration*(1000/120)*0.001
        # condition only on instruments that allow tied notes
        if self.instrument == 1:
            if self.tied:
                newDuration = -1

#        if self.trackID == 0:
 #           self.instrument = Constants.SNARE
  #      elif self.trackID == 1:
#            self.instrument = Constants.WOOD
#        elif self.trackID == 2:
#            self.instrument = Constants.HHC
#        elif self.trackID == 3:
#            self.instrument = Constants.BD

        return "perf.InputMessage('i 101.%d 0 %f %f 1. 0.05 %d %f %f')\n" % ( self.trackID, newDuration, newPitch, Constants.INSTRUMENT_TABLE_OFFSET +      
                                                                            self.instrument, self.amplitude, self.pan)
    def getTranspositionFactor( self, pitch ):
        return pow( GenerationConstants.TWO_ROOT_TWELVE, pitch - 36 )	#-----------------------------------	# adjustment functions	#-----------------------------------
    def adjustDuration( self, amount ):        self.duration += amount    def adjustAmplitude( self, amount ):        self.amplitude += amount    def transpose( self, amount ):        self.pitch += amount
