import random
import math

import Utils
import Drunk

from Framework.CSound.CSoundConstants import CSoundConstants
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
        self.density = density
        self.repete = repete
        self.step = step
        self.articule = articule
        self.panner = panner
        self.scale = scale
        self.pattern = pattern

class Generator:   
    def __init__( self, volumeFunctions, getTempoCallback, trackInstruments, trackDictionary, selectedTrackIDs, selectedPageIDs ):
        self.volumeFunctions = volumeFunctions
        self.getTempoCallback = getTempoCallback
        self.trackInstruments = trackInstruments
        self.trackDictionary = trackDictionary
        self.selectedTrackIDs = selectedTrackIDs
        self.selectedPageIDs = selectedPageIDs

    def generate( self, parameters, trackID ):
        trackNotes = []
        makeRythm = GenerationRythm( self.trackInstruments[ trackID ] )

        table_repetition = Utils.scale((1 - parameters.repete), GenerationConstants.REPETITION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_STEPS)
        table_onset = Utils.scale((1 - parameters.density), GenerationConstants.DENSITY_SCALE_MIN_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_MAX_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_STEPS)
        table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pan = Utils.scale(math.fabs(float( parameters.panner )), GenerationConstants.PAN_SCALE_MIN_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_MAX_MAPPING, 
                                                                            GenerationConstants.PAN_SCALE_STEPS)
        table_pitch = GenerationConstants.SCALES[parameters.scale]

        if parameters.pattern == 'Drunk': pitchMethod = Drunk.Drunk(len(table_pitch)-1)
        elif parameters.pattern == 'DroneAndJump': pitchMethod = Drunk.DroneAndJump(len(table_pitch)-1)           
        elif parameters.pattern == 'Repeter': pitchMethod = Drunk.Repeter(len(table_pitch)-1)            
        elif parameters.pattern == 'Loopseg': pitchMethod = Drunk.Loopseg(len(table_pitch)-1)

        if CSoundConstants.INSTRUMENTS[ self.trackInstruments[ trackID ] ].soundClass == 'drum':
            rythmSequence = makeRythm.drumRythmSequence(parameters, table_onset, table_repetition)
            pitchSequence = self.makePitchSequence2(len(rythmSequence), parameters.step, pitchMethod, table_pitch)
        elif CSoundConstants.INSTRUMENTS[ self.trackInstruments[ trackID ] ].soundClass == 'melo':
            rythmSequence = makeRythm.celluleRythmSequence(parameters, table_onset, table_repetition)
            pitchSequence = self.makePitchSequence(len(rythmSequence), parameters.step, pitchMethod, table_pitch)
        gainSequence = self.makeGainSequence(rythmSequence)
        panSequence = self.makePanSequence(len(rythmSequence), parameters.panner, table_pan)
        durationSequence, tiedSequence = self.makeDurationSequence(rythmSequence, parameters, table_duration)

        for i in range(len(rythmSequence)):
            trackNotes.append(CSoundNote(rythmSequence[i], pitchSequence[i], gainSequence[i], panSequence[i], durationSequence[i], trackID, 
                                                            self.volumeFunctions[trackID], self.getTempoCallback, tiedSequence[i]))
        return trackNotes
    
    def makePitchSequence(self, length, step, pitchMethod, table_pitch):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append((table_pitch[pitchMethod.getNextValue(step, (len(table_pitch)-1))]) + GenerationConstants.DEFAULT_TONIQUE)
        return pitchSequence

    def makePitchSequence2(self, length, step, pitchMethod=None, table_pitch=None):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(36 + random.choice( [ -5, 0, 0, 0, 0 ] ))         
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
