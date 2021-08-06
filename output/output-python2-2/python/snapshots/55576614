# -*- coding: utf-8 -*-

import os

TAM_TAM_ROOT = os.path.dirname(os.path.abspath(__file__))
print 'INFO: loaded TAMTAM_ROOT=%s' % TAM_TAM_ROOT

#BUFFERING
NOTELOOPER_HORIZON = 0.150
NOTELOOPER_SLEEP = 0.05

#PATHS
SOUNDS_DIR = TAM_TAM_ROOT + "/Resources/Sounds"
FILES_DIR = TAM_TAM_ROOT + "/Resources"
    
#SERVER
SERVER_ADDRESS = "localhost"
SERVER_PORT = 40007

SERVER_REQUIRED = 0

INIT_ATTEMPTS = 2
INIT_DELAY = 1.0

##############
## SOUNDS
##############
class Instrument:
    def __init__( self, instrumentId, csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur ):
        self.instrumentId = instrumentId
        self.csoundInstrumentId = csoundInstrumentId
        self.instrumentRegister = instrumentRegister
        self.soundClass = soundClass
        self.category = category
        self.loopStart = loopStart
        self.loopEnd = loopEnd
        self.crossDur = crossDur

# animals
OUNK = "ounk"
DOG = "dog"
DUCK = "duck"
BIRD = "bird"
CAT = "cat"
DUCK2 = "duck2"
HORSE = "horse"

# synthesis
FM2 = "fm2"

# melodic percussion
GAM = "gam"
GONG = "gong"
PIANO = "piano"
RHODES = "rhodes"
KALIMBA = "kalimba"

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
CLING = "cling"
DOOR = "door"
LAUGH = "laugh"
OW = "ow"
SHEEP = "sheep"
TCHIWO = "tchiwo"
WATER = "water"
ZAP = "zap"
DICEINST = "diceinst"
GUIDICE1 = "guidice1"
GUIDICE2 = "guidice2"
GUIDICE3 = "guidice3"
GUIDICE4 = "guidice4"
GUIDICE5 = "guidice5"
GUIDICE6 = "guidice6"
GUIDICE7 = "guidice7"
GUIDICE8 = "guidice8"
GUIDICE9 = "guidice9"
GUIDICE10 = "guidice10"

# string
ACGUIT = "acguit"
BASSE = "basse"
GUIT = "guit"
KOTO = "koto"
MANDO = "mando"
SITAR = "sitar"
VIOLIN = "violin"

# perc
MARACAS = "maracas"
MARIMBA = "marimba"
TRIANGLE = "triangle"

# wind
CLARINETTE = "clarinette"
FLUTE = "flute"
TRUMPET = 'trumpet'
VOIX = "voix"
DIDJERIDU = "didjeridu"
HARMONICA = "harmonica"
HARMONIUM = "harmonium"
OCARINA = "ocarina"
SAXO = "saxo"
SHENAI = "shenai"
TUBA = "tuba"

# recorded snds
MIC1 = "mic1"
MIC2 = "mic2"
MIC3 = "mic3"
MIC4 = "mic4"

# synthLab snds
LAB1 = "lab1"
LAB2 = "lab2"
LAB3 = "lab3"
LAB4 = "lab4"

LOW, MID, HIGH, PUNCH = range( 4 )

# Sounds categories: musicInst, animals, drum, people, electronic, concret, mic
#INSTRUMENTS ( csound table, csound instrument, register, instrumentClass, category )
INSTRUMENT_TABLE_OFFSET = 5000
INST_FREE = 5000
INST_TIED = 5001
INST_PERC = 5002
INST_SIMP = 5003

