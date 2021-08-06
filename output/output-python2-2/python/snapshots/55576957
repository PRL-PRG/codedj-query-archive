class SynthLabConstants:

    PIC_SIZE = 60
    HALF_SIZE = PIC_SIZE / 2 

    INIT_LOCATIONS = [ [90,760], [150,760], [210,760], [270, 760], [450,760], [510,760], 
                       [570,760], [630, 760], [810,760], [870,760], [930,760], [990, 760], [540, 660]]

    FLOAT = [.01, 2]
    INTEGER = [1, 0]

    # s1 s2 s3 s4 s1min s1max s2min s2max s3min s3max [s1step s1digits] [s2step s2digits] [s3step s3digits]
    TYPES = {   'lfo': [.5, 1, 0, 0, 0, 1, 0, 20, 0, 5, FLOAT, FLOAT, INTEGER],
                'rand': [.5, 1.5, 2, 0, 0, 2, 0, 2, 0, 20, FLOAT, FLOAT, FLOAT],
                'adsr': [.02, .05, .8, .1, 0, 1, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'fm': [1, .5, 5, 1, 0, 2, 0, 2, 0, 10, FLOAT, FLOAT, FLOAT],
                'buzz': [1, 30, .85, 1, 0, 2, 0, 40, 0, 1, FLOAT, INTEGER, FLOAT],
                'vco': [1, 1, .2, 1, 0, 2, 0, 2, 0, .5, FLOAT, INTEGER, FLOAT],
                'pluck': [1, 5000, 0, 1, 0, 2, 100, 8000, 0, 8, FLOAT, INTEGER, FLOAT],
                'noise': [0, 3000, 4000, 1, 0, 2, 0, 8000, 0, 6000, INTEGER, FLOAT, FLOAT],
                'sample': [1, 5, 5000, 1, 0, 2, 0, 85, 100, 8000, FLOAT, INTEGER, FLOAT],
                'voice': [1, 3, 5, 1, 0, 2, 0, 15, 1, 10, FLOAT, INTEGER, FLOAT],
                'wguide': [100, 3000, .8, 1, 0, 200, 100, 5000, 0, 1, FLOAT, FLOAT, FLOAT],
                'distort': [800, .7, .7, 1, 0, 1000, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'filter': [1000, .6, 0, 1, 200, 5000, 0, 1, 0, 2, FLOAT, FLOAT, INTEGER],
                'ring': [500, 1, 0, 1, 0, 1000, 0, 1, 0, 5, FLOAT, FLOAT, INTEGER],
                'reverb': [1.5, 3000, .5, 1, 0, 4, 100, 7000, 0, 1, FLOAT, FLOAT, FLOAT],
                'harmon': [1.25, .04, .5, 1, 0, 2, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT]}

    CONTROL_TYPES = ['lfo', 'rand', 'adsr']
    CONTROL_TYPES_SEL = ['lfosel', 'randsel', 'adsrsel']
    CONTROL_TYPES_PLUS = ['lfo+', 'rand+', 'adsr+']
    SOURCE_TYPES = ['fm', 'buzz', 'vco', 'pluck', 'noise', 'sample', 'voice']
    SOURCE_TYPES_SEL = ['fmsel', 'buzzsel', 'vcosel', 'plucksel', 'noisesel', 'samplesel', 'voicesel']
    SOURCE_TYPES_PLUS = ['fm+', 'buzz+', 'vco+', 'pluck+', 'noise+', 'sample+', 'voice+']
    FX_TYPES = ['wguide', 'distort','filter', 'ring', 'reverb', 'harmon']
    FX_TYPES_SEL = ['wguidesel', 'distortsel','filtersel', 'ringsel', 'reverbsel', 'harmonsel']
    FX_TYPES_PLUS = ['wguide+', 'distort+','filter+', 'ring+', 'reverb+', 'harmon+']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES]
    CHOOSE_TYPE2 = [CONTROL_TYPES_SEL, SOURCE_TYPES_SEL, FX_TYPES_SEL]
    CHOOSE_TYPE_PLUS = [CONTROL_TYPES_PLUS, SOURCE_TYPES_PLUS, FX_TYPES_PLUS]

    PRESET = ['docu1', 'docu2', 'docu3', 'docu4', 'docu5', 'docu6', 'docu7', 'docu8', 'docu9', 'docu10']
