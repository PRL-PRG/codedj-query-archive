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
            self.handleReachedEndOfPage()
        else:
            self.currentTick += 1

        return True
            
    def handleReachedEndOfPage( self ):
        self.currentTick = 0

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