INSTRUMENTS = { 
                OUNK :                    Instrument(  0, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                GAM :                     Instrument(  1, INST_TIED, HIGH, 'melo', 'musicInst', .69388, .7536, .02922 ),
                GONG :                    Instrument(  2, INST_SIMP, LOW, 'melo', 'musicInst', 0, 0, 0 ),
                GUIT :                    Instrument(  3, INST_TIED, MID, 'melo', 'musicInst', .08592, .75126, .33571 ),
                KOTO :                    Instrument(  4, INST_TIED, HIGH, 'melo', 'musicInst', .56523, .70075, .05954 ),
                CLARINETTE :              Instrument(  5, INST_TIED, MID, 'melo', 'musicInst', .57905, .73319, .04934 ),
                FLUTE :                   Instrument(  6, INST_TIED, MID, 'melo', 'musicInst', .47169, .53693, .02481 ),
                MIC1:                     Instrument(  7, INST_TIED, MID, 'melo', 'mic', .1, .9, .1 ),
                MIC2:                     Instrument(  8, INST_TIED, MID, 'melo', 'mic', .1, .9, .1 ),
                MIC3:                     Instrument(  9, INST_TIED, MID, 'melo', 'mic', .1, .9, .1 ),
                MIC4:                     Instrument( 10, INST_TIED, MID, 'melo', 'mic', .1, .9, .1 ),
                DRUM1HATPEDAL:            Instrument( 11, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM1HATSHOULDER:         Instrument( 12, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM1HARDRIDE:            Instrument( 13, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM1RIDEBELL:            Instrument( 14, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM1SNARE:               Instrument( 15, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM1SNARESIDESTICK:      Instrument( 16, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM1CRASH:               Instrument( 17, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM1SPLASH:              Instrument( 18, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM1TOM:                 Instrument( 19, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM1FLOORTOM:            Instrument( 20, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0),
                DRUM1CHINE:               Instrument( 21, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM1KICK:                Instrument( 22, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                PIANO:                    Instrument( 23, INST_TIED, MID, 'melo', 'musicInst', 2.39418, 2.53339, .01323 ),
                DOG:                      Instrument( 24, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                DUCK:                     Instrument( 25, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                DRUM2DARBUKADOOM:         Instrument( 26, INST_SIMP, LOW, 'drum', 'drum', 0, 0 ,0 ),
                DRUM2DARBUKAPIED:         Instrument( 27, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                DRUM2DARBUKAPIEDSOFT:     Instrument( 28, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                DRUM2HATFLANGER:          Instrument( 29, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM2DARBUKATAK:          Instrument( 30, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM2DARBUKAFINGER:       Instrument( 31, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM2DARBUKAROLL:         Instrument( 32, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM2DARBUKASLAP:         Instrument( 33, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM2HATPIED:             Instrument( 34, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM2TAMBOURINEPIED:      Instrument( 35, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM2HATPIED2:            Instrument( 36, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM2TAMBOURINEPIEDSOFT:  Instrument( 37, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3COWBELL:             Instrument( 38, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3COWBELLTIP:          Instrument( 39, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0),
                DRUM3CUP:                 Instrument( 40, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3DJEMBELOW:           Instrument( 41, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                DRUM3DJEMBEMID:           Instrument( 42, INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3DJEMBESIDESTICK:     Instrument( 43, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM3DJEMBESLAP:          Instrument( 44, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                DRUM3DJEMBESTICKMID:      Instrument( 45, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM3METALSTAND:          Instrument( 46, INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 ),
                DRUM3PEDALPERC:           Instrument( 47, INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 ),
                DRUM3RAINSTICK:           Instrument( 48, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3TAMBOURINEHIGH:      Instrument( 49, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                DRUM3TAMBOURINELOW:       Instrument( 50, INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 ),
                HARMONICA:                Instrument( 51, INST_TIED, MID, 'melo', 'electronic', .1531, .19188, .01792 ),
                FM2:                      Instrument( 52, INST_TIED, MID, 'melo', 'electronic', .43443, .5784, .05127 ),
                BIRD:                     Instrument( 53, INST_TIED, MID, 'melo', 'animals', .1, 1, .05 ),
                CAT:                      Instrument( 54, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                DUCK2:                    Instrument( 55, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                BOTTLE:                   Instrument( 56, INST_TIED, MID, 'melo', 'concret', .20532, .41064, .05292 ),
                CLANG:                    Instrument( 57, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                OW:                       Instrument( 58, INST_SIMP, MID, 'melo', 'people', 0, 0, 0 ),
                SHEEP:                    Instrument( 59, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                WATER:                    Instrument( 60, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                ZAP:                      Instrument( 61, INST_TIED, MID, 'melo', 'electronic', .299, .7323, .09895 ),
                TRUMPET:                  Instrument( 62, INST_TIED, MID, 'melo', 'musicInst', .39934, .45537, .02729),
                MARACAS:                  Instrument( 63, INST_SIMP, MID, "melo", 'musicInst', 0, 0, 0),
                MARIMBA:                  Instrument( 64, INST_TIED, MID, "melo", 'musicInst', .26545, .33098, .03087),
                TRIANGLE:                 Instrument( 65, INST_TIED, MID, "melo", 'musicInst', 1.21002, 1.31805, .01268),
                LAUGH:                    Instrument( 66, INST_SIMP, MID, 'melo', 'people', 0, 0, 0 ),
                VOIX:                     Instrument( 67, INST_TIED, MID, 'melo', 'people', .89608, .96092, .02343 ), 
                CLING:                    Instrument( 68, INST_TIED, MID, 'melo', 'electronic', .09096, .7878, .18026 ),
                TCHIWO:                   Instrument( 69, INST_TIED, MID, 'melo', 'electronic', .91515, 1.00094, .02122 ),
                DOOR:                     Instrument( 70, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                BASSE :                   Instrument( 71, INST_TIED, MID, 'melo', 'musicInst', .58455, .67433, .03638 ),
                ACGUIT :                  Instrument( 72, INST_TIED, MID, 'melo', 'musicInst', .58503, .8667, .13699 ),
                DICEINST :                   Instrument( 73, INST_SIMP, MID, 'melo', 'musicInst', 0, 0, 0 ),
                DIDJERIDU :               Instrument( 74, INST_TIED, LOW, 'melo', 'musicInst', .55669, 1.73704, .09178 ),
                HARMONIUM :               Instrument( 75, INST_TIED, MID, 'melo', 'musicInst', .04674, .41073, .18384 ),
                HORSE :                   Instrument( 76, INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 ),
                KALIMBA :                 Instrument( 77, INST_TIED, MID, 'melo', 'musicInst', .20751, .30161, .04658 ),
                MANDO :                   Instrument( 78, INST_TIED, MID, 'melo', 'musicInst', .50167, .54401, .01984 ),
                OCARINA :                 Instrument( 79, INST_TIED, MID, 'melo', 'musicInst', .12122, .18965, .02205 ),
                RHODES :                  Instrument( 80, INST_TIED, MID, 'melo', 'musicInst', .65013, .71429, .02205 ),
                SAXO :                    Instrument( 81, INST_TIED, MID, 'melo', 'musicInst', .53722, .6583, .05264 ),
                SHENAI :                  Instrument( 82, INST_TIED, MID, 'melo', 'musicInst', .29003, .33072, .00634 ),
                SITAR :                   Instrument( 83, INST_TIED, MID, 'melo', 'musicInst', .63187, .67882, .01654 ),
                TUBA :                    Instrument( 84, INST_TIED, LOW, 'melo', 'musicInst', .51063, .58384, .035 ),
                VIOLIN :                  Instrument( 85, INST_TIED, MID, 'melo', 'musicInst', .55094, .82054, .14498 ),
                LAB1 :                    Instrument( 86, INST_SIMP, MID, 'melo', 'musicInst', 0, 0, 0 ),
                LAB2 :                    Instrument( 87, INST_SIMP, MID, 'melo', 'musicInst', 0, 0, 0 ),
                LAB3 :                    Instrument( 88, INST_SIMP, MID, 'melo', 'musicInst', 0, 0, 0 ),
                LAB4 :                    Instrument( 89, INST_SIMP, MID, 'melo', 'musicInst', 0, 0, 0 ),
                GUIDICE1:		      Instrument( 90, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE2:		      Instrument( 91, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE3:		      Instrument( 92, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE4:		      Instrument( 93, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE5:		      Instrument( 94, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE6:		      Instrument( 95, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE7:		      Instrument( 96, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE8:		      Instrument( 97, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE9:		      Instrument( 98, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 ),
                GUIDICE10:		      Instrument( 99, INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )}
                

DRUM1INSTRUMENTS = {   24 :   DRUM1KICK,
                                                        26 : DRUM1FLOORTOM,
                                                        28 : DRUM1TOM,
                                                        30 : DRUM1CHINE,
                                                        32 : DRUM1SPLASH,  
                                                        34 : DRUM1CRASH,
                                                        36 : DRUM1SNARESIDESTICK,
                                                        38 : DRUM1SNARESIDESTICK,
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

#COMMANDS
LOAD_INSTRUMENT_COMMAND = \
        "perf.InputMessage('f%d 0 0 -1 \"%s\" 0 0 0')\n"
PLAY_NOTE_COMMAND = \
        "perf.InputMessage('i %d.%d %f %f %f %f %f %f %d %f %f %d %f %f %f %f')\n"
#TODO: add the extra three params to COMMAND_MINUS_DELAY, and instrument 5777 in univorc.csd
PLAY_NOTE_COMMAND_MINUS_DELAY = \
        "perf.InputMessage('i 5777 0.0 0.001 %d.%d %s %f %f %f %f %f %d %f %f %d %f')\n"
PLAY_NOTE_OFF_COMMAND =  \
        "perf.InputMessage('i %s.%s .2 0.01 1. 0. 0. 0.5 %d 0 0 0 0')\n" \
        % ('%d', '%d', INSTRUMENT_TABLE_OFFSET )
MIC_RECORDING_COMMAND = \
        "perf.InputMessage('i5201 0 10 %d')\n"
UNLOAD_TABLES_COMMAND = \
        "perf.InputMessage('i%d 0 0.1 %d')\n" % (INST_FREE, len(INSTRUMENTS))




#################
## GUI CONSTANTS
#################

LANGUAGE = 'En' 
IMAGE_ROOT = TAM_TAM_ROOT + '/Resources/Images/'

NOTE_HEIGHT = 6     # pixels
NOTE_BORDER_SIZE = 1
NOTE_BORDER_SIZE_DIV2 = NOTE_BORDER_SIZE/2.0
MAIN_WINDOW_PADDING = 5
TRACK_SPACING = 1
BORDER_SIZE = 2
BORDER_SIZE_DIV2 = BORDER_SIZE/2.0
BORDER_SIZE_MUL2 = BORDER_SIZE*2
BEAT_LINE_SIZE = 1
BEAT_LINE_SIZE_DIV2 = BEAT_LINE_SIZE/2.0
PLAYHEAD_SIZE = 2
PLAYHEAD_SIZE_DIV2 = PLAYHEAD_SIZE/2.0
        
INST_BCK_COLOR = '#979DA8'
PANEL_BCK_COLOR =  '#FFFFFF'
PANEL_COLOR = '#707F93'
PANEL_RADIUS = 10

PAGE_BORDER_SIZE = 2
PAGE_SELECTED_BORDER_SIZE = 5
PAGE_WIDTH = 100
PAGE_HEIGHT = 25

PAGE_THUMBNAIL_WIDTH = 70
PAGE_THUMBNAIL_WIDTH_DIV2 =     PAGE_THUMBNAIL_WIDTH/2
PAGE_THUMBNAIL_HEIGHT = 50
PAGE_THUMBNAIL_PADDING = 4
PAGE_THUMBNAIL_PADDING_MUL2 = PAGE_THUMBNAIL_PADDING*2
PAGE_THUMBNAIL_PADDING_DIV2 = PAGE_THUMBNAIL_PADDING/2
    
NUMBER_OF_PAGE_BANK_ROWS = 2
NUMBER_OF_PAGE_BANK_COLUMNS = 20


# hardware keycodes for mod keys
MOD_LSHIFT = 50
MOD_RSHIFT = 62
MOD_LCTRL = 37
MOD_RCTRL = 109
MOD_LALT = 64
MOD_RALT = 113



########
##   Things that don't belong!
#######
class _ModKeys:
    def __init__( self ):        
        self.shiftDown = False
        self.ctrlDown = False
        self.altDown = False

    def keyPress( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = True
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = True
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = True   

    def keyRelease( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = False
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = False
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = False        

ModKeys = _ModKeys()


############
## EDIT DEFAULTS
############

#DEFAULTS
PLAYER_TEMPO = 120
PLAYER_TEMPO_LOWER = 40
PLAYER_TEMPO_UPPER = 200
DEFAULT_VOLUME = 80

#NUMERICAL CONSTANTS
NUMBER_OF_POSSIBLE_PITCHES = 25.0
MINIMUM_PITCH = 24.0
MAXIMUM_PITCH = MINIMUM_PITCH + NUMBER_OF_POSSIBLE_PITCHES - 1
MINIMUM_NOTE_DURATION = 1 # ticks
MS_PER_MINUTE = 60000.0
TICKS_PER_BEAT = 12
NUMBER_OF_TRACKS = 5
NUMBER_OF_PAGES = 2

MINIMUM_AMPLITUDE = 0
MAXIMUM_AMPLITUDE = 1


####################
## ToolTips
####################
class Tooltips:
    
    #English
    if LANGUAGE == 'En':
        #miniTamTam
        VOL = 'Volume'
        REV = 'Reverb'
        PLAY = 'Play / Stop'
        STOP = 'Stop'
        SEQ = 'Sequencer'
        GEN = 'Generate'
        COMPL = 'Complexity of beat'
        BEAT = 'Beats per bar'
        TEMPO = 'Tempo'
        JAZZ = 'Jazz / Rock Kit'
        AFRI = 'African Kit'
        ARAB = 'Arabic Kit'
        RECMIC = 'Record using the microphone'
        RECLAB = 'Open SynthLab to create sounds'
        
        #Synthlab
        SOURCE = 'Source'
        EFFECT = 'Effect'
        CONTROL = 'Control'
        SOUNDOUT = 'Sound Output'
        SOUNDDUR = 'Sound Duration'
        SAVE = 'Save'
        LOAD = 'Load'
        SAVEMINI = 'Save to miniTamTam'
        CLOSE = 'Close'
        RESET = 'Reset'
        
        #Controls
        LFO = 'LFO'
        AMP = 'Amplitude'
        FREQ = 'Frequency'
        WAVEFORM = 'Waveform'
        LFO_WAVEFORMS = ['Sine', 'Triangle', 'Bi-Square', 'Uni-Square', 'Sawtooth', 'Sawtooth-down']
        OFFSET = 'Offset'
        
        RANDOM = 'Random'
        MIN = 'Minimum'
        MAX = 'Maximum'
        FREQ = FREQ
        SEED = 'Seed'
        
        ADSR = 'Envelope'
        ATTACK = 'Attack'
        DECAY = 'Decay'
        SUSTAIN = 'Sustain'
        RELEASE = 'Release'
        
        #Source
        FM = 'Fequency Modulator'
        CAR = 'Carrier Frequency'
        MOD = 'Modulator Frequency'
        INDEX = 'Index'
        GAIN = 'Gain'
        
        BUZZ = 'Buzz'
        FREQ = FREQ
        NHARM = 'Number of harmonics'
        FSLOPE = 'Filter Slope'
        GAIN = GAIN
        
        VCO = 'Voltage Controlled Oscillator'
        FREQ = FREQ
        WAVEFORM = WAVEFORM
        VCO_WAVEFORMS = ['Sawtooth', 'Square', 'Triangle']
        FSLOPE = FSLOPE
        GAIN = GAIN
        
        PLUCK = 'Pluck'
        FREQ = FREQ
        LFILTER = 'Lowpass Filter'
        VIBRATO = 'Vibrato'
        GAIN = GAIN
        
        NOISE = 'Noise'
        NOISETYPE = 'Type'
        NOISE_TYPES = ['White', 'Pink', 'Gauss']
        FREQ = FREQ
        BANDWITH = 'Bandwith'
        GAIN = GAIN
        
        SAMPLE = 'Sound Sample'
        FREQ = FREQ
        SAMPLEN = 'Sample Number'
        SAMPLE_NAMES = [name for i in range(len(INSTRUMENTS)) for name in INSTRUMENTS.keys() if INSTRUMENTS[ name ].instrumentId == i]
        LFILTER = LFILTER
        GAIN = GAIN
        
        VOICE = 'Voice'
        FREQ = FREQ
        VOWEL = 'Vowel'
        VOWEL_TYPES = ['i', 'e', 'ee', 'a', 'u', 'o1', 'o2', 'oa', 'oe']
        VIBRATO = VIBRATO
        GAIN = GAIN
        
        #Effects
        DELAY = 'Delay'
        FREQ = FREQ
        LFILTER = LFILTER
        FEEDBACK = 'Feedback'
        GAIN = GAIN
        
        DIST = 'Distortion'
        FREQ = FREQ
        RESON = 'Resonance'
        DISTL = 'Distotion Level'
        GAIN = GAIN
        
        FILTER = 'Filter'
        FREQ = FREQ
        FSLOPE = FSLOPE
        FTYPE = 'Type'
        FILTER_TYPES = ['Lowpass', 'Highpass', 'Bandpass']
        GAIN = GAIN
        
        RINGMOD = 'Ring Modulator'
        FREQ = FREQ
        AMP = 'Amplitude'
        WAVEFORM = WAVEFORM
        LFO_WAVEFORMS = LFO_WAVEFORMS
        GAIN = GAIN
        
        REVERB = 'Reverb'
        REVERBD = 'Length'
        REVERBF = 'Lowpass Filter'
        REVERBL = 'Reverb Level'
        GAIN = GAIN
        
        HARMON = 'Harmonizer'
        FREQ = FREQ
        DRYDELAY = 'Dry delay'
        MIX = 'Mix'
        GAIN = GAIN

        SYNTHTYPES = [[LFO, RANDOM, ADSR], [FM, BUZZ, VCO, PLUCK, NOISE, SAMPLE, VOICE], [DELAY, DIST, FILTER, RINGMOD, REVERB, HARMON], [ADSR]]
        SYNTHPARA = {	'lfo': [AMP, FREQ, WAVEFORM, OFFSET],
			            'rand': [MIN, MAX, FREQ, SEED],
			            'adsr': [ATTACK, DECAY, SUSTAIN, RELEASE],
			            'fm': [CAR, MOD, INDEX, GAIN],
			            'buzz': [FREQ, NHARM, FSLOPE, GAIN],
			            'vco': [FREQ, WAVEFORM, FSLOPE, GAIN],
			            'pluck': [FREQ, LFILTER, VIBRATO, GAIN],
			            'noise': [NOISETYPE, FREQ, BANDWITH, GAIN],
			            'sample': [FREQ, SAMPLEN, LFILTER, GAIN],
			            'voice': [FREQ, VOWEL, VIBRATO, GAIN],
			            'wguide': [FREQ, LFILTER, FEEDBACK, GAIN],
			            'distort': [FREQ, RESON, DISTL, GAIN],
			            'filter': [FREQ, FSLOPE, FTYPE, GAIN],
			            'ring': [FREQ, AMP, WAVEFORM, GAIN],
			            'reverb': [REVERBD, REVERBF, REVERBL, GAIN],
			            'harmon': [FREQ, DRYDELAY, MIX, GAIN]} 
    #French
    elif LANGUAGE == 'Fr':
        #miniTamTam
        VOL = 'Volume'
        REV = 'Réverbération'
        PLAY = 'Lecture / Arrêt'
        STOP = 'Stop'
        GEN = 'Générer'
        COMPL = 'Complexité du rythme'
        BEAT = 'Temps par mesure'
        TEMPO = 'Tempo'
        JAZZ = 'Kit Jazz / Rock'
        AFRI = 'Kit Africain'
        ARAB = 'Kit Arabe'
        RECMIC = 'Enregistrer avec le micro'
        RECLAB = 'Ouvrir SynthLab pour créer des sons'
        
        #Synthlab
        SOURCE = 'Source'
        EFFECT = 'Effet'
        CONTROL = 'Contrôle'
        SOUNDOUT = 'Sortie sonore'
        SOUNDDUR = 'Durée du son'
        SAVE = 'Sauvegarder'
        LOAD = 'Ouvrir'
        SAVEMINI = 'Sauvegarder dans miniTamTam'
        CLOSE = 'Fermer'
        RESET = 'Réinitialiser'
        
        #Controls
        LFO = 'Oscillateur basse fréquence'
        AMP = 'Amplitude'
        FREQ = 'Fréquence'
        WAVEFORM = "Forme d'onde'"        
        RANDOM = 'Aléatoire'
        MIN = 'Minimum'
        MAX = 'Maximum'
        FREQ = FREQ
        
        ADSR = 'Envelope ADSR'
        ATTACK = 'Attaque'
        DECAY = 'Chute'
        SUSTAIN = 'Tenue'
        RELEASE = 'Relâche'
        
        #Source
        FM = 'Modulateur de fréquence'
        CAR = 'Fréquence porteuse'
        MOD = 'Fréquence modulatrice'
        INDEX = 'Index'
        GAIN = 'Gain'
        
        BUZZ = 'Buzz'
        FREQ = FREQ
        NHARM = "Nombre d'harmoniques"
        FSLOPE = 'Pente du filtre'
        GAIN = GAIN
        
        VCO = 'Oscillateur controlé par voltage'
        FREQ = FREQ
        WAVEFORM = WAVEFORM
        FSLOPE = FSLOPE
        GAIN = GAIN
        
        PLUCK = 'Corde pincée'
        FREQ = FREQ
        GAIN = GAIN
        
        NOISE = 'Bruit'
        NOISETYPE = 'Type: Blanc | Rose | Gaussien'
        GAIN = GAIN
        
        SAMPLE = 'Échantillon sonore'
        FREQ = FREQ
        SAMPLEN = "Numéro d'échantillon"
        GAIN = GAIN
        
        VOICE = 'Voix'
        FREQ = FREQ
        VOWEL = 'Voyelle: U->A->I'
        
        #Effects
        DELAY = 'Délai'
        FREQ = FREQ
        LFILTER = 'Filtre passe-bas'
        FEEDBACK = 'Réinjection'
        GAIN = GAIN
        
        DIST = 'Distorsion'
        FREQ = FREQ
        RESON = 'Résonance'
        DISTL = 'Niveau de distosion'
        GAIN = GAIN
        
        FILTER = 'Filtre'
        FREQ = FREQ
        FSLOPE = FSLOPE
        FTYPE = 'Type de filtre: Passe-bas | Passe-haut | Passe-bande'
        GAIN = GAIN
        
        RINGMOD = 'Modulateur par anneaux'
        FREQ = FREQ
        AMP = 'Amplitude'
        GAIN = GAIN
        
        REVERB = 'Réverbération'
        REVERBD = 'Durée'
        REVERBF = 'Filtre passe-bas'
        REVERBL = 'Niveau de réverbération'
        GAIN = GAIN
        
        HARMON = 'Harmonizer'
        FREQ = FREQ
        HARMONL = "Niveau de l'harmonizer"
        GAIN = GAIN


####################
## KeyMapping
####################

# Key = Hardware Keycode Value = Note

KEY_MAP_PIANO = {24:36,    #Q
                 25:38,    #W
                 26:40,    #E
                 27:41,    #R
                 28:43,    #T
                 29:45,    #Y
                 30:47,    #U
                 31:48,    #I
            
                 11:37,    #2
                 12:39,    #3
                 14:42,    #5
                 15:44,    #6
                 16:46,    #7
           
                 39:25,    #S
                 40:27,    #D
                 42:30,    #G
                 43:32,    #H
                 44:34,    #J
                 46:37,    #L
           
                 52:24,    #Z
                 53:26,    #X
                 54:28,    #C
                 55:29,    #V
                 56:31,    #B
                 57:33,    #N
                 58:35,    #M
                 59:36}    #,

KEY_MAP_NOTPIANO = {24:24,    #Q
           25:25,    #W
           26:26,    #E
           27:27,    #R
           28:28,    #T
           29:29,    #Y
           30:30,    #U
           31:31,    #I
           32:32,    #O
           33:33,    #P
           
           38:34,    #A
           39:35,    #S
           40:36,    #D
           41:37,    #F
           42:38,    #G
           43:39,    #H
           44:40,    #J
           45:41,    #K
           46:42,    #L
           
           52:43,    #Z
           53:44,    #X
           54:45,    #C
           55:46,    #V
           56:47,    #B
           57:48}    #N

KEY_MAP = KEY_MAP_PIANO
