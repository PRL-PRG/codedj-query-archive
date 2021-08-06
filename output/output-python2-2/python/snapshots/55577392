class SynthLabConstants:

    PIC_SIZE = 60
    HALF_SIZE = PIC_SIZE / 2 

    INIT_LOCATIONS = [ [100,790], [160,790], [220,790], [280, 790], [510,790], [570,790], 
                       [630,790], [690, 790], [920,790], [980,790], [1040,790], [1100, 790], [600, 670]]

    FLOAT = [.01, 2]
    INTEGER = [1, 0]

    # s1 s2 s3 s4 s1min s1max s2min s2max s3min s3max [s1step s1digits] [s2step s2digits] [s3step s3digits]
    TYPES = {   'LFO': [.5, 1, 0, 1, 0, 1, 0, 20, 0, 5, FLOAT, FLOAT, INTEGER],
                'random': [.5, 1.5, 2, 1, 0, 2, 0, 2, 0, 20, FLOAT, FLOAT, FLOAT],
                'ADSR': [.02, .05, .8, .1, 0, 1, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'FM': [1, .5, 5, 1, 0, 2, 0, 2, 0, 10, FLOAT, FLOAT, FLOAT],
                'buzz': [1, 30, .85, 1, 0, 2, 0, 40, 0, 1, FLOAT, INTEGER, FLOAT],
                'vco': [1, 1, .2, 1, 0, 2, 0, 2, 0, .5, FLOAT, INTEGER, FLOAT],
                'pluck': [1, 0, 0, 1, 0, 2, 0, 0, 0, 0, FLOAT, INTEGER, INTEGER],
                'noise': [1, 0, 0, 1, 0, 2, 0, 0, 0, 0, INTEGER, INTEGER, INTEGER],
                'samples': [1, 5, 0, 1, 0, 2, 0, 75, 0, 0, FLOAT, INTEGER, INTEGER],
                'voice': [1, 3, 0, 1, 0, 2, 0, 15, 0, 0, FLOAT, INTEGER, INTEGER],
                'wguide': [100, 3000, .8, 1, 0, 200, 100, 5000, 0, 1, FLOAT, FLOAT, FLOAT],
                'distortion': [800, .7, .7, 1, 0, 1000, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'filter': [1000, .6, 0, 1, 200, 5000, 0, 1, 0, 2, FLOAT, FLOAT, INTEGER],
                'ringMod': [500, 1, 0, 1, 0, 1000, 0, 1, 0, 0, FLOAT, FLOAT, INTEGER],
                'reverb': [.8, .8, .5, 1, 0, 1, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'harmon': [1.25, .5, 0, 1, 0, 2, 0, 1, 0, 0, FLOAT, FLOAT, INTEGER]}

    CONTROL_TYPES = ['LFO', 'random', 'ADSR']
    SOURCE_TYPES = ['FM', 'buzz', 'vco', 'pluck', 'noise', 'samples', 'voice']
    FX_TYPES = ['wguide', 'distortion','filter', 'ringMod', 'reverb', 'harmon']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES]
