from TrackPlayerBase import TrackPlayerBase

from Framework.Generation.Generator import Generator
from Framework.Generation.Generator import GenerationParameters
from Framework.CSound.CSoundConstants import CSoundConstants

#------------------------------------------------------------------------------
# A Track is a collection of events.
# TrackPlayer allows the user to create, generate, manipulate and play Tracks
#------------------------------------------------------------------------------
class TrackPlayer( TrackPlayerBase ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, getTempoCallback, getBeatsPerPageCallback, playTickCallback, volumeFunctions, trackIDs ):
        TrackPlayerBase.__init__( self, getTempoCallback, getBeatsPerPageCallback, playTickCallback, volumeFunctions, trackIDs )
        
        self.trackDictionary = {} #maps trackIDs to lists of events

    #-----------------------------------
    # add/remove/update methods
    #-----------------------------------
    def addTrack( self, trackID, instrument, events = [] ):
        if ( len( events ) != 0 ) and ( trackID not in self.mutedTrackIDs ):
            self.addMultiple( events )
    
        self.trackDictionary[ trackID ] = events
        self.trackInstruments[ trackID ] = instrument
    
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
        
        # TODO: this stuff is temporary and should be done in Generator
        # i.e. generated notes should already have their instrument set to 
        # self.trackInstruments[ trackID ]
        if self.trackInstruments.has_key( trackID ):
            instrument = self.trackInstruments[ trackID ]
        else:
            instrument = CSoundConstants.CELLO
        for event in events:
            event.instrument = instrument
        
        self.addTrack( trackID, instrument, events )

    #-----------------------------------
    # misc methods
    #-----------------------------------    
    def getEvents( self, trackID ):
        return self.trackDictionary[ trackID ]
    
    def update( self ):
        self.clear()
        
        for trackID in self.getActiveTrackIDs():
            self.addMultiple( self.trackDictionary[ trackID ] )

    def generate( self, generationParameters = GenerationParameters() ):
        if len( self.selectedTrackIDs ) == 0:
            trackIDs = self.trackIDs
        else:
            trackIDs = self.selectedTrackIDs

        for trackID in trackIDs:
            self.updateTrack( trackID, self.generator.generate( generationParameters, trackID, self.trackDictionary ) )