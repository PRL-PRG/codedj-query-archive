import Utils
import random
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Constants import Constants

class GenerationRythm:
    def __init__( self, trackInstrument, barLength ):
        self.trackInstrument = trackInstrument
        self.barLength = barLength

    def celluleRythmSequence(self, parameters, table_onset, table_repetition ):
        rythmSequence = [0, ]
        self.count = 0
        lastOnsetTime = 0
        onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(table_onset))]

        for i in range(int(parameters.bar) * 32):
            if self.count == 0:   
                repetitionFlag = Utils.prob2(table_repetition)
                if repetitionFlag != 0:
                    onsetDelta = GenerationConstants.TABLE_ONSET_VALUES[int(Utils.prob2(table_onset))]

            self.makeCellule(onsetDelta, GenerationConstants.TRIPLE_TICK_DUR, GenerationConstants.TRIPLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.TRIPLE_TRIPLET_TICK_DUR, GenerationConstants.TRIPLE_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_QUINTUPLETS_TICK_DUR, 
                                                                 GenerationConstants.DOUBLE_QUINTUPLETS_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.DOUBLE_TICK_DUR, GenerationConstants.DOUBLE_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HALF_TRIPLET_TICK_DUR, GenerationConstants.HALF_TRIPLET_HOW_MANY)
            self.makeCellule(onsetDelta, GenerationConstants.HOLE_TRIPLET_TICK_DUR, GenerationConstants.HOLE_TRIPLET_HOW_MANY)

            onsetTime = onsetDelta + lastOnsetTime 
            lastOnsetTime = onsetTime
            
            if onsetTime < ( self.barLength * parameters.bar):
                rythmSequence.append(onsetTime)
            else:
                break                
        return rythmSequence

    def xnoiseRythmSequence(self, parameters, data1= None, data2 =None ):
        rythmSequence = []
        onsetTime = None
        randomParamScaler = parameters.repete * 2 + 0.5
        whichRandomGenerator = random.randint(0, 4)
        maximumNumberOfNotes = int( (parameters.density) * GenerationConstants.MAX_NOTES_PER_BAR)
#        tempDict = {0:'expo_min', 1:'expo_max', 2:'gauss', 3:'beta', 4:'weibull'}
#        print tempDict[whichRandomGenerator]
 
        for i in range(maximumNumberOfNotes):
            while onsetTime in rythmSequence:
                if whichRandomGenerator == 0:
                    onsetTime = random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM * randomParamScaler)
                elif whichRandomGenerator == 1:
                    onsetTime = 1 - random.expovariate(GenerationConstants.RANDOM_EXPO_PARAM * randomParamScaler)
                elif whichRandomGenerator == 2:
                    onsetTime = random.gauss(GenerationConstants.RANDOM_GAUSS_PARAM1, 
                                                            GenerationConstants.RANDOM_GAUSS_PARAM2 * (3 - randomParamScaler))
                elif whichRandomGenerator == 3:
                    onsetTime = random.betavariate(GenerationConstants.RANDOM_BETA_PARAM * randomParamScaler,           
                                                                    GenerationConstants.RANDOM_BETA_PARAM * randomParamScaler)
                elif whichRandomGenerator == 4:
                    onsetTime = random.weibullvariate(GenerationConstants.RANDOM_WEIBULL_PARAM1,                                                                        
                                                                          GenerationConstants.RANDOM_WEIBULL_PARAM2 * randomParamScaler)

                onsetTime = int(onsetTime * (int(( self.barLength - 1) / GenerationConstants.TRIPLE_TICK_DUR))) * GenerationConstants.TRIPLE_TICK_DUR

            if onsetTime < 0:
                onsetTime = 0
            elif onsetTime > ( self.barLength - GenerationConstants.TRIPLE_TICK_DUR):
                onsetTime = ( self.barLength - GenerationConstants.TRIPLE_TICK_DUR)
            else:
                onsetTime = onsetTime

            rythmSequence.append(onsetTime)

        rythmSequence.sort()
        return rythmSequence  

    def drumRythmSequence(self, parameters, data1=None, data2=None ):
        rythmSequence = []
        binSelection = []
        countDown = 0
        onsetTime = None
        beatsPerPage = self.barLength / Constants.TICKS_PER_BEAT    

        if CSoundConstants.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == CSoundConstants.LOW:
            DownBeatRecurence = 4
            if beatsPerPage == 3:
                tableDown = GenerationConstants.LOW_DOWN_3
                tableUp = GenerationConstants.LOW_UP_3
            elif beatsPerPage == 4:
                tableDown = GenerationConstants.LOW_DOWN_4
                tableUp = GenerationConstants.LOW_UP_4
            elif beatsPerPage == 5:
                tableDown = GenerationConstants.LOW_DOWN_5
                tableUp = GenerationConstants.LOW_UP_5
        elif CSoundConstants.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == CSoundConstants.MID: 
            DownBeatRecurence = 1
            if beatsPerPage == 3:
                tableDown = GenerationConstants.MID_DOWN_3
                tableUp = GenerationConstants.MID_UP_3
            elif beatsPerPage == 4:
                tableDown = GenerationConstants.MID_DOWN_4
                tableUp = GenerationConstants.MID_UP_4
            elif beatsPerPage == 5:
                tableDown = GenerationConstants.MID_DOWN_5
                tableUp = GenerationConstants.MID_UP_5
        elif CSoundConstants.INSTRUMENTS[ self.trackInstrument ].instrumentRegister == CSoundConstants.HIGH:
            DownBeatRecurence = 1
            if beatsPerPage == 3:
                tableDown = GenerationConstants.HIGH_DOWN_3
                tableUp = GenerationConstants.HIGH_UP_3
            elif beatsPerPage == 4:
                tableDown = GenerationConstants.HIGH_DOWN_4
                tableUp = GenerationConstants.HIGH_UP_4
            elif beatsPerPage == 5:
                tableDown = GenerationConstants.HIGH_DOWN_5
                tableUp = GenerationConstants.HIGH_UP_5
        for i in range( int( parameters.density * len( tableDown ) ) ):
            if random.randint( 0, 100 ) < parameters.repete * 100 * DownBeatRecurence: binSelection.append( 1 )        
            else: binSelection.append( 0 )

        for i in binSelection:
            if i == 1 : countDown += 1

        for i in range( countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = Utils.prob2( tableDown )
            rythmSequence.append( onsetTime )

        for i in range( len( binSelection ) - countDown ):
            while onsetTime in rythmSequence or onsetTime == None:
                onsetTime = Utils.prob2( tableUp )
            rythmSequence.append( onsetTime )

        rythmSequence.sort()
        return rythmSequence

    def makeCellule( self, currentDuration, targetDuration, threshold ):
        threshold = threshold - 1
        if currentDuration == targetDuration:
            if self.count < threshold:
                self.count += 1
            else:
                self.count = 0  
