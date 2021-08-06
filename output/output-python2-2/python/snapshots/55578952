import random
import math

import Utils
import Drunk

from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParameters:
    def __init__( self, 
                  bar = GenerationConstants.DEFAULT_BEAT,
                  density = GenerationConstants.DEFAULT_DENSITY,
                  repete = GenerationConstants.DEFAULT_REPETE,
                  step = GenerationConstants.DEFAULT_STEP,
                  articule = GenerationConstants.DEFAULT_ARTICULE,
                  panner = GenerationConstants.DEFAULT_PANNER ):
        self.bar = bar
        self.density = density
        self.repete = repete
        self.step = step
        self.articule = articule
        self.panner = panner

class Generator:
    # TODO: 
    # - replace magic numbers with constants
    
    def generate( self, parameters, trackID ):
        self.table_repetition = Utils.scale(parameters.repete, 0, 25, 25)
        self.table_onset = Utils.scale(parameters.density, 0, 42, 42)
        self.table_duration = Utils.scale(parameters.articule, .2, 1., 30)
        self.table_pan = Utils.scale(math.fabs(float( parameters.panner )), .5, 1, 100)
        self.trackNotes = []
        self.choosePitchTable = random.choice( [ GenerationConstants.MAJOR_SCALE,
                                                 GenerationConstants.HARMONIC_MINOR_SCALE,
                                                 GenerationConstants.NATURAL_MINOR_SCALE,
                                                 GenerationConstants.PENTATONIC_SCALE,
                                                 GenerationConstants.BLUES_SCALE,
                                                 GenerationConstants.PHRYGIEN_SCALE ] )
        self.choosePitchTable = GenerationConstants.HARMONIC_MINOR_SCALE
        self.chooseNewPitch = Drunk.Loopseg(len(self.choosePitchTable)-1)

        rythmSequence = self.makeRythmSequence(parameters)
        pitchSequence = self.makePitchSequence(len(rythmSequence), parameters.step)
        gainSequence = self.makeGainSequence(rythmSequence)
        panSequence = self.makePanSequence(len(rythmSequence), parameters.panner)
        durationSequence = self.makeDurationSequence(rythmSequence, parameters)

        for i in range(len(rythmSequence)):
            self.trackNotes.append(CSoundNote(rythmSequence[i], pitchSequence[i], gainSequence[i], panSequence[i], durationSequence[i], trackID))

        return self.trackNotes

    def makeRythmSequence(self, parameters ):

        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0
        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(self.table_onset))]

        for i in range(int(parameters.bar) * 32):
            if self.count == 0:   
                repetitionFlag = Utils.prob2(self.table_repetition)
                if repetitionFlag != 0:
                    onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(self.table_onset))]

            self.makeCellule(onsetDelta, 15, 3)
            self.makeCellule(onsetDelta, 20, 2)
            self.makeCellule(onsetDelta, 24, 4)
            self.makeCellule(onsetDelta, 30, 1)
            self.makeCellule(onsetDelta, 40, 2)
            self.makeCellule(onsetDelta, 80, 2)

            onsetTime = onsetDelta + lastOnsetTime 
            lastOnsetTime = onsetTime
            
            if onsetTime < (480 * parameters.bar):
                rythmSequence.append(onsetTime)
            else:
                break    
            
        return rythmSequence    
    
    def makePitchSequence(self, length, step):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(self.choosePitchTable[self.chooseNewPitch.getNextValue(step, (len(self.choosePitchTable)-1))])
        return pitchSequence
    
    def makeGainSequence(self, onsetList):
        gainSequence = []
        
        for onset in onsetList:
            accentOnset = (onset % 480)

            if accentOnset == 0:
                gain = random.uniform(.8, 1.)
            elif accentOnset == 120 or accentOnset == 240 or accentOnset == 360:
                gain = random.uniform(.7, .9)
            else:     
                gain = random.uniform(.5, .7)
            gainSequence.append(gain)
        return gainSequence            

    def makePanSequence(self, length, panner):
        panSequence = []
        for i in range(length):
            if -0.02 < panner < 0.02:
                pan = Utils.prob2(self.table_pan)
            else:
                pan = pow(Utils.prob2(self.table_pan), (2. - (math.fabs(panner) + 0.01)))

            if math.floor(panner) == 0:
                panSequence.append(pan)
            else:
                panSequence.append(1. - pan)  
        return panSequence    
                
    def makeDurationSequence(self, onsetList, parameters):
        durationSequence = []
        for i in range(len(onsetList) - 1):
            duration = ((onsetList[i+1] - onsetList[i]) * Utils.prob2(self.table_duration))
#            if duration == (onsetList[i+1] - onsetList[i]):
#                duration = -1
            durationSequence.append(duration)
            
        durationSequence.append(((480 * parameters.bar) - onsetList[-1]) * Utils.prob2(self.table_duration))
        return durationSequence
            
    def makeCellule( self, currentDuration, targetDuration, threshold ):
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0  

