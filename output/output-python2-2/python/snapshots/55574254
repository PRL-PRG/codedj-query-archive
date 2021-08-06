import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject
import time
import Config
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client
from Util.NoteDB import Note

class MiniSequencer:
    def __init__( self, recordButtonState ):
        self.notesList = []
        self.sequencer = []
        self.pitchs = []
        self.beat = 4
        self.tempo = Config.PLAYER_TEMPO 
        self.tickDuration = 60. / self.tempo / 12.
        self.tick = 15
        self.id = 1000
        self.csnd = new_csound_client()
        self.startLooking = 0
        self.recordState = 0
        self.recordButtonState = recordButtonState
        self.playbackTimeout = None
        self.playState = 0

    def setTempo( self, tempo ):
        self.tempo = tempo
        self.tickDuration = 60. / self.tempo / 12.
        gobject.source_remove( self.playBackTimeout )
        self.playState = 0

    def handleRecordButton( self, widget, event ):
        if not self.startLooking:
            if 1: #widget.get_active() == True:
                self.beats = [i*4 for i in range(self.beat)]
                self.upBeats = [i+2 for i in self.beats]
                self.realTick = [i for i in range(self.beat*4)]
                if event.button == 1:
                    for n in self.notesList:
                        self.csnd.loopDelete(n)
                    self.notesList = []
                self.startLooking = 1
                self.startPlayback()

    def getPlayState( self ):
        return self.playState

    def startPlayback( self ):
        if not self.playState:
            self.playbackTimeout = gobject.timeout_add( int(60000/self.tempo/12), self.handleClock )
            self.handleClock()
            self.playState = 1

    def stopPlayback( self ):
        if self.playbackTimeout != None:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None
            self.playState = 0

    def recording( self, note ):
        if self.recordState:
            self.pitchs.append( note.pitch )
            self.sequencer.append( note )

    def adjustDuration( self, pitch, onset ):
        if pitch in self.pitchs:
            offset = self.csnd.loopGetTick()
            for note in self.sequencer:
                if note.pitch == pitch and note.onset == onset:
                    if offset > note.onset:
                        note.duration = ( offset - note.onset ) + 6
                    else:
                        note.duration = ( (offset+(self.beat*Config.TICKS_PER_BEAT)) - note.onset ) + 6

                    n = Note(0, note.trackId, self.id, note)
                    self.notesList.append(n)
                    self.id = self.id + 1
                    self.csnd.loopPlay(n,1)                    #add as active

            self.pitchs.remove( pitch )

    def handleClock( self ):
        t = self.csnd.loopGetTick() / 3
        if self.tick != t:
            self.tick = t
            if self.startLooking:
                if self.tick in self.beats:
                    self.recordButtonState(True)
                if self.tick in self.upBeats:
                    self.recordButtonState(False)
                if self.tick == 0:
                    self.sequencer = []
                    self.pitchs = []
                    self.recordState = 1
                    self.startLooking = 0

            if self.tick >= (4 * self.beat - 1):
                if self.recordState:
                    self.recordState = 0
                    self.recordButtonState(False)

        return True

