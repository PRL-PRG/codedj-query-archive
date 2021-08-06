#!/bin/env python
import os

##############
## SOUNDS
##############

#INSTRUMENTS ( csound table, csound instrument, register, instrumentClass, category )
CSOUND_INSTRUMENT = {'inst_free' : 5000, 'inst_tied' : 5001, 'inst_simp': 5011, 'inst_perc': 5021}

SOUND_ROOT = os.getenv("HOME") + '/cvs/tamtam/snd'
DRUM_ROOT = SOUND_ROOT + "/drum"
INST_ROOT = SOUND_ROOT + "/inst"

INST = {}
inst_load_dynamic = 0
class Instrument:
    REGISTER = {'low': 0, 'mid':1, 'high':2, 'punch':3}
    def __init__( self, name, csoundInstrumentName, registerName, category, loopStart, loopEnd, crossDur, wav, img ):
        self.name = name
        self.instrumentId = len(INST)
        self.csoundInstrumentId = CSOUND_INSTRUMENT[csoundInstrumentName]
        self.instrumentRegister = self.REGISTER[registerName]
        self.category = category
        self.loopStart = loopStart
        self.loopEnd = loopEnd
        self.crossDur = crossDur
        self.wav = wav
        self.img = img

        if None == self.wav: self.wav = os.getenv('HOME') + 'cvs/tamtam/Resources/Sounds/' + name
        if None == self.img: self.wav = os.getenv('HOME') + 'cvs/tamtam/Resources/Images/' + name

        INST[name] = self

if inst_load_dynamic:
    for I in os.listdir(INST_ROOT):
        print 'inst:', I
