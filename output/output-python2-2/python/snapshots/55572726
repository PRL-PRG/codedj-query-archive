from gettext import gettext as _

class SynthLabConstants:

    PIC_SIZE = 80
    HALF_SIZE = PIC_SIZE // 2
    PIC_SIZE_HIGHLIGHT = PIC_SIZE + 4

    GT_CONTROL_OUTPUT = 0
    GT_CONTROL_INPUT = 1
    GT_SOUND_OUTPUT = 2
    GT_SOUND_INPUT = 3
    # GATE_POINT[ojecttype][gatetype][gatenum] = (x,y)
    # relative to object center
    GATE_POINT = [ [ [ (0,34) ] ],
                   [ [], [ (-25,-35),(-9,-35),(8,-35),(25,-35) ], [ (0,35) ] ],
                   [ [], [ (33,-20),(33,-7),(33,7),(33,20) ], [ (-2,34) ], [ (-2,-34) ] ],
                   [ [], [], [], [ (0,-35) ] ] ]
    # GATE_MAP[objecttype][gatetype][gatenum] = [ sx, sy, ex, ey, (wireX,wireY) ]
    # gate locations relative to object center
    GATE_MAP = [ [ [ [-6,28,6,40] ] ],
                 [ [], [[-31,-40,-18,-28], [-16,-40,-3,-28], [2,-40,15,-28], [19,-40,32,-28]], [[-6,28,7,40]] ],
                 [ [], [[26,-26,38,-13], [26,-13,38,0], [26,0,38,13], [26,13,38,26]], [[-8,28,5,40]], [[-8,-40,5,-28]] ],
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
    CONTROL_TYPES_SEL = [type + 'sel' for type in CONTROL_TYPES]
    CONTROL_TYPES_PLUS = [type + '+' for type in CONTROL_TYPES]
    SOURCE_TYPES = ['fm', 'buzz', 'vco', 'pluck', 'noise', 'sample', 'voice', 'grain', 'addSynth']
    SOURCE_TYPES_SEL = [type + 'sel' for type in SOURCE_TYPES]
    SOURCE_TYPES_PLUS = [type + '+' for type in SOURCE_TYPES]
    FX_TYPES = ['wguide', 'distort','filter', 'ring', 'reverb', 'harmon', 'eq4band', 'chorus']
    FX_TYPES_SEL = [type + 'sel' for type in FX_TYPES]
    FX_TYPES_PLUS = [type + '+' for type in FX_TYPES]
    OUTPUT_TYPE = ['adsr']
    OUTPUT_TYPE_SEL = ['adsrsel']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES, OUTPUT_TYPE]
    CHOOSE_TYPE2 = [CONTROL_TYPES_SEL, SOURCE_TYPES_SEL, FX_TYPES_SEL, OUTPUT_TYPE_SEL]
    CHOOSE_TYPE_PLUS = [CONTROL_TYPES_PLUS, SOURCE_TYPES_PLUS, FX_TYPES_PLUS]

