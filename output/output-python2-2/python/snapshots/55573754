class SynthLabConstants:

    PIC_SIZE = 80
    HALF_SIZE = PIC_SIZE // 2 

    GT_CONTROL_OUTPUT = 0
    GT_CONTROL_INPUT = 1
    GT_SOUND_OUTPUT = 2
    GT_SOUND_INPUT = 3
    # GATE_POINT[ojecttype][gatetype][gatenum] = (x,y)
    # relative to object center
    GATE_POINT = [ [ [ (0,34) ] ], 
                   [ [], [ (-25,-35),(-9,-35),(8,-35),(25,-35) ], [ (0,35) ] ],
                   [ [], [ (35,-20),(35,-7),(35,7),(35,20) ], [ (0,34) ], [ (0,-34) ] ],
                   [ [], [], [], [ (0,-35) ] ] ]
    # GATE_MAP[objecttype][gatetype][gatenum] = [ sx, sy, ex, ey, (wireX,wireY) ]
    # gate locations relative to object center
    GATE_MAP = [ [ [ [-6,28,6,40] ] ],
                 [ [], [[-31,-40,-18,-28], [-16,-40,-3,-28], [2,-40,15,-28], [19,-40,32,-28]], [[-6,28,7,40]] ],
                 [ [], [[28,-26,40,-13], [28,-13,40,0], [28,0,40,13], [28,13,40,26]], [[-6,28,7,40]], [[-6,-40,7,-28]] ], 
                 [ [], [], [], [[-6,-40,7,-28]] ] ]
    # insert wire locations into map
    GATE_OFFSET = 7
    for oT in GATE_MAP:
        for gT in oT:
            for m in gT:
                x = (m[2]+m[0])//2
                y = (m[3]+m[1])//2
                # snap to edges
                if x < -HALF_SIZE+GATE_OFFSET: x = -HALF_SIZE
                elif x > HALF_SIZE-GATE_OFFSET: x = HALF_SIZE
                if y < -HALF_SIZE+GATE_OFFSET: y = -HALF_SIZE
                elif y > HALF_SIZE-GATE_OFFSET: y = HALF_SIZE
                m.append( ( x, y ) )

    INIT_LOCATIONS = [ [420,750], [500,750], [580,750], [660, 750], [55,750], [135,750], [215,750], [295, 750], [785,750], [865,750], [945,750], [1025, 750], [540, 645]]

    FLOAT = [.01, False]
    INTEGER = [1, 1]

    # s1 s2 s3 s4 s1min s1max s2min s2max s3min s3max s4min s4max [s1step s1digits] [s2step s2digits] [s3step s3digits]
    TYPES = {   'lfo': [.5, 1, 0, 0, 0, 1, 0, 20, 0, 5, 0, 1,  FLOAT, FLOAT, INTEGER],
                'rand': [.5, 1.5, 2, 0, 0, 2, 0, 2, 0, 20, 0, 1, FLOAT, FLOAT, FLOAT],
                'adsr': [.02, .05, .8, .1, 0, 1, 0, 1, 0, 1, 0, 1, FLOAT, FLOAT, FLOAT],
                'trackpadX': [0, 1, 0, 0, -1, 1, 0, 2, 0, 1, 0, 1, FLOAT, FLOAT, INTEGER],
                'trackpadY': [0, 1, 0, 0, -1, 1, 0, 2, 0, 1, 0, 1, FLOAT, FLOAT, INTEGER],
                'fm': [1, .5, 5, 1, 0, 2, 0, 2, 0, 10, 0, 2, FLOAT, FLOAT, FLOAT],
                'buzz': [1, 30, .85, 1, 0, 2, 0, 40, 0, 1, 0, 2, FLOAT, INTEGER, FLOAT],
                'vco': [1, 1, .2, 1, 0, 2, 0, 2, 0, .5, 0, 2, FLOAT, INTEGER, FLOAT],
                'pluck': [1, 5000, 0, 1, 0, 2, 100, 8000, 0, 8, 0, 2, FLOAT, INTEGER, FLOAT],
                'noise': [0, 3000, 4000, 1, 0, 2, 0, 8000, 0, 6000, 0, 2, INTEGER, FLOAT, FLOAT],
                'sample': [1, 5, 5000, 1, 0, 2, 0, 99, 100, 8000, 0, 2, FLOAT, INTEGER, FLOAT],
                'voice': [1, 3, 5, 1, 0, 2, 0, 8, 1, 10, 0, 2, FLOAT, INTEGER, FLOAT],
                'grain': [1, 4, 1, 1, 0, 2, 0, 99, 0, 1, 0, 2, FLOAT, INTEGER, FLOAT],
                'addSynth': [1, .005, 5, 1, 0, 2, 0, 20, 0, 9, 0, 2, FLOAT, FLOAT, INTEGER],
                'wguide': [100, 3000, .8, 1, 0, 200, 100, 5000, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT],
                'distort': [800, .7, .7, 1, 0, 1000, 0, 1, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT],
                'filter': [1000, .6, 0, 1, 200, 5000, 0, 1, 0, 2, 0, 2, FLOAT, FLOAT, INTEGER],
                'ring': [500, 1, 0, 1, 0, 1000, 0, 1, 0, 5, 0, 2, FLOAT, FLOAT, INTEGER],
                'reverb': [1.5, 3000, .5, 1, 0, 4, 100, 7000, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT],
                'harmon': [1.25, .04, .5, 1, 0, 2, 0, 1, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT]}

    CONTROL_TYPES = ['lfo', 'rand', 'adsr', 'trackpadX', 'trackpadY']
    CONTROL_TYPES_SEL = ['lfosel', 'randsel', 'adsrsel', 'trackpadXsel', 'trackpadYsel']
    CONTROL_TYPES_PLUS = ['lfo+', 'rand+', 'adsr+', 'trackpadX+', 'trackpadY+']
    SOURCE_TYPES = ['fm', 'buzz', 'vco', 'pluck', 'noise', 'sample', 'voice', 'grain', 'addSynth']
    SOURCE_TYPES_SEL = ['fmsel', 'buzzsel', 'vcosel', 'plucksel', 'noisesel', 'samplesel', 'voicesel', 'grainsel', 'addSynthsel']
    SOURCE_TYPES_PLUS = ['fm+', 'buzz+', 'vco+', 'pluck+', 'noise+', 'sample+', 'voice+', 'grain+', 'addSynth+']
    FX_TYPES = ['wguide', 'distort','filter', 'ring', 'reverb', 'harmon']
    FX_TYPES_SEL = ['wguidesel', 'distortsel','filtersel', 'ringsel', 'reverbsel', 'harmonsel']
    FX_TYPES_PLUS = ['wguide+', 'distort+','filter+', 'ring+', 'reverb+', 'harmon+']
    OUTPUT_TYPE = ['adsr']
    OUTPUT_TYPE_SEL = ['adsrsel']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES, OUTPUT_TYPE]
    CHOOSE_TYPE2 = [CONTROL_TYPES_SEL, SOURCE_TYPES_SEL, FX_TYPES_SEL, OUTPUT_TYPE_SEL]
    CHOOSE_TYPE_PLUS = [CONTROL_TYPES_PLUS, SOURCE_TYPES_PLUS, FX_TYPES_PLUS]

    PRESET = ['docu1', 'docu2', 'docu3', 'docu4', 'docu5', 'docu6', 'docu7', 'docu8', 'docu9', 'docu10']
