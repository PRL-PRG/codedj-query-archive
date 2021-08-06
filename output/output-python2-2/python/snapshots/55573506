import sys
import types
import time

import Config

from Util import NoteDB
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client

class TamTamOStream:
    def __init__(self, file):
        self.file = file

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
        l = [ 'page_add', str(pid), str(page.beats), str(page.color), str(page.instruments) ]
        print pid, page.instruments
        self.file.write( " ".join([str(i) for i in l]))
        self.file.write('\n')

    def tune_set(self, tune):
        self.file.write('tune_set ')
        self.file.write(" ".join([str(t) for t in tune]))
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

class TamTamTable:

    def __init__(self, noteDB):
        self.noteDB = noteDB
        self.csnd = new_csound_client()
        self.pid = {}   #stream_pid : local_pid

    def parseTable(self):
        return {
                'note_set':self.note_set,
                'note_add':self.note_add,
                'page_add':self.page_add,
                'page_set':self.page_set,
                'track_vol':self.track_vol,
                'master_vol':self.master_vol,
                'tempo':self.tempo,
                'tune_set':self.tune_set,
                'sleep':self.sleep,
                'quit':self.quit}

    def parseFile(self, ifile):
        table = self.parseTable()
        while True:
            l = ifile.readline()
            if l == '\n': break
            if l == '': break
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
        cs = CSoundNote( 
                int(argv[3]),
                int(argv[4]),
                float(argv[5]),
                float(argv[6]),
                float(argv[7]),
                int(argv[8]),
                int(argv[9]),
                float(argv[10]),
                float(argv[11]),
                float(argv[12]),
                float(argv[13]),
                float(argv[14]),
                bool(argv[15]),
                argv[16])

        self.noteDB.addNote(nid, page, track, cs )

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
        print pid, insts
        instruments = eval( insts )
        after = self.noteDB.tune[-1]
        self.pid[pid] = self.noteDB.addPage(-1, NoteDB.Page(beats,color,instruments), after)

    def page_set(self, argv):
        print 'page_set', argv

    def track_vol(self, argv):
        self.tracks_volume = []
        for i in range(len(argv)):
            self.tracks_volume.append(float(argv[i]))
    def master_vol(self, argv):
        self.masterVolume = argv[0]
    def tempo(self, argv):
        self.tempo = argv[0]

    def tune_set(self, argv):
        if Config.DEBUG > 3: print 'tune_set', argv

        if Config.DEBUG > 3: print 'ERROR: tune_set is not handled properly by mainwindow yet... skipping\n'
        return

        self.noteDB.tune = [int(i) for i in argv]
        pids = self.noteDB.pages.keys()
        pids_to_del = [pid for pid in self.noteDB.pages.keys() 
                if pid not in self.noteDB.tune]
        self.noteDB.deletePages( pids_to_del )

    def sleep(self, argv):
        t = float(argv[0])
        print 'sleeping for %i seconds' % t
        time.sleep(t)
    def quit(self, argv):
        print 'quitting...'
        sys.exit(0)

