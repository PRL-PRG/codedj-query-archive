class GenerationConstants:

    TWO_ROOT_TWELVE = pow( 2, 1./12 )
    MAX_NOTES_PER_BAR = 12

    # Default parameters for algorithmic generation
    DEFAULT_DENSITY = 0.7
    DEFAULT_RYTHM_REGULARITY = .75
    DEFAULT_STEP = -3
    DEFAULT_PITCH_REGULARITY = 0.5
    DEFAULT_ARTICULE = 0.7

    DEFAULT_RYTHM_METHOD = 0
    DEFAULT_PITCH_METHOD = 0
    DEFAULT_PAN = 0.5

    DEFAULT_PATTERN = 0

    DEFAULT_PITCH_VARIATION = 0
    DEFAULT_RYTHM_VARIATION = 0

    # Onset probability table for makeRythmSequence function ( remove 15 and 20 )
#    TABLE_ONSET_VALUES = [ 3, 3, 3, 3, 3, 3, 3, 3, 4, 4,
#                           4, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 12, 12, 12, 12,
#                           12, 12, 12, 18, 18, 18, 24, 24, 24, 36, 36, 48, 48 ]

    TABLE_ONSET_VALUES = [ 3, 4, 6, 8, 12, 18, 24, 36, 48 ]

    # scaling constants
    MAJOR_SCALE = 0
    HARMONIC_MINOR_SCALE = 1
    NATURAL_MINOR_SCALE =2
    PHRYGIEN_SCALE = 3
    PENTATONIC_SCALE = 4
    BLUES_SCALE = 5

                                                       
    SCALES = { MAJOR_SCALE : [ -12, -10, -8, -7, -5, -3, -1, 0, 2, 4, 5, 7, 9, 11, 12 ],    
                        HARMONIC_MINOR_SCALE : [ -12, -10, -9, -7, -5, -4, -1, 0, 2, 3, 5, 7, 8, 11, 12 ],
                        NATURAL_MINOR_SCALE : [ -12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12 ],
                         PENTATONIC_SCALE : [ -12, -10, -7, -5, -3, 0, 2, 5, 7, 9, 12 ],                           
                        BLUES_SCALE : [ -12, -10, -9, -8, -7, -5, -3, -2, 0, 2, 3, 4, 5, 7, 9, 10, 12 ],
                        PHRYGIEN_SCALE : [ -12, -11, -9, -7, -5, -4, -2, 0, 1, 3, 5, 7, 8, 10, 12 ] }

    DEFAULT_SCALE = MAJOR_SCALE

    DEFAULT_TONIQUE = 36

    I = [ 0, 2, 4, 7, 9, 11, 14 ]
    II = [ 1, 3, 5, 8, 10, 12 ]
    III = [ 2, 4, 6, 9, 11, 13 ]
    IV = [ 0, 3, 5, 7, 10, 12, 14 ]
    V = [ 1, 4, 6, 8, 11, 13 ]
    VI = [ 0, 2, 5, 7, 9, 12, 14 ]
    VII = [ 1, 3, 6, 8, 10, 13 ]

    CHORDS_TABLE = [ I, V, I, II, V, I, VI, II, V, I, IV, VI, II, V, I, V, VI ]
