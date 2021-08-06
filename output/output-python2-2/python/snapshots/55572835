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

    OBJ_Y_LOC = 710
#    INIT_LOCATIONS = [ [330,OBJ_Y_LOC], [405,OBJ_Y_LOC], [480,OBJ_Y_LOC], [555, OBJ_Y_LOC], [15,OBJ_Y_LOC], [90,OBJ_Y_LOC],
#                        [170,OBJ_Y_LOC], [250, OBJ_Y_LOC], [635,OBJ_Y_LOC], [710,OBJ_Y_LOC], [785,OBJ_Y_LOC], [860, OBJ_Y_LOC],
#                        [600, 625]]
    INIT_LOCATIONS = [ [450,OBJ_Y_LOC], [450,OBJ_Y_LOC], [450,OBJ_Y_LOC], 
                        [450, OBJ_Y_LOC], [225,OBJ_Y_LOC], [225,OBJ_Y_LOC],
                        [225,OBJ_Y_LOC], [225, OBJ_Y_LOC], [675,OBJ_Y_LOC], 
                        [675,OBJ_Y_LOC], [675,OBJ_Y_LOC], [675, OBJ_Y_LOC],
                        [450, 625]]

    FLOAT1 = [.1, 1]
    FLOAT = [.01, 2]
    INTEGER = [1, 0]

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
                'noise': [0, 3000, 4000, 1, 0, 2, 0, 8000, 0, 6000, 0, 2, INTEGER, INTEGER,INTEGER],
                'sample': [1, 5, 5000, 1, 0, 2, 0, 99, 100, 8000, 0, 2, FLOAT, INTEGER, INTEGER],
                'voice': [1, 3, 5, 1, 0, 2, 0, 8, 1, 10, 0, 2, FLOAT, INTEGER, FLOAT],
                'grain': [1, 4, 1, 1, 0, 2, 0, 99, 0, 1, 0, 2, FLOAT, INTEGER, FLOAT],
                'addSynth': [1, .005, 5, 1, 0, 2, 0, 20, 0, 9, 0, 2, FLOAT, FLOAT, INTEGER],
                'wguide': [100, 3000, .8, 1, 0, 200, 100, 5000, 0, 1, 0, 2, FLOAT1, INTEGER, FLOAT],
                'distort': [800, .7, .7, 1, 0, 1000, 0, 1, 0, 1, 0, 2, INTEGER, FLOAT, FLOAT],
                'filter': [1000, .6, 0, 1, 200, 5000, 0, 1, 0, 2, 0, 2, INTEGER, FLOAT, INTEGER],
                'ring': [500, 1, 0, 1, 0, 1000, 0, 1, 0, 5, 0, 2, INTEGER, FLOAT, INTEGER],
                'reverb': [1.5, 3000, .5, 1, 0, 4, 100, 7000, 0, 1, 0, 2, FLOAT, INTEGER, FLOAT],
                'harmon': [1.25, .04, .5, 1, 0, 2, 0, 1, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT],
                'eq4band': [1., 1., 1., 1., 0, 2, 0, 2, 0, 2, 0, 2, FLOAT, FLOAT, FLOAT],
                'chorus': [.5, 1., 5., .5, 0, 3, 0, 10, 0, 30, 0, 1, FLOAT, FLOAT, FLOAT]}

    CONTROL_TYPES = ['lfo', 'rand', 'adsr', 'trackpadX', 'trackpadY']
    CONTROL_TYPES_SEL = ['lfosel', 'randsel', 'adsrsel', 'trackpadXsel', 'trackpadYsel']
    CONTROL_TYPES_PLUS = ['lfo+', 'rand+', 'adsr+', 'trackpadX+', 'trackpadY+']
    SOURCE_TYPES = ['fm', 'buzz', 'vco', 'pluck', 'noise', 'sample', 'voice', 'grain', 'addSynth']
    SOURCE_TYPES_SEL = ['fmsel', 'buzzsel', 'vcosel', 'plucksel', 'noisesel', 'samplesel', 'voicesel', 'grainsel', 'addSynthsel']
    SOURCE_TYPES_PLUS = ['fm+', 'buzz+', 'vco+', 'pluck+', 'noise+', 'sample+', 'voice+', 'grain+', 'addSynth+']
    FX_TYPES = ['wguide', 'distort','filter', 'ring', 'reverb', 'harmon', 'eq4band', 'chorus']
    FX_TYPES_SEL = ['wguidesel', 'distortsel','filtersel', 'ringsel', 'reverbsel', 'harmonsel', 'eq4bandsel', 'chorussel']
    FX_TYPES_PLUS = ['wguide+', 'distort+','filter+', 'ring+', 'reverb+', 'harmon+', 'eq4band+', 'chorus+']
    OUTPUT_TYPE = ['adsr']
    OUTPUT_TYPE_SEL = ['adsrsel']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES, OUTPUT_TYPE]
    CHOOSE_TYPE2 = [CONTROL_TYPES_SEL, SOURCE_TYPES_SEL, FX_TYPES_SEL, OUTPUT_TYPE_SEL]
    CHOOSE_TYPE_PLUS = [CONTROL_TYPES_PLUS, SOURCE_TYPES_PLUS, FX_TYPES_PLUS]

    PRESET = ['docu1', 'docu2', 'docu3', 'docu4', 'docu5', 'docu6', 'docu7', 'docu8', 'docu9', 'docu10']
