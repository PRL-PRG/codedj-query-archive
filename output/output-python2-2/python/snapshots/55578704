import random
import math

import Utils
import Drunk

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.Generation.GenerationRythm import GenerationRythm
from Framework.Generation.GenerationPitch import GenerationPitch

class GenerationParameters:
    def __init__( self, 
                  density = GenerationConstants.DEFAULT_DENSITY,
                  repete = GenerationConstants.DEFAULT_REPETE,
                  step = GenerationConstants.DEFAULT_STEP,
                  articule = GenerationConstants.DEFAULT_ARTICULE,
                  scale = GenerationConstants.DEFAULT_SCALE,
                  pattern = GenerationConstants.DEFAULT_PATTERN ):
        self.density = density
        self.repete = repete
        self.step = step
        self.articule = articule
        self.scale = scale
        self.pattern = pattern

class Generator:   
    def __init__( self, volumeFunctions, getTempoCallback, trackInstruments, trackDictionary, 
                            getBeatsPerPageCallback, getActiveTrackIDsCallback, selectedPageIDs ):
        self.volumeFunctions = volumeFunctions
        self.getTempoCallback = getTempoCallback
        self.trackInstruments = trackInstruments
        self.getBeatsPerPageCallback = getBeatsPerPageCallback
        self.trackDictionary = trackDictionary
        self.getActiveTrackIDsCallback = getActiveTrackIDsCallback
        self.selectedPageIDs = selectedPageIDs

        self.makePitch = GenerationPitch()
        self.makeHarmonicSequence = Drunk.Drunk( 7 )

    def generate( self, parameters ):
        self.harmonicSequence = []
        for i in range( self.getBeatsPerPageCallback() ):
            self.harmonicSequence.append( GenerationConstants.CHORDS_TABLE[  self.makeHarmonicSequence.getNextValue( -2, len( GenerationConstants.CHORDS_TABLE ) - 1 ) ] )
        
        for trackID in self.getActiveTrackIDsCallback():
            selectedPageCount = 0
            lastPageID = 0
            for pageID in self.selectedPageIDs:
                self.pageGenerate( parameters, trackID, pageID, selectedPageCount, lastPageID )
                selectedPageCount += 1
                lastPageID = pageID

    def pageGenerate( self, parameters, trackID, pageID, selectedPageCount, lastPageID ):
        trackNotes = []
        barLength = Constants.TICKS_PER_BEAT * self.getBeatsPerPageCallback()
        makeRythm = GenerationRythm( self.trackInstruments[ trackID ], barLength )

        if CSoundConstants.INSTRUMENTS[ self.trackInstruments[ trackID ] ].soundClass == 'drum':
            if random.randint( 0, 4) > 0 and selectedPageCount != 0:
                del self.trackDictionary[ trackID ][ pageID ]
                self.trackDictionary[ trackID ][ pageID ] = self.trackDictionary[ trackID ][ lastPageID ]
                return
            
        table_repetition = Utils.scale((1 - parameters.repete), GenerationConstants.REPETITION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.REPETITION_SCALE_STEPS)
        table_onset = Utils.scale((1 - parameters.density), GenerationConstants.DENSITY_SCALE_MIN_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_MAX_MAPPING, 
                                                           GenerationConstants.DENSITY_SCALE_STEPS)
        table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pitch = GenerationConstants.SCALES[parameters.scale]

        if CSoundConstants.INSTRUMENTS[ self.trackInstruments[ trackID ] ].soundClass == 'drum':
            rythmSequence = makeRythm.drumRythmSequence(parameters, table_onset, table_repetition)
            pitchSequence = self.makePitch.drumPitchSequence(len(rythmSequence), parameters, table_pitch)
        elif CSoundConstants.INSTRUMENTS[ self.trackInstruments[ trackID ] ].soundClass == 'melo':
            rythmSequence = makeRythm.celluleRythmSequence(parameters, table_onset, table_repetition)
            pitchSequence = self.makePitch.drunkPitchSequence(len(rythmSequence), parameters, table_pitch)
#            pitchSequence = self.makePitch.harmonicPitchSequence( rythmSequence, parameters, table_pitch, self.harmonicSequence )
        gainSequence = self.makeGainSequence(rythmSequence)
        durationSequence, tiedSequence = self.makeDurationSequence(rythmSequence, parameters, table_duration, barLength)

        for i in range(len(rythmSequence)):
            trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], 
                                           GenerationConstants.DEFAULT_PAN, durationSequence[i], trackID, 
                                           tiedSequence[i], self.trackInstruments[ trackID ] ) )
        del self.trackDictionary[ trackID ][ pageID ]
        self.trackDictionary[ trackID ][ pageID ] = trackNotes
    
    def makeGainSequence(self, onsetList ):
        gainSequence = []
        
        for onset in onsetList:
            if onset == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MAX_BOUNDARY, GenerationConstants.GAIN_MAX_BOUNDARY)
            elif ( onset % Constants.TICKS_PER_BEAT) == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MAX_BOUNDARY)
            else:     
                gain = random.uniform(GenerationConstants.GAIN_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MIN_BOUNDARY)
            gainSequence.append(gain)
        return gainSequence  
                
    def makeDurationSequence(self, onsetList, parameters, table_duration, barLength ):
        durationSequence = []
        tiedSequence = []
        if len( onsetList ) > 1:
            for i in range(len(onsetList) - 1):
                duration = ((onsetList[i+1] - onsetList[i]) * Utils.prob2(table_duration))
                if duration == (onsetList[i+1] - onsetList[i]):
                    tiedSequence.append(True)
                else:   
                    tiedSequence.append(False)
                durationSequence.append(duration)      
            durationSequence.append(( barLength - onsetList[-1]) * Utils.prob2(table_duration))
            tiedSequence.append(False)
        elif len( onsetList ) == 1:
            durationSequence.append( ( barLength - onsetList[ 0 ] ) * Utils.prob2(table_duration))
            tiedSequence.append( False )
        return durationSequence,  tiedSequence
