class GenerationConstants:

    TWO_ROOT_TWELVE = pow( 2, 1./12 )
    MAX_NOTES_PER_BAR = 24

    # Default parameters for algorithmic generation
    DEFAULT_DENSITY = 0.7
    DEFAULT_REPETE = .5
    DEFAULT_STEP = -3
    DEFAULT_ARTICULE = 0.7

    DEFAULT_PAN = 0.5

    # Onset probability table for makeRythmSequence function ( remove 15 and 20 )
    TABLE_ONSET_VALUES = [ 30, 24, 30, 30, 30, 30, 30, 30, 30, 40, 40,
                           40, 60, 60, 60, 60, 60, 60, 60, 80, 80, 80, 120, 120, 120, 120,
                           120, 120, 120, 180, 180, 180, 240, 240, 240, 360, 360, 480, 480 ]

    # scaling constants
    MAJOR_SCALE = 'major'
    HARMONIC_MINOR_SCALE = 'harmonic minor'
    NATURAL_MINOR_SCALE = 'natural minor'
    PENTATONIC_SCALE = 'pentatonic'
    BLUES_SCALE = 'blues'
    PHRYGIEN_SCALE = 'phrygien'
                                                       
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

    # pitch patterns constants
    PITCH_PATTERNS = [ 'Drunk', 'DroneAndJump', 'Repeter', 'Loopseg' ]
    DEFAULT_PATTERN = 'Loopseg'

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
    TRIPLE_TICK_DUR = 15
    TRIPLE_HOW_MANY = 4
    TRIPLE_TRIPLET_TICK_DUR = 20
    TRIPLE_TRIPLET_HOW_MANY = 3
    DOUBLE_QUINTUPLETS_TICK_DUR = 24
    DOUBLE_QUINTUPLETS_HOW_MANY = 5
    DOUBLE_TICK_DUR = 30
    DOUBLE_HOW_MANY = 2
    HALF_TRIPLET_TICK_DUR = 40
    HALF_TRIPLET_HOW_MANY = 3
    HOLE_TRIPLET_TICK_DUR = 80
    HOLE_TRIPLET_HOW_MANY = 3

    # Random generators default values (xnoiseRythmSequence)
    RANDOM_BETA_PARAM = 0.004
    RANDOM_EXPO_PARAM = 5
    RANDOM_GAUSS_PARAM1 = 0.5
    RANDOM_GAUSS_PARAM2 = 0.1
    RANDOM_WEIBULL_PARAM1 = 0.5
    RANDOM_WEIBULL_PARAM2 = 2.5

    # Onsets probability tables (drumRythmSequence)

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

    # Gain boundaries
    GAIN_MAX_BOUNDARY = 1.
    GAIN_MID_MAX_BOUNDARY = .85
    GAIN_MID_MIN_BOUNDARY = .7
    GAIN_MIN_BOUNDARY = .5
