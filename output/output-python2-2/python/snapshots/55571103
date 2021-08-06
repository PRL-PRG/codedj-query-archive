from gettext import gettext as _

class SynthLabConstants:

    PIC_SIZE = 80
    HALF_SIZE = PIC_SIZE // 2
    PIC_SIZE_HIGHLIGHT = PIC_SIZE + 4

    GT_CONTROL_OUTPUT = 0
    GT_CONTROL_INPUT = 1
    GT_SOUND_OUTPUT = 2
    GT_SOUND_INPUT = 3
    # GATE_POINT[objecttype][gatetype][gatenum] = (x,y)
    # relative to object center
    GATE_POINT = [ [ [ (-1,33) ] ],
                   [ [], [ (-24,-34),(-9,-34),(8,-34),(24,-34) ], [ (-1,33) ] ],
                   [ [], [ (31,-20),(31,-6),(31,6),(31,19) ], [ (-3,33) ], [ (-3,-34) ] ],
                   [ [], [], [], [ (2,-35) ] ] ]
    # GATE_MAP[objecttype][gatetype][gatenum] = [ sx, sy, ex, ey, (wireX,wireY) ]
    # gate locations relative to object center
    GATE_MAP = [ [ [ [-7,26,4,39] ] ],
                 [ [], [[-30,-40,-19,-28], [-15,-40,-3,-28], [3,-40,14,-28], [19,-40,28,-28]], [[-6,28,5,40]] ],
                 [ [], [[25,-25,37,-14], [25,-12,37,-1], [25,1,37,12], [25,13,37,25]], [[-8,27,3,40]], [[-8,-40,3,-27]] ],
                 [ [], [], [], [[-4,-40,7,-29]] ] ]
    # insert wire locations into map
    GATE_OFFSET = 15
    for oT in GATE_MAP:
        for gT in oT:
            for m in gT:
                x = (m[2]+m[0])//2
                y = (m[3]+m[1])//2
                # snap to edges
                if x < -HALF_SIZE+GATE_OFFSET: x = m[0]
                elif x > HALF_SIZE-GATE_OFFSET: x = m[2]
                if y < -HALF_SIZE+GATE_OFFSET: y = m[1]
                elif y > HALF_SIZE-GATE_OFFSET: y = m[3]
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
                'mic': [5, 1, 1, 1, 1, 10, 1, 4, 0, 4, 0, 2, FLOAT, INTEGER, FLOAT],
                'wguide': [100, 3000, .8, 1, 0, 200, 100, 5000, 0, 1, 0, 2, FLOAT1, INTEGER, FLOAT],
                'distort': [800, .7, .7, 1, 0, 1000, 0, 1, 0, 1, 0, 2, INTEGER, FLOAT, FLOAT],
                'filter': [1000, .6, 0, 1, 200, 5000, 0, 1, 0, 2, 0, 2, INTEGER, FLOAT, INTEGER],
                'ring': [500, 1, 0, 1, 0, 1000, 0, 1, 0, 5, 0, 2, INTEGER, FLOAT, INTEGER],
                'reverb': [1.5, 3000, .5, 1, 0, 4, 100, 7000, 0, 1, 0, 2, FLOAT, INTEGER, FLOAT],
                'harmon': [1.25, .04, .5, 1, 0, 2, 0, 1, 0, 1, 0, 2, FLOAT, FLOAT, FLOAT],
                'eq4band': [1., 1., 1., 1., 0, 2, 0, 2, 0, 2, 0, 2, FLOAT, FLOAT, FLOAT],
                'chorus': [.5, 1., 5., .5, 0, 3, 0, 10, 0, 30, 0, 1, FLOAT, FLOAT, FLOAT]}

    CONTROL_TYPES = ['lfo', 'rand', 'adsr', 'trackpadX', 'trackpadY']
    CONTROL_TYPES_PLUS = [type + '+' for type in CONTROL_TYPES]
    SOURCE_TYPES = ['fm', 'buzz', 'vco', 'pluck', 'noise', 'sample', 'voice', 'grain', 'addSynth', 'mic']
    SOURCE_TYPES_PLUS = [type + '+' for type in SOURCE_TYPES]
    FX_TYPES = ['wguide', 'distort','filter', 'ring', 'reverb', 'harmon', 'eq4band', 'chorus']
    FX_TYPES_PLUS = [type + '+' for type in FX_TYPES]
    OUTPUT_TYPE = ['adsr']
    OUTPUT_TYPE_SEL = ['adsrsel']
    CHOOSE_TYPE = [CONTROL_TYPES, SOURCE_TYPES, FX_TYPES, OUTPUT_TYPE]
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
    LFO_INFO = _('A low frequency oscillation (LFO) is an inaudible, pulsing wave used to change another sound.')
    LFO_PARA1 = _('The volume of the LFO wave. More volume means more effect.')
    LFO_PARA2 = _('The speed of the wave.')
    LFO_PARA3 = _('The type of wave that will be used for the LFO.')
    LFO_PARA4 = _('The time by which the LFO is delayed.')

    RANDOM = _('Random')
    MIN = _('Minimum')
    MAX = _('Maximum')
    FREQ = FREQ
    SEED = _('Seed')
    RANDOM_INFO = _('A sequence of numbers without repetition chosen by the computer.')
    RANDOM_PARA1 = _('The smallest number allowed')
    RANDOM_PARA2 = _('The biggest number allowed.')
    RANDOM_PARA3 = _('The speed of the sequence.')
    RANDOM_PARA4 = _('The number to initialize the number generator')

    ADSR = _('Envelope')
    ATTACK = _('Attack')
    DECAY = _('Decay')
    SUSTAIN = _('Sustain')
    RELEASE = _('Release')
    ADSR_INFO = _("An ADSR envelope is the shape of the sound's volume over time.")
    ADSR_PARA1 = _('(A) how quickly the sound reaches full volume.')
    ADSR_PARA2 = _('(D) how quickly the sound drops after the attack.')
    ADSR_PARA3= _('(S) the volume of the sound until the note is released.')
    ADSR_PARA4 = _('(R) how quickly the sound goes away.')

    TRACKPADX = _('Trackpad X')
    MIN = MIN
    MAX = MAX
    SCALING = _('Scaling')
    SCALING_TYPES =    [_('Lin'), _('Log')]
    POLL = _('Poll time')
    TRACKPADX_INFO = _('The trackpad can be used to modify the sound. This is from left to right.')
    TRACKPADX_PARA1 = _('The minimum value the trackpad will send.')
    TRACKPADX_PARA2 = _('The maximum value the trackpad will send.')
    TRACKPADX_PARA3 = _('The shape of the value reapartition. In a straight line (linear) or a curved line (logarithmic).')
    TRACKPADX_PARA4= _('The time interval between each event coming from the trackpad.')

    TRACKPADY = _('Trackpad Y')
    MIN = MIN
    MAX = MAX
    SCALING = SCALING
    SCALING_TYPES = SCALING_TYPES
    POLL = POLL
    TRACKPADY_INFO = _('The trackpad can be used to modify the sound. This is from top to bottom.')
    TRACKPADY_PARA1 = _('The minimum value the trackpad will send.')
    TRACKPADY_PARA2 = _('The maximum value de trackpad will send.')
    TRACKPADY_PARA3 = _('The shape of the value repartition. In a straight line (linear) or a curved line (logarithmic).')
    TRACKPADY_PARA4 = _('The time interval between each event coming from the trackpad.')

    #Source
    FM = _('FM')
    CAR = _('Carrier Frequency')
    MOD = _('Modulator Frequency')
    INDEX = _('Index')
    GAIN = _('Gain')
    FM_INFO = _('Frequency modulation synthesis (FM) creates an electronic sound by combining the frequency of two waves (the carrier and the modulator).')
    FM_PARA1 = _('The main wave frequency.')
    FM_PARA2 = _('The frequency of the wave that will modify the Carrier wave.')
    FM_PARA3 = _('The variation in frequency of the Carrier wave.')
    FM_PARA4 = _('The volume of the sound.')

    BUZZ = _('Buzz')
    FREQ = FREQ
    NHARM = _('Number of harmonics')
    FSLOPE = _('Filter Slope')
    GAIN = GAIN
    BUZZ_INFO = _('A buzz is a very bright sound with many harmonics.')
    BUZZ_PARA1 = _('The pitch of the buzz sound.')
    BUZZ_PARA2 = _('The harmonic thickness of the sound.')
    BUZZ_PARA3 = _('The brightness of the sound.')
    BUZZ_PARA4 = _('The volume of the sound.')

    VCO = _('VCO')
    FREQ = FREQ
    WAVEFORM = WAVEFORM
    VCO_WAVEFORMS = [_('Sawtooth'), _('Square'), _('Triangle')]
    FSLOPE = FSLOPE
    GAIN = GAIN
    VCO_INFO = _('A voltage-controlled oscillator (VCO) creates an electronic sound by combining the shape of two waves.')
    VCO_PARA1 = _('The wave that will be modified by the VCO.')
    VCO_PARA2 = _("The shape of the VCO's wave.")
    VCO_PARA3 = _('The brightness of the sound.')
    VCO_PARA4 = _('The volume of the sound.')

    PLUCK = _('Pluck')
    FREQ = FREQ
    LFILTER = _('Lowpass Filter')
    VIBRATO = _('Vibrato')
    GAIN = GAIN
    PLUCK_INFO = _('An electronic string instrument (like a guitar).')
    PLUCK_PARA1 = _('The pitch of the instrument.')
    PLUCK_PARA2 = _('The brightness of the sound.')
    PLUCK_PARA3 = _('The speed of the wave.')
    PLUCK_PARA4 = _('The volume of the sound.')

    NOISE = _('Noise')
    NOISETYPE = _('Type')
    NOISE_TYPES = [_('White'), _('Pink'), _('Gauss')]
    FREQ = FREQ
    BANDWITH = _('Bandwith')
    GAIN = GAIN
    NOISE_INFO = _('Noise is a sound with energy on every frequency.')
    NOISE_PARA1 = _('The shape of noise to be used (white = bright, pink = dark, gauss = colored).')
    NOISE_PARA2 = _('The brightness of the sound.')
    NOISE_PARA3 = _('The thickness of the sound.')
    NOISE_PARA4 = _('The volume of the sound.')

    SAMPLE = _('Sound Sample')
    FREQ = FREQ
    SAMPLEN = _('Sample Number')
    SAMPLE_NAMES = _('Sample name')
    LFILTER = LFILTER
    GAIN = GAIN
    SAMPLE_INFO = _("A sample is a real sound that has been recorded and that can be played back.")
    SAMPLE_PARA1 = _('The pitch of the sample.')
    SAMPLE_PARA2 = _('The sample to be used.')
    SAMPLE_PARA3 = _('The brightness of the sound.')
    SAMPLE_PARA4 = _('The volume of the sound.')

    VOICE = _('Voice')
    FREQ = FREQ
    VOWEL = _('Vowel')
    VOWEL_TYPES = ['i', 'e', 'ee', 'a', 'u', 'o1', 'o2', 'oa', 'oe']
    VIBRATO = VIBRATO
    GAIN = GAIN
    VOICE_INFO = _('An electronic voice.')
    VOICE_PARA1 = _('The pitch of the sound.')
    VOICE_PARA2 = _('The shape of the sound based on vowels.')
    VOICE_PARA3 = _('The speed of the wave.')
    VOICE_PARA4 = _('The volume of the sound.')

    GRAIN = _('Grain')
    FREQ = FREQ
    SAMPLEN = SAMPLEN
    INDEX = _('Index')
    GAIN = GAIN
    GRAIN_INFO = _('The grain effect splits the sound in tiny bits which can be rearranged in time.')
    GRAIN_PARA1 = _('The pitch of grains.')
    GRAIN_PARA2 = _('The sample to be used')
    GRAIN_PARA3 = _('The variation in pitch of grains.')
    GRAIN_PARA4 = _('The volume of the sound.')

    ADDSYNTH = _('Additive Synthesis')
    FREQ = FREQ
    SPREAD = _('Spread')
    WAVE = _('Waveform')
    GAIN = GAIN
    ADDSYNTH_INFO = _('Additive synthesis creates musical timbre by combining different waves.')
    ADDSYNTH_PARA1 = _('The pitch of the sound.')
    ADDSYNTH_PARA2 = _('The separation between the different waves.')
    ADDSYNTH_PARA3 = _('The shape of the wave.')
    ADDSYNTH_PARA4 = _('The volume of the sound.')

    MIC = _('Microphone input')
    DURATION = _('Length of the memory')
    BIN = _('memory number')
    SPEED = _('Playback speed')
    GAIN = GAIN
    MIC_INFO = _('Microphone input is record into a buffer for playback (right-clic on the objet to record sound)')
    MIC_PARA1 = _('Length of the memory')
    MIC_PARA2 = _('This parameter can not be modified')
    MIC_PARA3 = _('Speed playback changes duration and pitch of the sound')
    MIC_PARA4 = _('The volume of the sound')

    #Effects
    DELAY = _('Delay')
    FREQ = FREQ
    LFILTER = LFILTER
    FEEDBACK = _('Feedback')
    GAIN = GAIN
    DELAY_INFO = _('Delay is an audio effect that repeats the sound over and over.')
    DELAY_PARA1 = _('The speed of the delay.')
    DELAY_PARA2 = _('The brightness of the sound.')
    DELAY_PARA3 = _('The time it takes for the sound to go away.')
    DELAY_PARA4 = _('The volume of the sound.')

    DIST = _('Distortion')
    FREQ = FREQ
    RESON = _('Resonance')
    DISTL = _('Distortion Level')
    GAIN = GAIN
    DIST_INFO = _("Distortion is the deformation of a wave which creates harsh sounds.")
    DIST_PARA1 = _('The pitch of the distorted sound.')
    DIST_PARA2 = _('The amount of vibration the instrument has against itself.')
    DIST_PARA3 = _('The volume of the distorted sound.')
    DIST_PARA4 = _('The volume of the sound.')

    FILTER = _('Filter')
    FREQ = FREQ
    FSLOPE = FSLOPE
    FTYPE = _('Type')
    FILTER_TYPES = [_('Lowpass'), _('Highpass'), _('Bandpass')]
    GAIN = GAIN
    FILTER_INFO = _('An audio filter is designed to brighten, darken or color a sound.')
    FILTER_PARA1 = _('The point in the sound to be filtered.')
    FILTER_PARA2 = _('The size of the region affected by the filter.')
    FILTER_PARA3 = _('The type of filter used: lowpass = dark, highpass = bright, bandpass = colored.')
    FILTER_PARA4 = _('The volume of the sound.')

    RINGMOD = _('Ring Modulator')
    FREQ = FREQ
    MIX = _('Mix')
    WAVEFORM = WAVEFORM
    LFO_WAVEFORMS = LFO_WAVEFORMS
    GAIN = GAIN
    RINGMOD_INFO = _('Ring modulation is an audio effect that creates metallic sounds.')
    RINGMOD_PARA1 = _('The pitch of the ring modulator.')
    RINGMOD_PARA2 = _('The volume of the modulated sound.')
    RINGMOD_PARA3 = _('The shape of the wave used for modulation.')
    RINGMOD_PARA4 = _('The volume of the sound.')

    REVERB = _('Reverb')
    REVERBD = _('Length')
    REVERBF = _('Lowpass Filter')
    REVERBL = _('Reverb Level')
    GAIN = GAIN
    REVERB_INFO = _('Reverberation is the length a sound stays in a room.')
    REVERB_PARA1 = _('The size of the room.')
    REVERB_PARA2 = _('The brightness of the reverberated sound.')
    REVERB_PARA3 = _('The amount of reverb to be applied.')
    REVERB_PARA4 = _('The volume of the sound.')

    HARMON = _('Harmonizer')
    FREQ = FREQ
    DRYDELAY = _('Dry delay')
    MIX = MIX
    GAIN = GAIN
    HARMON_INFO = _('A harmonizer doubles the sound musically.')
    HARMON_PARA1 = _('The pitch of the doubled sound.')
    HARMON_PARA2 = _('The start time of the original sound.')
    HARMON_PARA3 = _('The balance between the original and the doubled sound.')
    HARMON_PARA4 = _('The volume of the sound.')

    EQ4BAND = _('Equalizer 4 bands')
    FREQ1 = _('Band one gain')
    FREQ2 = _('Band two gain')
    FREQ3 = _('Band three gain')
    FREQ4 = _('Band four gain')
    EQ4BAND_INFO = _('A 4 band equalizer chooses slices (bands) in the sound and makes them louder or softer.')
    EQ4BAND_PARA1 = _('The volume of band 1 (low).')
    EQ4BAND_PARA2 = _('The volume of band 2 (mid-low).')
    EQ4BAND_PARA3 = _('The volume of band 3 (mid-high).')
    EQ4BAND_PARA4 = _('The volume of band 4 (high).')

    CHORUS = _('Chorus')
    LFODEPTH = _('LFO Depth')
    LFOFREQ = _('LFO Frequency')
    DELAY = _('Delay')
    FEEDBACK = FEEDBACK
    CHORUS_INFO = _('The chorus effect plays copies of the same sound with a slight variation.')
    CHORUS_PARA1 = _('The volume of the LFO signal.')
    CHORUS_PARA2 = _('The pitch of the LFO signal.')
    CHORUS_PARA3 = _('The amount of delay between the two signals.')
    CHORUS_PARA4 = _('The volume of the sound.')

    SYNTHTYPES = [[LFO, RANDOM, ADSR, TRACKPADX, TRACKPADY],
                    [FM, BUZZ, VCO, PLUCK, NOISE, SAMPLE, VOICE, GRAIN, ADDSYNTH, MIC],
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
                    _('mic'): [DURATION, BIN, SPEED, GAIN, MIC_INFO, MIC_PARA1, MIC_PARA2, MIC_PARA3, MIC_PARA4],
                    _('wguide'): [FREQ, LFILTER, FEEDBACK, GAIN, DELAY_INFO, DELAY_PARA1, DELAY_PARA2, DELAY_PARA3, DELAY_PARA4],
                    _('distort'): [FREQ, RESON, DISTL, GAIN, DIST_INFO, DIST_PARA1, DIST_PARA2, DIST_PARA3, DIST_PARA4],
                    _('filter'): [FREQ, FSLOPE, FTYPE, GAIN, FILTER_INFO, FILTER_PARA1, FILTER_PARA2, FILTER_PARA3, FILTER_PARA4],
                    _('ring'): [FREQ, MIX, WAVEFORM, GAIN, RINGMOD_INFO, RINGMOD_PARA1, RINGMOD_PARA2, RINGMOD_PARA3, RINGMOD_PARA4],
                    _('reverb'): [REVERBD, REVERBF, REVERBL, GAIN, REVERB_INFO, REVERB_PARA1, REVERB_PARA2, REVERB_PARA3, REVERB_PARA4],
                    _('harmon'): [FREQ, DRYDELAY, MIX, GAIN, HARMON_INFO, HARMON_PARA1, HARMON_PARA2, HARMON_PARA3, HARMON_PARA4],
                    _('eq4band'): [FREQ1, FREQ2, FREQ3, FREQ4, EQ4BAND_INFO, EQ4BAND_PARA1, EQ4BAND_PARA2, EQ4BAND_PARA3, EQ4BAND_PARA4],
                    _('chorus'): [LFODEPTH, LFOFREQ, DELAY, FEEDBACK, CHORUS_INFO, CHORUS_PARA1, CHORUS_PARA2, CHORUS_PARA3, CHORUS_PARA4]}
