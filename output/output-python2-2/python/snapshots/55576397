import sys
import time
from Player.RythmGenerator import *
from sugar import env

#from Util.Sound import Sound

import Config
from Util.Clooper.SClient import *

def load_instruments( ):
    home_path = env.get_profile_path() + Config.PREF_DIR
    for instrumentSoundFile in Config.INSTRUMENTS.keys():
        if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
            fileName = home_path + '/' + instrumentSoundFile
        else:
            fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
        sc_inputMessage( Config.RAW_LOAD_INSTRUMENT_COMMAND % (instrumentId, fileName) )



def regenerate(kit, beat, regularity, reverb):
    def flatten(ll):
        rval = []
        for l in ll:
            rval += l
        return rval

    notesList = [(x.onset, x) for x in flatten( generator(kit, beat, regularity, reverb, None))]
    notesList.sort()
    return notesList

if __name__ == "__main__":     
    sc_initialize( Config.TAM_TAM_ROOT + '/Resources/univorc.csd' )
    sc_setMasterVolume(50.0)

    load_instruments()
    time.sleep(0.2)

    inst = 'sitar'
    while True:
        i = raw_input()

        if i == 'w':   # stop note
            n = CSoundNote(0.0, 0, 1.0, 0.0, 1.0, 1.0, instrument = inst, decay=0.7)
            n.playNow(1.0)
        elif i == 'q': # quit
            break
        elif i == 'b': # generate and start a beat
            sc_loop_setNumTicks( 4 * Config.TICKS_PER_BEAT)
            sc_loop_clear()
            sc_loop_setTickDuration(1.0 / 23.0)
            notesList = regenerate('drum1kit', 4, 0.75, 0.1)
            for (o,n) in notesList:
                n.playLoop()
            sc_loop_setTick(sc_loop_getTick())
            sc_loop_playing(1)
            print 'playing true!'
        elif i == 's': # stop a beat
            sc_loop_playing(0)
        elif i == 'u': # start csound
            sc_start()
            load_instruments()
            time.sleep(0.2)
            sc_setMasterVolume(50.0)
        elif i == 'd': # stop  csound
            sc_stop()
        else:          # play a sitar
            n = CSoundNote(0.0, 24 + 4, 1.0, 0.0, -1.0, 1.0, instrument = inst)
            n.playNow(1.0)

    sc_destroy()



sys.exit(0);





class Music :
    def __init__(self):
        self.secs_per_tick = 60.0 / (Config.PLAYER_TEMPO * Config.TICKS_PER_BEAT)
        self.loopMode = False

    def setTempo(bpm):
        self.secs_per_tick = 60.0 / (bpm * Config.TICKS_PER_BEAT)

    def playNote( self,
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

        duration = self.secs_per_tick * duration

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
        if (self.loopMode):
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
