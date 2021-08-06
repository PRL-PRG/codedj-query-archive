import random
import math

import Utils
import Drunk

from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParameters:
    def __init__( self, 
                  beat = GenerationConstants.DEFAULT_BEAT,
                  density = GenerationConstants.DEFAULT_DENSITY,
                  repete = GenerationConstants.DEFAULT_REPETE,
                  step = GenerationConstants.DEFAULT_STEP,
                  articule = GenerationConstants.DEFAULT_ARTICULE,
                  panner = GenerationConstants.DEFAULT_PANNER ):
        self.beat = beat
        self.density = density
        self.repete = repete
        self.step = step
        self.articule = articule
        self.panner = panner

class Generator:
    # TODO: 
    # - modify this method so that it creates Notes as it goes
    #        now we are creating lists of lists of parameters, and converting 
    #        to lists of notes when generation is complete
    # - remove code duplication
    # - replace magic numbers with constants
    
    def generate( self, parameters ):
        table_repetition = Utils.scale(parameters.repete, 0, 25, 25)
        table_onset = Utils.scale(parameters.density, 0, 42, 42)
        table_duration = Utils.scale(parameters.articule, .2, 1., 30)
        self.table_pan = Utils.scale(math.fabs(float( parameters.panner )), .5, 1, 100)
        self.track1Notes = []
        notesList = []
        durationList = []
        onsetDelta = 0
        lastOnsetTime = 0
        self.count = 0
        self.choosePitchTable = random.choice( [ GenerationConstants.MAJOR_SCALE,
                                                 GenerationConstants.HARMONIC_MINOR_SCALE,
                                                 GenerationConstants.NATURAL_MINOR_SCALE,
                                                 GenerationConstants.PENTATONIC_SCALE,
                                                 GenerationConstants.BLUES_SCALE,
                                                 GenerationConstants.PHRYGIEN_SCALE ] )
        chooseNewPitch = Drunk.Loopseg(len(self.choosePitchTable)-1)

        pit = self.choosePitchTable[chooseNewPitch.getNextValue(parameters.step, (len(self.choosePitchTable)-1))]

        gain = random.uniform(.8, 1.)

        pan = self.choosePan( parameters.panner )

        notesList = [[0, pit, gain, pan]]

        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(table_onset))]

        for i in range(int(parameters.beat) * 32):
            if self.count == 0:   
                repetitionFlag = Utils.prob2(table_repetition)
                if repetitionFlag != 0:
                    onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(table_onset))]

            self.makeCellule(onsetDelta, 15, 3)
            self.makeCellule(onsetDelta, 20, 2)
            self.makeCellule(onsetDelta, 24, 4)
            self.makeCellule(onsetDelta, 30, 1)
            self.makeCellule(onsetDelta, 40, 2)
            self.makeCellule(onsetDelta, 80, 2)

            onsetTime = onsetDelta + lastOnsetTime

            if onsetTime < (480 * parameters.beat):
                pit = self.choosePitchTable[chooseNewPitch.getNextValue(parameters.step, (len(self.choosePitchTable)-1))]

                lastOnsetTime = onsetTime

                accentOnset = (lastOnsetTime % 480)

                if accentOnset == 0:
                    gain = random.uniform(.8, 1.)
                elif accentOnset == 120 or accentOnset == 240 or accentOnset == 360:
                    gain = random.uniform(.7, .9)
                else:     
                    gain = random.uniform(.5, .7)

                pan = self.choosePan( parameters.panner )

                notesList.append( [onsetTime, pit, gain, pan] )

                durationList.append(onsetDelta * Utils.prob2(table_duration))
            else: 
                break

        durationList.append(((480 * parameters.beat) - lastOnsetTime) * Utils.prob2(table_duration))

        for i in range(len(notesList)):
            notesList[i].append(durationList[i]) 

        for notes in notesList:
            self.track1Notes.append(CSoundNote(notes[0], notes[1], notes[2], notes[3], notes[4]))

        return self.track1Notes
#        return notesList

    def makeCellule( self, currentDuration, targetDuration, threshold ):
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0

    def choosePan( self, pannerValue ):
        if -0.02 < pannerValue < 0.02:
            pan = Utils.prob2(self.table_pan)
        else:
            pan = pow(Utils.prob2(self.table_pan), (2. - (math.fabs(pannerValue) + 0.01)))

        if math.floor(pannerValue) == 0:
            return pan
        else:
            return (1. - pan)

