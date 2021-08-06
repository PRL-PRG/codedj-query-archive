from EventPlayer import EventPlayer

from Framework.Generation.Generator import Generator
from Framework.Generation.Generator import GenerationParameters

#------------------------------------------------------------------------------
# A Track is a collection of events.
# TrackPlayer allows the user to create, generate, manipulate and play Tracks
#------------------------------------------------------------------------------
class TrackPlayer( EventPlayer ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, getTempoFunction, getBeatsPerPageFunction, playTickCallback, numberOfTracks = 0 ):
        EventPlayer.__init__( self, getTempoFunction, getBeatsPerPageFunction, playTickCallback )
        
        self.trackIDs = set( range( 0, numberOfTracks ) )
        self.selectedTrackIDs = set()
        self.mutedTrackIDs = set()
        
        self.trackDictionary = {} #maps trackIDs to lists of events
        for trackID in self.trackIDs:
            self.addTrack( trackID )

        self.generator = Generator()
        
    #-----------------------------------
    # add/remove/update/generate methods
    #-----------------------------------
    def addTrack( self, trackID, events = [] ):
        if ( len( events ) != 0 ) and ( trackID not in self.mutedTrackIDs ):
            self.addMultiple( events )
    
        self.trackDictionary[ trackID ] = events
    
    def addToTrack( self, trackID, event ):
        self.add( event )
        self.trackDictionary[ trackID ].add( event )

    def addMultipleToTrack( self, trackID, events ):
        self.addMultiple( events )
        for event in events:
            self.trackDictionary[ trackID ].add( event )
    
    def removeTrack( self, trackID ):
        if self.trackDictionary.has_key( trackID ):
            self.removeMultiple( self.trackDictionary[ trackID ] )
            del self.trackDictionary[ trackID ]
    
    def removeFromTrack( self, trackID, event ):
        self.remove( event )
        self.trackDictionary[ trackID ].remove( event )
    
    def removeMultipleFromTrack( self, trackID, events ):
        self.removeMultiple( events )
        for event in events:
            self.trackDictionary[ trackID ].remove( event )
        
    def updateTrack( self, trackID, events = [] ):
        if self.trackDictionary.has_key( trackID ):
            self.removeTrack( trackID )
        
        self.addTrack( trackID, events )
    
    def generate( self, generationParameters = GenerationParameters() ):
        # TODO choose which collection of trackIDs based on whether self.selectedTrackIDs is empty
        if len( self.selectedTrackIDs ) == 0:
            for trackID in self.trackIDs:
                self.updateTrack( trackID, self.generator.generate( generationParameters ) )
        else:
            for trackID in self.selectedTrackIDs:
                self.updateTrack( trackID, self.generator.generate( generationParameters ) )

    #-----------------------------------
    # misc methods
    #-----------------------------------    
    def getEvents( self, trackID ):
        return self.trackDictionary[ trackID ]
    
    # to be called whenever the muted/selected tracks change
    def update( self ):
        self.clear()
            
        # TODO: there's probably a nicer way to do this...
        if len( self.selectedTrackIDs ) != 0:
            trackIDs = self.selectedTrackIDs
        else:
            trackIDs = set( self.trackDictionary.keys() ).difference( self.mutedTrackIDs )
        
        for trackID in trackIDs:
            self.addMultiple( self.trackDictionary[ trackID ] )

    def toggleMuteTrack( self, trackID ):
        if trackID in self.mutedTrackIDs:    
            self.mutedTrackIDs.discard( trackID )
        else:
            self.mutedTrackIDs.add( trackID )

        self.update()