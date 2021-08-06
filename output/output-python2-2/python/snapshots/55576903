import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants
from Framework.CSound.CSoundNote import CSoundNote
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants

class RythmPlayer:
    def __init__( self, client, recordButtonState ):
	self.csnd = client
        self.notesList = []
	self.sequencer = []
	self.pitchs = []
        self.tempo = 120
        self.currentTick = 0	
	self.sequencerPlayback = 0
	self.startLooking = 0
	self.recordState = 0
	self.recordButtonState = recordButtonState
        self.playbackTimeout = None
        self.beat = 4
	self.playState = 0

    def getCurrentTick(self):
        return self.currentTick

    def setTempo( self, tempo ):
        self.tempo = tempo
        if self.playbackTimeout != None:
            gobject.source_remove(self.playbackTimeout)
	    self.playState = 0
            self.startPlayback()

    def handleRecordButton( self, widget, data=None ):
	if not self.startLooking:
	    if widget.get_active() == True:
	        self.beats = [i*12 for i in range(self.beat)]
	        self.upBeats = [i+6 for i in self.beats]
	        self.startLooking = 1

    def recording( self, note ):
	if self.recordState:
	    self.pitchs.append( note.pitch )
	    note.onset = self.currentTick-2
	    self.sequencer.append( note )

    def adjustDuration( self, pitch, onset ):
	if pitch in self.pitchs:
	    offset = self.currentTick
	    for note in self.sequencer:
		if note.pitch == pitch and note.onset == onset:
		    if offset > note.onset:
			note.duration = offset - note.onset
		    else:
                        note.duration = (offset+(self.beat*12)) - note.onset
	    self.pitchs.remove( pitch )

    def playing( self ):
        return self.playbackTimeout != None
    
    def startPlayback( self ):
        if not self.playState:
            self.playbackTimeout = gobject.timeout_add( int(60000/self.tempo/12) , self.handleClock )
            self.handleClock()
	    self.playState = 1

    def stopPlayback( self ):
        if self.playbackTimeout != None:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None
	    self.playState = 0

    def handleClock( self ) :
        rval = ""
        for stream in self.notesList:
            for note in stream:
                if note.onset == self.currentTick:
                    note.play()

	if self.sequencer and self.sequencerPlayback:
	    for note in self.sequencer:
		if note.onset == self.currentTick:
		    note.play()

	if self.startLooking:
	    self.sequencerPlayback = 0
	    if self.currentTick in self.beats:
		self.recordButtonState(True)
	    if self.currentTick in self.upBeats:
		self.recordButtonState(False)
	    if self.currentTick == 0:
		self.sequencer = []
                self.pitchs = []
		self.recordState = 1
		self.startLooking = 0
		
        self.currentTick = self.currentTick + 1
        if self.currentTick >= (Constants.TICKS_PER_BEAT * self.beat):
	    if self.recordState:
		self.recordState = 0
		self.sequencerPlayback = 1
		self.recordButtonState(False)

            self.currentTick = 0

        return True

                      
