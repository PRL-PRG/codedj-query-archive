from EventPlayer import EventPlayer

from Framework.Generation.Generator import Generator
from Framework.Generation.Generator import GenerationParameters
from Framework.CSound.CSoundConstants import CSoundConstants

#------------------------------------------------------------------------------
# A Track is a collection of events.
# TrackPlayer allows the user to create, generate, manipulate and play Tracks
#------------------------------------------------------------------------------
class TrackPlayer( EventPlayer ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, getTempoFunction, getBeatsPerPageFunction, playTickCallback, trackIDs ):
        EventPlayer.__init__( self, getTempoFunction, getBeatsPerPageFunction, playTickCallback )
        
        self.trackIDs = trackIDs
        self.selectedTrackIDs = set()
        self.mutedTrackIDs = set()
        
        self.trackDictionary = {} #maps trackIDs to lists of events
        self.trackInstruments = {} #maps trackIDs to instrumentNames
        
        self.generator = Generator()
        
    #-----------------------------------
    # add/remove/update/generate methods
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
    
    def generate( self, generationParameters = GenerationParameters() ):
        # TODO choose which collection of trackIDs based on whether self.selectedTrackIDs is empty
        if len( self.selectedTrackIDs ) == 0:
            for trackID in self.trackIDs:
                self.updateTrack( trackID, self.generator.generate( generationParameters, trackID, self.trackDictionary ) )
        else:
            for trackID in self.selectedTrackIDs:
                self.updateTrack( trackID, self.generator.generate( generationParameters, trackID, self.trackDictionary ) )

    #-----------------------------------
    # misc methods
    #-----------------------------------    
    def getEvents( self, trackID ):
        return self.trackDictionary[ trackID ]
    
    # data is a tuple ( trackID, instrumentName )
    def setInstrument( self, data ):
        trackID = data[0]
        instrument = data[1]
        for event in self.getEvents( trackID ):
            event.instrument = instrument

        self.trackInstruments[ trackID ] = instrument
    
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
