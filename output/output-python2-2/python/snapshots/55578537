import random
import math
import Utils
import Drunk

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundNote import CSoundNote
from Framework.Generation.VariationPitch import *
from Framework.Generation.VariationRythm import *
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.Generation.GenerationRythm import GenerationRythm
from Framework.Generation.GenerationPitch import GenerationPitch

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
        volume,     # [trackID: float(volume) ]
        instrument, # [trackID: instrument]
        tempo,      # integer bpm
        nbeats,     # integer
        trackIDs,   # list of trackIDs to generate
        pageIDs,    # list of pageIDs to generate
        trackDictionary # map [ trackID : [ pageID : events ] ]
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
            elif ( onset % Constants.TICKS_PER_BEAT) == 0:
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

                if CSoundConstants.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                    duration = GenerationConstants.DOUBLE_TICK_DUR / 2

                durationSequence.append(duration)      

            if CSoundConstants.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                durationSequence.append( GenerationConstants.DOUBLE_TICK_DUR / 2)
            else:
                durationSequence.append(( barLength - onsetList[-1]) * Utils.prob2( table_duration ))
            fullDurationSequence.append(False)
        elif len( onsetList ) == 1:
            if CSoundConstants.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
                durationSequence.append( GenerationConstants.DOUBLE_TICK_DUR / 2 )
            else:
                durationSequence.append( ( barLength - onsetList[ 0 ] ) * Utils.prob2( table_duration ))
            fullDurationSequence.append( False )
        return durationSequence,  fullDurationSequence

    def pageGenerate( parameters, trackID, pageID, selectedPageCount, lastPageID, trackOfNotes, drumPitch = None ):
        trackNotes = trackOfNotes
        barLength = Constants.TICKS_PER_BEAT * nbeats
        if drumPitch:
            currentInstrument = CSoundConstants.DRUM1INSTRUMENTS[ drumPitch[ 0 ]  ]
        else:
            drumPitch = [ 36 ]
            currentInstrument = instrument[ trackID ]

        makeRythm = GenerationRythm( currentInstrument, barLength )

        table_duration = Utils.scale(parameters.articule, GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, 
                                                               GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pitch = GenerationConstants.SCALES[parameters.scale]

        if CSoundConstants.INSTRUMENTS[ currentInstrument ].soundClass == 'drum':
            rythmSequence = makeRythm.drumRythmSequence(parameters)
            pitchSequence = makePitch.drumPitchSequence(len(rythmSequence), parameters, drumPitch, table_pitch )
        elif CSoundConstants.INSTRUMENTS[ currentInstrument ].soundClass == 'melo':
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
                                           GenerationConstants.DEFAULT_PAN, durationSequence[i], trackID, 
                                           fullDurationSequence[i], instrument[ trackID ] ) )
#        del trackDictionary[ trackID ][ pageID ]
        trackDictionary[ trackID ][ pageID ] = trackNotes

################################################################################## 
    #  begin generate() 
    harmonicSequence = []
    for i in range( nbeats ):
        harmonicSequence.append( 
                GenerationConstants.CHORDS_TABLE[ makeHarmonicSequence.getNextValue( 2, len( GenerationConstants.CHORDS_TABLE ) - 1 ) ] )
 
    for trackID in trackIDs:
        if instrument[ trackID ] == 'drum1kit':
            if parameters.rythmRegularity > 0.75:
                pitchOfStream = [ [ 24 ], [30] , [ 40 ], [ 46 ]  ]
            elif parameters.rythmRegularity > 0.5:
                pitchOfStream = [ [ 24, 28 ], [ 30, 32 ], [ 36, 38, 40 ], [ 46, 48 ]  ]
            elif parameters.rythmRegularity > 0.25:
                pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 46, 48 ]  ] 
            else:
                pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 44, 46, 48 ]  ] 
        selectedPageCount = 0
        lastPageID = 0
        for pageID in pageIDs:
            trackOfNotes = []
#            del trackDictionary[ trackID ][ pageID ]
            if instrument[ trackID ] == 'drum1kit':
                for drumPitch in pitchOfStream:
                    pageGenerate( parameters, trackID, pageID, selectedPageCount, lastPageID, trackOfNotes, drumPitch )
            else:
                pageGenerate( parameters, trackID, pageID, selectedPageCount, lastPageID, trackOfNotes, drumPitch = None )

            selectedPageCount += 1
            lastPageID = pageID


class VariationParameters:
    def __init__( self, sourceVariation, pitchVariation = 0, rythmVariation = 0 ):
        self.sourceVariation = sourceVariation
        self.pitchVariation = pitchVariation
        self.rythmVariation = rythmVariation


def variate( 
        parameters, # algorithm-specific parameters
        volume,     # [trackID: float(volume) ]
        instrument, # [trackID: instrument]
        tempo,      # integer bpm
        nbeats,     # integer
        trackIDs,   # list of trackIDs to generate
        pageIDs,    # list of pageIDs to generate
        trackDictionary # map [ trackID : [ pageID : events ] ]
        ):

    pitchMarkov = PitchMarkov()
    pitchReverse = PitchReverse()
    pitchSort = PitchSort()
    pitchShuffle = PitchShuffle()

    makePitch = GenerationPitch()
    makeHarmonicSequence = Drunk.Drunk( 7 )
    rythmShuffle = RythmShuffle( )
    rythmReverse = RythmReverse( )

    def pageVariate(  parameters, trackID, pageID ):
        tempTrackNotes = []
        trackNotes = []
        for note in trackDictionary[ trackID ][ parameters.sourceVariation ]:
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

        del trackDictionary[ trackID ][ pageID ]
        trackDictionary[ trackID ][ pageID ] = trackNotes

        tempTrackNotes = []
        trackNotes = []
        for note in trackDictionary[ trackID ][ parameters.sourceVariation ]:
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

        del trackDictionary[ trackID ][ pageID ]
        trackDictionary[ trackID ][ pageID ] = trackNotes

    for trackID in trackIDs:
        for pageID in pageIDs:
            pageVariate( parameters, trackID, pageID )

    
