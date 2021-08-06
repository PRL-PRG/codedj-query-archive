from Framework.Constants import Constants
from Framework.CSound.Instrument import Instrument

class CSoundConstants:
    #PATHS
    SOUNDS_DIR = Constants.TAM_TAM_ROOT + "/Framework/CSound/Sounds"
    FILES_DIR = Constants.TAM_TAM_ROOT + "/Framework/CSound/Files"
    
    #SERVER
    SERVER_ADDRESS = "localhost"
    SERVER_PORT = 40002

    #COMMANDS
    LOAD_INSTRUMENT_COMMAND = "perf.InputMessage('f%d 0 0 -1 \"%s\" 0 0 0')\n"
    PLAY_NOTE_COMMAND = "perf.InputMessage('i %d.%d 0 %f %f 0.05 %f %f %d')\n"

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
    BD = "bd"
    CLAP = "clap"
    COW = "cow"
    CYMBAL = "cymbal"
    HHC = "hhc"
    HHO = "hho"
    RIDE = "ride"
    SNARE = "snare"
    TAMB = "tamb"
    TOM = "tom"
    WOOD = "wood"
    
    # plucked string
    GUIT = "guit"
    PIZZ = "pizz"
    
    # woodwind
    CLARINETTE = "clarinette"
    FLUTE = "flute"

    #INSTRUMENTS ( csound table, csound instrument )
    INSTRUMENT_TABLE_OFFSET = 300
    INSTRUMENTS = { CELLO : Instrument( 0, 101, 'low', 'melo' ),
                    VIOLIN : Instrument( 1, 101, 'high', 'melo' ),
                    TRUMPET : Instrument( 2, 101, 'high', 'melo' ),
                    OUNK1 : Instrument( 3, 103, 'mid', 'melo' ),
                    OUNK2 : Instrument( 4, 103, 'mid', 'melo' ),
                    OUNK3 : Instrument( 5, 103, 'mid', 'melo' ),
                    OUNK4 : Instrument( 6, 103, 'mid', 'melo' ),
                    GAM1 : Instrument( 7, 102, 'high', 'melo' ),
                    GAM2 : Instrument( 8, 102, 'high', 'melo' ),
                    GAM3 : Instrument( 9, 102, 'high', 'melo' ),
                    GAM4 : Instrument( 10, 102, 'high', 'melo' ),
                    GAM5 : Instrument( 11, 102, 'high', 'melo' ),
                    GAM6 : Instrument( 12, 102, 'high', 'melo' ),
                    GAM7 : Instrument( 13, 102, 'high', 'melo' ),
                    GAM8 : Instrument( 14, 102, 'high', 'melo' ),
                    GONG : Instrument( 15, 102, 'low', 'melo' ),
                    BD : Instrument( 16, 103, 'low', 'drum' ),
                    CLAP : Instrument( 17, 103, 'mid', 'drum' ),
                    COW : Instrument( 18, 103, 'mid', 'drum' ),
                    CYMBAL : Instrument( 19, 103, 'low', 'drum' ),
                    HHC : Instrument( 20, 103, 'high', 'drum' ),
                    HHO : Instrument( 21, 103, 'high', 'drum' ),
                    RIDE : Instrument( 22, 103, 'high', 'drum' ),
                    SNARE : Instrument( 23, 103, 'mid', 'drum' ),
                    TAMB : Instrument( 24, 103, 'mid', 'drum' ),
                    TOM : Instrument( 25, 103, 'low', 'drum' ),
                    WOOD : Instrument( 26, 103, 'high', 'drum' ),
                    GUIT : Instrument( 27, 102, 'mid', 'melo' ),
                    PIZZ : Instrument( 28, 102, 'high', 'melo' ),
                    CLARINETTE : Instrument( 29, 101, 'mid', 'melo' ),
                    FLUTE : Instrument( 30, 101, 'mid', 'melo' ) }
