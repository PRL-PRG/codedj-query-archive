class GenerationConstants:

    TWO_ROOT_TWELVE = pow( 2, 1./12 )
    MAX_NOTES_PER_BAR = 32
    BAR_LENGTH = 480 ### Eventualy, should be Framework.Constants.TICKS_PER_BEAT * getBeatsPerPage ###

    # Default parameters for algorithmic generation
    DEFAULT_BAR = 1
    DEFAULT_DENSITY = 0.3
    DEFAULT_REPETE = .5
    DEFAULT_STEP = -3
    DEFAULT_ARTICULE = 0.7
    DEFAULT_PANNER = 0

    # Onset probability table for makeRythmSequence function 
    TABLE_ONSET_VALUES = [ 15, 15, 15, 15, 20, 24, 30, 30, 30, 30, 30, 30, 30, 40, 40,
                           40, 60, 60, 60, 60, 60, 60, 60, 80, 80, 80, 120, 120, 120, 120,
                           120, 120, 120, 180, 180, 180, 240, 240, 240, 360, 360, 480, 480 ]

    # Possible scales for picked pitches
    # TODO: change absolutes values with scale degrees
    MAJOR_SCALE = [ 24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48 ]
    HARMONIC_MINOR_SCALE = [ 24, 26, 27, 29, 31, 32, 35, 36, 38, 39, 41, 43, 44, 47, 48 ]
    NATURAL_MINOR_SCALE = [ 24, 26, 27, 29, 31, 32, 34, 36, 38, 39, 41, 43, 44, 46, 48 ]
    PENTATONIC_SCALE = [ 24, 26, 29, 31, 33,  36, 38, 41, 43, 45, 48 ]
    BLUES_SCALE = [ 24, 26, 27, 28, 29, 31, 33, 34, 36, 38, 39, 40, 41, 43, 45, 46, 48 ]
    PHRYGIEN_SCALE = [ 24, 25, 27, 29, 31, 32, 34, 36, 37, 39, 41, 43, 44, 46, 48 ]

    # Parameters for probability scaling function
    REPETITION_SCALE_MIN_MAPPING = 0
    REPETITION_SCALE_MAX_MAPPING = 25
    REPETITION_SCALE_STEPS = 25
    DENSITY_SCALE_MIN_MAPPING = 0
    DENSITY_SCALE_MAX_MAPPING = 42
    DENSITY_SCALE_STEPS = 42
    ARTICULATION_SCALE_MIN_MAPPING = .3
    ARTICULATION_SCALE_MAX_MAPPING = 1
    ARTICULATION_SCALE_STEPS = 30
    PAN_SCALE_MIN_MAPPING = .5
    PAN_SCALE_MAX_MAPPING = 1
    PAN_SCALE_STEPS = 100

    # Rythmic durations, in ticks, and how many to complete figure (makeRythmSequence)
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

    # Random generators defaults values (makeRythmSequence2)
    RANDOM_BETA_PARAM = 0.004
    RANDOM_EXPO_PARAM = 5
    RANDOM_GAUSS_PARAM1 = 0.5
    RANDOM_GAUSS_PARAM2 = 0.1
    RANDOM_WEIBULL_PARAM1 = 0.5
    RANDOM_WEIBULL_PARAM2 = 2.5

    # Gain boundaries
    GAIN_MAX_BOUNDARY = 1.
    GAIN_MID_MAX_BOUNDARY = .85
    GAIN_MID_MIN_BOUNDARY = .7
    GAIN_MIN_BOUNDARY = .5
