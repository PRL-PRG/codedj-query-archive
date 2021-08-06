import random
import Drunk

import common.Config as Config
from common.Generation.GenerationConstants import GenerationConstants

class GenerationPitch:
    def __init__( self ):
        MIN = 0
        MAX = 14
        self.drunkMethod = Drunk.Drunk( MIN, MAX )
        self.droneMethod = Drunk.DroneAndJump( MIN, MAX )
        self.repeatMethod = Drunk.Repeter( MIN, MAX )
        self.loopMethod = Drunk.Loopseg( MIN, MAX )
        self.methodList = [self.drunkMethod, self.droneMethod, self.repeatMethod, self.loopMethod]

    def drunkPitchSequence(self, length, parameters, table_pitch, trackId):
        pitchSequence = []
        append = pitchSequence.append
        numberOfPitch = int( ( 1 - (parameters.pitchRegularity[trackId]*.8) )  * 10 + 1 )
        step = -(int(parameters.step[trackId] * 10))
        max = len(table_pitch)-1
        nextValue = self.methodList[parameters.pattern[trackId]].getNextValue
        tonique = GenerationConstants.DEFAULT_TONIQUE
        for i in range(numberOfPitch):
            append((table_pitch[nextValue(step, max)]) + tonique)
        restOfNotes = range( length - numberOfPitch )
        for i in restOfNotes:
            position = i % numberOfPitch
            append( pitchSequence[ position ] )
        return pitchSequence

    def drumPitchSequence(self, length, parameters, drumPitch, table_pitch=None):
        pitchSequence = []
        append = pitchSequence.append
        max = len(drumPitch) - 1
        rand = random.randint
        for i in range(length):
            append(drumPitch[ rand( 0, max ) ] )
        return pitchSequence
