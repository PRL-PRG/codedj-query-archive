#----------------------------------------------------------------------
# A base class for things that can be played at a given instant
# An onset < 0 implies the Event should be played immediately
#----------------------------------------------------------------------
class Event:
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, onset ):
	   self.onset = onset

    #-----------------------------------
    # playback (must be implemented by subclasses)
    #-----------------------------------
    def play( self ):
	   raise NotImplementedError

    #-----------------------------------
    # adjustment
    #-----------------------------------
    def adjustOnset( self, amount ):
	   self.onset += amount
