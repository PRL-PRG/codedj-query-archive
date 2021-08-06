import random
import math

import Utils
import Drunk

import common.Util.InstrumentDB as InstrumentDB
import common.Config as Config
from common.Util.CSoundNote import CSoundNote
from common.Generation.GenerationConstants import GenerationConstants
from common.Generation.GenerationRythm import GenerationRythm
from common.Generation.GenerationPitch import GenerationPitch

instrumentDB = InstrumentDB.getRef()

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
        instrument, # [pageId][trackId: instrument]
        tempo,      # integer bpm
        nbeats,     # map [ pageId : beats ]
        trackIds,   # list of trackIds to generate
        pageIds,    # list of pageIds to generate
        trackDictionary, # map [ trackId : [ pageId : events ] ]
        nPagesCycle = 4 # META ALGO number of pages in a section
        ):

    makeRythm = GenerationRythm()
    makePitch = GenerationPitch()

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
        if instrumentDB.instNamed[currentInstrument].name[0:4] == 'drum':
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

    def pageGenerate(parameters, trackId, pageId, trackOfNotes, drumPitch = None):

        trackNotes = trackOfNotes

        if drumPitch:
            currentInstrument = instrumentDB.instNamed[instrument[pageId][trackId]].kit[drumPitch[0]]
            rythmSequence = makeRythm.drumRythmSequence(parameters, currentInstrument, barLength)
            pitchSequence = makePitch.drumPitchSequence(len(rythmSequence), parameters, drumPitch, table_pitch )
        else:
            currentInstrument = instrument[pageId][trackId]
            rythmSequence = makeRythm.celluleRythmSequence(parameters, barLength, trackId, currentInstrument)
            pitchSequence = makePitch.drunkPitchSequence(len(rythmSequence),parameters, table_pitch, trackId)

        gainSequence = makeGainSequence(rythmSequence)
        durationSequence = makeDurationSequence(rythmSequence, parameters, table_duration, barLength, currentInstrument)

        numOfNotes = range(len(rythmSequence))
        rand = random.random
        append = trackNotes.append
        pan = GenerationConstants.DEFAULT_PAN
        instrument_id = instrumentDB.instNamed[instrument[pageId][trackId]].instrumentId
        for i in numOfNotes:
            if drumPitch:
                if ( rand() * fillDrum ) > ( parameters.silence[0] * .5 ):
                    if fillDrum != 1:
                        if rythmSequence[i] not in trackOnsets or pitchSequence[i] not in trackPitchs:
                            append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
                    else:
                        append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.098, 0.1, 0, 1000, False, 'edit' ) )
            else:
                if rand() > parameters.silence[trackId]:
                    append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], pan, durationSequence[i], trackId, instrument_id, 0.002, 0.1, 0.1, 0, 1000, False, 'edit' ) )

        trackDictionary[ trackId ][ pageId ] = trackNotes

##################################################################################
    #  begin generate()

    table_pitch = GenerationConstants.SCALES[parameters.scale]

    for trackId in trackIds:
        if trackId == 4: # drum index
            table_duration = Utils.scale(parameters.articule[0], GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, GenerationConstants.ARTICULATION_SCALE_STEPS)
            if parameters.rythmRegularity > 0.75:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY1
            elif parameters.rythmRegularity > 0.5:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY2
            elif parameters.rythmRegularity > 0.25:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY3
            else:
                streamOfPitch = GenerationConstants.DRUM_COMPLEXITY4
        else:
            table_duration = Utils.scale(parameters.articule[trackId], GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, GenerationConstants.ARTICULATION_SCALE_STEPS)

        selectedPageCount = 0
        lastPageId = 0
        for pageId in pageIds:
            barLength = Config.TICKS_PER_BEAT * nbeats[ pageId ]
            trackOfNotes = []
            pageCycle = selectedPageCount % nPagesCycle

            #if instrument[pageId][trackId][0:4] == 'drum':
            if trackId == 4:
                if pageCycle not in [0,nPagesCycle-1] and nbeats[pageId] == nbeats[lastPageId]:
                    trackDictionary[trackId][pageId] = []
                    for n in trackDictionary[trackId][lastPageId]:
                        trackDictionary[trackId][pageId].append(n.clone())
                elif pageCycle == (nPagesCycle-1) and nbeats[pageId] == nbeats[lastPageId]:
                    for n in trackDictionary[trackId][lastPageId]:
                        trackOfNotes.append(n.clone())
                    trackOnsets = [n.onset for n in trackOfNotes]
                    trackPitchs = [n.pitch for n in trackOfNotes]
                    fillDrum = .5
                    rythmRegTemp = parameters.rythmRegularity[0]
                    parameters.rythmRegularity[0] = 0.5
                    for drumPitch in GenerationConstants.DRUM_COMPLEXITY4:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )
                    parameters.rythmRegularity[0] = rythmRegTemp
                else:
                    fillDrum = 1
                    for drumPitch in streamOfPitch:
                        pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch )

            else:
                pageGenerate( parameters, trackId, pageId, trackOfNotes, drumPitch = None )
            selectedPageCount += 1
            lastPageId = pageId
