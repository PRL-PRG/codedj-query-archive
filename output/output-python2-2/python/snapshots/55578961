import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class EventPlayer:
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, getTempoCallback, getBeatsPerPageCallback, playTickCallback ):
        self.getTempoCallback = getTempoCallback
        self.getBeatsPerPageCallback = getBeatsPerPageCallback
        self.playTickCallback = playTickCallback
        
        self.eventDictionary = {}
        
        self.playbackTimeout = None
        self.currentTick = 0
        
    #-----------------------------------
    # playback functions
    #-----------------------------------
    def playing( self ):
        return self.playbackTimeout != None
    
    def startPlayback( self ):
        msPerTick = Constants.MS_PER_MINUTE / self.getTempoCallback() / Constants.TICKS_PER_BEAT
        self.playbackTimeout = gobject.timeout_add( msPerTick, self.handlePlayTick )
        self.handlePlayTick()

    def stopPlayback( self ):
        if self.playbackTimeout != None:
            gobject.source_remove( self.playbackTimeout )
            self.playbackTimeout = None

    def play( self, onset ):
        if self.eventDictionary.has_key( onset ):
            for event in self.eventDictionary[ onset ]:
                event.play()
    
    def handlePlayTick( self ):
        self.play( self.currentTick )
        self.playTickCallback( self.currentTick )
        
        if self.currentTick >= Constants.TICKS_PER_BEAT * self.getBeatsPerPageCallback():
            self.currentTick = 0
        else:
            self.currentTick += 1
        
        return True

    #-----------------------------------
    # add/remove event functions (event(s) must be Event instances)
    #----------------------------------- 
    def add( self, event ):
        if self.eventDictionary.has_key( event.onset ):
            self.eventDictionary[ event.onset ].add( event )
        else:
            self.eventDictionary[ event.onset ] = set( [ event ] )

    def addMultiple( self, events ):
        for event in events:
            self.add( event )

    def remove( self, event ):
        if self.eventDictionary.has_key( event.onset ) and event in self.eventDictionary[ event.onset ]:
            self.eventDictionary[ event.onset ].remove( event )

    def removeMultiple( self, events ):
        for event in events:
            self.remove( event )

    def clear( self ):
        self.eventDictionary.clear()