#    CHORDS_TABLE = [I, V, I, V, I, V, I, V, I, V, I, V, I, V ]
    # pitch patterns constants
 #   PITCH_PATTERNS = [ 'Drunk', 'DroneAndJump', 'Repeter', 'Loopseg' ]

    # Parameters for probability scaling function
    REPETITION_SCALE_MIN_MAPPING = 0
    REPETITION_SCALE_MAX_MAPPING = 25
    REPETITION_SCALE_STEPS = 25
    DENSITY_SCALE_MIN_MAPPING = 0
    DENSITY_SCALE_MAX_MAPPING = 38
    DENSITY_SCALE_STEPS = 38
    ARTICULATION_SCALE_MIN_MAPPING = .3
    ARTICULATION_SCALE_MAX_MAPPING = 1
    ARTICULATION_SCALE_STEPS = 30

    # Rythmic durations, in ticks, and how many to complete figure (celluleRythmSequence)
    DOUBLE_TICK_DUR = 3
    DOUBLE_HOW_MANY = 2
    HALF_TRIPLET_TICK_DUR = 4
    HALF_TRIPLET_HOW_MANY = 3
    HOLE_TRIPLET_TICK_DUR = 8
    HOLE_TRIPLET_HOW_MANY = 3

    # Random generators default values (xnoiseRythmSequence)
    RANDOM_BETA_PARAM = 0.004
    RANDOM_EXPO_PARAM = 5
    RANDOM_GAUSS_PARAM1 = 0.5
    RANDOM_GAUSS_PARAM2 = 0.1
    RANDOM_WEIBULL_PARAM1 = 0.5
    RANDOM_WEIBULL_PARAM2 = 2.5

    # Onsets probability tables (drumRythmSequence)

    PUNCH_ACCENTS = [ [],
                                            [ 0 ],
                                            [ 0, 1 ],
                                            [ 0, 2, 1 ],
                                            [ 0, 2, 3, 1 ],
                                            [ 0, 3, 2, 4, 1],
                                            [ 0, 3, 2, 5, 1, 4 ],
                                            [ 0, 2, 4, 6, 5, 3, 1 ],
                                            [ 0, 4, 2, 6, 3, 7, 5, 1 ] ]
 
    LOW_ACCENTS = [ [],
                                            [ 0 ],
                                            [ 0, 1 ],
                                            [ 0, 2, 1 ],
                                            [ 0, 2, 3, 1 ],
                                            [ 0, 3, 2, 4, 1],
                                            [ 0, 3, 2, 5, 1, 4 ],
                                            [ 0, 2, 4, 6, 5, 3, 1 ],
                                            [ 0, 4, 2, 6, 3, 7, 5, 1 ] ]
                                          
    MID_ACCENTS = [   [],
                                            [ 0, 1 ],
                                            [ 0, 2, 3, 1 ],
                                            [ 0, 2, 4, 3, 1, 5 ],    
                                            [ 0, 4, 6, 2, 7, 1, 3, 5 ],
                                            [ 0, 6, 4, 8, 2, 1, 5, 3, 9, 7 ],
                                            [ 0, 6, 11, 5, 3, 9, 10, 2, 8, 7, 1, 4 ],
                                            [ 0, 4, 8, 12, 10, 13, 11, 9, 3, 2, 6, 5, 7, 1 ],
                                            [ 0, 8, 4, 12, 6, 14, 2, 10, 7, 15, 1, 9, 3, 11, 5, 13 ]  ]

    HIGH_ACCENTS = [   [],
                                            [ 1, 0 ],
                                            [ 1, 3, 2, 0 ],
                                            [ 5, 1, 3, 4, 2, 0 ],    
                                            [ 5, 3, 1, 7, 2, 6, 4, 0 ],
                                            [ 7, 9, 3, 5, 1, 2, 8, 4, 6, 0 ],
                                            [ 4, 1, 7, 5, 3, 9, 10, 2, 8, 11, 6, 0 ],
                                            [ 1, 7, 8, 5, 10, 13, 11, 9, 3, 2, 6, 12, 4, 0 ],
                                            [ 13, 5, 11, 3, 9, 1, 15, 10, 7, 2, 14, 6, 12, 4, 8, 0 ]  ]

    # Gain boundaries
    GAIN_MAX_BOUNDARY = 1.
    GAIN_MID_MAX_BOUNDARY = .9
    GAIN_MID_MIN_BOUNDARY = .75
    GAIN_MIN_BOUNDARY = .65

    # pitch mapping for drum kit
    DRUMPITCH = {25: 24, 27: 26, 29: 28, 31: 30, 33: 32, 35: 34, 37: 36, 39: 38, 41: 40, 43: 42, 45: 44, 47: 46 }
