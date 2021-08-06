import Utils
import random
from math import sqrt
import Config
from Generation.GenerationConstants import GenerationConstants

class GenerationRythm:

    def celluleRythmSequence(self, parameters, barLength, trackInstrument ):
        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0
        onsetLen = len(GenerationConstants.TABLE_ONSET_VALUES)

        onsetValue  = int( ( 1 -  parameters.density ) * onsetLen )
        onsetDeviation = int( ( 1 - parameters.rythmRegularity ) * 20 )
        currentOnsetValue = onsetValue + ( random.randint( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
        if currentOnsetValue < 0:
            currentOnsetValue = 0
        elif currentOnsetValue > onsetLen:
            currentOnsetValue = onsetLen
        else:
            currentOnsetValue = currentOnsetValue

        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]
        for i in range( int( barLength / Config.TICKS_PER_BEAT * 8 ) ):
            if self.count == 0:   
                currentOnsetValue = onsetValue + ( random.randint( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
                if currentOnsetValue < 0:
                    currentOnsetValue = 0
                elif currentOnsetValue > onsetLen:
                    currentOnsetValue = onsetLen
                else:
                    currentOnsetValue = currentOnsetValue
                onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]
           
            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_TICK_DUR, GenerationConstants.DOUBLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HALF_TRIPLET_TICK_DUR, GenerationConstants.HALF_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HOLE_TRIPLET_TICK_DUR, GenerationConstants.HOLE_TRIPLET_HOW_MANY)

            onsetTime = onsetDelta + lastOnsetTime 
            lastOnsetTime = onsetTime
            
            if onsetTime < barLength:
                rythmSequence.append(onsetTime)
            else:
                break                
        return rythmSequence

    def xnoiseRythmSequence(self, parameters, barLength ):
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

                onsetTime = int(onsetTime * (int(( barLength - 1) / GenerationConstants.DOUBLE_TICK_DUR))) * GenerationConstants.DOUBLE_TICK_DUR

            if onsetTime < 0:
                onsetTime = 0
            elif onsetTime > ( barLength - GenerationConstants.DOUBLE_TICK_DUR):
                onsetTime = ( barLength - GenerationConstants.DOUBLE_TICK_DUR)
            else:
                onsetTime = onsetTime

            rythmSequence.append(onsetTime)

        rythmSequence.sort()
        return rythmSequence  

    def drumRythmSequence(self, parameters, trackInstrument, barLength ):
        density = sqrt(parameters.density)
        rythmSequence = []
        binSelection = []
        downBeats = []
        upBeats = []
        beats = []
        countDown = 0
        onsetTime = None
        beatsPerPage = int( barLength / Config.TICKS_PER_BEAT )    

        if Config.INSTRUMENTS[ trackInstrument ].instrumentRegister == Config.PUNCH:
            registerDensity = 0.5
            downBeatRecurence = 4
            downBeats = [x for x in GenerationConstants.DRUM_PUNCH_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeats.append( downBeat + Config.TICKS_PER_BEAT / 2 )

        if Config.INSTRUMENTS[ trackInstrument ].instrumentRegister == Config.LOW:
            registerDensity = 1.5
            downBeatRecurence = 4
            downBeats = [x for x in GenerationConstants.DRUM_LOW_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeats.append( downBeat + Config.TICKS_PER_BEAT / 2 )

        if Config.INSTRUMENTS[ trackInstrument ].instrumentRegister == Config.MID:
            registerDensity = 1
            downBeatRecurence = 1
            downBeats = [x for x in GenerationConstants.DRUM_MID_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeats.append( downBeat + Config.TICKS_PER_BEAT / 4 )

        if Config.INSTRUMENTS[ trackInstrument ].instrumentRegister == Config.HIGH:
            registerDensity = 1.5
            downBeatRecurence = 1
            downBeats = [x for x in GenerationConstants.DRUM_HIGH_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeats.append( downBeat + Config.TICKS_PER_BEAT / 4 )

        for i in range( int( density * registerDensity * len( downBeats ) ) ):
            if random.random() < ( parameters.rythmRegularity * downBeatRecurence ) and binSelection.count( 1 ) < len( downBeats ): 
                binSelection.append( 1 )        
            else:
                if binSelection.count( 0 ) < len( downBeats ): 
                    binSelection.append( 0 )
                else:
                    binSelection.append( 1 )

        countDown = binSelection.count( 1 )

        length = len(downBeats) - 1
        for i in range( countDown ):
            ran1 = random.randint(0, length)
            ran2 = random.randint(0, length)
            randMin = min(ran1, ran2)
            onsetTime = downBeats.pop(randMin)
            rythmSequence.append( onsetTime )
            length -= 1

        length = len(upBeats) - 1
        for i in range( len( binSelection ) - countDown ):
            ran1 = random.randint(0, length)
            ran2 = random.randint(0, length)
            randMin = min(ran1, ran2)
            onsetTime = upBeats.pop(randMin)
            rythmSequence.append( onsetTime )
            length -= 1

        rythmSequence.sort()
        return rythmSequence

    def makeCellule( self, currentDuration, targetDuration, threshold ):
        threshold = threshold - 1
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0  
