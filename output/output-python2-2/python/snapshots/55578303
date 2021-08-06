import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants
from Framework.CSound.CSoundNote import CSoundNote
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants

class RythmPlayer:
    def __init__( self ):

        self.eventDictionary = {}

        self.currentTick = 0
        self.playbackTimeout = None
        self.tempo = Constants.DEFAULT_TEMPO

    def getCurrentTick(self):
        # used by keyboard
        return self.currentTick

    def getTempo( self ):
        return self.tempo

    def playing( self ):
        return self.playbackTimeout != None
    
    def startPlayback( self ):
        #schedule the handler...
        self.playbackTimeout = gobject.timeout_add( int(60000/self.tempo/12) , self.handleClock )
        #and call it right away too.
        self.handleClock()

    def stopPlayback( self ):
        if self.playbackTimeout != None:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None
            self.shutOff()

    def handleClock( self ) :
        rval = ""
#        if self.eventDictionary.has_key( onset ):
#            for event in self.eventDictionary[ onset ]:
#                rval += event.getText( tempo )

        print self.currentTick
        self.currentTick = self.currentTick + 1
        if self.currentTick >= (Constants.TICKS_PER_BEAT * 4):
            self.currentTick = 0

        return True

    def shutOff( self ):
        for track in range( Constants.NUMBER_OF_TRACKS ):
            for i in range( 3 ):
                csoundInstrument = i + 101
                CSoundClient.sendText( CSoundConstants. PLAY_NOTE_OFF_COMMAND % ( csoundInstrument, track ) )
                      
