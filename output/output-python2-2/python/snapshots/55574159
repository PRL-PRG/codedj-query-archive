import random
import math

import Utils
import Drunk

import Config
from Util.CSoundNote import CSoundNote
from Generation.VariationPitch import *
from Generation.VariationRythm import *
from Generation.GenerationConstants import GenerationConstants
from Generation.GenerationRythm import GenerationRythm
from Generation.GenerationPitch import GenerationPitch

class GenerationParameters:
    def __init__( self, 
                  density = GenerationConstants.DEFAULT_DENSITY,
                  rythmRegularity = GenerationConstants.DEFAULT_RYTHM_REGULARITY,
                  step = GenerationConstants.DEFAULT_STEP,
                  pitchRegularity = GenerationConstants.DEFAULT_PITCH_REGULARITY,
                  articule = GenerationConstants.DEFAULT_DURATION,
                  silence = GenerationConstants.DEFAULT_SILENCE,
                  rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD,
                  pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD,
                  pattern = GenerationConstants.DEFAULT_PATTERN,
                  scale = GenerationConstants.DEFAULT_SCALE ):
        self.density = density
        self.rythmRegularity = rythmRegularity
        self.step = step
        self.pitchRegularity = pitchRegularity
        self.articule = articule
        self.silence = silence
        self.rythmMethod = rythmMethod
        self.pitchMethod = pitchMethod
        self.pattern = pattern
        self.scale = scale

def generator1( 
        parameters, # algorithm-specific parameters
        volume,     # [trackId: float(volume) ]
        instrument, # [trackId: instrument]
        tempo,      # integer bpm
        nbeats,     # map [ pageId : beats ]
        trackIds,   # list of trackIds to generate
        pageIds,    # list of pageIds to generate
        trackDictionary # map [ trackId : [ pageId : events ] ]
        ):

    makeRythm = GenerationRythm()
    makePitch = GenerationPitch(parameters.pattern)
    #makeHarmonicSequence = Drunk.Drunk( 0, 7 )

    def makeGainSequence( onsetList ):
        gainSequence = []
        max = GenerationConstants.GAIN_MAX_BOUNDARY
        midMax = GenerationConstants.GAIN_MID_MAX_BOUNDARY
        for onset in onsetList:
            if onset == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MAX_BOUNDARY, GenerationConstants.GAIN_MAX_BOUNDARY)
            elif ( onset % Config.TICKS_PER_BEAT) == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MAX_BOUNDARY)
            else:     
                gain = random.uniform(GenerationConstants.GAIN_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MIN_BOUNDARY)
            gainSequence.append(gain)
        return gainSequence  
                
    def makeDurationSequence( onsetList, parameters, table_duration, barLength, currentInstrument ):
        durationSequence = []
        if Config.INSTRUMENTS[currentInstrument].soundClass == 'drum':
            duration = GenerationConstants.DOUBLE_TICK_DUR / 2
            durationSequence = [duration] * len(onsetList)
            return durationSequence

        if len( onsetList ) > 1:
            for i in range(len(onsetList) - 1):
                duration = (onsetList[i+1] - onsetList[i]) * Utils.prob2( table_duration )
                durationSequence.append(duration)      
            durationSequence.append(( barLength - onsetList[-1]) * Utils.prob2( table_duration ))
        elif len( onsetList ) == 1:
            durationSequence.append( ( barLength - onsetList[0] ) * Utils.prob2( table_duration ))
        return durationSequence

    def pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch = None ):

        trackNotes = trackOfNotes

        if drumPitch: 
            currentInstrument = Config.INSTRUMENTS[instrument[ trackId ]].kit[drumPitch[0]].name
            rythmSequence = makeRythm.drumRythmSequence(parameters, currentInstrument, barLength)
            pitchSequence = makePitch.drumPitchSequence(len(rythmSequence), parameters, drumPitch, table_pitch )
        else:  
            currentInstrument = instrument[ trackId ]
            rythmSequence = makeRythm.celluleRythmSequence(parameters, barLength, currentInstrument)
            pitchSequence = makePitch.drunkPitchSequence(len(rythmSequence),parameters, table_pitch)
            makePitch.pitchMethod.__init__(5, 12)

        gainSequence = makeGainSequence(rythmSequence)
        durationSequence = makeDurationSequence(rythmSequence, parameters, table_duration, barLength, currentInstrument)

        numOfNotes = range(len(rythmSequence))
        for i in numOfNotes:
            if drumPitch:
                if ( random.random() * fillDrum ) > ( parameters.silence * .7 ):
                    if fillDrum != 1:
                        if rythmSequence[i] not in trackOnsets or pitchSequence[i] not in trackPitchs:
                            trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], GenerationConstants.DEFAULT_PAN, durationSequence[i], trackId, Config.INSTRUMENTS[instrument[ trackId ]].instrumentId, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
                    else:
                        trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], GenerationConstants.DEFAULT_PAN, durationSequence[i], trackId, Config.INSTRUMENTS[instrument[ trackId ]].instrumentId, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
            else:
                if random.random() > parameters.silence:
                    trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], GenerationConstants.DEFAULT_PAN, durationSequence[i], trackId, Config.INSTRUMENTS[instrument[ trackId ]].instrumentId, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )

        trackDictionary[ trackId ][ pageId ] = trackNotes

