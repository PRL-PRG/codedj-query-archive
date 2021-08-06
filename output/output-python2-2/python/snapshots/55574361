import random
import math

import Config
from Util.CSoundNote import CSoundNote
from Generation.GenerationConstants import GenerationConstants
from GenRythm import GenRythm

def generator( instrument, nbeats, regularity, reverbSend ):

    def makePitchSequence(length, drumPitch):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(drumPitch[ random.randint( 0, ( len( drumPitch ) - 1 )  ) ] )         
        return pitchSequence 

    def makeGainSequence( onsetList ):
        gainSequence = []
        for onset in onsetList:
            if onset == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MAX_BOUNDARY, GenerationConstants.GAIN_MAX_BOUNDARY)
            elif ( onset % Config.TICKS_PER_BEAT) == 0:
                gain = random.uniform(GenerationConstants.GAIN_MID_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MAX_BOUNDARY)
            else:     
                gain = random.uniform(GenerationConstants.GAIN_MIN_BOUNDARY, GenerationConstants.GAIN_MID_MIN_BOUNDARY)
            gainSequence.append(gain*2)
        return gainSequence  
                
    def makeDurationSequence( onsetList ):
        durationSequence = []
        if len( onsetList ) > 1:
            for i in range(len(onsetList)):
                duration = GenerationConstants.DOUBLE_TICK_DUR / 2
                durationSequence.append(duration)      
        elif len( onsetList ) == 1:
            durationSequence.append( GenerationConstants.DOUBLE_TICK_DUR / 2 )
        return durationSequence

    def pageGenerate( regularity, drumPitch ):
        barLength = Config.TICKS_PER_BEAT * nbeats

        if Config.INSTRUMENTS[instrument].kit != None:
            currentInstrument = Config.INSTRUMENTS[instrument].kit[drumPitch[0]].name

        makeRythm = GenRythm( currentInstrument, barLength, nbeats )

        rythmSequence = makeRythm.drumRythmSequence(regularity)
        pitchSequence = makePitchSequence(len(rythmSequence), drumPitch )
        gainSequence = makeGainSequence(rythmSequence)
        durationSequence = makeDurationSequence(rythmSequence)

        trackId = 5
        pan = 0.5
        attack = 0.005
        decay = 0.095
        filterType = 0
        filterCutoff = 1000
        tied = False
        mode = 'mini'
        trackNotes = []
        for i in range(len(rythmSequence)):
            trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], 
                                           pan, durationSequence[i], trackId, 
                                           Config.INSTRUMENTS[instrument].instrumentId, attack, decay, reverbSend, filterType, filterCutoff, tied, mode ) )
        return trackNotes
################################################################################## 
    #  begin generate() 
    if regularity > 0.75:
        pitchOfStream = [ [ 24 ], [30] , [ 40 ], [ 46 ]  ]
    elif regularity > 0.5:
        pitchOfStream = [ [ 24, 28 ], [ 30, 32 ], [ 36, 38, 40 ], [ 46, 48 ]  ]
    elif regularity > 0.25:
        pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 46, 48 ]  ] 
    else:
        pitchOfStream = [ [ 24, 26, 28 ], [ 30, 32, 34 ], [ 38, 40 ], [ 42, 44, 46, 48 ]  ] 

    trackNotes = []
    for drumPitch in pitchOfStream:
        trackNotes.append(pageGenerate( regularity, drumPitch ))
    return trackNotes