# SynthLab Tooltips
    SOURCE = _('Source')
    EFFECT = _('Effect')
    CONTROL = _('Control')
    SOUNDOUT = _('Sound Output')

    #Controls
    LFO = _('LFO')
    AMP = _('Amplitude')
    FREQ = _('Frequency')
    WAVEFORM = _('Waveform')
    LFO_WAVEFORMS = [_('Sine'), _('Triangle'), _('Bi-Square'), _('Uni-Square'), _('Sawtooth'), _('Sawtooth-down')]
    OFFSET = _('Offset')

    RANDOM = _('Random')
    MIN = _('Minimum')
    MAX = _('Maximum')
    FREQ = FREQ
    SEED = _('Seed')

    ADSR = _('Envelope')
    ATTACK = _('Attack')
    DECAY = _('Decay')
    SUSTAIN = _('Sustain')
    RELEASE = _('Release')

    TRACKPADX = _('Trackpad X')
    MIN = MIN
    MAX = MAX
    SCALING = _('Scaling')
    SCALING_TYPES =    [_('Lin'), _('Log')]
    POLL = _('Poll time')

    TRACKPADY = _('Trackpad Y')
    MIN = MIN
    MAX = MAX
    SCALING = SCALING
    SCALING_TYPES = SCALING_TYPES
    POLL = POLL

    #Source
    FM = _('FM')
    CAR = _('Carrier Frequency')
    MOD = _('Modulator Frequency')
    INDEX = _('Index')
    GAIN = _('Gain')

    BUZZ = _('Buzz')
    FREQ = FREQ
    NHARM = _('Number of harmonics')
    FSLOPE = _('Filter Slope')
    GAIN = GAIN

    VCO = _('VCO')
    FREQ = FREQ
    WAVEFORM = WAVEFORM
    VCO_WAVEFORMS = [_('Sawtooth'), _('Square'), _('Triangle')]
    FSLOPE = FSLOPE
    GAIN = GAIN

    PLUCK = _('Pluck')
    FREQ = FREQ
    LFILTER = _('Lowpass Filter')
    VIBRATO = _('Vibrato')
    GAIN = GAIN

    NOISE = _('Noise')
    NOISETYPE = _('Type')
    NOISE_TYPES = [_('White'), _('Pink'), _('Gauss')]
    FREQ = FREQ
    BANDWITH = _('Bandwith')
    GAIN = GAIN

    SAMPLE = _('Sound Sample')
    FREQ = FREQ
    SAMPLEN = _('Sample Number')
    SAMPLE_NAMES = _('Sample name')
    LFILTER = LFILTER
    GAIN = GAIN

    VOICE = _('Voice')
    FREQ = FREQ
    VOWEL = _('Vowel')
    VOWEL_TYPES = ['i', 'e', 'ee', 'a', 'u', 'o1', 'o2', 'oa', 'oe']
    VIBRATO = VIBRATO
    GAIN = GAIN

    GRAIN = _('Grain')
    FREQ = FREQ
    SAMPLEN = SAMPLEN
    INDEX = _('Index')
    GAIN = GAIN

    ADDSYNTH = _('Additive Synthesis')
    FREQ = FREQ
    SPREAD = _('Spread')
    WAVE = _('Waveform')
    GAIN = GAIN

    #Effects
    DELAY = _('Delay')
    FREQ = FREQ
    LFILTER = LFILTER
    FEEDBACK = _('Feedback')
    GAIN = GAIN

    DIST = _('Distortion')
    FREQ = FREQ
    RESON = _('Resonance')
    DISTL = _('Distotion Level')
    GAIN = GAIN

    FILTER = _('Filter')
    FREQ = FREQ
    FSLOPE = FSLOPE
    FTYPE = _('Type')
    FILTER_TYPES = [_('Lowpass'), _('Highpass'), _('Bandpass')]
    GAIN = GAIN

    RINGMOD = _('Ring Modulator')
    FREQ = FREQ
    MIX = _('Mix')
    WAVEFORM = WAVEFORM
    LFO_WAVEFORMS = LFO_WAVEFORMS
    GAIN = GAIN

    REVERB = _('Reverb')
    REVERBD = _('Length')
    REVERBF = _('Lowpass Filter')
    REVERBL = _('Reverb Level')
    GAIN = GAIN

    HARMON = _('Harmonizer')
    FREQ = FREQ
    DRYDELAY = _('Dry delay')
    MIX = MIX
    GAIN = GAIN

    EQ4BAND = _('Equalizer 4 bands')
    FREQ1 = _('Band one gain')
    FREQ2 = _('Band two gain')
    FREQ3 = _('Band three gain')
    FREQ4 = _('Band four gain')

    CHORUS = _('Chorus')
    LFODEPTH = _('LFO Depth')
    LFOFREQ = _('LFO Frequency')
    DELAY = _('Delay')
    FEEDBACK = FEEDBACK

    SYNTHTYPES = [[LFO, RANDOM, ADSR, TRACKPADX, TRACKPADY],
                    [FM, BUZZ, VCO, PLUCK, NOISE, SAMPLE, VOICE, GRAIN, ADDSYNTH],
                    [DELAY, DIST, FILTER, RINGMOD, REVERB, HARMON, EQ4BAND, CHORUS], [ADSR]]

    SYNTHPARA = {	_('lfo'): [AMP, FREQ, WAVEFORM, OFFSET],
                    _('rand'): [MIN, MAX, FREQ, SEED],
                    _('adsr'): [ATTACK, DECAY, SUSTAIN, RELEASE],
                    _('trackpadX'): [MIN, MAX, SCALING, POLL],
                    _('trackpadY'): [MIN, MAX, SCALING, POLL],
                    _('fm'): [CAR, MOD, INDEX, GAIN],
                    _('buzz'): [FREQ, NHARM, FSLOPE, GAIN],
                    _('vco'): [FREQ, WAVEFORM, FSLOPE, GAIN],
                    _('pluck'): [FREQ, LFILTER, VIBRATO, GAIN],
                    _('noise'): [NOISETYPE, FREQ, BANDWITH, GAIN],
                    _('sample'): [FREQ, SAMPLEN, LFILTER, GAIN],
                    _('voice'): [FREQ, VOWEL, VIBRATO, GAIN],
                    _('grain'): [FREQ, SAMPLEN, INDEX, GAIN],
                    _('addSynth'): [FREQ, SPREAD, WAVE, GAIN],
                    _('wguide'): [FREQ, LFILTER, FEEDBACK, GAIN],
                    _('distort'): [FREQ, RESON, DISTL, GAIN],
                    _('filter'): [FREQ, FSLOPE, FTYPE, GAIN],
                    _('ring'): [FREQ, MIX, WAVEFORM, GAIN],
                    _('reverb'): [REVERBD, REVERBF, REVERBL, GAIN],
                    _('harmon'): [FREQ, DRYDELAY, MIX, GAIN],
                    _('eq4band'): [FREQ1, FREQ2, FREQ3, FREQ4],
                    _('chorus'): [LFODEPTH, LFOFREQ, DELAY, FEEDBACK]}
