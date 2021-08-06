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
                  articule = GenerationConstants.DEFAULT_ARTICULE,
                  rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD,
                  pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD,
                  pattern = GenerationConstants.DEFAULT_PATTERN,
                  scale = GenerationConstants.DEFAULT_SCALE ):
        self.density = density
        self.rythmRegularity = rythmRegularity
        self.step = step
        self.pitchRegularity = pitchRegularity
        self.articule = articule
        self.rythmMethod = rythmMethod
        self.pitchMethod = pitchMethod
        self.pattern = pattern
        self.scale = scale

def generator1( 
        parameters, # algorithm-specific parameters
        volume,     # [trackId: float(volume) ]
        instrument, # [trackId: instrument]
        tempo,      # integer bpm
        nbeats,     # integer
        trackIds,   # list of trackIds to generate
        pageIds,    # list of pageIds to generate
        trackDictionary # map [ trackId : [ pageId : events ] ]
        ):

    pitchMarkov = PitchMarkov()
    pitchReverse = PitchReverse()
    pitchSort = PitchSort()
    pitchShuffle = PitchShuffle()

    makePitch = GenerationPitch()
    makeHarmonicSequence = Drunk.Drunk( 7 )

    rythmShuffle = RythmShuffle( )
    rythmReverse = RythmReverse( )

    def makeGainSequence( onsetList ):
        gainSequence = []
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
        fullDurationSequence = []
        if len( onsetList ) > 1:
            for i in range(len(onsetList) - 1):
                duration = (onsetList[i+1] - onsetList[i]) * Utils.prob2( table_duration )
                if duration == (onsetList[i+1] - onsetList[i]):
                    fullDurationSequence.append(True)
                else:
                    fullDurationSequence.append(False)

                if Config.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                    duration = GenerationConstants.DOUBLE_TICK_DUR / 2

                durationSequence.append(duration)      

            if Config.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                durationSequence.append( GenerationConstants.DOUBLE_TICK_DUR / 2)
            else:
                durationSequence.append(( barLength - onsetList[-1]) * Utils.prob2( table_duration ))
            fullDurationSequence.append(False)
        elif len( onsetList ) == 1:
            if Config.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                durationSequence.append( GenerationConstants.DOUBLE_TICK_DUR / 2 )
            else:
                durationSequence.append( ( barLength - onsetList[ 0 ] ) * Utils.prob2( table_duration ))
            fullDurationSequence.append( False )
        return durationSequence,  fullDurationSequence

    def pageGenerate( parameters, trackId, pageId, selectedPageCount, lastPageId, trackOfNotes, drumPitch = None ):
        trackNotes = trackOfNotes
        barLength = Config.TICKS_PER_BEAT * nbeats
        if drumPitch:
            currentInstrument = Config.DRUM1INSTRUMENTS[ drumPitch[ 0 ]  ]
        else:
            drumPitch = [ 36 ]
            currentInstrument = instrument[ trackId ]

        makeRythm = GenerationRythm( currentInstrument, barLength )

        table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pitch = GenerationConstants.SCALES[parameters.scale]

        if Config.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
            rythmSequence = makeRythm.drumRythmSequence(parameters)
            pitchSequence = makePitch.drumPitchSequence(len(rythmSequence), parameters, drumPitch, table_pitch )
        elif Config.INSTRUMENTS[ currentInstrument ].soundClass == 'melo':
            if parameters.rythmMethod == 0:
                rythmSequence = makeRythm.celluleRythmSequence(parameters)
            elif parameters.rythmMethod == 1:
                rythmSequence = makeRythm.xnoiseRythmSequence(parameters)                
            if parameters.pitchMethod == 0:
                pitchSequence = makePitch.drunkPitchSequence(len(rythmSequence), parameters, table_pitch)
            elif parameters.pitchMethod == 1:
                pitchSequence = makePitch.harmonicPitchSequence( rythmSequence, parameters, table_pitch, harmonicSequence )
        gainSequence = makeGainSequence(rythmSequence)
        durationSequence, fullDurationSequence = makeDurationSequence(rythmSequence, parameters, table_duration, barLength, currentInstrument)

        for i in range(len(rythmSequence)):
            trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], 
                                           GenerationConstants.DEFAULT_PAN, durationSequence[i], trackId, 
                                           fullDurationSequence[i], instrument[ trackId ] ) )
#        del trackDictionary[ trackId ][ pageId ]
        trackDictionary[ trackId ][ pageId ] = trackNotes

################################################################################## 
    #  begin generate() 
    harmonicSequence = []
    for i in range( nbeats ):
        harmonicSequence.append( 
                GenerationConstants.CHORDS_TABLE[ makeHarmonicSequence.getNextValue( 2, len( GenerationConstants.CHORDS_TABLE ) - 1 ) ] )
 
    for trackId in trackIds:
        if instrument[ trackId ] == 'drum1kit':
            if parameters.rythmRegularity > 0.75:
                pitchOfStream = [ [ 24 ], [30] , [ 40 ], [ 46 ]  ]
            elif parameters.rythmRegularity > 0.5:
                pitchOfStream = [ [ 24, 28 ], [ 30, 32 ], [ 36, 38, 40 ], [ 46, 48 ]  ]
            elif parameters.rythmRegularity > 0.25:
                pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 46, 48 ]  ] 
            else:
                pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 44, 46, 48 ]  ] 
        selectedPageCount = 0
        lastPageId = 0
        for pageId in pageIds:
            trackOfNotes = []
#            del trackDictionary[ trackId ][ pageId ]
            if instrument[ trackId ] == 'drum1kit':
                for drumPitch in pitchOfStream:
                    pageGenerate( parameters, trackId, pageId, selectedPageCount, lastPageId, trackOfNotes, drumPitch )
            else:
                pageGenerate( parameters, trackId, pageId, selectedPageCount, lastPageId, trackOfNotes, drumPitch = None )

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
        nbeats,     # integer
        trackIds,   # list of trackIds to generate
        pageIds,    # list of pageIds to generate
        trackDictionary # map [ trackId : [ pageId : events ] ]
        ):

    pitchMarkov = PitchMarkov()
    pitchReverse = PitchReverse()
    pitchSort = PitchSort()
    pitchShuffle = PitchShuffle()

    makePitch = GenerationPitch()
    makeHarmonicSequence = Drunk.Drunk( 7 )
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
            for note in rythmReverse.getNewList( tempTrackNotes, nbeats ):
                trackNotes.append( note.clone() )
        if parameters.rythmVariation == 2:
            for note in rythmShuffle.getNewList( tempTrackNotes , nbeats):
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

    
