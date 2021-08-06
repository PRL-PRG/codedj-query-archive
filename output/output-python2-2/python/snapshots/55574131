
from Util.CSoundClient import new_csound_client
from Util.NoteDB import PARAMETER, NoteDB
from Util.ControlStream import *


class MiniPlayer:
    def __init__(self, ndb):
        self.csnd = new_csound_client()
        self.csnd.connect(True)
        self.csnd.loopSetNumTicks(50)
        self.csnd.loopStart()
        self.csnd.setMasterVolume(1.0)
        self.csnd.setTrackVolume(1.0, 1)

        self.noteDB = ndb
        ndb.addListener( self, page=True, note=True )

    def recompose( self, algo, params, genOrVar):

        newtracks = set(range(Config.NUMBER_OF_TRACKS))
        newpages = self.tuneInterface.getSelectedIds()

        if genOrVar == 0:
            dict = {}
            for t in newtracks:
                dict[t] = {}
                for p in newpages:
                    dict[t][p] = self.noteDB.getCSNotesByTrack( p, t )
        else:
            dict = {}
            for t in newtracks:
                dict[t] = {}
                dict[t][1] = self.noteDB.getCSNotesByTrack( 1, t )

        beatsOfPages = {}        
        for pageId in newpages:
            beatsOfPages[pageId] = self.noteDB.pages[pageId].beats

        algo(
                params,
                self._data['track_volume'][:],
                [ i.name for i in self.trackInstrument ],
                self._data['tempo'],
                beatsOfPages,
                newtracks,
                newpages,
                dict)

        # filter & fix input ...WTF!?
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    intdur = int(note.duration)
                    note.duration = intdur
                    note.pageId = page
                    note.trackId = track

        # prepare the new notes
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += dict[tid][pid]

        # delete the notes and add the new
        self.noteDB.deleteNotesByTrack( newpages, newtracks )

        stream = []
        for page in newpages:
            for track in newtracks:
                stream += [ page, track, len(dict[track][page]) ]
                stream += dict[track][page]
        stream += [-1]
        self.noteDB.addNotes( stream )
    def notifyPageAdd( self, id, at ):
        return

    def notifyPageDelete( self, which, safe ):
        pass

    def notifyPageDuplicate( self, new, at ):
        return

    def notifyPageMove( self, which, low, high ):
        return

    def notifyNoteAdd( self, page, track, id ):
        print 'INFO: adding note to loop', page, track, id
        n = self.noteDB.getNote(page, track, id)
        self.csnd.loopPlay(n,0)
        onset = n.cs.onset + 0 #self.page_onset[n.page]
        self.csnd.loopUpdate(n, PARAMETER.ONSET, onset, 1) #set onset + activate

    def notifyNoteDelete( self, page, track, id ):
        print 'INFO: deleting note from loop', page, track, id
        self.csnd.loopDelete1(page,id)
    def notifyNoteUpdate( self, page, track, id, parameter, value ):
        print 'INFO: updating note ', page, id, parameter, value
        note = self.noteDB.getNote(page, track, id)
        self.csnd.loopUpdate(note, parameter, value, -1)

if __name__ == '__main__':
    ndb = NoteDB()
    mplayer = MiniPlayer(ndb)

    ttt = TamTamTable(ndb)
    table = ttt.parseTable()
    while True:
        l = sys.stdin.readline()
        if l == '\n': break
        cmdlist = l.split()
        if cmdlist[0] not in table:
            print 'ERROR: command %s not defined by parse table' % cmdlist[0]
        else:
            table[cmdlist[0]](cmdlist[1:])

