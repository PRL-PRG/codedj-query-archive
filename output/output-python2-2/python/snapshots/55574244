import random
import Drunk

import Config
from Generation.GenerationConstants import GenerationConstants

class GenerationPitch:
    def __init__( self ):
        MIN = 5
        MAX = 10
        self.drunk = Drunk.Drunk( MIN, MAX )
        self.droneAndJump = Drunk.DroneAndJump( MIN, MAX )           
        self.repeter = Drunk.Repeter( MIN, MAX )            
        self.loopseg = Drunk.Loopseg( MIN, MAX )

#        self.harmonicDrunk = Drunk.Drunk( MIN, MAX )
#        self.harmonicDroneAndJump = Drunk.DroneAndJump( MIN, MAX )           
#        self.harmonicRepeter = Drunk.Repeter( MIN, MAX )            
#        self.harmonicLoopseg = Drunk.Loopseg( MIN, MAX )

    def chooseMethod( self, pattern ):
        if pattern == 0: return self.drunk
        elif pattern == 1: return self.droneAndJump         
        elif pattern == 2: return self.repeter           
        elif pattern == 3: return self.loopseg    

#    def harmonicChooseMethod( self, pattern ):
#        if pattern == 0: return self.harmonicDrunk
#        elif pattern == 1: return self.harmonicDroneAndJump         
#        elif pattern == 2: return self.harmonicRepeter           
#        elif pattern == 3: return self.harmonicLoopseg   

    def drunkPitchSequence(self, length, parameters, table_pitch):
        self.pitchMethod = self.chooseMethod( parameters.pattern )
        pitchSequence = []
        numberOfPitch = int( ( 1 - (parameters.pitchRegularity*.8) )  * 10 + 1 )
        for i in range(numberOfPitch):
            pitchSequence.append((table_pitch[self.pitchMethod.getNextValue(-(8 - (int(parameters.step * 8))), (len(table_pitch)-1))]) + GenerationConstants.DEFAULT_TONIQUE)
        for i in range( length - numberOfPitch ):
            position = i % numberOfPitch
            pitchSequence.append( pitchSequence[ position ] )
        return pitchSequence

    def drumPitchSequence(self, length, parameters, drumPitch, table_pitch=None):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(drumPitch[ random.randint( 0, ( len( drumPitch ) - 1 )  ) ] )         
        return pitchSequence  

#    def harmonicPitchSequence( self, rythmSequence, parameters, table_pitch, harmonicSequence ):
#        pitchSequence = []
#        pitchMethod = self.harmonicChooseMethod( parameters.pattern )
#        for onset in rythmSequence:
#            beat = int( onset / Config.TICKS_PER_BEAT )
#            pitchSequence.append( ( table_pitch[ harmonicSequence[ beat ] [ pitchMethod.getNextValue(3, ( len( harmonicSequence[ beat ]) - 1) ) ]] ) + GenerationConstants.DEFAULT_TONIQUE )
#        return pitchSequence


