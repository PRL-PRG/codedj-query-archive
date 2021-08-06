import Utils
import random

import Config
from Generation.GenerationConstants import GenerationConstants

class GenerationRythm:
    def __init__( self, trackInstrument, barLength ):
        self.trackInstrument = trackInstrument
        self.barLength = barLength

    def celluleRythmSequence(self, parameters ):
        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0

        onsetValue  = int( ( 1 -  parameters.density ) * 8 )
        onsetDeviation = int( ( 1 - parameters.rythmRegularity ) * 4 )
        currentOnsetValue = onsetValue + ( random.randint( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
        if currentOnsetValue < 0:
            currentOnsetValue == 0
        elif currentOnsetValue > 8:
            currentOnsetValue == 8
        else:
            currentOnsetValue = currentOnsetValue

        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]

        for i in range( int( self.barLength / Config.TICKS_PER_BEAT * 8 ) ):
            if self.count == 0:   
                currentOnsetValue = onsetValue + ( random.randint( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
                if currentOnsetValue < 0:
                    currentOnsetValue == 0
                elif currentOnsetValue > 8:
                    currentOnsetValue == 8
                else:
                    currentOnsetValue = currentOnsetValue
                onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]

            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_TICK_DUR, GenerationConstants.DOUBLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HALF_TRIPLET_TICK_DUR, GenerationConstants.HALF_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HOLE_TRIPLET_TICK_DUR, GenerationConstants.HOLE_TRIPLET_HOW_MANY)

            onsetTime = onsetDelta + lastOnsetTime 
            lastOnsetTime = onsetTime
            
            if onsetTime < self.barLength:
                rythmSequence.append(onsetTime)
            else:
                break                
        return rythmSequence

    def xnoiseRythmSequence(self, parameters ):
        rythmSequence = []
        onsetTime = None
        randomParamScaler = parameters.rythmRegularity * 2 + 0.5
# need radioButton with 0 for random choose and each generator independant
        whichRandomGenerator = random.randint(0, 4)
        maximumNumberOfNotes = int( (parameters.density) * GenerationConstants.MAX_NOTES_PER_BAR)
 
        for i in range(maximumNumberOfNotes):
            while onsetTime in rythmSequence:
                if whichRandomGenerator == 0:
                    onsetTime = random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM * randomParamScaler)
                elif whichRandomGenerator == 1:
                    onsetTime = 1 - random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM * randomParamScaler)
                elif whichRandomGenerator == 2:
                    onsetTime = random.gauss(GenerationConstants.RANDOM_GAUSS_PARAM1, 
                                                            GenerationConstants.RANDOM_GAUSS_PARAM2 * (3 - randomParamScaler))
                elif whichRandomGenerator == 3:
                    onsetTime = random.betavariate(GenerationConstants.RANDOM_BETA_PARAM * randomParamScaler,           
                                                                    GenerationConstants.RANDOM_BETA_PARAM * randomParamScaler)
                elif whichRandomGenerator == 4:
                    onsetTime = random.weibullvariate(GenerationConstants.RANDOM_WEIBULL_PARAM1,                                                                        
                                                                          GenerationConstants.RANDOM_WEIBULL_PARAM2 * randomParamScaler)

                onsetTime = int(onsetTime * (int(( self.barLength - 1) / GenerationConstants.DOUBLE_TICK_DUR))) * GenerationConstants.DOUBLE_TICK_DUR

            if onsetTime < 0:
                onsetTime = 0
            elif onsetTime > ( self.barLength - GenerationConstants.DOUBLE_TICK_DUR):
                onsetTime = ( self.barLength - GenerationConstants.DOUBLE_TICK_DUR)
            else:
                onsetTime = onsetTime

            rythmSequence.append(onsetTime)

        rythmSequence.sort()
        return rythmSequence  

    def drumRythmSequence(self, parameters ):
        rythmSequence = []
        binSelection = []
        downBeats = []
        upBeats = []
        beats = []
        countDown = 0
        onsetTime = None
        beatsPerPage = int( self.barLength / Config.TICKS_PER_BEAT )    

        if Config.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == Config.PUNCH:
            registerDensity = 0.5
            downBeatRecurence = 4
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
            for i in range( len( beats ) ):
                if (Config.DEBUG > 3) : print 'INFO: GenerationRythm::drumRythmSequence', ( beats[ GenerationConstants.PUNCH_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.)
                downBeats.append( ( beats[ GenerationConstants.PUNCH_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == Config.LOW:
            registerDensity =1.5
            downBeatRecurence = 4
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.LOW_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 2 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == Config.MID:
            registerDensity = 1
            downBeatRecurence = 1
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
                beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.MID_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        if Config.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == Config.HIGH:
            registerDensity = 1.5
            downBeatRecurence = 1
            for beat in range( beatsPerPage ):
                beats.append( beat * Config.TICKS_PER_BEAT )
                beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
            for i in range( len( beats ) ):
                downBeats.append( ( beats[ GenerationConstants.HIGH_ACCENTS[ beatsPerPage ][ i ] ], pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) )
            for downBeat in downBeats:
                upBeats.append( ( downBeat[ 0 ] +  Config.TICKS_PER_BEAT / 4 , downBeat[ 1 ] ) )

        for i in range( int( parameters.density * registerDensity * len( downBeats ) ) ):
            if random.randint( 0, 100 ) < ( parameters.rythmRegularity * 100 * downBeatRecurence ) and binSelection.count( 1 ) < len( downBeats ): 
                binSelection.append( 1 )        
            else:
                if binSelection.count( 0 ) < len( downBeats ): 
                    binSelection.append( 0 )
                else:
                    binSelection.append( 1 )

        countDown = binSelection.count( 1 )

        for i in range( countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = Utils.prob2( downBeats )
            rythmSequence.append( onsetTime )

        for i in range( len( binSelection ) - countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = Utils.prob2( upBeats )
            rythmSequence.append( onsetTime )

        rythmSequence.sort()
        return rythmSequence

    def makeCellule( self, currentDuration, targetDuration, threshold ):
        threshold = threshold - 1
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0  
