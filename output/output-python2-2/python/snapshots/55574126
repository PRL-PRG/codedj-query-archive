import random
import Drunk

import Config
from Generation.GenerationConstants import GenerationConstants

class GenerationPitch:
    def __init__( self, pattern ):
        MIN = 5
        MAX = 10
        if pattern == 0:
            self.pitchMethod = Drunk.Drunk( MIN, MAX )
        elif pattern == 1:
            self.pitchMethod = Drunk.DroneAndJump( MIN, MAX )
        elif pattern == 2:
            self.pitchMethod = Drunk.Repeter( MIN, MAX )
        elif pattern == 3:
            self.pitchMethod = Drunk.Loopseg( MIN, MAX )

#        self.harmonicDrunk = Drunk.Drunk( MIN, MAX )
#        self.harmonicDroneAndJump = Drunk.DroneAndJump( MIN, MAX )           
#        self.harmonicRepeter = Drunk.Repeter( MIN, MAX )            
#        self.harmonicLoopseg = Drunk.Loopseg( MIN, MAX )

#    def harmonicChooseMethod( self, pattern ):
#        if pattern == 0: return self.harmonicDrunk
#        elif pattern == 1: return self.harmonicDroneAndJump         
#        elif pattern == 2: return self.harmonicRepeter           
#        elif pattern == 3: return self.harmonicLoopseg   

    def drunkPitchSequence(self, length, parameters, table_pitch):
        pitchSequence = []
        append = pitchSequence.append
        numberOfPitch = int( ( 1 - (parameters.pitchRegularity*.8) )  * 10 + 1 )
        step = -(8 - (int(parameters.step * 8)))
        max = len(table_pitch)-1
        nextValue = self.pitchMethod.getNextValue
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

#    def harmonicPitchSequence( self, rythmSequence, parameters, table_pitch, harmonicSequence ):
#        pitchSequence = []
#        pitchMethod = self.harmonicChooseMethod( parameters.pattern )
#        for onset in rythmSequence:
#            beat = int( onset / Config.TICKS_PER_BEAT )
#            pitchSequence.append( ( table_pitch[ harmonicSequence[ beat ] [ pitchMethod.getNextValue(3, ( len( harmonicSequence[ beat ]) - 1) ) ]] ) + GenerationConstants.DEFAULT_TONIQUE )
#        return pitchSequence


