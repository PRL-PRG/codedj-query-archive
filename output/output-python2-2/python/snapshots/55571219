#!/bin/env python
import os

##############
## SOUNDS
##############

class Instrument:
    def __init__(self, id):
        self.instrumentId = id

    # build an Instrument instance from argument list
    def loadFromArgs( self, name, csoundInstrumentId, register, loopStart,
            loopEnd, crossDur, ampScale, kit, wav, img, category ):
        self.name = name
        self.csoundInstrumentId = csoundInstrumentId
        self.instrumentRegister = register
        self.loopStart = loopStart
        self.loopEnd = loopEnd
        self.crossDur = crossDur
        self.ampScale = ampScale
        self.kit = kit
        self.wav = wav
        self.img = img
        self.category = category

    # build an Instrument instance by parsing a file
    def loadFromPath(self, path ):
        f = file(path, 'r')
        magic = f.readline()[:-1]
        if (magic != 'TamTam idf v1'):
            raise 'given file has wrong header'
        self.name = f.readline()
        self.csoundInstrumentId = f.readline()
        self.register = f.readline()
        self.loopStart = float( f.readline())
        self.loopEnd = float( f.readline())
        self.crossDur = float( f.readline())
        self.ampScale = float( f.readline())
        self.kit = None
        self.wav = f.readline()
        self.img = f.readline()
        self.category = f.readline().split()
        f.close()

class InstrumentDB:

    # initialize an empty InstrumentDB instance
    def __init__(self):
        self.labelSet = {'All':set([])}  # <key>  -> all instruments labelled by <key>
        self.inst = []      # all instruments
        self.instNamed = {} # <name> -> instrument with that name
        self.instId = {}    # <instrumentId> -> instrument

    # TEMP? add instrument from args
    def addInstrumentFromArgs( self, name, csoundInstrumentId, register, loopStart,
            loopEnd, crossDur, ampScale, kit, wav, img, category ):
        i = Instrument(len(self.inst))
        self.inst += [ i ]
        i.loadFromArgs( name, csoundInstrumentId, register, loopStart, loopEnd, crossDur, ampScale, kit, wav, img, category )
        self.instNamed[ i.name ] = i
        self.instId[i.instrumentId] = i

        self.labelSet['All'].add(i)
        if not self.labelSet.has_key(category):
            self.labelSet[category] = set([])
        self.labelSet[category].add( i )

    # add an instrument to the DB by reading from an instrument definition file
    def addInstrument( self, path ):
        i = Instrument(len(self.inst))
        self.inst += [ i ]
        i.loadFromPath( path )
        self.instNamed[ i.name ] = i
        self.instId[self.instNamed[i].instrumentId] = i
        #print 'labelSet... ', self.labelSet
        self.labelSet['All'].add(i)
        if not self.labelSet.has_key(category):
            self.labelSet[category] = set([])
        self.labelSet[category].add( i )

    # try to load each file in a given folder as an instrument def. file
    def scanInstrumentDir( self, path ):
        dirlist = os.listdir( path )
        for fpath in dirlist:
            try :
                self.addInstrument( path + fpath )
            except :
                print 'ERROR: scanning instrument path %s: file %s invalid' % (path, fpath)

    def getLabels( self ):
        return self.labelSet.keys()

    def getSet( self, label ):
        return self.labelSet[label]

    def getInstrument( self, id ):
        return self.inst[id]

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
