import random
import math

import Utils
import Drunk

from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParameters:
    def __init__( self, 
                  bar = GenerationConstants.DEFAULT_BAR,
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
    def generate( self, parameters, trackID ):
        self.table_repetition = Utils.scale(parameters.repete, GenerationConstants.REPETITION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_STEPS)
        self.table_onset = Utils.scale(parameters.density, GenerationConstants.DENSITY_SCALE_MIN_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_MAX_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_STEPS)
        self.table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        self.table_pan = Utils.scale(math.fabs(float( parameters.panner )), GenerationConstants.PAN_SCALE_MIN_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_MAX_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_STEPS)
        self.trackNotes = []
        self.trackID = trackID
        self.choosePitchTable = random.choice( [ GenerationConstants.MAJOR_SCALE,
                                                 GenerationConstants.HARMONIC_MINOR_SCALE,
                                                 GenerationConstants.NATURAL_MINOR_SCALE,
                                                 GenerationConstants.PENTATONIC_SCALE,
                                                 GenerationConstants.BLUES_SCALE,
                                                 GenerationConstants.PHRYGIEN_SCALE ] )
        self.choosePitchTable = GenerationConstants.HARMONIC_MINOR_SCALE
        self.chooseNewPitch = Drunk.Loopseg(len(self.choosePitchTable)-1)

        rythmSequence = self.makeRythmSequence2(parameters)
        pitchSequence = self.makePitchSequence(len(rythmSequence), parameters.step)
        gainSequence = self.makeGainSequence(rythmSequence)
        panSequence = self.makePanSequence(len(rythmSequence), parameters.panner)
        durationSequence = self.makeDurationSequence(rythmSequence, parameters)

        for i in range(len(rythmSequence)):
            self.trackNotes.append(CSoundNote(rythmSequence[i], pitchSequence[i], gainSequence[i], panSequence[i], durationSequence[i], self.trackID))

        return self.trackNotes

    def makeRythmSequence(self, parameters ):

        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0
        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(self.table_onset))]

        for i in range(int(parameters.bar) * GenerationConstants.MAX_NOTES_PER_BAR):
            if self.count == 0:   
                repetitionFlag = Utils.prob2(self.table_repetition)
                if repetitionFlag != 0:
                    onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(self.table_onset))]

            self.makeCellule(onsetDelta, GenerationConstants.TRIPLE_TICK_DUR, GenerationConstants.TRIPLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.TRIPLE_TRIPLET_TICK_DUR, GenerationConstants.TRIPLE_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_QUINTUPLETS_TICK_DUR, GenerationConstants.DOUBLE_QUINTUPLETS_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_TICK_DUR, GenerationConstants.DOUBLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HALF_TRIPLET_TICK_DUR, GenerationConstants.HALF_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HOLE_TRIPLET_TICK_DUR, GenerationConstants.HOLE_TRIPLET_HOW_MANY)

            onsetTime = onsetDelta + lastOnsetTime 
            lastOnsetTime = onsetTime
            
            if onsetTime < (GenerationConstants.BAR_LENGTH * parameters.bar):
                rythmSequence.append(onsetTime)
            else:
                break    
            
        return rythmSequence  

    def makeRythmSequence2(self, parameters):
        rythmSequence = []
        onsetTime = None
        whichRandomGenerator = random.randint(0, 4)
        maximumNumberOfNotes = int((1 - parameters.density) * GenerationConstants.MAX_NOTES_PER_BAR)
 
#TODO: link the different random variation parameters with sliders controls parameters
        for i in range(maximumNumberOfNotes):
            while onsetTime in rythmSequence:
                if whichRandomGenerator == 0:
                    onsetTime = random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM)
                elif whichRandomGenerator == 1:
                    onsetTime = 1 - random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM)
                elif whichRandomGenerator == 2:
                    onsetTime = random.gauss(GenerationConstants.RANDOM_GAUSS_PARAM1, GenerationConstants.RANDOM_GAUSS_PARAM2)
                elif whichRandomGenerator == 3:
                    onsetTime = random.betavariate(GenerationConstants.RANDOM_BETA_PARAM, GenerationConstants.RANDOM_BETA_PARAM)
                elif whichRandomGenerator == 4:
                    onsetTime = random.weibullvariate(GenerationConstants.RANDOM_WEIBULL_PARAM1, GenerationConstants.RANDOM_WEIBULL_PARAM2)

                onsetTime = int(onsetTime * (int((GenerationConstants.BAR_LENGTH - 1) / GenerationConstants.TRIPLE_TICK_DUR))) * GenerationConstants.TRIPLE_TICK_DUR

            if onsetTime < 0:
                onsetTime = 0
            elif onsetTime > (GenerationConstants.BAR_LENGTH - GenerationConstants.TRIPLE_TICK_DUR):
                onsetTime = (GenerationConstants.BAR_LENGTH - GenerationConstants.TRIPLE_TICK_DUR)
            else:
                onsetTime = onsetTime

            if onsetTime not in rythmSequence:
                rythmSequence.append(onsetTime)

        rythmSequence.sort()
        return rythmSequence  
    
    def makePitchSequence(self, length, step):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(self.choosePitchTable[self.chooseNewPitch.getNextValue(step, (len(self.choosePitchTable)-1))])
        return pitchSequence
    
    def makeGainSequence(self, onsetList):
        gainSequence = []
        
        for onset in onsetList:
            accentOnset = (onset % GenerationConstants.BAR_LENGTH)

            if accentOnset == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MAX_BOUNDARY, GenerationConstants.GAIN_MAX_BOUNDARY)
            elif (accentOnset % 120) == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MAX_BOUNDARY)
            else:     
                gain = random.uniform(GenerationConstants.GAIN_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MIN_BOUNDARY)
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
            if duration == (onsetList[i+1] - onsetList[i]):
                duration = -1
            durationSequence.append(duration)
            
        durationSequence.append(((GenerationConstants.BAR_LENGTH * parameters.bar) - onsetList[-1]) * Utils.prob2(self.table_duration))
        return durationSequence
            
    def makeCellule( self, currentDuration, targetDuration, threshold ):
        threshold = threshold - 1
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0  

