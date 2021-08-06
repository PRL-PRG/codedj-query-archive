import random
import Config

from Generation.GenerationConstants import GenerationConstants
from Generation.Utils import *

class GenRythm:
    def __init__( self, instrument, barLength, nbeats ):
        self.instrument = instrument
        self.barLength = barLength
        self.nbeats = nbeats

#############################################################################
    def drumRythmSequence(self, regularity ):
        rythmSequence = []
        binSelection = []
        downBeats = []
        upBeats = []
        beats = []
        density = 0.8
        countDown = 0
        onsetTime = None
        beatsPerPage = int( self.barLength / Config.TICKS_PER_BEAT )    

        if Config.INSTRUMENTS[ self.instrument ].instrumentRegister == Config.PUNCH:
            registerDensity = 0.5
            downBeatRecurence = 4
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.PUNCH_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.instrument ].instrumentRegister == Config.LOW:
            registerDensity =1.5
            downBeatRecurence = 4
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.LOW_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 2 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.instrument ].instrumentRegister == Config.MID:
            registerDensity = .75
            downBeatRecurence = 1
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
                beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.MID_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.instrument ].instrumentRegister == Config.HIGH:
            registerDensity = 1.5
            downBeatRecurence = 1
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
                beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.HIGH_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        for i in range( int( density * registerDensity * len( downBeats ) ) ):
            if random.randint( 0, 100 ) < ( regularity * 100 * downBeatRecurence ) and binSelection.count( 1 ) < len( downBeats ): 
                binSelection.append( 1 )        
            else:
                if binSelection.count( 0 ) < len( downBeats ): 
                    binSelection.append( 0 )
                else:
                    binSelection.append( 1 )

        countDown = binSelection.count( 1 )

        for i in range( countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = prob2( downBeats )
            rythmSequence.append( onsetTime )

        for i in range( len( binSelection ) - countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = prob2( upBeats )
            rythmSequence.append( onsetTime )

        rythmSequence.sort()
        return rythmSequence
