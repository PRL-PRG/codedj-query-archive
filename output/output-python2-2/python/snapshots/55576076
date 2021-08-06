import random
import Drunk

import Config
from Generation.GenerationConstants import GenerationConstants

class GenerationPitch:
    def __init__( self ):
        fakeMaximum = 4
        self.drunk = Drunk.Drunk( fakeMaximum )
        self.droneAndJump = Drunk.DroneAndJump( fakeMaximum )           
        self.repeter = Drunk.Repeter( fakeMaximum )            
        self.loopseg = Drunk.Loopseg( fakeMaximum )

        self.harmonicDrunk = Drunk.Drunk( fakeMaximum )
        self.harmonicDroneAndJump = Drunk.DroneAndJump( fakeMaximum )           
        self.harmonicRepeter = Drunk.Repeter( fakeMaximum )            
        self.harmonicLoopseg = Drunk.Loopseg( fakeMaximum )

    def chooseMethod( self, pattern ):
        if pattern == 0: return self.drunk
        elif pattern == 1: return self.droneAndJump         
        elif pattern == 2: return self.repeter           
        elif pattern == 3: return self.loopseg    

    def harmonicChooseMethod( self, pattern ):
        if pattern == 0: return self.harmonicDrunk
        elif pattern == 1: return self.harmonicDroneAndJump         
        elif pattern == 2: return self.harmonicRepeter           
        elif pattern == 3: return self.harmonicLoopseg   

    def drunkPitchSequence(self, length, parameters, table_pitch):
        pitchMethod = self.chooseMethod( parameters.pattern )
        pitchSequence = []
        numberOfPitch = int( ( 1 - parameters.pitchRegularity )  * 10 + 1 )
        for i in range(numberOfPitch):
            pitchSequence.append((table_pitch[pitchMethod.getNextValue(10 - (int(parameters.step * 10)), (len(table_pitch)-1))]) + GenerationConstants.DEFAULT_TONIQUE)
        for i in range( length - numberOfPitch ):
            position = i % numberOfPitch
            pitchSequence.append( pitchSequence[ position ] )
        return pitchSequence

    def drumPitchSequence(self, length, parameters, drumPitch, table_pitch=None):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(drumPitch[ random.randint( 0, ( len( drumPitch ) - 1 )  ) ] )         
        return pitchSequence  

    def harmonicPitchSequence( self, rythmSequence, parameters, table_pitch, harmonicSequence ):
        pitchSequence = []
        pitchMethod = self.harmonicChooseMethod( parameters.pattern )
        for onset in rythmSequence:
            beat = int( onset / Config.TICKS_PER_BEAT )
            pitchSequence.append( ( table_pitch[ harmonicSequence[ beat ] [ pitchMethod.getNextValue(3, ( len( harmonicSequence[ beat ]) - 1) ) ]] ) + GenerationConstants.DEFAULT_TONIQUE )
 #           pitchSequence.append( ( table_pitch[ random.choice( harmonicSequence[ beat ] ) ] ) + GenerationConstants.DEFAULT_TONIQUE )
        return pitchSequence