################################################################################## 
    #  begin generate() 
#    harmonicSequence = []
#    for i in range( nbeats ):
#        harmonicSequence.append( 
#                GenerationConstants.CHORDS_TABLE[ makeHarmonicSequence.getNextValue( 2, len( GenerationConstants.CHORDS_TABLE ) - 1 ) ] )

    table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, GenerationConstants.ARTICULATION_SCALE_STEPS)
    table_pitch = GenerationConstants.SCALES[parameters.scale]

    for trackId in trackIds:
        if instrument[ trackId ][0:4] == 'drum':
            if parameters.rythmRegularity > 0.75:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY1
            elif parameters.rythmRegularity > 0.5:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY2
            elif parameters.rythmRegularity > 0.25:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY3
            else:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY4
        selectedPageCount = 0
        lastPageId = 0
        for pageId in pageIds:
            barLength = Config.TICKS_PER_BEAT * nbeats[ pageId ]
            trackOfNotes = []
            if instrument[ trackId ][0:4] == 'drum':
                if ( selectedPageCount % 4 ) in [1,2]:
                    trackDictionary[ trackId ][ pageId ] = [ n for n in trackDictionary[ trackId ][ lastPageId ] ]
                elif ( selectedPageCount % 4 ) == 3:
                    trackOfNotes = [ n for n in trackDictionary[ trackId ][ lastPageId ] ]
                    trackOnsets = [n.onset for n in trackOfNotes]
                    trackPitchs = [n.pitch for n in trackOfNotes]
                    fillDrum = .5
                    rythmRegTemp = parameters.rythmRegularity
                    parameters.rythmRegularity = 0.
                    for drumPitch in GenerationConstants.DRUM_COMPLEXITY4:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )
                    parameters.rythmRegularity = rythmRegTemp
                elif ( selectedPageCount % 4 ) == 0:
                    fillDrum = 1
                    for drumPitch in streamOfPitch:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )

            else:
                pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch = None )

            selectedPageCount += 1
            lastPageId = pageId

class VariationParameters:
    def __init__( self, sourceVariation, pitchVariation = 0, rythmVariation = 0 ):
        self.sourceVariation = sourceVariation
        self.pitchVariation = pitchVariation
        self.rythmVariation = rythmVariation


def variate( 
        parameters, # algorithm-specific parameters
        volume,     # [trackId: float(volume) ]
        instrument, # [trackId: instrument]
        tempo,      # integer bpm
        nbeats,     # map [ pageId : beats ]
        trackIds,   # list of trackIds to generate
        pageIds,    # list of pageIds to generate
        trackDictionary # map [ trackId : [ pageId : events ] ]
        ):

    pitchMarkov = PitchMarkov()
    pitchReverse = PitchReverse()
    pitchSort = PitchSort()
    pitchShuffle = PitchShuffle()

    makePitch = GenerationPitch()
    makeHarmonicSequence = Drunk.Drunk(0, 7 )
    rythmShuffle = RythmShuffle( )
    rythmReverse = RythmReverse( )

    def pageVariate(  parameters, trackId, pageId ):
        tempTrackNotes = []
        trackNotes = []
        for note in trackDictionary[ trackId ][ parameters.sourceVariation ]:
            tempTrackNotes.append( note.clone() )

        if parameters.rythmVariation == 0:
            for note in tempTrackNotes:
                trackNotes.append( note.clone() )
        if parameters.rythmVariation == 1:
            for note in rythmReverse.getNewList( tempTrackNotes, nbeats[ pageId ] ):
                trackNotes.append( note.clone() )
        if parameters.rythmVariation == 2:
            for note in rythmShuffle.getNewList( tempTrackNotes , nbeats[ pageId ] ):
                trackNotes.append( note.clone() )

        #del trackDictionary[ trackId ][ pageId ]
        trackDictionary[ trackId ][ pageId ] = trackNotes

        tempTrackNotes = []
        trackNotes = []
        for note in trackDictionary[ trackId ][ parameters.sourceVariation ]:
            tempTrackNotes.append( note.clone() )

        if parameters.pitchVariation == 0:
            for note in  tempTrackNotes:
                trackNotes.append( note.clone() )
        elif parameters.pitchVariation == 1:
            for note in pitchMarkov.getNewList( tempTrackNotes, 1 ):
                trackNotes.append( note.clone() )
        elif parameters.pitchVariation == 2:
            for note in pitchReverse.reorderPitch( tempTrackNotes ):
                trackNotes.append( note.clone() )
        elif parameters.pitchVariation == 3:
            for note in pitchSort.reorderPitch( tempTrackNotes ):
                trackNotes.append( note.clone() )
        elif parameters.pitchVariation == 4:
            for note in pitchShuffle.reorderPitch( tempTrackNotes ):
                trackNotes.append( note.clone() )                

        #del trackDictionary[ trackId ][ pageId ]
        trackDictionary[ trackId ][ pageId ] = trackNotes

    for trackId in trackIds:
        for pageId in pageIds:
            pageVariate( parameters, trackId, pageId )

    
