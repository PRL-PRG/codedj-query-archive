import pickle
import time

import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject
import math


from Framework.Constants import Constants
from Framework.CSound.CSoundNote import CSoundNote
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class EventPlayer:
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self ):

        self.time0 = 0
        self.horizonDelay = Constants.CSOUND_HORIZON
        self.horizonTime = 0
        self.horizonOnset = 0

        self.clockDelay      = Constants.CLOCK_DELAY
        self.eventDictionary = {}

        self.currentTick = 0

        self.playbackTimeout = None
        self.tempo = Constants.DEFAULT_TEMPO

        self.send_buffer = ""

    def getCurrentTick(self):
        # used by keyboard
        return self.currentTick

    def getTempo( self ):
        return self.tempo

        
    #-----------------------------------
    # playback functions
    #-----------------------------------
    def playing( self ):
        return self.playbackTimeout != None
    
    def startPlayback( self ):
        self.time0 = time.time()
        self.horizonTime = 0.0
        self.horizonOnset = 0
        #schedule the handler...
        self.playbackTimeout = gobject.timeout_add( int ( 1000 * self.clockDelay) , self.handleClock )
        #and call it right away too.
        self.handleClock()

    def stopPlayback( self ):
        if self.playbackTimeout != None:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None
            self.shutOff()

    # this will happen
    def handleClock( self ) :
        def onsetCommand( onset, tempo, delay ):
            rval = ""
            if self.eventDictionary.has_key( onset ):
                for event in self.eventDictionary[ onset ]:
                    rval += event.getText( tempo, delay)
            return rval
    
        onsetPerSecond = self.tempo / 60.0 * Constants.TICKS_PER_BEAT

        nowTime = time.time() - self.time0

        nextTime  = self.horizonTime
        nextOnset = self.horizonOnset
        horizonTime = nowTime + self.horizonDelay
        self.horizonOnset = int( horizonTime * onsetPerSecond )
        self.horizonTime  = self.horizonOnset * onsetPerSecond

        self.send_buffer = ""
        for i in range( nextOnset, self.horizonOnset ) :
            self.delay = i / onsetPerSecond - nowTime
            if (self.delay > 0.0 ) :
                ev = self.eventDictionary
                self.hookTick( )  # may modify currentTick, eventDictionary
                self.send_buffer += onsetCommand( self.currentTick, self.tempo, self.delay )
                self.currentTick = self.currentTick + 1
            else :
                print 'WARNING: excessive latency... dropping note with delay %f' % self.delay
                
        if self.send_buffer != "" :
            CSoundClient.sendText( self.send_buffer ) 

        self.hookClock()
        return True

    #this is meant to handle things that happen once per clock... like the GUI
    def hookClock( self ) :
        pass
    # this is meant to be overridden by things that need to happen on every onset
    def hookTick( self ) :   
        pass

    #
    # serialization
    #
    VERSION = '_testing_'
    def serialize(self, f):
        pickle.dump( self.VERSION, f)
        pickle.dump( self.tempo, f )

    def unserialize(self, f):
        if pickle.load( f ) != self.VERSION :
            raise WrongVersionError
        self.tempo = pickle.load( f )

    # hack for shutOff tied notes when stop playing ( don't work when tracks are selected, probably not for mute... 
    def shutOff( self ):
        for track in range( Constants.NUMBER_OF_TRACKS ):
            for i in range( 3 ):
                csoundInstrument = i + 101
                CSoundClient.sendText( CSoundConstants. PLAY_NOTE_OFF_COMMAND % ( csoundInstrument, track ) )
                      
    #-----------------------------------
    # add/remove event functions (event(s) must be Event instances)
    #----------------------------------- 
    def add( self, event ):
        self.addToDictionary( self.eventDictionary )

    def addToDictionary( self, event, eventDictionary ):
        if eventDictionary.has_key( event.onset ):
            eventDictionary[ event.onset ].add( event )
        else:
            eventDictionary[ event.onset ] = set( [ event ] )

    def addMultiple( self, events ):
        self.addMultipleToDictionary( events, self.eventDictionary )

    def addMultipleToDictionary( self, events, eventDictionary ):
        for event in events:
            self.addToDictionary( event, eventDictionary )

    def remove( self, event ):
        self.removeFromDictionary( event, self.eventDictionary )

    def removeFromDictionary( self, event, eventDictionary ):
        if eventDictionary.has_key( event.onset ) and event in eventDictionary[ event.onset ]:
            eventDictionary[ event.onset ].remove( event )

    def removeMultiple( self, events ):
        self.removeMultipleFromDictionary( events, self.eventDictionary )

    def removeMultipleFromDictionary( self, events, eventDictionary ):
        for event in events:
            self.removeFromDictionary( event, eventDictionary )

    def clear( self ):
        self.eventDictionary.clear()

