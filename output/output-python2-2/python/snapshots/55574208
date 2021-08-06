import random
import Config

from Generation.GenerationConstants import GenerationConstants
from Generation.Utils import *

class GenRythm:
    def drumRythmSequence(self, instrument, nbeats, density, regularity ):
        rythmSequence = []
        binSelection = []
        downBeats = []
        upBeats = []
        beats = []
        countDown = 0
        onsetTime = None

        if Config.INSTRUMENTS[ instrument ].instrumentRegister == Config.PUNCH:
            registerDensity = 0.5
            downBeatRecurence = 4
            downBeats = GenerationConstants.DRUM_PUNCH_PROB[ nbeats ]
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ instrument ].instrumentRegister == Config.LOW:
            registerDensity =1
            downBeatRecurence = 4
            downBeats = GenerationConstants.DRUM_LOW_PROB[ nbeats ]
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 2 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ instrument ].instrumentRegister == Config.MID:
            registerDensity = .75
            downBeatRecurence = 1
            downBeats = GenerationConstants.DRUM_MID_PROB[ nbeats ]
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ instrument ].instrumentRegister == Config.HIGH:
            registerDensity = 1.5
            downBeatRecurence = 1
            downBeats = GenerationConstants.DRUM_HIGH_PROB[ nbeats ]
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        realDensity = density * registerDensity
        if realDensity > 1.:
            realDensity = 1.

        list = range( int( realDensity  * len( downBeats ) ) )
        for i in list:
            if random.random() < ( regularity * downBeatRecurence ) and binSelection.count( 1 ) < len( downBeats ): 
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
