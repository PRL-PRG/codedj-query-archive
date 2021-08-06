#!/bin/env python
import os

##############
## SOUNDS
##############

class Instrument:
    def __init__(self, id):
        self.id = id

    # build an Instrument instance from argument list
    def loadFromArgs( self, name, csoundInstrumentName, register, loopStart,
            loopEnd, crossDur, wav, img, labels ):
        self.name = name
        self.csoundInstrumentName = csoundInstrumentName
        self.register = register
        self.loopStart = loopStart
        self.loopEnd = loopEnd
        self.crossDur = crossDur
        self.wav = wav
        self.img = img
        self.labels = labels

    # build an Instrument instance by parsing a file
    def loadFromPath(self, path ):
        f = file(path, 'r')
        magic = f.readline()[:-1]
        if (magic != 'TamTam idf v1'):
            raise 'given file has wrong header'
        self.name = f.readline()
        self.csoundInstrumentName = f.readline()
        self.register = f.readline()
        self.loopStart = float( f.readline())
        self.loopEnd = float( f.readline())
        self.crossDur = float( f.readline())
        self.wav = f.readline()
        self.img = f.readline()
        self.labels = f.readline().split()
        f.close()

class InstrumentDB:

    # initialize an empty InstrumentDB instance
    def __init__(self):
        self.labelSet = {'all':set([])}  # <key>  -> all instruments labelled by <key>
        self.inst = []      # all instruments
        self.instNamed = {} # <name> -> instrument with that name
        self.kit = []       # all kits.  kits are lists of 13 instruments
        self.kitNamed = {}  # <name> -> kit with that name

    # add an instrument to the DB by reading from an instrument definition file
    def addInstrument( self, path ):
        i = Instrument(len(self.inst)) 
        self.inst += [ i ]
        i.loadFromPath( path )
        self.instNamed[ i.name ] = i
        print 'labelSet... ', self.labelSet
        self.labelSet['all'].add(i)
        for l in i.labels:
            if l not in self.labelSet:
                self.labelSet[l] = set([])
            self.labelSet[l].add( i )

    # add a kit by reading from a kit definition file
    def addKit( self, path ):
        strlist = file(path, 'r').readline().split()
        if len(strlist) != 14:
            raise 'kit length != 13'
        for str in strlist[1:]:
            if str not in self.inst_named:
                raise 'invalid instrument'
        kit = [ self.instNamed[name] for name in strlist ]
        self.kit += [ kit ]

    # try to load each file in a given folder as an instrument def. file
    def scanInstrumentDir( self, path ):
        dirlist = os.listdir( path )
        for fpath in dirlist:
            try :
                self.addInstrument( path + fpath )
            except :
                print 'ERROR: scanning instrument path %s: file %s invalid' % (path, fpath)

    # try to load each file in a given folder as a kit def. file
    def scanKitDir( self, path ):
        dirlist = os.listdir( path )
        for fpath in dirlist:
            try :
                self.addKit( fpath )
            except :
                print 'ERROR: scanning kit path %s: file %s invalid' % (path, fpath)

    def debug_summarize(self):
        for i in self.inst:
            print i.id, i.name

        for l in self.labelSet:
            print l, [ i.name for i in self.labelSet[l]]


db_instance = None
def getRef():
    global db_instance
    if (None == db_instance):
        db_instance = InstrumentDB()
    return db_instance


if __name__ == "__main__":
    i1 = getRef()
    i2 = getRef()

    print i1, i2

    import sys
    i1.scanInstrumentDir( sys.argv[1] )

    i1.debug_summarize()

