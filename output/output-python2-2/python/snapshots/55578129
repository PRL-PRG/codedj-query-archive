from Framework.Constants import Constants
from Framework.CSound.Instrument import Instrument

class CSoundConstants:
    #PATHS
    SOUNDS_DIR = Constants.TAM_TAM_ROOT + "/Resources/Sounds"
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
    MIC_RECORDING_COMMAND =   "perf.InputMessage('i201 0 10 %d')\n"

    #SOUNDS
    
    # animals
    OUNK = "ounk"
    DOG = "dog"
    DUCK = "duck"
    BIRD = "bird"
    CAT = "cat"
    DUCK2 = "duck2"
    
    # synthesis
    ADD = "add"
    FM1 = "fm1"
    FM2 = "fm2"

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

    DRUM2DARBUKADOOM = "drum2darbukadoom"
    DRUM2DARBUKAFINGER = "drum2darbukafinger"
    DRUM2DARBUKAPIED = "drum2darbukapied"
    DRUM2DARBUKAPIEDSOFT = "drum2darbukapiedsoft"
    DRUM2DARBUKAROLL = "drum2darbukaroll"
    DRUM2DARBUKASLAP = "drum2darbukaslap"
    DRUM2DARBUKATAK = "drum2darbukatak"
    DRUM2HATFLANGER = "drum2hatflanger"
    DRUM2HATPIED = "drum2hatpied"

    DRUM2HATPIED2 = "drum2hatpied2"
    DRUM2TAMBOURINEPIED = "drum2tambourinepied"
    DRUM2TAMBOURINEPIEDSOFT = "drum2tambourinepiedsoft"
    DRUM2KIT = "drum2kit"

    DRUM3COWBELL = "drum3cowbell"
    DRUM3COWBELLTIP = "drum3cowbelltip"
    DRUM3CUP = "drum3cup"
    DRUM3DJEMBELOW = "drum3djembelow"
    DRUM3DJEMBEMID = "drum3djembemid"
    DRUM3DJEMBESIDESTICK = "drum3djembesidestick"
    DRUM3DJEMBESLAP = "drum3djembeslap"
    DRUM3DJEMBESTICKMID = "drum3djembestickmid"
    DRUM3METALSTAND = "drum3metalstand"
    DRUM3PEDALPERC = "drum3pedalperc"
    DRUM3RAINSTICK = "drum3rainstick"
    DRUM3TAMBOURINEHIGH = "drum3tambourinehigh"
    DRUM3TAMBOURINELOW = "drum3tambourinelow"
    DRUM3KIT = "drum3kit"

    # weird
    BOTTLE = "bottle"
    CLANG = "clang"
    LAUGH = "laugh"
    OW = "ow"
    SHEEP = "sheep"
    SPRING = "spring"
    WATER = "water"
    ZAP = "zap"

    # plucked string
    GUIT = "guit"
    KOTO = "koto"

    # perc
    MARACAS = "maracas"
    MARIMBA = "marimba"
    TRIANGLE = "triangle"
    
    # woodwind
    CLARINETTE = "clarinette"
    FLUTE = "flute"
    TRUMPET = 'trumpet'

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
                    PIANO: Instrument( 24, 102, MID, 'melo'),
                    DOG: Instrument( 25, 103, MID, 'melo'),
                    DUCK: Instrument( 26, 103, MID, 'melo'),
                    DRUM2DARBUKADOOM: Instrument( 27, 103, LOW, 'drum'),
                    DRUM2DARBUKAPIED: Instrument( 28, 103, LOW, 'drum'),
                    DRUM2DARBUKAPIEDSOFT: Instrument( 29, 103, LOW, 'drum'),
                    DRUM2HATFLANGER: Instrument( 30, 103, PUNCH, 'drum'),
                    DRUM2DARBUKATAK: Instrument( 31, 103, PUNCH, 'drum'),
                    DRUM2DARBUKAFINGER: Instrument( 32, 103, MID, 'drum'),
                    DRUM2DARBUKAROLL: Instrument( 33, 103, HIGH, 'drum'),
                    DRUM2DARBUKASLAP: Instrument( 34, 103, MID, 'drum'),
                    DRUM2HATPIED: Instrument( 35, 103, MID, 'drum'),
                    DRUM2TAMBOURINEPIED: Instrument( 36, 103, MID, 'drum'),
                    DRUM2HATPIED2: Instrument( 37, 103, HIGH, 'drum'),
                    DRUM2TAMBOURINEPIEDSOFT: Instrument( 38, 103, HIGH, 'drum'),
                    DRUM3COWBELL: Instrument( 39, 103, HIGH, 'drum'),
                    DRUM3COWBELLTIP: Instrument( 40, 103, MID, 'drum'),
                    DRUM3CUP: Instrument( 41, 103, HIGH, 'drum'),
                    DRUM3DJEMBELOW: Instrument( 42, 103, LOW, 'drum'),
                    DRUM3DJEMBEMID: Instrument( 43, 103, HIGH, 'drum'),
                    DRUM3DJEMBESIDESTICK: Instrument( 44, 103, MID, 'drum'),
                    DRUM3DJEMBESLAP: Instrument( 45, 103, LOW, 'drum'),
                    DRUM3DJEMBESTICKMID: Instrument( 46, 103, MID, 'drum'),
                    DRUM3METALSTAND: Instrument( 47, 103, MID, 'drum'),
                    DRUM3PEDALPERC: Instrument( 48, 103, LOW, 'drum'),
                    DRUM3RAINSTICK: Instrument( 49, 103, PUNCH, 'drum'),
                    DRUM3TAMBOURINEHIGH: Instrument( 50, 103, PUNCH, 'drum'),
                    DRUM3TAMBOURINELOW: Instrument( 51, 103, PUNCH, 'drum'),
                    ADD: Instrument(52, 101, MID, 'melo'),
                    FM1: Instrument(53, 101, MID, 'melo'),
                    FM2: Instrument(54, 101, MID, 'melo'),
                    BIRD: Instrument(55, 101, MID, 'melo'),
                    CAT: Instrument(56, 101, MID, 'melo'),
                    DUCK2: Instrument(57, 101, MID, 'melo'),
                    BOTTLE: Instrument(58, 101, MID, 'melo'),
                    CLANG: Instrument(59, 103, MID, 'melo'),
                    OW: Instrument(60, 103, MID, 'melo'),
                    SHEEP: Instrument(61, 103, MID, 'melo'),
                    SPRING: Instrument(62, 103, MID, 'melo'),
                    WATER: Instrument(63, 103, MID, 'melo'),
                    ZAP: Instrument(64, 101, MID, 'melo'),
                    TRUMPET: Instrument(65, 101, MID, 'melo'),
                    MARACAS: Instrument(66, 103, MID, "melo"),
                    MARIMBA: Instrument(67, 102, MID, "melo"),
                    TRIANGLE: Instrument(68, 102, MID, "melo"),
                    LAUGH: Instrument(69, 101, MID, 'melo') }

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

    DRUM2INSTRUMENTS = {   24 : DRUM2DARBUKADOOM,
                                                            26 : DRUM2DARBUKAPIED,
                                                            28 : DRUM2DARBUKAPIEDSOFT,
                                                            30 : DRUM2HATFLANGER,
                                                            32 : DRUM2DARBUKATAK,
                                                            34 : DRUM2DARBUKATAK,
                                                            36 : DRUM2DARBUKAFINGER,
                                                            38 : DRUM2DARBUKAROLL,
                                                            40 : DRUM2DARBUKASLAP,
                                                            42 : DRUM2HATPIED,
                                                            44 : DRUM2TAMBOURINEPIED,
                                                            46 : DRUM2HATPIED2,
                                                            48 : DRUM2TAMBOURINEPIEDSOFT } 

    DRUM3INSTRUMENTS = {   24 : DRUM3DJEMBELOW,
                                                            26 : DRUM3PEDALPERC,
                                                            28 : DRUM3DJEMBESLAP,
                                                            30 : DRUM3TAMBOURINEHIGH,
                                                            32 : DRUM3TAMBOURINELOW,
                                                            34 : DRUM3RAINSTICK,
                                                            36 : DRUM3DJEMBEMID,
                                                            38 : DRUM3DJEMBESIDESTICK,
                                                            40 : DRUM3DJEMBESTICKMID,
                                                            42 : DRUM3COWBELL,
                                                            44 : DRUM3COWBELLTIP,
                                                            46 : DRUM3CUP,
                                                            48 : DRUM3METALSTAND }              

    RECORDABLE_INSTRUMENTS = set( [ MIC1, MIC2, MIC3, MIC4 ] )
    RECORDABLE_INSTRUMENT_CSOUND_IDS = {  MIC1 : 7,
                                                                                                        MIC2 : 8,
                                                                                                        MIC3 : 9,
                                                                                                        MIC4 : 10 }
