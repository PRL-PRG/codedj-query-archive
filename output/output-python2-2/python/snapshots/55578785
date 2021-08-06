import random
import Drunk

from Framework.Constants import Constants
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationPitch:
    def __init__( self ):
        fakeMaximum = 4
        self.drunk = Drunk.Drunk( fakeMaximum )
        self.droneAndJump = Drunk.DroneAndJump( fakeMaximum )           
        self.repeter = Drunk.Repeter( fakeMaximum )            
        self.loopseg = Drunk.Loopseg( fakeMaximum )

    def chooseMethod( self, pattern ):
        if pattern == 'Drunk': return self.drunk
        elif pattern == 'DroneAndJump': return self.droneAndJump         
        elif pattern == 'Repeter': return self.repeter           
        elif pattern == 'Loopseg': return self.loopseg      

    def drunkPitchSequence(self, length, parameters, table_pitch):
        pitchMethod = self.chooseMethod( parameters.pattern )
        pitchSequence = []
        for i in range(length):
            pitchSequence.append((table_pitch[pitchMethod.getNextValue(parameters.step, (len(table_pitch)-1))]) + GenerationConstants.DEFAULT_TONIQUE)
        return pitchSequence

    def drumPitchSequence(self, length, parameters, table_pitch=None):
        pitchSequence = []
        for i in range(length):
            pitchSequence.append(36 + random.choice( [ -5, 0, 0, 0, 0 ] ))         
        return pitchSequence  

    def harmonicPitchSequence( self, rythmSequence, parameters, table_pitch, harmonicSequence ):
        pitchSequence = []
        pitchMethod = self.chooseMethod( parameters.pattern )
        for onset in rythmSequence:
            beat = int( onset / Constants.TICKS_PER_BEAT )
            pitchSequence.append( ( table_pitch[ random.choice( harmonicSequence[ beat ] ) ] ) + GenerationConstants.DEFAULT_TONIQUE )
#            pitchSequence.append( ( table_pitch[ harmonicSequence[ beat ][ pitchMethod.getNextValue( 2, ( len( harmonicSequence[ beat ] ) - 1 ) ) ] ] ) + GenerationConstants.DEFAULT_TONIQUE )
        return pitchSequence


