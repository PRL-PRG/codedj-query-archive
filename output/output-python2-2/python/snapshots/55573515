# -*- coding: utf-8 -*-
import os


SugarMode = True
try:
    from sugar import env
except ImportError:
    SugarMode = False

if os.path.isfile("DEBUG"):
    f = open("DEBUG")
    l = f.read(10)
    f.close()
    if len(l): DEBUG = int( l )
    else: DEBUG = 99
else:
    DEBUG = 0
print "Debug Level %d" % (DEBUG)


TAM_TAM_ROOT = os.path.dirname(os.path.abspath(__file__))
print 'INFO: loaded TAMTAM_ROOT=%s' % TAM_TAM_ROOT


#PATHS
SOUNDS_DIR = TAM_TAM_ROOT + "/Resources/Sounds"
FILES_DIR = TAM_TAM_ROOT + "/Resources"
TUNE_DIR='/'
SYNTH_DIR='/'
if SugarMode == True:
    PREF_DIR = env.get_profile_path() + '/tamtam'
    TUNE_DIR=env.get_profile_path() + '/tamtam/tunes'
    SYNTH_DIR=env.get_profile_path() + '/tamtam/synthlab'
else:
    PREF_DIR = SOUNDS_DIR + '/temp'
    TUNE_DIR= os.getenv('HOME') + '/.tamtam/tunes'
    SYNTH_DIR= os.getenv('HOME') + '/.tamtam/synthlab'


#PLUGIN
PLUGIN_DEBUG = os.getenv('HOME')+"/.tamtam/clooper.log"
PLUGIN_VERBOSE = 0
PLUGIN_UNIVORC = TAM_TAM_ROOT + "/Resources/univorc.csd"
PLUGIN_KSMPS = 64
PLUGIN_RATE  = 16000
## PLUGIN ALSA PARAMETERS:

## for macbook pro
#PLUGIN_PERIOD = 1024
#PLUGIN_NPERIODS = 4

## for XO with root
#PLUGIN_PERIOD = 256
#PLUGIN_NPERIODS = 2

## for XO as normal user
PLUGIN_PERIOD = 256 #512
PLUGIN_NPERIODS = 2

##############
## SOUNDS
##############
class Instrument:
    def __init__( self, name, instrumentId, csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit = None ):
        self.name = name
        self.instrumentId = instrumentId
        self.csoundInstrumentId = csoundInstrumentId
        self.instrumentRegister = instrumentRegister
        self.soundClass = soundClass
        self.category = category
        self.loopStart = loopStart
        self.loopEnd = loopEnd
        self.crossDur = crossDur
        self.kit = kit

LOW, MID, HIGH, PUNCH = range( 4 )

# Sounds categories: musicInst, animals, drum, people, electronic, concret, mic
#INSTRUMENTS ( csound table, csound instrument, register, instrumentClass, category )
INSTRUMENT_TABLE_OFFSET = 5000
INST_FREE = 5000
INST_TIED = 5001
INST_SIMP = 5011
INST_PERC = 5021

CATEGORIES = ['all','animals','concret','electronic','keyboard','people','percussions','strings','winds']

_nextInstrumentId = [0]
INSTRUMENTS = {}
def _addInstrument( name, csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit = None ):
    INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0], csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit )
    _nextInstrumentId[0] += 1

