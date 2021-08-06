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
        append = gainSequence.append
        rand = random.uniform
        max = GenerationConstants.GAIN_MAX_BOUNDARY
        midMax = GenerationConstants.GAIN_MID_MAX_BOUNDARY
        midMin = GenerationConstants.GAIN_MID_MIN_BOUNDARY
        min = GenerationConstants.GAIN_MIN_BOUNDARY
        for onset in onsetList:
            if onset == 0:
                append(rand(midMax, max))
            elif ( onset % Config.TICKS_PER_BEAT) == 0:
                append(rand(midMin, midMax))
            else:     
                append(rand(min, midMin))
        return gainSequence  
                
    def makeDurationSequence( onsetList, parameters, table_duration, barLength, currentInstrument ):
        durationSequence = []
        if Config.INSTRUMENTS[currentInstrument].soundClass == 'drum':
            duration = GenerationConstants.DOUBLE_TICK_DUR / 2
            durationSequence = [duration] * len(onsetList)
            return durationSequence

        append = durationSequence.append
        proba = Utils.prob2
        if len( onsetList ) > 1:
            for i in range(len(onsetList) - 1):
                append((onsetList[i+1] - onsetList[i]) * proba( table_duration ))
            append(( barLength - onsetList[-1]) * proba( table_duration ))
        elif len( onsetList ) == 1:
            append( ( barLength - onsetList[0] ) * proba( table_duration ))
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
        rand = random.random
        append = trackNotes.append
        pan = GenerationConstants.DEFAULT_PAN
        instrument_id = Config.INSTRUMENTS[instrument[trackId]].instrumentId
        for i in numOfNotes:
            if drumPitch:
                if ( rand() * fillDrum ) > ( parameters.silence * .5 ):
                    if fillDrum != 1:
                        if rythmSequence[i] not in trackOnsets or pitchSequence[i] not in trackPitchs:
                            append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
                    else:
                        append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
            else:
                if rand() > parameters.silence:
                    append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.1, 0.1, 0, 1000, False, 'edit' ) )

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
            pageCycle = selectedPageCount % 4 # this should be fix in the meta algo

            if instrument[ trackId ][0:4] == 'drum':
                if pageCycle in [1,2]:
                    trackDictionary[trackId][pageId] = []
                    for n in trackDictionary[trackId][lastPageId]:
                        trackDictionary[trackId][pageId].append(n.clone())
                elif pageCycle == 3:
                    for n in trackDictionary[trackId][lastPageId]:
                        trackOfNotes.append(n.clone())
                    trackOnsets = [n.onset for n in trackOfNotes]
                    trackPitchs = [n.pitch for n in trackOfNotes]
                    fillDrum = .5
                    rythmRegTemp = parameters.rythmRegularity
                    parameters.rythmRegularity = 0.5
                    for drumPitch in GenerationConstants.DRUM_COMPLEXITY4:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )
                    parameters.rythmRegularity = rythmRegTemp
                elif pageCycle == 0:
                    fillDrum = 1
                    for drumPitch in streamOfPitch:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )

            else:
                if (selectedPageCount % 2) == 0 or random.randint(0, 5) < 2 or selectedPageCount == 0:
                    pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch = None )
                else:
                    trackDictionary[trackId][pageId] = []
                    for n in trackDictionary[trackId][lastPageId]:
                        trackDictionary[trackId][pageId].append(n.clone())
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

    
