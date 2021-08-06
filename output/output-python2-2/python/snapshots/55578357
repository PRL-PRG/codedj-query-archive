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
    PLAY_NOTE_COMMAND_MINUS_DELAY =       "perf.InputMessage('i %d.%d %s %f %f %f %f %f %d %f %f %d %f')\n"
    PLAY_NOTE_OFF_COMMAND =   "perf.InputMessage('i %d.%d .2 0.01 1. 0. 0. 0.5 300 0 0 0 0')\n"
    MIC_RECORDING_COMMAND =   "perf.InputMessage('i201 0 .5 %d')\n"

    #SOUNDS
    
    # funny
    OUNK = "ounk"

    
    # melodic percussion
    GAM = "gam"
    GONG = "gong"
    PIANO = "piano"
    
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
    KOTO = "koto"
    
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
    INSTRUMENTS = { 
                    OUNK : Instrument( 0, 103, MID, 'melo' ),
                    GAM : Instrument( 1, 102, HIGH, 'melo' ),
                    GONG : Instrument( 2, 102, LOW, 'melo' ),
                    GUIT : Instrument( 3, 102, MID, 'melo' ),
                    KOTO : Instrument( 4, 102, HIGH, 'melo' ),
                    CLARINETTE : Instrument( 5, 101, MID, 'melo' ),
                    FLUTE : Instrument( 6, 101, MID, 'melo' ),
                    MIC1: Instrument( 7, 101, MID, 'melo' ),
                    MIC2: Instrument( 8, 101, MID, 'melo' ),
                    MIC3: Instrument( 9, 101, MID, 'melo' ),
                    MIC4: Instrument( 10, 101, MID, 'melo' ),
                    DRUM1HATPEDAL: Instrument( 11, 103, MID, 'drum'),
                    DRUM1HATSHOULDER: Instrument( 12, 103, HIGH, 'drum'),
                    DRUM1HARDRIDE: Instrument( 13, 103, MID, 'drum'),
                    DRUM1RIDEBELL: Instrument( 14, 103, HIGH, 'drum'),
                    DRUM1SNARE: Instrument( 15, 103, MID, 'drum'),
                    DRUM1SNARERIMSHOT: Instrument( 16, 103, HIGH, 'drum'),
                    DRUM1SNARESIDESTICK: Instrument( 17, 103, MID, 'drum'),
                    DRUM1CRASH: Instrument( 18, 103, PUNCH, 'drum'),
                    DRUM1SPLASH: Instrument( 19, 103, PUNCH, 'drum'),
                    DRUM1TOM: Instrument( 20, 103, MID, 'drum'),
                    DRUM1FLOORTOM: Instrument( 21, 103, LOW, 'drum'),
                    DRUM1CHINE: Instrument( 22, 103, PUNCH, 'drum'),
                    DRUM1KICK: Instrument( 23, 103, LOW, 'drum'),
                    PIANO: Instrument( 24, 102, MID, 'melo') }

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
    RECORDABLE_INSTRUMENT_CSOUND_IDS = {  MIC1 : 7,
                                                                                                        MIC2 : 8,
                                                                                                        MIC3 : 9,
                                                                                                        MIC4 : 10 }
