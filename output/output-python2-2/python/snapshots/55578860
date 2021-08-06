from TrackPlayerBase import TrackPlayerBase

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

class PagePlayer( TrackPlayerBase ):
    def __init__( self, getTempoCallback, getBeatsCallback, playTickCallback, updatePageCallback, volumeFunctions, trackIDs ):
        TrackPlayerBase.__init__( self, self.getTempo, self.getBeats, playTickCallback, volumeFunctions, trackIDs )
        
        self.pageTempoDictionary = {}
        self.pageBeatsDictionary = {}
        self.getCurrentTempoCallback = getTempoCallback
        self.getCurrentBeatsCallback = getBeatsCallback
        
        self.updatePageCallback = updatePageCallback
        self.currentPageID = 0
        self.selectedPageIDs = set()
        
        self.pageDictionary = {} #map: [ pageID : [ onset : events ] ]
        self.trackDictionary = {} #map [ trackID : [ pageID : events ] ]

    def setCurrentPage( self, pageID ):
        self.currentPageID = pageID
        self.eventDictionary = self.pageDictionary[ self.currentPageID ]

    #-----------------------------------
    # playback overrides
    #-----------------------------------
    def handleReachedEndOfPage( self ):
        if self.currentPageID >= Constants.NUMBER_OF_PAGES -1:
            self.setCurrentPage( 0 )
        else:
            self.setCurrentPage( self.currentPageID + 1 )
            
        TrackPlayerBase.handleReachedEndOfPage( self )
        
        if Constants.NUMBER_OF_PAGES > 1:
            self.updatePageCallback()

    #-----------------------------------
    # add/remove/update methods
    #-----------------------------------
    def addPage( self, trackID, pageID, events = [] ):
        self.pageDictionary[ pageID ] = {}
        self.addMultipleToDictionary( events, self.pageDictionary[ pageID ] )
    
        if ( not self.trackDictionary.has_key( trackID ) ):
            self.trackDictionary[ trackID ] = {}
    
        self.trackDictionary[ trackID ][ pageID ] = events
        self.pageTempoDictionary[ pageID ] = self.getCurrentTempoCallback()
        self.pageBeatsDictionary[ pageID ] =  self.getCurrentBeatsCallback()
        
    def addToPage( self, trackID, pageID, event ):
        self.addToDictionary( event, self.pageDictionary[ pageID ] )
        self.trackDictionary[ trackID ][ pageID ].add( event )

    def addMultipleToPage( self, trackID, pageID, events ):
        for event in events:
            self.addToPage( trackID, pageID, event )
    
    def removePage( self, pageID ):
        del self.pageDictionary[ pageID ]
        for trackID in self.trackIDs:
            del self.trackDictionary[ trackID ][ pageID ]
    
    def removeFromPage( self, trackID, pageID, event ):
        self.removeFromDictionary( event, pageDictionary[ pageID ] )
        self.trackPageDictionary[ trackID ][ pageID ].remove( event )
    
    def removeMultipleFromPage( self, trackID, pageID, events ):
        for event in events:
            self.removeFromPage( trackID, pageID, event )
        
    def update( self ):
        self.clear()
        
        for pageID in self.pageDictionary.keys():
            self.pageDictionary[ pageID ].clear()
            
            for trackID in self.getActiveTrackIDs():
                self.addMultipleToDictionary( self.trackDictionary[ trackID ][ pageID ], self.pageDictionary[ pageID ] )
                
        self.eventDictionary = self.pageDictionary[ self.currentPageID ]
        
    def updatePage( self, trackID, pageID, events = [] ):
        if self.trackDictionary.has_key( trackID ) and self.trackDictionary[ trackID ].has_key( pageID ):
            del self.trackDictionary[ trackID ][ pageID ]
        
        # TODO: this stuff is temporary and should be done in Generator
        # i.e. generated notes should already have their instrument set to 
        # self.trackInstruments[ trackID ]
        if self.trackInstruments.has_key( trackID ):
            instrument = self.trackInstruments[ trackID ]
        else:
            instrument = CSoundConstants.CELLO
        for event in events:
            event.instrument = instrument
        
        self.addPage( trackID, pageID, events )
    
    #-----------------------------------
    # tempo/beats-per-page methods
    #-----------------------------------        
    def getTempo( self ):
        return self.pageTempoDictionary[ self.currentPageID ]
    
    def setTempo( self, tempo ):
        for pageID in self.pageTempoDictionary.keys():
            self.setTempoForPage( tempo, pageID )

    def setTempoForPage( self, tempo, pageID ):
        self.pageTempoDictionary[ pageID ] = tempo
    
    def getBeats( self ):
        return self.pageBeatsDictionary[ self.currentPageID ]
    
    def setBeats( self, beats ):
        self.setBeatsForPage( self, beats, self.currentPageID )
        
    def setBeatsPerPage( self, beats, pageID ):
        self.pageBeatsDictionary[ beats ]

    #-----------------------------------
    # misc methods
    #-----------------------------------    
    def toggleSelectPage( self, pageID ):
        toggle( self.selectedPageIDs, pageID )

    def getEvents( self, trackID ):
        return self.getEventsForPage( trackID, self.currentPageID )
        
    def getEventsForPage( self, trackID, pageID ):
        return self.trackDictionary[ trackID ][ pageID ]
    
    # data is a tuple ( trackID, instrumentName )
    def setInstrument( self, data ):
        trackID = data[0]
        instrument = data[1]
        
        for pageID in self.trackDictionary[ trackID ].keys():
            for event in self.getEventsForPage( trackID, pageID ):
                event.instrument = instrument

        self.trackInstruments[ trackID ] = instrument
            
    def generate( self, generationParameters = GenerationParameters() ):
        for pageID in range( Constants.NUMBER_OF_PAGES ):
            for trackID in self.getActiveTrackIDs():
                self.updatePage( trackID, pageID, self.generator.generate( generationParameters, trackID, self.trackDictionary ) )
                
        self.update()