_addInstrument( "ounk", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "gam", INST_TIED, HIGH, 'melo', 'percussions', .69388, .7536, .02922 )
_addInstrument( "gong", INST_SIMP, LOW, 'melo', 'percussions', 0, 0, 0 )
_addInstrument( "guit", INST_TIED, MID, 'melo', 'strings', .08592, .75126, .33571 )
_addInstrument( "koto", INST_TIED, HIGH, 'melo', 'strings', .56523, .70075, .05954 )
_addInstrument( "clarinette", INST_TIED, MID, 'melo', 'winds', .57905, .73319, .04934 )
_addInstrument( "flute", INST_TIED, MID, 'melo', 'winds', .47169, .53693, .02481 )
_addInstrument( "mic1", INST_TIED, MID, 'melo', 'mic', .01, .99, .01 )
_addInstrument( "mic2", INST_TIED, MID, 'melo', 'mic', .01, .99, .01 )
_addInstrument( "mic3", INST_TIED, MID, 'melo', 'mic', .01, .99, .01 )
_addInstrument( "mic4", INST_TIED, MID, 'melo', 'mic', .01, .99, .01 )
_addInstrument( "drum1hatpedal", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1hatshoulder", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1hardride", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1ridebell", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1snare", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1snaresidestick", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1crash", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1splash", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1tom", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1floortom", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0)
_addInstrument( "drum1chine", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum1kick", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "piano", INST_TIED, MID, 'melo', 'keyboard', 2.39418, 2.53339, .01323 )
_addInstrument( "dog", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "duck", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "drum2darbukadoom", INST_SIMP, LOW, 'drum', 'drum', 0, 0 ,0 )
_addInstrument( "drum2darbukapied", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2darbukapiedsoft", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2hatflanger", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2darbukatak", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2darbukafinger", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2darbukaroll", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2darbukaslap", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2hatpied", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2tambourinepied", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2hatpied2", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum2tambourinepiedsoft", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3cowbell", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3cowbelltip", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0)
_addInstrument( "drum3cup", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3djembelow", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3djembemid", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3djembesidestick", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3djembeslap", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3djembestickmid", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3metalstand", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3pedalperc", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3rainstick", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3tambourinehigh", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum3tambourinelow", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "harmonica", INST_TIED, MID, 'melo', 'winds', .1531, .19188, .01792 )
_addInstrument( "fm2", INST_TIED, MID, 'melo', 'electronic', .43443, .5784, .05127 )
_addInstrument( "bird", INST_TIED, MID, 'melo', 'animals', .1, 1, .05 )
_addInstrument( "cat", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "duck2", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "bottle", INST_TIED, MID, 'melo', 'concret', .20532, .41064, .05292 )
_addInstrument( "clang", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "ow", INST_SIMP, MID, 'melo', 'people', 0, 0, 0 )
_addInstrument( "sheep", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "water", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "zap", INST_TIED, MID, 'melo', 'electronic', .299, .7323, .09895 )
_addInstrument( "trumpet", INST_TIED, MID, 'melo', 'winds', .39934, .45537, .02729)
_addInstrument( "maracas", INST_SIMP, MID, "melo", 'percussions', 0, 0, 0)
_addInstrument( "marimba", INST_TIED, MID, "melo", 'percussions', .26545, .33098, .03087)
_addInstrument( "triangle", INST_TIED, MID, "melo", 'percussions', 1.21002, 1.31805, .01268)
_addInstrument( "laugh", INST_SIMP, MID, 'melo', 'people', 0, 0, 0 )
_addInstrument( "voix", INST_TIED, MID, 'melo', 'people', .89608, .96092, .02343 )
_addInstrument( "cling", INST_TIED, MID, 'melo', 'electronic', .09096, .7878, .18026 )
_addInstrument( "tchiwo", INST_TIED, MID, 'melo', 'electronic', .91515, 1.00094, .02122 )
_addInstrument( "door", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "basse", INST_TIED, MID, 'melo', 'strings', .58455, .67433, .03638 )
_addInstrument( "acguit", INST_TIED, MID, 'melo', 'strings', .58503, .8667, .13699 )
_addInstrument( "diceinst", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "didjeridu", INST_TIED, LOW, 'melo', 'winds', .55669, 1.73704, .09178 )
_addInstrument( "harmonium", INST_TIED, MID, 'melo', 'keyboard', .04674, .41073, .18384 )
_addInstrument( "horse", INST_SIMP, MID, 'melo', 'animals', 0, 0, 0 )
_addInstrument( "kalimba", INST_TIED, MID, 'melo', 'percussions', .20751, .30161, .04658 )
_addInstrument( "mando", INST_TIED, MID, 'melo', 'strings', .50167, .54401, .01984 )
_addInstrument( "ocarina", INST_TIED, MID, 'melo', 'winds', .12122, .18965, .02205 )
_addInstrument( "rhodes", INST_TIED, MID, 'melo', 'keyboard', .65013, .71429, .02205 )
_addInstrument( "saxo", INST_TIED, MID, 'melo', 'winds', .53722, .6583, .05264 )
_addInstrument( "shenai", INST_TIED, MID, 'melo', 'winds', .29003, .33072, .00634 )
_addInstrument( "sitar", INST_TIED, MID, 'melo', 'strings', .63187, .67882, .01654 )
_addInstrument( "tuba", INST_TIED, LOW, 'melo', 'winds', .51063, .58384, .035 )
_addInstrument( "violin", INST_TIED, MID, 'melo', 'strings', .55094, .82054, .14498 )
_addInstrument( "lab1", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "lab2", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "lab3", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "lab4", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "lab5", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "lab6", INST_SIMP, MID, 'melo', 'lab', 0, 0, 0 )
_addInstrument( "guidice1", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice2", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice3", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice4", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice5", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice6", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice7", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice8", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice9", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "guidice10", INST_SIMP, MID, 'melo', 'concret', 0, 0, 0 )
_addInstrument( "drum4afrofeet", INST_SIMP, LOW, 'drum', 'drum', 0, 0 ,0 )
_addInstrument( "drum4fingersn", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4mutecuic", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4stompbass", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tambouri", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr707clap", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr707open", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr808closed", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr808sn", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr909bass", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr909kick", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum4tr909sn", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5timablesslap", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5congagraveouvert", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5timablesaiguslap", INST_SIMP, LOW, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5congagraveferme", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5guiroretour", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5vibraslap", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5congaaiguouvert", INST_SIMP, MID, 'drum', 'drum', 0, 0 ,0 )
_addInstrument( "drum5quicamedium", INST_SIMP, PUNCH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5quicaaigu", INST_SIMP, MID, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5agogograve", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5bongoaiguouvert", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5agogoaigu", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )
_addInstrument( "drum5bongograveouvert", INST_SIMP, HIGH, 'drum', 'drum', 0, 0, 0 )

DRUM1KIT = { 24 : INSTRUMENTS["drum1kick"],
             26 : INSTRUMENTS["drum1floortom"],
             28 : INSTRUMENTS["drum1tom"],
             30 : INSTRUMENTS["drum1chine"],
             32 : INSTRUMENTS["drum1splash"],
             34 : INSTRUMENTS["drum1crash"],
             36 : INSTRUMENTS["drum1snaresidestick"],
             38 : INSTRUMENTS["drum1snaresidestick"],
             40 : INSTRUMENTS["drum1snare"],
             42 : INSTRUMENTS["drum1ridebell"],
             44 : INSTRUMENTS["drum1hardride"],
             46 : INSTRUMENTS["drum1hatshoulder"],
             48 : INSTRUMENTS["drum1hatpedal"] }

DRUM2KIT = { 24 : INSTRUMENTS["drum2darbukadoom"],
             26 : INSTRUMENTS["drum2darbukapied"],
             28 : INSTRUMENTS["drum2darbukapiedsoft"],
             30 : INSTRUMENTS["drum2hatflanger"],
             32 : INSTRUMENTS["drum2darbukatak"],
             34 : INSTRUMENTS["drum2darbukatak"],
             36 : INSTRUMENTS["drum2darbukafinger"],
             38 : INSTRUMENTS["drum2darbukaroll"],
             40 : INSTRUMENTS["drum2darbukaslap"],
             42 : INSTRUMENTS["drum2hatpied"],
             44 : INSTRUMENTS["drum2tambourinepied"],
             46 : INSTRUMENTS["drum2hatpied2"],
             48 : INSTRUMENTS["drum2tambourinepiedsoft"] }

DRUM3KIT = { 24 : INSTRUMENTS["drum3djembelow"],
             26 : INSTRUMENTS["drum3pedalperc"],
             28 : INSTRUMENTS["drum3djembeslap"],
             30 : INSTRUMENTS["drum3tambourinehigh"],
             32 : INSTRUMENTS["drum3tambourinelow"],
             34 : INSTRUMENTS["drum3rainstick"],
             36 : INSTRUMENTS["drum3djembemid"],
             38 : INSTRUMENTS["drum3djembesidestick"],
             40 : INSTRUMENTS["drum3djembestickmid"],
             42 : INSTRUMENTS["drum3cowbell"],
             44 : INSTRUMENTS["drum3cowbelltip"],
             46 : INSTRUMENTS["drum3cup"],
             48 : INSTRUMENTS["drum3metalstand"] }

DRUM4KIT = { 24 : INSTRUMENTS["drum4afrofeet"],
             26 : INSTRUMENTS["drum4tr909kick"],
             28 : INSTRUMENTS["drum4tr909bass"],
             30 : INSTRUMENTS["drum4stompbass"],
             32 : INSTRUMENTS["drum4tr707open"],
             34 : INSTRUMENTS["drum4mutecuic"],
             36 : INSTRUMENTS["drum4tr808sn"],
             38 : INSTRUMENTS["drum4tr707clap"],
             40 : INSTRUMENTS["drum4tr909sn"],
             42 : INSTRUMENTS["drum4tambouri"],
             44 : INSTRUMENTS["drum4fingersn"],
             46 : INSTRUMENTS["drum4fingersn"],
             48 : INSTRUMENTS["drum4tr808closed"] }

DRUM5KIT = { 24 : INSTRUMENTS["drum5timablesslap"],
             26 : INSTRUMENTS["drum5timablesaiguslap"],
             28 : INSTRUMENTS["drum5congagraveouvert"],
             30 : INSTRUMENTS["drum5quicamedium"],
             32 : INSTRUMENTS["drum5guiroretour"],
             34 : INSTRUMENTS["drum5vibraslap"],
             36 : INSTRUMENTS["drum5congagraveferme"],
             38 : INSTRUMENTS["drum5quicaaigu"],
             40 : INSTRUMENTS["drum5congaaiguouvert"],
             42 : INSTRUMENTS["drum5agogoaigu"],
             44 : INSTRUMENTS["drum5bongograveouvert"],
             46 : INSTRUMENTS["drum5agogograve"],
             48 : INSTRUMENTS["drum5bongoaiguouvert"] }

_addInstrument( "drum1kit", 0, 0, 0, "kit", 0, 0, 0, DRUM1KIT )
_addInstrument( "drum2kit", 0, 0, 0, "kit", 0, 0, 0, DRUM2KIT )
_addInstrument( "drum3kit", 0, 0, 0, "kit", 0, 0, 0, DRUM3KIT )
_addInstrument( "drum4kit", 0, 0, 0, "kit", 0, 0, 0, DRUM4KIT )
_addInstrument( "drum5kit", 0, 0, 0, "kit", 0, 0, 0, DRUM5KIT )

INSTRUMENTSID = {}
for i in INSTRUMENTS:
    INSTRUMENTSID[INSTRUMENTS[i].instrumentId] = INSTRUMENTS[i]


#DRUMKITS = ['drum1kit', 'drum2kit', 'drum3kit', 'drum4kit']
#DRUMSINSTRUMENTSDICT = [DRUM1KIT, DRUM2KIT, DRUM3KIT, DRUM4KIT]

RECORDABLE_INSTRUMENTS = set( [ "mic1", "mic2", "mic3", "mic4" ] )
RECORDABLE_INSTRUMENT_CSOUND_IDS = {  "mic1" : 7,
                                      "mic2" : 8,
                                      "mic3" : 9,
                                      "mic4" : 10 }

#CSOUND COMMANDS
CSOUND_LOAD_INSTRUMENT = 'f%d 0 0 -1 "%s" 0 0 0'
CSOUND_MIC_RECORD = 'i5201 0 5 %d'
CSOUND_UNLOAD_TABLES = 'i%d 0 0.1 %d' % (INST_FREE, len(INSTRUMENTS))
CSOUND_NOTE_OFF = 'i %s.%s .2 0.01 1. 0. 0. 0.5 %d 0 0 0 0' %('%d','%d',INSTRUMENT_TABLE_OFFSET)

#CSOUND COMMANDS - DEPRECATED

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
        "perf.InputMessage('i5201 0 5 %d')\n"
UNLOAD_TABLES_COMMAND = \
        "perf.InputMessage('i%d 0 0.1 %d')\n" % (INST_FREE, len(INSTRUMENTS))




#################
## GUI CONSTANTS
#################

LANGUAGE = 'En'
IMAGE_ROOT = TAM_TAM_ROOT + '/Resources/Images/'

MAIN_WINDOW_PADDING = 5

BG_COLOR = "#8CAF97"

NOTE_HEIGHT = 9     # pixels
NOTE_IMAGE_PADDING = 6
NOTE_IMAGE_PADDING_MUL2 = NOTE_IMAGE_PADDING*2
NOTE_IMAGE_TAIL = 1059
NOTE_IMAGE_ENDLENGTH = 12
HIT_HEIGHT = 13    # pixels
HIT_IMAGE_PADDING = 6
HIT_IMAGE_PADDING_MUL2 = HIT_IMAGE_PADDING*2
TRACK_SPACING = 4
TRACK_SPACING_DIV2 = TRACK_SPACING//2
TRACK_COLORS = [ ( "#00290B", "#00E847" ), \
                 ( "#3F0200", "#E72500" ), \
                 ( "#002642", "#0090EA" ), \
                 ( "#313D00", "#F9EF00" ), \
                 ( "#17083B", "#4A00ED" ) ]
#TRACK_COLORS = [ ( "#00591B", "#00E847" ), \
#                 ( "#6F1200", "#E72500" ), \
#                 ( "#004682", "#0090EA" ), \
#                 ( "#716D00", "#F9EF00" ), \
#                 ( "#37187B", "#4A00ED" ) ]
BEAT_COLOR = "#999999"
BEAT_LINE_SIZE = 2
PLAYHEAD_COLOR = "#666666"
PLAYHEAD_SIZE = 2
PLAYHEAD_SIZE_DIV2 = PLAYHEAD_SIZE/2.0
MARQUEE_COLOR = "#FFFFFF"
MARQUEE_SIZE = 2

PAGE_BORDER_SIZE = 2
PAGE_SELECTED_BORDER_SIZE = 5
PAGE_WIDTH = 100
PAGE_HEIGHT = 25

PAGE_THUMBNAIL_WIDTH = 92
PAGE_THUMBNAIL_WIDTH_DIV2 =     PAGE_THUMBNAIL_WIDTH/2
PAGE_THUMBNAIL_HEIGHT = 65

THUMBNAIL_TRACK_RECT = [ ( 2, 4, 83, 10 ), \
                         ( 2, 14, 83, 10 ), \
                         ( 2, 24, 83, 10 ), \
                         ( 2, 34, 83, 10 ), \
                         ( 2, 44, 83, 13 ) ]
THUMBNAIL_DRAG_COLOR = "#000000"
THUMBNAIL_TRACK_COLOR = "#FF0000"
THUMBNAIL_SELECTED_COLOR = "#2266FF"
THUMBNAIL_DISPLAYED_COLOR = "#CC1133"

### miniTamTam/SYNTHLAB SPECIFIC ###
INST_BCK_COLOR = '#979DA8'
PANEL_BCK_COLOR =  '#FFFFFF'
PANEL_COLOR = '#707F93'
SL_LINE_COLOR = "#666666"
SL_OVER_WIRE_COLOR = "#FFFFFF"
SL_OVER_GATE_COLOR = "#00FF18"
SL_OVER_GATE_REJECT_COLOR = "#B30000"
PANEL_RADIUS = 10
PANEL_SPACING = 2

###Welcome Screen Specific###
WS_PANEL_COLOR = '#72785B'
WS_BCK_COLOR = '#DFE5C5'

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
PLAYER_TEMPO = 100
PLAYER_TEMPO_LOWER = 30
PLAYER_TEMPO_UPPER = 180
DEFAULT_VOLUME = 80

#NUMERICAL CONSTANTS
NUMBER_OF_POSSIBLE_PITCHES = 25
MINIMUM_PITCH = 24
MAXIMUM_PITCH = MINIMUM_PITCH + NUMBER_OF_POSSIBLE_PITCHES - 1
NUMBER_OF_POSSIBLE_PITCHES_DRUM = 13
PITCH_STEP_DRUM = 2
MINIMUM_PITCH_DRUM = 24
MAXIMUM_PITCH_DRUM = MINIMUM_PITCH_DRUM + PITCH_STEP_DRUM*(NUMBER_OF_POSSIBLE_PITCHES_DRUM - 1)
MINIMUM_NOTE_DURATION = 1 # ticks
MS_PER_MINUTE = 60000.0
TICKS_PER_BEAT = 12
TICKS_PER_BEAT_DIV2 = TICKS_PER_BEAT/2
MAXIMUM_BEATS = 12 # maximum beats per page
NUMBER_OF_TRACKS = 5
NUMBER_OF_PAGES = 2

MINIMUM_AMPLITUDE = 0
MAXIMUM_AMPLITUDE = 1


####################
## ToolTips
####################
LANGUAGE = 'en'
exec 'from Resources.tooltips_%s import Tooltips' % LANGUAGE


####################
## KeyMapping
####################

LOOP_KEYS = [17, 18, 19, 20, 21, 32, 33, 34, 35, 45, 46, 47, 48, 51, 60, 61]
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
