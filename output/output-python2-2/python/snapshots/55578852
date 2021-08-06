class GenerationConstants:

    TWO_ROOT_TWELVE = pow( 2, 1./12 )
    MAX_NOTES_PER_BAR = 24
    BAR_LENGTH = 480 ### Eventualy, should be Framework.Constants.TICKS_PER_BEAT * getBeatsPerPage ###

    # Default parameters for algorithmic generation
    DEFAULT_BAR = 1
    DEFAULT_DENSITY = 0.7
    DEFAULT_REPETE = .5
    DEFAULT_STEP = -3
    DEFAULT_ARTICULE = 0.7
    DEFAULT_PANNER = 0

    # Onset probability table for makeRythmSequence function 
    TABLE_ONSET_VALUES = [ 15, 20, 24, 30, 30, 30, 30, 30, 30, 30, 40, 40,
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

    # pitch patterns constants
    PITCH_PATTERNS = [ 'Drunk', 'DroneAndJump', 'Repeter', 'Loopseg' ]
    DEFAULT_PATTERN = 'Loopseg'

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
    LOW_DOWN = [ (0, 100), (240, 75), (360, 50), (120, 25) ]
    LOW_UP = [ (180, 100), (60, 75), (420, 50), (300, 25) ]
    MID_DOWN = [ (0, 100), (360, 88), (420, 76), (120, 64), (240, 52), (300, 40), (180, 28), (60, 16) ]
    MID_UP = [ (330, 100), (390, 88), (450, 76), (150, 64), (270, 52), (30, 40), (210, 28), (90, 16) ]
    HIGH_DOWN = [ (330, 100), (390, 88), (450, 76), (150, 64), (270, 52), (30, 40), (210, 28), (90, 16) ]
    HIGH_UP = [ (0, 100), (360, 88), (420, 76), (120, 64), (240, 52), (300, 40), (180, 28), (60, 16) ]
#    HIGH_DOWN = [ (15, 100), (375, 88), (435, 76), (135, 64), (255, 52), (315, 40), (195, 28), (75, 16) ]
#    HIGH_UP = [ (345, 100), (405, 88), (465, 76), (165, 64), (285, 52), (45, 40), (225, 28), (105, 16) ]

    # Gain boundaries
    GAIN_MAX_BOUNDARY = 1.
    GAIN_MID_MAX_BOUNDARY = .85
    GAIN_MID_MIN_BOUNDARY = .7
    GAIN_MIN_BOUNDARY = .5
