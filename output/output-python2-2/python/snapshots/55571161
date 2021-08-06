import common.Config as Config
import common.Util.InstrumentDB as InstrumentDB

LOW = Config.LOW
MID = Config.MID
HIGH = Config.HIGH
PUNCH = Config.PUNCH

INSTRUMENT_TABLE_OFFSET = Config.INSTRUMENT_TABLE_OFFSET
INST_FREE = Config.INST_FREE
INST_TIED = Config.INST_TIED
INST_SIMP = Config.INST_SIMP
INST_PERC = Config.INST_PERC

instrumentDB = InstrumentDB.getRef()


def _addInstrument( name, csoundInstrumentId, instrumentRegister, category, loopStart, loopEnd, crossDur, ampScale = 1, kit = None ):
    instrumentDB.addInstrumentFromArgs( name, csoundInstrumentId, instrumentRegister, loopStart, loopEnd, crossDur, ampScale, kit, name, Config.LIB_DIR+"/Images/"+name+".png", category )


_addInstrument( "mic1", INST_TIED, MID, 'mysounds', .01, 1.99, .01, 1 )
_addInstrument( "mic2", INST_TIED, MID, 'mysounds', .01, 1.99, .01, 1 )
_addInstrument( "mic3", INST_TIED, MID, 'mysounds', .01, 1.99, .01, 1 )
_addInstrument( "mic4", INST_TIED, MID, 'mysounds', .01, 1.99, .01, 1 )
_addInstrument( "lab1", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "lab2", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "lab3", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "lab4", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "lab5", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "lab6", INST_SIMP, MID, 'mysounds', 0, 0, 0, 1 )
_addInstrument( "ounk", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "gam", INST_TIED, HIGH, 'percussions', .69388, .7536, .02922, 1 )
_addInstrument( "guit", INST_TIED, MID, 'strings', .08592, .75126, .33571, 1.3 )
_addInstrument( "guitmute", INST_SIMP, MID, 'strings', 0, 0, 0, 0.7 )
_addInstrument( "guitshort", INST_SIMP, MID, 'strings', 0, 0, 0, 0.8 )
_addInstrument( "koto", INST_TIED, HIGH, 'strings', .56523, .70075, .05954, 1 )
_addInstrument( "banjo", INST_TIED, MID, 'strings', .8928046875, 1.6325390625, .0525, 0.6 )
_addInstrument( "ukulele", INST_TIED, MID, 'strings', .64097090625, 1.0887984375, .17375, 0.5 )
_addInstrument( "harpsichord", INST_TIED, MID, 'strings', .57529609375, .936075, .2, 0.6 )
_addInstrument( "clarinette", INST_TIED, MID, 'winds', 1.635276375, 2.72956523438, .2, 0.6 )
_addInstrument( "flute", INST_TIED, MID, 'winds', .47169, .53693, .02481, 1 )
_addInstrument( "drum1hatpedal", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1hatshoulder", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1hardride", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1ridebell", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1snare", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1snaresidestick", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1crash", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1splash", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1tom", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1floortom", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1)
_addInstrument( "drum1chine", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum1kick", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "piano", INST_TIED, MID, 'keyboard', 0.8883, 1.420524, .13575, 1 )
_addInstrument( "dog", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "chiken", INST_TIED, MID, 'animals', .1972125, .8689675781, .02, 0.5 )
_addInstrument( "duck", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "armbone", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "drum2darbukadoom", INST_SIMP, LOW, 'percussions', 0, 0 ,0, 1 )
_addInstrument( "drum2darbukapied", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2darbukapiedsoft", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2hatflanger", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2darbukatak", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2darbukafinger", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2darbukaroll", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2darbukaslap", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2hatpied", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2tambourinepied", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2hatpied2", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum2tambourinepiedsoft", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3cowbell", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3cowbelltip", INST_SIMP, MID, 'percussions', 0, 0, 0, 1)
_addInstrument( "drum3cup", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3djembelow", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3djembemid", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3djembesidestick", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3djembeslap", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3djembestickmid", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3metalstand", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3pedalperc", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3rainstick", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3tambourinehigh", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum3tambourinelow", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "harmonica", INST_TIED, MID, 'winds', .1531, .19188, .01792, 1 )
_addInstrument( "alarm", INST_TIED, MID, 'concret', 1.37555859375, 2.0286015625, .0675, 0.5 )
_addInstrument( "bird", INST_TIED, MID, 'animals', .1, 1, .05, 1 )
_addInstrument( "frogs", INST_TIED, MID, 'animals', 1.954453125, 4.350234375, .2, 0.7 )
_addInstrument( "cat", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "cow", INST_SIMP, MID, 'animals', 0, 0, 0, 0.7 )
_addInstrument( "cricket", INST_SIMP, MID, 'animals', 0, 0, 0, 0.5 )
_addInstrument( "duck2", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "bottle", INST_TIED, MID, 'concret', .20532, .41064, .05292, 1 )
_addInstrument( "clang", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "clang2", INST_SIMP, MID, 'concret', 0, 0, 0, 0.7 )
_addInstrument( "ow", INST_SIMP, MID, 'people', 0, 0, 0, 1 )
_addInstrument( "hey", INST_SIMP, MID, 'people', 0, 0, 0, 0.5 )
_addInstrument( "sheep", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "water", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "zap", INST_TIED, MID, 'keyboard', .299, .7323, .09895, 1 )
_addInstrument( "trumpet", INST_TIED, MID, 'winds', .91195, 1.652909375, .05375, 0.5)
_addInstrument( "clavinet", INST_TIED, MID, 'winds', .6398328125, .9401625, .094, 0.4)
_addInstrument( "flugel", INST_TIED, MID, 'winds', 1.291740625, 2.37588007813, .065, 0.4)
_addInstrument( "foghorn", INST_TIED, LOW, 'winds', 2.07005, 3.758775, .2, 0.5)
_addInstrument( "bubbles", INST_TIED, MID, 'concret', 0.02, 1.177, 0.02, 1)
_addInstrument( "marimba", INST_TIED, MID, 'percussions', .18883789, .343623047, .07625, 0.5)
_addInstrument( "triangle", INST_TIED, MID, 'percussions', 2.27261836, 3.2965453, .2, 0.6)
_addInstrument( "fingercymbals", INST_TIED, HIGH, 'percussions', 1.29635195312, 1.92448125, .094, 0.6)
_addInstrument( "laugh", INST_SIMP, MID, 'people', 0, 0, 0, 1 )
_addInstrument( "babylaugh", INST_TIED, MID, 'people', 0.72920078, 1.63253906, 0.01, 0.5)
_addInstrument( "babyuhoh", INST_SIMP, MID, 'people', 0, 0, 0, 0.6 )
_addInstrument( "voix", INST_TIED, MID, 'people', .89608, .96092, .02343, 1 )
_addInstrument( "cling", INST_TIED, MID, 'keyboard', .09096, .7878, .18026, 1 )
_addInstrument( "byke", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "door", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "basse", INST_TIED, MID, 'strings', 0.50470875, 0.833315, 0.09375, 1 )
_addInstrument( "acguit", INST_TIED, MID, 'strings', 1.4037, 1.84235625, 0.2, 0.8 )
_addInstrument( "diceinst", INST_SIMP, MID, 'concret', 0, 0, 0, 1.3 )
_addInstrument( "didjeridu", INST_TIED, LOW, 'winds', .55669, 1.73704, .09178, 2 )
_addInstrument( "harmonium", INST_TIED, MID, 'keyboard', .242032, .898165625, .2, 0.5 )
_addInstrument( "horse", INST_SIMP, MID, 'animals', 0, 0, 0, 1 )
_addInstrument( "kalimba", INST_TIED, MID, 'percussions', .20751, .30161, .04658, 1.3 )
_addInstrument( "mando", INST_TIED, MID, 'strings', 0.507107031, 0.934144531, 0.2, 0.5 )
_addInstrument( "ocarina", INST_TIED, MID, 'winds', .06612, .19033, .01776, 1 )
_addInstrument( "rhodes", INST_TIED, MID, 'keyboard', 0.58100625, 0.821625, 0.067, 1 )
_addInstrument( "saxo", INST_TIED, MID, 'winds', 1.161051953, 3.001209765, .05, 0.5 )
_addInstrument( "saxsoprano", INST_TIED, HIGH, 'winds', .90721015625, 1.71199335938, .07675, 0.4 )
_addInstrument( "shenai", INST_TIED, MID, 'winds', .29003, .33072, .00634, 0.7 )
_addInstrument( "sitar", INST_TIED, MID, 'strings', 1.1361625, 1.575134375, .183, 0.4 )
_addInstrument( "tuba", INST_TIED, LOW, 'winds', .51063, .58384, .035, 1 )
_addInstrument( "violin", INST_TIED, MID, 'strings', .105, .30656, .028, 1 )
_addInstrument( "guidice1", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice2", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice3", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice4", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice5", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice6", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice7", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice8", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice9", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guidice10", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "drum4afrofeet", INST_SIMP, LOW, 'percussions', 0, 0 ,0, 1 )
_addInstrument( "drum4fingersn", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4mutecuic", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4stompbass", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tambouri", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr707clap", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr707open", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr808closed", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr808sn", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr909bass", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr909kick", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum4tr909sn", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5timablesslap", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5congagraveouvert", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5timablesaiguslap", INST_SIMP, LOW, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5congagraveferme", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5guiroretour", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5vibraslap", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5congaaiguouvert", INST_SIMP, MID, 'percussions', 0, 0 ,0, 1 )
_addInstrument( "drum5quicamedium", INST_SIMP, PUNCH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5quicaaigu", INST_SIMP, MID, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5agogograve", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5bongoaiguouvert", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5agogoaigu", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "drum5bongograveouvert", INST_SIMP, HIGH, 'percussions', 0, 0, 0, 1 )
_addInstrument( "camera", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "car", INST_TIED, MID, 'concret', .67, 1.05761, .01, 0.7 )
_addInstrument( "carhorn", INST_SIMP, MID, 'concret', 0, 0, 0, 0.4 )
_addInstrument( "cello", INST_TIED, MID, 'strings', 0.4761, 0.92244375, 0.19125, .75 )
_addInstrument( "chimes", INST_TIED, MID, 'percussions', 4.104825, 5.644134375, .02, 1 )
_addInstrument( "crash", INST_SIMP, MID, 'concret', 0, 0, 0, 1 )
_addInstrument( "guit2", INST_TIED, MID, 'strings', 1.186341406, 1.929568266, .2, 0.5 )
_addInstrument( "plane", INST_SIMP, MID, 'concret', 0, 0, 0, 0.7 )
_addInstrument( "slap", INST_SIMP, MID, 'concret', 0, 0, 0, 0.7 )

try:
    ifile = open(PREF_DIR + '/sounds_settings', 'r')
    for line in ifile.readlines():
        list = line.split()
        _addInstrument(list[0], int(list[1]), int(list[2]), list[3], list[4], float(list[5]), float(list[6]), float(list[7]), float(list[8]))
except:
    pass


DRUM1KIT = { 24 : "drum1kick",
                 26 : "drum1floortom",
                 28 : "drum1tom",
                 30 : "drum1chine",
                 32 : "drum1splash",
                 34 : "drum1crash",
                 36 : "drum1snaresidestick",
                 38 : "drum1snaresidestick",
                 40 : "drum1snare",
                 42 : "drum1ridebell",
                 44 : "drum1hardride",
                 46 : "drum1hatshoulder",
                 48 : "drum1hatpedal" }

DRUM2KIT = { 24 : "drum2darbukadoom",
                 26 : "drum2darbukapied",
                 28 : "drum2darbukapiedsoft",
                 30 : "drum2hatflanger",
                 32 : "drum2darbukatak",
                 34 : "drum2darbukatak",
                 36 : "drum2darbukafinger",
                 38 : "drum2darbukaroll",
                 40 : "drum2darbukaslap",
                 42 : "drum2hatpied",
                 44 : "drum2tambourinepied",
                 46 : "drum2hatpied2",
                 48 : "drum2tambourinepiedsoft" }

DRUM3KIT = { 24 : "drum3djembelow",
                 26 : "drum3pedalperc",
                 28 : "drum3djembeslap",
                 30 : "drum3tambourinehigh",
                 32 : "drum3tambourinelow",
                 34 : "drum3rainstick",
                 36 : "drum3djembemid",
                 38 : "drum3djembesidestick",
                 40 : "drum3djembestickmid",
                 42 : "drum3cowbell",
                 44 : "drum3cowbelltip",
                 46 : "drum3cup",
                 48 : "drum3metalstand" }

DRUM4KIT = { 24 : "drum4afrofeet",
                 26 : "drum4tr909kick",
                 28 : "drum4tr909bass",
                 30 : "drum4stompbass",
                 32 : "drum4tr707open",
                 34 : "drum4mutecuic",
                 36 : "drum4tr808sn",
                 38 : "drum4tr707clap",
                 40 : "drum4tr909sn",
                 42 : "drum4tambouri",
                 44 : "drum4fingersn",
                 46 : "drum4fingersn",
                 48 : "drum4tr808closed" }

DRUM5KIT = { 24 : "drum5timablesslap",
                 26 : "drum5timablesaiguslap",
                 28 : "drum5congagraveouvert",
                 30 : "drum5quicamedium",
                 32 : "drum5guiroretour",
                 34 : "drum5vibraslap",
                 36 : "drum5congagraveferme",
                 38 : "drum5quicaaigu",
                 40 : "drum5congaaiguouvert",
                 42 : "drum5agogoaigu",
                 44 : "drum5bongograveouvert",
                 46 : "drum5agogograve",
                 48 : "drum5bongoaiguouvert" }

_addInstrument( "drum1kit", 0, 0, "percussions", 0, 0, 0, 1, DRUM1KIT )
_addInstrument( "drum2kit", 0, 0, "percussions", 0, 0, 0, 1, DRUM2KIT )
_addInstrument( "drum3kit", 0, 0, "percussions", 0, 0, 0, 1, DRUM3KIT )
_addInstrument( "drum4kit", 0, 0, "percussions", 0, 0, 0, 1, DRUM4KIT )
_addInstrument( "drum5kit", 0, 0, "percussions", 0, 0, 0, 1, DRUM5KIT )
