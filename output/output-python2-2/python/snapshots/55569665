import Utils
import random
from math import sqrt
import common.Util.InstrumentDB as InstrumentDB
import common.Config as Config
from common.Generation.GenerationConstants import GenerationConstants

class GenerationRythm:
    def __init__(self):
        self.instrumentDB = InstrumentDB.getRef()

    def celluleRythmSequence(self, parameters, barLength, trackId, trackInstrument=None ):
        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0
        onsetLen = len(GenerationConstants.TABLE_ONSET_VALUES)

        onsetValue  = int( ( 1 -  (parameters.density[trackId]*0.98+0.02) ) * onsetLen )
        onsetDeviation = int( ( 1 - parameters.rythmRegularity[trackId] ) * 20 )
        currentOnsetValue = onsetValue + ( random.randint( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
        if currentOnsetValue < 0:
            currentOnsetValue = 0
        elif currentOnsetValue >= onsetLen:
            currentOnsetValue = onsetLen - 1
        else:
            currentOnsetValue = currentOnsetValue

        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]
        listLen = range( int( barLength / Config.TICKS_PER_BEAT * 8 ) )
        randInt = random.randint
        for i in listLen:
            if self.count == 0:
                currentOnsetValue = onsetValue + ( randInt( 0, onsetDeviation ) - ( onsetDeviation / 2 ) )
                if currentOnsetValue < 0:
                    currentOnsetValue = 0
                elif currentOnsetValue >= onsetLen:
                    currentOnsetValue = onsetLen - 1
                else:
                    currentOnsetValue = currentOnsetValue
                onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[ currentOnsetValue ]

            if onsetDelta == GenerationConstants.DOUBLE_TICK_DUR:
                if self.count < (GenerationConstants.DOUBLE_HOW_MANY - 1):
                    self.count += 1
                else:
                    self.count = 0
                onsetTime = onsetDelta + lastOnsetTime
                lastOnsetTime = onsetTime
                if onsetTime < barLength-2:
                    rythmSequence.append(onsetTime)
                    continue
                else:
                    break
            elif onsetDelta == GenerationConstants.HALF_TRIPLET_TICK_DUR:
                if self.count < (GenerationConstants.HALF_TRIPLET_HOW_MANY - 1):
                    self.count += 1
                else:
                    self.count = 0
                onsetTime = onsetDelta + lastOnsetTime
                lastOnsetTime = onsetTime
                if onsetTime < barLength-2:
                    rythmSequence.append(onsetTime)
                    continue
                else:
                    break
            elif onsetDelta == GenerationConstants.HOLE_TRIPLET_TICK_DUR:
                if self.count < (GenerationConstants.HOLE_TRIPLET_HOW_MANY - 1):
                    self.count += 1
                else:
                    self.count = 0
                onsetTime = onsetDelta + lastOnsetTime
                lastOnsetTime = onsetTime
                if onsetTime < barLength-2:
                    rythmSequence.append(onsetTime)
                    continue
                else:
                    break

            onsetTime = onsetDelta + lastOnsetTime
            lastOnsetTime = onsetTime
            if onsetTime < barLength-2:
                rythmSequence.append(onsetTime)
            else:
                break
        return rythmSequence

    def xnoiseRythmSequence(self, parameters, barLength ):
        rythmSequence = []
        onsetTime = None
        randomParamScaler = parameters.rythmRegularity[trackId] * 2 + 0.5
        whichRandomGenerator = random.randint(0, 4)
        maximumNumberOfNotes = int( (parameters.density[trackId]) * GenerationConstants.MAX_NOTES_PER_BAR)

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
        density = sqrt(parameters.density[0])
        rythmSequence = []
        binSelection = []
        downBeats = []
        upBeats = []
        beats = []
        countDown = 0
        onsetTime = None
        beatsPerPage = int( barLength / Config.TICKS_PER_BEAT )
        randInt = random.randint

        upBeatsAppend = upBeats.append

        if self.instrumentDB.instNamed[ trackInstrument ].instrumentRegister == Config.PUNCH:
            registerDensity = 0.5
            downBeatRecurence = 4
            upBeatOffset = Config.TICKS_PER_BEAT / 2
            downBeats = [x for x in GenerationConstants.DRUM_PUNCH_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeatsAppend( downBeat + upBeatOffset )

        elif self.instrumentDB.instNamed[ trackInstrument ].instrumentRegister == Config.LOW:
            registerDensity = 1.5
            downBeatRecurence = 4
            upBeatOffset = Config.TICKS_PER_BEAT / 2
            downBeats = [x for x in GenerationConstants.DRUM_LOW_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeatsAppend( downBeat + upBeatOffset )

        elif self.instrumentDB.instNamed[ trackInstrument ].instrumentRegister == Config.MID:
            registerDensity = 1
            downBeatRecurence = 1
            upBeatOffset = Config.TICKS_PER_BEAT / 4
            downBeats = [x for x in GenerationConstants.DRUM_MID_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeatsAppend( downBeat + upBeatOffset )

        elif self.instrumentDB.instNamed[ trackInstrument ].instrumentRegister == Config.HIGH:
            registerDensity = 1.5
            downBeatRecurence = 1
            upBeatOffset = Config.TICKS_PER_BEAT / 4
            downBeats = [x for x in GenerationConstants.DRUM_HIGH_ACCENTS[ beatsPerPage ]]
            for downBeat in downBeats:
                upBeatsAppend( downBeat + upBeatOffset )

        list = range( int( density * registerDensity * len( downBeats ) ) )
        rand = random.random
        binCount = binSelection.count
        binAppend = binSelection.append
        for i in list:
            if rand() < ( parameters.rythmRegularity[0] * downBeatRecurence ) and binCount( 1 ) < len( downBeats ):
                binAppend( 1 )
            else:
                if binCount( 0 ) < len( downBeats ):
                    binAppend( 0 )
                else:
                    binAppend( 1 )

        countDown = binCount( 1 )

        seqAppend = rythmSequence.append
        length = len(downBeats) - 1
        downPop = downBeats.pop
        for i in range( countDown ):
            ran1 = randInt(0, length)
            ran2 = randInt(0, length)
            randMin = min(ran1, ran2)
            onsetTime = downPop(randMin)
            seqAppend( onsetTime )
            length -= 1

        length = len(upBeats) - 1
        upPop = upBeats.pop
        for i in range( len( binSelection ) - countDown ):
            ran1 = randInt(0, length)
            ran2 = randInt(0, length)
            randMin = min(ran1, ran2)
            onsetTime = upPop(randMin)
            seqAppend( onsetTime )
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