else:
    Instrument( 'mic1',  'inst_tied', 'mid', 'mic', .01, .99, .01, None, None)
    Instrument( 'mic2',  'inst_tied', 'mid', 'mic', .01, .99, .01, None, None)
    Instrument( "mic3", 'inst_tied', 'mid', 'mic', .01, .99, .01, None, None)
    Instrument( "mic4", 'inst_tied', 'mid', 'mic', .01, .99, .01, None, None)
    Instrument( "lab1", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "lab2", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "lab3", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "lab4", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "lab5", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "lab6", 'inst_simp', 'mid', 'lab', 0, 0, 0, None, None)
    Instrument( "ounk", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "gam", 'inst_tied', 'high', 'percussions', .69388, .7536, .02922 , None, None)
    Instrument( "guit", 'inst_tied', 'mid', 'strings', .08592, .75126, .33571 , None, None)
    Instrument( "koto", 'inst_tied', 'high', 'strings', .56523, .70075, .05954 , None, None)
    Instrument( "clarinette", 'inst_tied', 'mid', 'winds', .57905, .73319, .04934 , None, None)
    Instrument( "flute", 'inst_tied', 'mid', 'winds', .47169, .53693, .02481 , None, None)
    Instrument( "drum1hatpedal", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1hatshoulder", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1hardride", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1ridebell", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1snare", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1snaresidestick", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1crash", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1splash", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1tom", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1floortom", 'inst_simp', 'low', 'drum', 0, 0, 0, None, None)
    Instrument( "drum1chine", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum1kick", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "piano", 'inst_tied', 'mid', 'keyboard', 2.39418, 2.53339, .01323 , None, None)
    Instrument( "dog", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "duck", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "drum2darbukadoom", 'inst_simp', 'low', 'drum', 0, 0 ,0 , None, None)
    Instrument( "drum2darbukapied", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2darbukapiedsoft", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2hatflanger", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2darbukatak", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2darbukafinger", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2darbukaroll", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2darbukaslap", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2hatpied", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2tambourinepied", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2hatpied2", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum2tambourinepiedsoft", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3cowbell", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3cowbelltip", 'inst_simp', 'mid', 'drum', 0, 0, 0, None, None)
    Instrument( "drum3cup", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3djembelow", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3djembemid", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3djembesidestick", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3djembeslap", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3djembestickmid", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3metalstand", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3pedalperc", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3rainstick", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3tambourinehigh", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum3tambourinelow", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "harmonica", 'inst_tied', 'mid', 'winds', .1531, .19188, .01792 , None, None)
    Instrument( "alarm", 'inst_tied', 'mid', 'concret', .02, .19133, .01 , None, None)
    Instrument( "bird", 'inst_tied', 'mid', 'animals', .1, 1, .05 , None, None)
    Instrument( "cat", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "duck2", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "bottle", 'inst_tied', 'mid', 'concret', .20532, .41064, .05292 , None, None)
    Instrument( "clang", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "ow", 'inst_simp', 'mid', 'people', 0, 0, 0 , None, None)
    Instrument( "sheep", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "water", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "zap", 'inst_tied', 'mid', 'electronic', .299, .7323, .09895 , None, None)
    Instrument( "trumpet", 'inst_tied', 'mid', 'winds', .39934, .45537, .02729, None, None)
    Instrument( "bubbles", 'inst_tied', 'mid', 'concret', 0.02, 1.177, 0.02, None, None)
    Instrument( "marimba", 'inst_tied', 'mid', 'percussions', .26545, .33098, .03087, None, None)
    Instrument( "triangle", 'inst_tied', 'mid', 'percussions', 1.21002, 1.31805, .01268, None, None)
    Instrument( "laugh", 'inst_simp', 'mid', 'people', 0, 0, 0 , None, None)
    Instrument( "voix", 'inst_tied', 'mid', 'people', .89608, .96092, .02343 , None, None)
    Instrument( "cling", 'inst_tied', 'mid', 'electronic', .09096, .7878, .18026 , None, None)
    Instrument( "byke", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "door", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "basse", 'inst_tied', 'mid', 'strings', .58455, .67433, .03638 , None, None)
    Instrument( "acguit", 'inst_tied', 'mid', 'strings', .58503, .8667, .13699 , None, None)
    Instrument( "diceinst", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "didjeridu", 'inst_tied', 'low', 'winds', .55669, 1.73704, .09178 , None, None)
    Instrument( "harmonium", 'inst_tied', 'mid', 'keyboard', .04674, .41073, .18384 , None, None)
    Instrument( "horse", 'inst_simp', 'mid', 'animals', 0, 0, 0 , None, None)
    Instrument( "kalimba", 'inst_tied', 'mid', 'percussions', .20751, .30161, .04658 , None, None)
    Instrument( "mando", 'inst_tied', 'mid', 'strings', .50167, .54401, .01984 , None, None)
    Instrument( "ocarina", 'inst_tied', 'mid', 'winds', .06612, .19033, .01776 , None, None)
    Instrument( "rhodes", 'inst_tied', 'mid', 'keyboard', .65013, .71429, .02205 , None, None)
    Instrument( "saxo", 'inst_tied', 'mid', 'winds', .53722, .6583, .05264 , None, None)
    Instrument( "shenai", 'inst_tied', 'mid', 'winds', .29003, .33072, .00634 , None, None)
    Instrument( "sitar", 'inst_tied', 'mid', 'strings', .63187, .67882, .01654 , None, None)
    Instrument( "tuba", 'inst_tied', 'low', 'winds', .51063, .58384, .035 , None, None)
    Instrument( "violin", 'inst_tied', 'mid', 'strings', .105, .30656, .028 , None, None)
    Instrument( "guidice1", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice2", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice3", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice4", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice5", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice6", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice7", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice8", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice9", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guidice10", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "drum4afrofeet", 'inst_simp', 'low', 'drum', 0, 0 ,0 , None, None)
    Instrument( "drum4fingersn", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4mutecuic", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4stompbass", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tambouri", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr707clap", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr707open", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr808closed", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr808sn", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr909bass", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr909kick", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum4tr909sn", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5timablesslap", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5congagraveouvert", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5timablesaiguslap", 'inst_simp', 'low', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5congagraveferme", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5guiroretour", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5vibraslap", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5congaaiguouvert", 'inst_simp', 'mid', 'drum', 0, 0 ,0 , None, None)
    Instrument( "drum5quicamedium", 'inst_simp', 'punch', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5quicaaigu", 'inst_simp', 'mid', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5agogograve", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5bongoaiguouvert", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5agogoaigu", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "drum5bongograveouvert", 'inst_simp', 'high', 'drum', 0, 0, 0 , None, None)
    Instrument( "camera", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "car", 'inst_tied', 'mid', 'concret', .67, 1.05761, .01 , None, None)
    Instrument( "chimes", 'inst_tied', 'mid', 'percussions', .09, 2.97633, .01 , None, None)
    Instrument( "crash", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "guit2", 'inst_tied', 'mid', 'strings', .33, 1.1583, .02 , None, None)
    Instrument( "plane", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)
    Instrument( "slap", 'inst_simp', 'mid', 'concret', 0, 0, 0 , None, None)

DRUM = {}
drum_load_dynamic = 0
if drum_load_dynamic:
    for D in os.listdir(DRUM_ROOT):
        print 'drum:',D
else:
    DRUM = {
        'drum1kit' : [ 'drum1kick', 'drum1floortom', 'drum1tom',
             'drum1chine', 'drum1splash', 'drum1crash', 
             'drum1snaresidestick', 'drum1snaresidestick', 'drum1snare',
             'drum1ridebell', 'drum1hardride', 'drum1hatshoulder', 
             'drum1hatpedal'],
        'drum2kit' : [ "drum2darbukadoom", "drum2darbukapied", "drum2darbukapiedsoft",
             "drum2hatflanger", "drum2darbukatak", "drum2darbukatak",
             "drum2darbukafinger", "drum2darbukaroll", "drum2darbukaslap",
             "drum2hatpied", "drum2tambourinepied", "drum2hatpied2",
             "drum2tambourinepiedsoft"],
        'drum3kit' : [ "drum3djembelow", "drum3pedalperc", "drum3djembeslap",
             "drum3tambourinehigh", "drum3tambourinelow", "drum3rainstick",
             "drum3djembemid", "drum3djembesidestick", "drum3djembestickmid",
             "drum3cowbell", "drum3cowbelltip", "drum3cup",
             "drum3metalstand"],
        'drum4kit' : [ "drum4afrofeet", "drum4tr909kick", "drum4tr909bass",
             "drum4stompbass", "drum4tr707open", "drum4mutecuic",
             "drum4tr808sn", "drum4tr707clap", "drum4tr909sn",
             "drum4tambouri", "drum4fingersn", "drum4fingersn",
             "drum4tr808closed" ],
        'drum5kit' : [ "drum5timablesslap", "drum5timablesaiguslap", "drum5congagraveouvert",
             "drum5quicamedium", "drum5guiroretour", "drum5vibraslap",
             "drum5congagraveferme", "drum5quicaaigu", "drum5congaaiguouvert",
             "drum5agogoaigu", "drum5bongograveouvert", "drum5agogograve",
             "drum5bongoaiguouvert" ]
        }

for name in DRUM:
    DRUM[name] = [ INST[e] for e in DRUM[name] ]



if 0:
    _nextInstrumentId = [0]
    INSTRUMENTS = {}
    write_instrument_config = 0
    hard_coded_instruments = 0
    if hard_coded_instruments:
        def _addInstrument( name, csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit = None ):
            if write_instrument_config:
                if (kit == None):
                    blah = file('snd/'+name+'.snd', 'w')
                    print >>blah,  (csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur)
                    blah.close()
                else:
                    sys.exit(1)

            INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0], csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit )
            _nextInstrumentId[0] += 1

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

    else:

        sndlist = os.listdir('snd/')
        #print sndlist
        sndpaths = [p for p in sndlist if p[-4:] == '.snd']

        #load the normal instruments
        for sndpath in sndpaths:
            name = sndpath[0:-4]
            sndfile = file('snd/'+sndpath, 'r')
            line = sndfile.readline()
            #print sndpath, line
            tup = eval(line)
            if len(tup) == 7:
                csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur = tup
                INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0],
                        csoundInstrumentId, instrumentRegister, soundClass,
                        category, loopStart, loopEnd, crossDur, None, None, None )
            else:
                print 'loading a gooddmammn img and wav thing!'
                csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, wav, img = tup
                INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0],
                        csoundInstrumentId, instrumentRegister, soundClass,
                        category, loopStart, loopEnd, crossDur, None , wav, img)
            _nextInstrumentId[0] += 1
            sndfile.close()

        #load the kits
        if 0:
            kitpaths = [p for p in sndlist if p[-4:] == '.kit']   #don't use these for now..
            for kitpath in kitpaths:
                kitidx = int( kitpath[4] )
                print kitpath, kitidx
                exec  "DRUM%iKIT = {} ; curkit = DRUM%iKIT" % (kitidx,kitidx) 
                name = kitpath[0:-4]
                kitfile = file('snd/'+ kitpath, 'r')
                line = kitfile.readline()
                print kitpath, line

                keyval = eval(line)
                for kv in keyval:
                    curkit[kv[0]] = INSTRUMENTS[kv[1]]

                #csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur = eval( line )
                INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0], 0, 0, 0, "kit", 0, 0, 0, curkit)
                _nextInstrumentId[0] += 1
                kitfile.close()

        else:
            def _addInstrument( name, csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit = None ):
                blah = file('snd/' + name + '.kit', 'w')
                print >>blah, [(k, kit[k].name) for k in kit.keys()]
                blah.close()
                INSTRUMENTS[name] = Instrument( name, _nextInstrumentId[0], csoundInstrumentId, instrumentRegister, soundClass, category, loopStart, loopEnd, crossDur, kit )
                _nextInstrumentId[0] += 1
                print INSTRUMENTS[name]


            _addInstrument( "drum1kit", 0, 0, 0, "kit", 0, 0, 0, DRUM1KIT )
            _addInstrument( "drum2kit", 0, 0, 0, "kit", 0, 0, 0, DRUM2KIT )
            _addInstrument( "drum3kit", 0, 0, 0, "kit", 0, 0, 0, DRUM3KIT )
            _addInstrument( "drum4kit", 0, 0, 0, "kit", 0, 0, 0, DRUM4KIT )
            _addInstrument( "drum5kit", 0, 0, 0, "kit", 0, 0, 0, DRUM5KIT )

    #import sys
    #sys.exit(0)

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

