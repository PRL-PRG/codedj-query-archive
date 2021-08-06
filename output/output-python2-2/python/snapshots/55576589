import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject
import time
import Config
from Util.CSoundNote import CSoundNote

class RythmPlayer:
    def __init__( self, client, recordButtonState, noteLooper ):
        self.notesList = []
        self.sequencer = []
        self.pitchs = []
        self.tempo = 120
        self.csnd = client
        self.sequencerPlayback = 0
        self.startLooking = 0
        self.recordState = 0
        self.recordButtonState = recordButtonState
        self.playbackTimeout = None
        self.beat = 4
        self.playState = 0
        self.noteLooper = noteLooper
        self.record_button_loop = -2
        self.inserted = 1

    def getCurrentTick( self ):
        return self.noteLooper.getTick(time.time(), True)

    def currentLoop(self):
        return self.noteLooper.getTick(time.time(), False) / (self.beat * Config.TICKS_PER_BEAT)

    def handleRecordButton( self, widget, data=None ):
        # record which loop we are in
        if widget.get_active() == True:
            print '******************** record pushed **************************' 
            self.record_button_loop = self.currentLoop()

            #button flash stuff
#        if not self.startLooking:
#            if widget.get_active() == True:
#                self.beats = [i*12 for i in range(self.beat)]
#                self.upBeats = [i+6 for i in self.beats]
#                self.startLooking = 1

    def recording( self, note ):
        if self.currentLoop() == self.record_button_loop + 1:
            print '******************* note received ****************************'
            self.pitchs.append( note.pitch )
            note.onset = self.noteLooper.getTick(time.time(), True)
            self.sequencer.append( note )

        if len(self.pitchs) == 0 and self.currentLoop() >= self.record_button_loop + 2 and self.inserted:
            print '***************** sequence inserted **********************'
            self.noteLooper.insert([(x.onset, x) for x in self.sequencer])
            self.recordButtonState(False)
            self.inserted = 0

    def adjustDuration( self, pitch, onset ):
        if pitch in self.pitchs:
            print '**************** duration adjusted for pitch %i *************' % pitch
            offset = self.noteLooper.getTick(time.time(), True)
            for note in self.sequencer:
                if note.pitch == pitch and note.onset == onset:
                    if offset > note.onset:
                        note.duration = offset - note.onset + 6
                        note.onset = note.onset
                    else:
                        note.duration = (offset+(self.beat*12)) - note.onset + 6
                        note.onset = note.onset
            self.pitchs.remove( pitch )

