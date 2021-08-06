import sys
import types
import time

import common.Config as Config

import os

from common.Util import NoteDB
from common.Util.CSoundNote import CSoundNote
from common.Util.CSoundClient import new_csound_client

import common.Util.InstrumentDB as InstrumentDB

try:
    from Jam import Block
except:
    pass # we're not in Jam mode

class TamTamOStream:
    def __init__(self, file):
        self.file = file
        self.instrumentDB = InstrumentDB.getRef()

    def note_add(self, note):
        l = ['note_add', note.id, note.page, note.track,
                note.cs.onset,
                note.cs.pitch,
                note.cs.amplitude,
                note.cs.pan,
                note.cs.duration,
                note.cs.trackId,
                note.cs.instrumentId,
                note.cs.attack,
                note.cs.decay,
                note.cs.reverbSend,
                note.cs.filterType,
                note.cs.filterCutoff,
                int(note.cs.tied),
                note.cs.mode]

        self.file.write( " ".join([str(i) for i in l]))
        self.file.write('\n')

    def page_add(self, pid, page):
        l = [ 'page_add', str(pid), str(page.beats), str(page.color), str( [ self.instrumentDB.getInstrument(i).name for i in page.instruments ] ) ]
        self.file.write( " ".join([str(i) for i in l]))
        self.file.write('\n')

    def track_vol(self, vols):
        self.file.write('track_vol ')
        self.file.write(" ".join([str(t) for t in vols]))
        self.file.write('\n')

    def master_vol(self, volume):
        self.file.write('master_vol ')
        self.file.write(str(volume))
        self.file.write('\n')

    def tempo(self, tempo):
        self.file.write('tempo ')
        self.file.write(str(tempo))
        self.file.write('\n')

    def block_add( self, typeStr, active, centerX, centerY, child, data ):
        if typeStr == "Drum" or typeStr == "Instrument":
            realId = data["id"] # don't overwrite the block's data
            data["id"] = self.instrumentDB.getInstrument( data["id"] ).name
        l = [ "block_add", typeStr, str(active), str(centerX), str(centerY), str(child), str(data) ]
        self.file.write( " ".join([str(i) for i in l]))
        self.file.write('\n')
        if typeStr == "Drum" or typeStr == "Instrument":
            data["id"] = realId

    def desktop_store( self, filename, id ):
        self.file.write( "desktop_store %d\n" % id )
        try:
            file = open( filename, "r" )
            for line in file:
                self.file.write( line )
            file.close()
        except:
            if Config.DEBUG > 3: print "ERROR:: desktop_store could not open file: " + filename
        self.file.write( "desktop_store END\n" )

    def desktop_set( self, id ):
        self.file.write( "desktop_set %d\n" % id )

    def sync_beats( self, beats ):
        self.file.write( "sync_beats %d\n" % beats )

