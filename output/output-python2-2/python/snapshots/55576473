import Config
from Util.CSoundClient import CSoundClient
from Generation.GenerationConstants import GenerationConstants

from Util.Clooper.SClient import *
from sugar import env

def CSound_loadInstruments( ):
    home_path = env.get_profile_path() + Config.PREF_DIR
    for instrumentSoundFile in Config.INSTRUMENTS.keys():
        if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
            fileName = home_path + '/' + instrumentSoundFile
        else:
            fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
        sc_instrumentLoad(instrumentId, fileName)

def CSound_playNote( loopMode, secs_per_tick,
        onset, 
        pitch, 
        amplitude, 
        pan, 
        duration, 
        trackId, 
        fullDuration = False, 
        attack = 0.002, 
        decay = 0.098, 
        reverbSend = 0.1, 
        filterType = 0, 
        filterCutoff = 1000,
        tied = False,
        overlap = False,
        instr = Config.FLUTE  ):

    if not loopMode: duration = secs_per_tick * duration

    if instr[0:4] == 'drum':
        if pitch in GenerationConstants.DRUMPITCH:
            key = GenerationConstants.DRUMPITCH[ pitch ]
        else: 
            key = pitch

        if instr == 'drum1kit':
            instr = Config.DRUM1INSTRUMENTS[ key ]
        if instr == 'drum2kit':
            instr = Config.DRUM2INSTRUMENTS[ key ]
        if instr == 'drum3kit':
            instr = Config.DRUM3INSTRUMENTS[ key ]
        pitch = 1
    else:
        pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]

        # condition for tied notes
        if Config.INSTRUMENTS[ instr ].csoundInstrumentId  == 101  and tied and fullDuration:
            duration= -1.0
        # condition for overlaped notes
        if Config.INSTRUMENTS[ instr ].csoundInstrumentId == 102 and overlap:
            duration += 1.0

    # condition for tied notes
    if Config.INSTRUMENTS[ instr].csoundInstrumentId  == Config.INST_TIED  and tied and fullDuration:
        duration = -1
    # condition for overlaped notes
    if Config.INSTRUMENTS[ instr ].csoundInstrumentId == Config.INST_PERC and overlap:
        duration = duration + 1.0
    if loopMode :
        sc_loop_addScoreEvent15( 'i',
                Config.INSTRUMENTS[ instr ].csoundInstrumentId + 0.1,# trackId * 0.01,
                onset,
                duration,
                pitch,
                reverbSend,
                amplitude,
                pan,
                Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[instr].instrumentId,
                max(attack*duration, 0.002),
                max(decay *duration, 0.002),
                filterType,
                filterCutoff,
                Config.INSTRUMENTS[ instr ].loopStart,
                Config.INSTRUMENTS[ instr ].loopEnd,
                Config.INSTRUMENTS[ instr ].crossDur )
    else:
        sc_scoreEvent15( 'i',
                Config.INSTRUMENTS[ instr ].csoundInstrumentId + 0.1,# trackId * 0.01,
                onset * secs_per_tick,
                duration,
                pitch,
                reverbSend,
                amplitude,
                pan,
                Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[instr].instrumentId,
                max(attack*duration, 0.002),
                max(decay *duration, 0.002),
                filterType,
                filterCutoff,
                Config.INSTRUMENTS[ instr ].loopStart,
                Config.INSTRUMENTS[ instr ].loopEnd,
                Config.INSTRUMENTS[ instr ].crossDur )

