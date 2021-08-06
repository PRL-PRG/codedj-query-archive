import random
import math

import Config
from Util.CSoundNote import CSoundNote
from Generation.GenerationConstants import GenerationConstants
from GenRythm import GenRythm

def generator( instrument, nbeats, regularity, reverbSend ):

    makeRythm = GenRythm()

    noteDuration = GenerationConstants.DOUBLE_TICK_DUR / 2
    trackId = 5
    pan = 0.5
    attack = 0.005
    decay = 0.095
    filterType = 0
    filterCutoff = 1000
    tied = False
    mode = 'mini'
 
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
                
    def pageGenerate( regularity, drumPitch ):
        barLength = Config.TICKS_PER_BEAT * nbeats

        if Config.INSTRUMENTS[instrument].kit != None:
            currentInstrument = Config.INSTRUMENTS[instrument].kit[drumPitch[0]].name

        rythmSequence = makeRythm.drumRythmSequence(currentInstrument, nbeats, regularity)
        pitchSequence = makePitchSequence(len(rythmSequence), drumPitch )
        gainSequence = makeGainSequence(rythmSequence)

        trackNotes = []
        for i in range(len(rythmSequence)):
            trackNotes.append( CSoundNote( rythmSequence[i], pitchSequence[i], gainSequence[i], 
                                           pan, noteDuration, trackId, 
                                           Config.INSTRUMENTS[instrument].instrumentId, attack, decay, reverbSend, filterType, filterCutoff, tied, mode ) )
        return trackNotes
################################################################################## 
    #  begin generate() 
    if regularity > 0.75:
        streamOfPitch = GenerationConstants.DRUM_COMPLEXITY1 
    elif regularity > 0.5:
        streamOfPitch = GenerationConstants.DRUM_COMPLEXITY2
    elif regularity > 0.25:
        streamOfPitch = GenerationConstants.DRUM_COMPLEXITY3
    else:
        streamOfPitch = GenerationConstants.DRUM_COMPLEXITY4

    trackNotes = []
    for drumPitch in streamOfPitch:
        trackNotes.append(pageGenerate( regularity, drumPitch ))
    return trackNotes