class TamTamTable:

    def __init__(self, noteDB = None, jam = None ):
        self.noteDB = noteDB
        self.jam = jam
        self.csnd = new_csound_client()
        self.pid = {}   #stream_pid : local_pid
        self.instrumentDB = InstrumentDB.getRef()

    def parseTable(self):
        return {
                'note_set':self.note_set,
                'note_add':self.note_add,
                'page_add':self.page_add,
                'page_set':self.page_set,
                'track_vol':self.track_vol,
                'master_vol':self.master_vol,
                'tempo':self.tempo,
                'block_add':self.block_add,
                'desktop_store':self.desktop_store,
                'desktop_set':self.desktop_set,
                'sync_beats':self.sync_beats,
                'sleep':self.sleep,
                'quit':self.quit}

    def parseFile(self, ifile):
        table = self.parseTable()
        self.file = ifile
        for l in self.file:
            #print "---", l
            cmdlist = l.split()
            if len(cmdlist) > 0:
                if cmdlist[0] not in table:
                    print 'ERROR: skipping command %s not found in parse table' % cmdlist[0]
                else:
                    table[cmdlist[0]](cmdlist[1:])

    def note_add(self, argv):
        if Config.DEBUG > 3: print 'note_add', argv

        nid = int(argv[0])
        page = self.pid[int(argv[1])]
        track = int(argv[2])
        instId = self.noteDB.getPage( page ).instruments[ track ]
        cs = CSoundNote(
                int(argv[3]),
                int(argv[4]),
                float(argv[5]),
                float(argv[6]),
                float(argv[7]),
                int(argv[8]),
                instId,
                float(argv[10]),
                float(argv[11]),
                float(argv[12]),
                float(argv[13]),
                float(argv[14]),
                bool(argv[15]),
                argv[16])

        self.noteDB.addNote( -1, page, track, cs )

    def note_set(self, argv):
        print 'note_set', argv

    def page_add(self, argv):
        if Config.DEBUG > 3: print 'page_add', argv
        pid = int (argv[0])
        beats = int (argv[1])
        color = int( argv[2] )
        insts = ""
        for str in argv[3:]:
            insts += str
        #print pid, insts
        instrumentNames = eval( insts )
        instruments = [ self.instrumentDB.getInstrumentByName( i ).instrumentId for i in instrumentNames ] 
        for id in instruments:
            i = self.instrumentDB.getInstrument( id )
            if i.kit == None:
                self.csnd.load_instrument(i.name)
            else:
                self.csnd.load_drumkit(i.name)
        if len( self.noteDB.tune ):
            after = self.noteDB.tune[-1]
        else:
            after = False
        self.pid[pid] = self.noteDB.addPage(-1, NoteDB.Page(beats,color,instruments), after)

    def page_set(self, argv):
        print 'page_set', argv

    def track_vol(self, argv):
        self.tracks_volume = []
        for i in range(len(argv)):
            self.tracks_volume.append(float(argv[i]))

    def master_vol(self, argv):
        self.masterVolume = eval( argv[0] )

    def tempo(self, argv):
        self.tempo = eval( argv[0] )

    def block_add( self, argv ):
        blockClass = Block.StrToClass[argv[0]]
        active = eval( argv[1] )
        x = int( argv[2] )
        y = int( argv[3] )
        child = eval( argv[4] )
        data = ""
        for str in argv[5:]:
            data += str
        data = eval( data )

        if   blockClass == Block.Drum:
            data["page"] = self.pid[ data["page"] ]
            data["id"] = self.instrumentDB.getInstrumentByName( data["id"] ).instrumentId
        elif blockClass == Block.Instrument:
            data["id"] = self.instrumentDB.getInstrumentByName( data["id"] ).instrumentId
        elif blockClass == Block.Loop:
            data["id"] = self.pid[ data["id"] ]
            self.jam.updateLoopImage( data["id"] )

        if child:
            block = blockClass( self.jam.getDesktop(), data )
            self.lastBlock.addChild( block )
        else:
            block = self.jam.getDesktop().addBlock( blockClass, data, ( x, y ) )

        block.setPlaced( True )

        if blockClass == Block.Instrument and active:
            self.jam.getDesktop().activateInstrument( block )

        self.lastBlock = block

    def desktop_store( self, argv ):
        filename = self.jam.getDesktopScratchFile( int( argv[0] ) )
        #try:
        if os.path.isfile( filename ):
            os.remove( filename )

        file = open( filename, "w" )
        for line in self.file:
            if line == "desktop_store END\n":
                break
            file.write( line )
        file.close
        #except:
        #    if Config.DEBUG > 3: print "ERROR:: desktop_store could not open file: " + filename

    def desktop_set( self, argv ):
        self.jam.setDesktop( int( argv[0] ), True )

    def sync_beats( self, argv ):
        self.jam.setSyncBeats( int( argv[0] ) )

    def sleep(self, argv):
        t = float(argv[0])
        print 'sleeping for %i seconds' % t
        time.sleep(t)
    def quit(self, argv):
        print 'quitting...'
        sys.exit(0)
