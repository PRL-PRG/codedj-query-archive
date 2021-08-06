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
    LFO_INFO = _('A low frequency oscillation (LFO) is a signal usually below 20 Hz which creates a pulsating rythm rather than a audible tone. This signal is commonly used to control sound parameters.')
    LFO_PARA1 = _('The amplitude (volume) of the LFO signal. More amplitude means the effect will be more noticeable.')
    LFO_PARA2 = _('The speed of the signal beteen 0 Hz and 20 hz.')    
    LFO_PARA3 = _('The type of waveform that will be used for the LFO.')
    LFO_PARA4 = _('Offset: ')
    
    RANDOM = _('Random')
    MIN = _('Minimum')
    MAX = _('Maximum')
    FREQ = FREQ
    SEED = _('Seed')
    RANDOM_INFO = _('A random number generator is a computational device designed to generate a sequence of numbers that lack any pattern.')
    RANDOM_PARA1 = _('')
    RANDOM_PARA2 = _('The minimum value that can be generated.')
    RANDOM_PARA3 = _('The maximum value that can be generated.')
    RANDOM_PARA4 = _('')
    
    ADSR = _('Envelope')
    ATTACK = _('Attack')
    DECAY = _('Decay')
    SUSTAIN = _('Sustain')
    RELEASE = _('Release')
    ADSR_INFO = _("An ADSR envelope is a component of many sythesizers, samplers, and other electronic musical instruments. Its function is to modulate some aspect of the instrument's sound - often its volume - over time.")
    ADSR_PARA1 = _('How quickly the sound reaches full volume after the sound is activated.')
    ADSR_PARA2 = _('How quickly the sound drops to the sustain level after the initial peak.')
    ADSR_PARA3= _('The "constant" volume that the sound takes after decay until the note is released.')
    ADSR_PARA4 = _('How quickly the sound fades when a note ends.')

    TRACKPADX = _('Trackpad X')
    MIN = MIN
    MAX = MAX
    SCALING = _('Scaling')
    SCALING_TYPES =    [_('Lin'), _('Log')]
    POLL = _('Poll time')
    TRACKPADX_INFO = _('The XO trackpad can be used to control sound parameters. This is the x axis, from left to right.')
    TRACKPADX_PARA1 = _('The minimum value the trackpad will send.')
    TRACKPADX_PARA2 = _('The maximum value de trackpad will send.')
    TRACKPADX_PARA3 = _('The type of scaling, logarithmic or linear.')
    TRACKPADX_PARA4= _('The time interval between each event coming from the trackpad.')

    TRACKPADY = _('Trackpad Y')
    MIN = MIN
    MAX = MAX
    SCALING = SCALING
    SCALING_TYPES = SCALING_TYPES
    POLL = POLL
    TRACKPADY_INFO = _('The XO trackpad can be used to control sound parameters. This is the y axis, from top to bottom.')
    TRACKPADY_PARA1 = _('The minimum value the trackpad will send.')
    TRACKPADY_PARA2 = _('The maximum value de trackpad will send.')
    TRACKPADY_PARA3 = _('The type of scaling, logarithmic or linear.')
    TRACKPADY_PARA4 = _('The time interval between each event coming from the trackpad.')

    #Source
    FM = _('FM')
    CAR = _('Carrier Frequency')
    MOD = _('Modulator Frequency')
    INDEX = _('Index')
    GAIN = _('Gain')
    FM_INFO = _('Frequency modulation synthesis is a form of audio synthesis where the timbre of a simple waveform is changed by frequency modulating it with a modulating frequency that is also in the audio range, resulting in a more complex waveform and a different-sounding tone.')
    FM_PARA1 = _('The main waveform frequency.')
    FM_PARA2 = _('The frequency of the waveform that will modulate the Carrier waveform.')
    FM_PARA3 = _('The variation in frequency of the Carrier waveform.')
    FM_PARA4 = _('The overall gain (volume) of the sound source.')

    BUZZ = _('Buzz')
    FREQ = FREQ
    NHARM = _('Number of harmonics')
    FSLOPE = _('Filter Slope')
    GAIN = GAIN
    BUZZ_INFO = _('')
    BUZZ_PARA1 = _('')
    BUZZ_PARA2 = _('')
    BUZZ_PARA3 = _('')
    BUZZ_PARA4 = _('The overall gain (volume) of the sound source.')

    VCO = _('VCO')
    FREQ = FREQ
    WAVEFORM = WAVEFORM
    VCO_WAVEFORMS = [_('Sawtooth'), _('Square'), _('Triangle')]
    FSLOPE = FSLOPE
    GAIN = GAIN
    VCO_INFO = _('')
    VCO_PARA1 = _('')
    VCO_PARA2 = _('')
    VCO_PARA3 = _('')
    VCO_PARA4 = _('The overall gain (volume) of the sound source.')

    PLUCK = _('Pluck')
    FREQ = FREQ
    LFILTER = _('Lowpass Filter')
    VIBRATO = _('Vibrato')
    GAIN = GAIN
    PLUCK_INFO = _('')
    PLUCK_PARA1 = _('')
    PLUCK_PARA2 = _('')
    PLUCK_PARA3 = _('')
    PLUCK_PARA4 = _('The overall gain (volume) of the sound source.')

    NOISE = _('Noise')
    NOISETYPE = _('Type')
    NOISE_TYPES = [_('White'), _('Pink'), _('Gauss')]
    FREQ = FREQ
    BANDWITH = _('Bandwith')
    GAIN = GAIN
    NOISE_INFO = _('')
    NOISE_PARA1 = _('')
    NOISE_PARA2 = _('')
    NOISE_PARA3 = _('')
    NOISE_PARA4 = _('The overall gain (volume) of the sound source.')

    SAMPLE = _('Sound Sample')
    FREQ = FREQ
    SAMPLEN = _('Sample Number')
    SAMPLE_NAMES = _('Sample name')
    LFILTER = LFILTER
    GAIN = GAIN
    SAMPLE_INFO = _('')
    SAMPLE_PARA1 = _('')
    SAMPLE_PARA2 = _('')
    SAMPLE_PARA3 = _('')
    SAMPLE_PARA4 = _('The overall gain (volume) of the sound source.')

    VOICE = _('Voice')
    FREQ = FREQ
    VOWEL = _('Vowel')
    VOWEL_TYPES = ['i', 'e', 'ee', 'a', 'u', 'o1', 'o2', 'oa', 'oe']
    VIBRATO = VIBRATO
    GAIN = GAIN
    VOICE_INFO = _('')
    VOICE_PARA1 = _('')
    VOICE_PARA2 = _('')
    VOICE_PARA3 = _('')
    VOICE_PARA4 = _('The overall gain (volume) of the sound source.')

    GRAIN = _('Grain')
    FREQ = FREQ
    SAMPLEN = SAMPLEN
    INDEX = _('Index')
    GAIN = GAIN
    GRAIN_INFO = _('')
    GRAIN_PARA1 = _('')
    GRAIN_PARA2 = _('')
    GRAIN_PARA3 = _('')
    GRAIN_PARA4 = _('The overall gain (volume) of the sound source.')

    ADDSYNTH = _('Additive Synthesis')
    FREQ = FREQ
    SPREAD = _('Spread')
    WAVE = _('Waveform')
    GAIN = GAIN
    ADDSYNTH_INFO = _('')
    ADDSYNTH_PARA1 = _('')
    ADDSYNTH_PARA2 = _('')
    ADDSYNTH_PARA3 = _('')
    ADDSYNTH_PARA4 = _('The overall gain (volume) of the sound source.')

    #Effects
    DELAY = _('Delay')
    FREQ = FREQ
    LFILTER = LFILTER
    FEEDBACK = _('Feedback')
    GAIN = GAIN
    DELAY_INFO = _('')
    DELAY_PARA1 = _('')
    DELAY_PARA2 = _('')
    DELAY_PARA3 = _('')
    DELAY_PARA4 = _('The overall gain (volume) of the sound source.')

    DIST = _('Distortion')
    FREQ = FREQ
    RESON = _('Resonance')
    DISTL = _('Distotion Level')
    GAIN = GAIN
    DIST_INFO = _('')
    DIST_PARA1 = _('')
    DIST_PARA2 = _('')
    DIST_PARA3 = _('')
    DIST_PARA4 = _('The overall gain (volume) of the sound source.')

    FILTER = _('Filter')
    FREQ = FREQ
    FSLOPE = FSLOPE
    FTYPE = _('Type')
    FILTER_TYPES = [_('Lowpass'), _('Highpass'), _('Bandpass')]
    GAIN = GAIN
    FILTER_INFO = _('')
    FILTER_PARA1 = _('')
    FILTER_PARA2 = _('')
    FILTER_PARA3 = _('')
    FILTER_PARA4 = _('The overall gain (volume) of the sound source.')

    RINGMOD = _('Ring Modulator')
    FREQ = FREQ
    MIX = _('Mix')
    WAVEFORM = WAVEFORM
    LFO_WAVEFORMS = LFO_WAVEFORMS
    GAIN = GAIN
    RINGMOD_INFO = _('')
    RINGMOD_PARA1 = _('')
    RINGMOD_PARA2 = _('')
    RINGMOD_PARA3 = _('')
    RINGMOD_PARA4 = _('The overall gain (volume) of the sound source.')

    REVERB = _('Reverb')
    REVERBD = _('Length')
    REVERBF = _('Lowpass Filter')
    REVERBL = _('Reverb Level')
    GAIN = GAIN
    REVERB_INFO = _('Reverberation is the persistence od sound in a particular space after the original sound is removed.')
    REVERB_PARA1 = _('')
    REVERB_PARA2 = _('')
    REVERB_PARA3 = _('')
    REVERB_PARA4 = _('The overall gain (volume) of the sound source.')

    HARMON = _('Harmonizer')
    FREQ = FREQ
    DRYDELAY = _('Dry delay')
    MIX = MIX
    GAIN = GAIN
    HARMON_INFO = _('')
    HARMON_PARA1 = _('')
    HARMON_PARA2 = _('')
    HARMON_PARA3 = _('')
    HARMON_PARA4 = _('The overall gain (volume) of the sound source.')

    EQ4BAND = _('Equalizer 4 bands')
    FREQ1 = _('Band one gain')
    FREQ2 = _('Band two gain')
    FREQ3 = _('Band three gain')
    FREQ4 = _('Band four gain')
    EQ4BAND_INFO = _('A 4 band equalizer is an effect that splits the spectrum into 4 bands and allows the bands to be set at different levels.')
    EQ4BAND_PARA1 = _('The gain (volume) of band 1.')
    EQ4BAND_PARA2 = _('The gain (volume) of band 2.')
    EQ4BAND_PARA3 = _('The gain (volume) of band 3.')
    EQ4BAND_PARA4 = _('The gain (volume) of band 4.')

    CHORUS = _('Chorus')
    LFODEPTH = _('LFO Depth')
    LFOFREQ = _('LFO Frequency')
    DELAY = _('Delay')
    FEEDBACK = FEEDBACK
    CHORUS_INFO = _('A chorus effect is a condition in the way people perceive similar sounds coming from multiple sources.')
    CHORUS_PARA1 = _('LFO Depth: The amplitude of the LFO signal.')
    CHORUS_PARA2 = _('LFO Frequency: the frequency of the LFO signal.')
    CHORUS_PARA3 = _('Delay: The amount of delay between the two signals.')
    CHORUS_PARA4 = _('The overall gain (volume) of the sound source.')
    
    SYNTHTYPES = [[LFO, RANDOM, ADSR, TRACKPADX, TRACKPADY],
                    [FM, BUZZ, VCO, PLUCK, NOISE, SAMPLE, VOICE, GRAIN, ADDSYNTH],
                    [DELAY, DIST, FILTER, RINGMOD, REVERB, HARMON, EQ4BAND, CHORUS], [ADSR]]

    SYNTHPARA = {	_('lfo'): [AMP, FREQ, WAVEFORM, OFFSET, LFO_INFO, LFO_PARA1, LFO_PARA2, LFO_PARA3, LFO_PARA4],
                    _('rand'): [MIN, MAX, FREQ, SEED, RANDOM_INFO, RANDOM_PARA1, RANDOM_PARA2, RANDOM_PARA3, RANDOM_PARA4],
                    _('adsr'): [ATTACK, DECAY, SUSTAIN, RELEASE, ADSR_INFO, ADSR_PARA1, ADSR_PARA2, ADSR_PARA3, ADSR_PARA4],
                    _('trackpadX'): [MIN, MAX, SCALING, POLL, TRACKPADX_INFO, TRACKPADX_PARA1, TRACKPADX_PARA2, TRACKPADX_PARA3, TRACKPADX_PARA4],
                    _('trackpadY'): [MIN, MAX, SCALING, POLL, TRACKPADY_INFO, TRACKPADY_PARA1, TRACKPADY_PARA2, TRACKPADY_PARA3, TRACKPADY_PARA4],
                    _('fm'): [CAR, MOD, INDEX, GAIN, FM_INFO, FM_PARA1, FM_PARA2, FM_PARA3, FM_PARA4],
                    _('buzz'): [FREQ, NHARM, FSLOPE, GAIN, BUZZ_INFO, BUZZ_PARA1, BUZZ_PARA2, BUZZ_PARA3, BUZZ_PARA4],
                    _('vco'): [FREQ, WAVEFORM, FSLOPE, GAIN, VCO_INFO, VCO_PARA1, VCO_PARA2, VCO_PARA3, VCO_PARA4],
                    _('pluck'): [FREQ, LFILTER, VIBRATO, GAIN, PLUCK_INFO, PLUCK_PARA1, PLUCK_PARA2, PLUCK_PARA3, PLUCK_PARA4],
                    _('noise'): [NOISETYPE, FREQ, BANDWITH, GAIN, NOISE_INFO, NOISE_PARA1, NOISE_PARA2, NOISE_PARA3, NOISE_PARA4],
                    _('sample'): [FREQ, SAMPLEN, LFILTER, GAIN, SAMPLE_INFO, SAMPLE_PARA1, SAMPLE_PARA2, SAMPLE_PARA3, SAMPLE_PARA4],
                    _('voice'): [FREQ, VOWEL, VIBRATO, GAIN, VOICE_INFO, VOICE_PARA1, VOICE_PARA2, VOICE_PARA3, VOICE_PARA4],
                    _('grain'): [FREQ, SAMPLEN, INDEX, GAIN, GRAIN_INFO, GRAIN_PARA1, GRAIN_PARA2, GRAIN_PARA3, GRAIN_PARA4],
                    _('addSynth'): [FREQ, SPREAD, WAVE, GAIN, ADDSYNTH_INFO, ADDSYNTH_PARA1, ADDSYNTH_PARA2, ADDSYNTH_PARA3, ADDSYNTH_PARA4],
                    _('wguide'): [FREQ, LFILTER, FEEDBACK, GAIN, DELAY_INFO, DELAY_PARA1, DELAY_PARA2, DELAY_PARA3, DELAY_PARA4],
                    _('distort'): [FREQ, RESON, DISTL, GAIN, DIST_INFO, DIST_PARA1, DIST_PARA2, DIST_PARA3, DIST_PARA4],
                    _('filter'): [FREQ, FSLOPE, FTYPE, GAIN, FILTER_INFO, FILTER_PARA1, FILTER_PARA2, FILTER_PARA3, FILTER_PARA4],
                    _('ring'): [FREQ, MIX, WAVEFORM, GAIN, RINGMOD_INFO, RINGMOD_PARA1, RINGMOD_PARA2, RINGMOD_PARA3, RINGMOD_PARA4],
                    _('reverb'): [REVERBD, REVERBF, REVERBL, GAIN, REVERB_INFO, REVERB_PARA1, REVERB_PARA2, REVERB_PARA3, REVERB_PARA4],
                    _('harmon'): [FREQ, DRYDELAY, MIX, GAIN, HARMON_INFO, HARMON_PARA1, HARMON_PARA2, HARMON_PARA3, HARMON_PARA4],
                    _('eq4band'): [FREQ1, FREQ2, FREQ3, FREQ4, EQ4BAND_INFO, EQ4BAND_PARA1, EQ4BAND_PARA2, EQ4BAND_PARA3, EQ4BAND_PARA4],
                    _('chorus'): [LFODEPTH, LFOFREQ, DELAY, FEEDBACK, CHORUS_INFO, CHORUS_PARA1, CHORUS_PARA2, CHORUS_PARA3, CHORUS_PARA4]}
