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
    PLAY_NOTE_COMMAND = "perf.InputMessage('i %d.%d 0 %f %f %f %f %f %d')\n"
    MIC_RECORDING_COMMAND = "perf.InputMessage('i201 0 .5 %d')\n"

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

    # recorded snds
    ZON1 = "zon1"
    ZON2 = "zon2"
    ZON3 = "zon3"
    ZON4 = "zon4"

    LOW, MID, HIGH = range( 3 )

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
                    BD : Instrument( 16, 103, LOW, 'drum' ),
                    CLAP : Instrument( 17, 103, MID, 'drum' ),
                    COW : Instrument( 18, 103, MID, 'drum' ),
                    CYMBAL : Instrument( 19, 103, LOW, 'drum' ),
                    HHC : Instrument( 20, 103, HIGH, 'drum' ),
                    HHO : Instrument( 21, 103, HIGH, 'drum' ),
                    RIDE : Instrument( 22, 103, HIGH, 'drum' ),
                    SNARE : Instrument( 23, 103, MID, 'drum' ),
                    TAMB : Instrument( 24, 103, MID, 'drum' ),
                    TOM : Instrument( 25, 103, LOW, 'drum' ),
                    WOOD : Instrument( 26, 103, HIGH, 'drum' ),
                    GUIT : Instrument( 27, 102, MID, 'melo' ),
                    PIZZ : Instrument( 28, 102, HIGH, 'melo' ),
                    CLARINETTE : Instrument( 29, 101, MID, 'melo' ),
                    FLUTE : Instrument( 30, 101, MID, 'melo' ),
                    ZON1: Instrument( 31, 101, MID, 'melo' ),
                    ZON2: Instrument( 32, 101, MID, 'melo' ),
                    ZON3: Instrument( 33, 101, MID, 'melo' ),
                    ZON4: Instrument( 34, 101, MID, 'melo' ) }
    
    RECORDABLE_INSTRUMENTS = set( [ ZON1, ZON2, ZON3, ZON4 ] )
    RECORDABLE_INSTRUMENT_CSOUND_IDS = { ZON1 : 31,
                                         ZON2 : 32,
                                         ZON3 : 33,
                                         ZON4 : 34 }