class CSoundNote :
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self,
            onset, 
            pitch, 
            amplitude, 
            pan, 
            duration, 
            trackId, 
            fullDuration = False, 
            instrument = Config.FLUTE, 
            attack = 0.002, 
            decay = 0.098, 
            reverbSend = 0.1, 
            filterType = 0, 
            filterCutoff = 1000,
            tied = False,
            overlap = False,
            instrumentFlag = Config.FLUTE  ):
        
        self.onset = onset
        self.pitch = pitch
        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackId = trackId
        self.instrument = instrument
        self.fullDuration = fullDuration
        self.attack = attack
        self.decay = decay
        self.reverbSend = reverbSend
        self.filterType = filterType
        self.filterCutoff = filterCutoff
        self.tied = tied
        self.overlap = overlap
        if self.instrument == 'drum1kit':
            self.instrumentFlag = Config.DRUM1INSTRUMENTS[ self.pitch ]
        else:
            self.instrumentFlag = self.instrument
        self.nchanges = 0

    def __getstate__(self):
        return {'onset': self.onset,
                'pitch': self.pitch,
                'amplitude': self.amplitude,
                'pan': self.pan,
                'duration': self.duration,
                'trackId': self.trackId,
                'instrument': self.instrument,
                'fullDuration': self.fullDuration,
                'attack': self.attack,
                'decay': self.decay,
                'reverbSend': self.reverbSend,
                'filterType': self.filterType,
                'filterCutoff': self.filterCutoff,
                'tied': self.tied,
                'overlap': self.overlap,
                'instrumentFlag': self.instrumentFlag }

    def __setstate__(self,dict):
        self.onset = dict['onset']
        self.pitch = dict['pitch']
        self.amplitude = dict['amplitude']
        self.pan = dict['pan']
        self.duration = dict['duration']
        self.trackId = dict['trackId']
        self.instrument = dict['instrument']
        self.fullDuration = dict['fullDuration']
        self.attack = dict['attack']
        self.decay = dict['decay']
        self.reverbSend = dict['reverbSend']
        self.filterType = dict['filterType']
        self.filterCutoff = dict['filterCutoff']
        self.tied = dict['tied']
        self.overlap = dict['overlap']
        self.instrumentFlag = dict['instrumentFlag']
        self.nchanges = 0

    def clone( self ):
        return CSoundNote( self.onset, self.pitch, self.amplitude, self.pan, 
                           self.duration, self.trackId, self.fullDuration,  self.instrument, 
                           self.attack, self.decay, self.reverbSend, self.filterType, self.filterCutoff, self.tied, self.overlap, self.instrumentFlag )

    def getText( self, secs_per_tick, delay ):
        if secs_per_tick > 1 : raise 'invalid secs_per_tick'
        if self.instrument == 'drum1kit':
            if GenerationConstants.DRUMPITCH.has_key( self.pitch ):
                self.pitch = GenerationConstants.DRUMPITCH[ self.pitch ]

            self.instrumentFlag = Config.DRUM1INSTRUMENTS[ self.pitch ]
            newPitch = 1
        else:
            self.instrumentFlag = self.instrument
            newPitch = pow( GenerationConstants.TWO_ROOT_TWELVE, self.pitch - 36 )

        newDuration = secs_per_tick * self.duration

        # condition for tied notes
        if Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId  == 101  and self.tied and self.fullDuration:
            newDuration = -1
        # condition for overlaped notes
        if Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId == 102 and self.overlap:
            newDuration = oneTickDuration * self.duration + 1.

        if True: newAmplitude = self.amplitude * 0.8
        else : newAmplitude = self.amplitude * music_volume_get( self.trackId )

        newAttack = newDuration * self.attack
        if newAttack <= 0.002:
            newAttack = 0.002

        newDecay = newDuration * self.decay
        if newDecay <= 0.002:
            newDecay = 0.002

        return Config.PLAY_NOTE_COMMAND %  ( \
                Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId, 
                self.trackId, 
                delay,
                newDuration, 
                newPitch, 
                self.reverbSend, 
                newAmplitude, 
                self.pan, 
                Config.INSTRUMENT_TABLE_OFFSET+Config.INSTRUMENTS[self.instrumentFlag].instrumentId,
                newAttack,
                newDecay,
                self.filterType,
                self.filterCutoff,
                Config.INSTRUMENTS[ self.instrumentFlag ].loopStart,
                Config.INSTRUMENTS[ self.instrumentFlag ].loopEnd,
                Config.INSTRUMENTS[ self.instrumentFlag ].crossDur )

    def playNow(self, secs_per_tick):
        CSound_playNote( False, secs_per_tick,
                self.onset, 
                self.pitch,
                self.amplitude,
                self.pan,
                self.duration, 
                self.trackId, 
                self.fullDuration, 
                self.attack,
                self.decay,
                self.reverbSend,
                self.filterType,
                self.filterCutoff,
                self.tied,
                self.overlap,
                self.instrumentFlag)
    def playLoop(self):
        CSound_playNote( True, 1.0,
                self.onset, 
                self.pitch,
                self.amplitude,
                self.pan,
                self.duration, 
                self.trackId, 
                self.fullDuration, 
                self.attack,
                self.decay,
                self.reverbSend,
                self.filterType,
                self.filterCutoff,
                self.tied,
                self.overlap,
                self.instrumentFlag)

