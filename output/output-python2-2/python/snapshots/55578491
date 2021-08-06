from Framework.Constants import Constants
from Framework.CSound.Instrument import Instrument

class CSoundConstants:
    #PATHS
    SOUNDS_DIR = Constants.TAM_TAM_ROOT + "/Framework/CSound/Sounds"
    FILES_DIR = Constants.TAM_TAM_ROOT + "/Framework/CSound/Files"
    
    #SERVER
    SERVER_ADDRESS = "localhost"
    SERVER_PORT = 40002

    SERVER_REQUIRED = 0

    #COMMANDS
    LOAD_INSTRUMENT_COMMAND = "perf.InputMessage('f%d 0 0 -1 \"%s\" 0 0 0')\n"
    PLAY_NOTE_COMMAND =       "perf.InputMessage('i %d.%d %f %f %f %f %f %f %d %f %f %d %f')\n"
    PLAY_NOTE_COMMAND_MINUS_DELAY =       "perf.InputMessage('i %d.%s %f %f %f %f %f %f %d %f %f %d %f')\n"
    PLAY_NOTE_OFF_COMMAND =   "perf.InputMessage('i %d.%d .2 0.01 1. 0. 0. 0.5 300 0 0 0 0')\n"
    MIC_RECORDING_COMMAND =   "perf.InputMessage('i201 0 .5 %d')\n"

    #SOUNDS
    # bowed
    CELLO = "cello"
    VIOLIN = "violin"
    
    # brass
    TRUMPET = "trumpet"
    
    # funny
    OUNK1 = "ounk1"
    OUNK2 = "ounk2"
    OUNK3 = "ounk3"
    OUNK4 = "ounk4"
    
    # melodic percussion
    GAM1 = "gam1"
    GAM2 = "gam2"
    GAM3 = "gam3"
    GAM4 = "gam4"
    GAM5 = "gam5"
    GAM6 = "gam6"
    GAM7 = "gam7"
    GAM8 = "gam8"
    GONG = "gong"
    
    # non-melodic percussion
    DRUM1CHINE = "drum1chine"
    DRUM1CRASH = "drum1crash"
    DRUM1FLOORTOM = "drum1floortom"
    DRUM1HARDRIDE = "drum1hardride"
    DRUM1HATPEDAL = "drum1hatpedal"
    DRUM1HATSHOULDER = "drum1hatshoulder"
    DRUM1KICK = "drum1kick"
    DRUM1RIDEBELL = "drum1ridebell"
    DRUM1SNARE = "drum1snare"
    DRUM1SNARERIMSHOT = "drum1snarerimshot"
    DRUM1SNARESIDESTICK = "drum1snaresidestick"
    DRUM1SPLASH = "drum1splash"
    DRUM1TOM = "drum1tom"

    DRUM1KIT = "drum1kit"

    # plucked string
    GUIT = "guit"
    PIZZ = "pizz"
    
    # woodwind
    CLARINETTE = "clarinette"
    FLUTE = "flute"

    # recorded snds
    MIC1 = "mic1"
    MIC2 = "mic2"
    MIC3 = "mic3"
    MIC4 = "mic4"

    LOW, MID, HIGH, PUNCH = range( 4 )

    #INSTRUMENTS ( csound table, csound instrument, register, instrumentClass )
    INSTRUMENT_TABLE_OFFSET = 300
    INSTRUMENTS = { CELLO : Instrument( 0, 101, LOW, 'melo' ),
                    VIOLIN : Instrument( 1, 101, HIGH, 'melo' ),
                    TRUMPET : Instrument( 2, 101, HIGH, 'melo' ),
                    OUNK1 : Instrument( 3, 103, MID, 'melo' ),
                    OUNK2 : Instrument( 4, 103, MID, 'melo' ),
                    OUNK3 : Instrument( 5, 103, MID, 'melo' ),
                    OUNK4 : Instrument( 6, 103, MID, 'melo' ),
                    GAM1 : Instrument( 7, 102, HIGH, 'melo' ),
                    GAM2 : Instrument( 8, 102, HIGH, 'melo' ),
                    GAM3 : Instrument( 9, 102, HIGH, 'melo' ),
                    GAM4 : Instrument( 10, 102, HIGH, 'melo' ),
                    GAM5 : Instrument( 11, 102, HIGH, 'melo' ),
                    GAM6 : Instrument( 12, 102, HIGH, 'melo' ),
                    GAM7 : Instrument( 13, 102, HIGH, 'melo' ),
                    GAM8 : Instrument( 14, 102, HIGH, 'melo' ),
                    GONG : Instrument( 15, 102, LOW, 'melo' ),
                    GUIT : Instrument( 16, 102, MID, 'melo' ),
                    PIZZ : Instrument( 17, 102, HIGH, 'melo' ),
                    CLARINETTE : Instrument( 18, 101, MID, 'melo' ),
                    FLUTE : Instrument( 19, 101, MID, 'melo' ),
                    MIC1: Instrument( 20, 101, MID, 'melo' ),
                    MIC2: Instrument( 21, 101, MID, 'melo' ),
                    MIC3: Instrument( 22, 101, MID, 'melo' ),
                    MIC4: Instrument( 23, 101, MID, 'melo' ),
                    DRUM1HATPEDAL: Instrument( 25, 103, MID, 'drum'),
                    DRUM1HATSHOULDER: Instrument( 26, 103, HIGH, 'drum'),
                    DRUM1HARDRIDE: Instrument( 28, 103, MID, 'drum'),
                    DRUM1RIDEBELL: Instrument( 29, 103, HIGH, 'drum'),
                    DRUM1SNARE: Instrument( 30, 103, MID, 'drum'),
                    DRUM1SNARERIMSHOT: Instrument( 31, 103, HIGH, 'drum'),
                    DRUM1SNARESIDESTICK: Instrument( 32, 103, MID, 'drum'),
                    DRUM1CRASH: Instrument( 33, 103, PUNCH, 'drum'),
                    DRUM1SPLASH: Instrument( 34, 103, PUNCH, 'drum'),
                    DRUM1TOM: Instrument( 35, 103, MID, 'drum'),
                    DRUM1FLOORTOM: Instrument( 36, 103, LOW, 'drum'),
                    DRUM1CHINE: Instrument( 37, 103, PUNCH, 'drum'),
                    DRUM1KICK: Instrument( 38, 103, LOW, 'drum') }

    DRUM1INSTRUMENTS = {   24 :   DRUM1KICK,
                                                            26 : DRUM1FLOORTOM,
                                                            28 : DRUM1TOM,
                                                            30 : DRUM1CHINE,
                                                            32 : DRUM1SPLASH,  
                                                            34 : DRUM1CRASH,
                                                            36 : DRUM1SNARESIDESTICK,
                                                            38 : DRUM1SNARERIMSHOT,
                                                            40 : DRUM1SNARE,
                                                            42 : DRUM1RIDEBELL,
                                                            44 : DRUM1HARDRIDE,
                                                            46 : DRUM1HATSHOULDER,
                                                            48 : DRUM1HATPEDAL }

    RECORDABLE_INSTRUMENTS = set( [ MIC1, MIC2, MIC3, MIC4 ] )
    RECORDABLE_INSTRUMENT_CSOUND_IDS = {  MIC1 : 20,
                                                                                                        MIC2 : 21,
                                                                                                        MIC3 : 22,
                                                                                                        MIC4 : 23 }
