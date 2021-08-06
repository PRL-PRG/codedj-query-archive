import random
import math

import Utils
import Drunk

from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.Generation.GenerationRythm import GenerationRythm

class GenerationParameters:
    def __init__( self, 
                  bar = GenerationConstants.DEFAULT_BAR,
                  density = GenerationConstants.DEFAULT_DENSITY,
                  repete = GenerationConstants.DEFAULT_REPETE,
                  step = GenerationConstants.DEFAULT_STEP,
                  articule = GenerationConstants.DEFAULT_ARTICULE,
                  panner = GenerationConstants.DEFAULT_PANNER,
                  scale = GenerationConstants.DEFAULT_SCALE,
                  pattern = GenerationConstants.DEFAULT_PATTERN ):
        self.bar = bar
        self.density = ( 1 - density )
        self.repete = repete
        self.step = step
        self.articule = articule
        self.panner = panner
        self.scale = scale
        self.pattern = pattern

# ?? les self, passer les tables au fonctions ??
class Generator:   
    def __init__( self, volumeFunctions, getTempoCallback ):
        self.volumeFunctions = volumeFunctions
        self.getTempoCallback = getTempoCallback

    def generate( self, parameters, trackID, trackDictionary ):
        self.trackDictionary = trackDictionary
        makeRythm = GenerationRythm()

        table_repetition = Utils.scale((1 - parameters.repete), GenerationConstants.REPETITION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_STEPS)
        table_onset = Utils.scale(parameters.density, GenerationConstants.DENSITY_SCALE_MIN_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_MAX_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_STEPS)
        table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pan = Utils.scale(math.fabs(float( parameters.panner )), GenerationConstants.PAN_SCALE_MIN_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_MAX_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_STEPS)
        self.trackNotes = []
        self.trackID = trackID
        self.choosePitchTable = GenerationConstants.SCALES[parameters.scale]

        if parameters.pattern == 'Drunk':
            self.chooseNewPitch = Drunk.Drunk(len(self.choosePitchTable)-1)
        elif parameters.pattern == 'DroneAndJump':
            self.chooseNewPitch = Drunk.DroneAndJump(len(self.choosePitchTable)-1)
        if parameters.pattern == 'Repeter':
            self.chooseNewPitch = Drunk.Repeter(len(self.choosePitchTable)-1)
        if parameters.pattern == 'Loopseg':
            self.chooseNewPitch = Drunk.Loopseg(len(self.choosePitchTable)-1)

        rythmSequence = makeRythm.celluleRythmSequence(parameters, table_onset, table_repetition)
        pitchSequence = self.makePitchSequence(len(rythmSequence), parameters.step)
        gainSequence = self.makeGainSequence(rythmSequence)
        panSequence = self.makePanSequence(len(rythmSequence), parameters.panner, table_pan)
        durationSequence, tiedSequence = self.makeDurationSequence(rythmSequence, parameters, table_duration)

        for i in range(len(rythmSequence)):
            self.trackNotes.append(CSoundNote(rythmSequence[i], pitchSequence[i], gainSequence[i], panSequence[i], durationSequence[i], self.trackID, self.volumeFunctions[self.trackID], self.getTempoCallback, tiedSequence[i]))

        return self.trackNotes
    
    def makePitchSequence(self, length, step):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(self.choosePitchTable[self.chooseNewPitch.getNextValue(step, (len(self.choosePitchTable)-1))])

#        if self.trackID == 1:
#            pitchSequence = []
#            for v in self.trackDictionary[0]: 
#                pitchSequence.append(v.pitch + 7)
        return pitchSequence

    def makePitchSequence2(self, length, step):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(36)         
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

    def makePanSequence(self, length, panner, table_pan):
        panSequence = []
        for i in range(length):
            if -0.02 < panner < 0.02:
                pan = Utils.prob2(table_pan)
            else:
                pan = pow(Utils.prob2(table_pan), (2. - (math.fabs(panner) + 0.01)))

            if math.floor(panner) == 0:
                panSequence.append(pan)
            else:
                panSequence.append(1. - pan)  
        return panSequence    
                
    def makeDurationSequence(self, onsetList, parameters, table_duration):
        durationSequence = []
        tiedSequence = []
        for i in range(len(onsetList) - 1):
            duration = ((onsetList[i+1] - onsetList[i]) * Utils.prob2(table_duration))
            if duration == (onsetList[i+1] - onsetList[i]):
                tiedSequence.append(True)
            else:   
                tiedSequence.append(False)
            durationSequence.append(duration)         
        durationSequence.append(((GenerationConstants.BAR_LENGTH * parameters.bar) - onsetList[-1]) * Utils.prob2(table_duration))
        tiedSequence.append(False)
        return durationSequence,  tiedSequence
            


