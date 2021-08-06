from TrackPlayerBase import TrackPlayerBase

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters
import pickle

class PagePlayer( TrackPlayerBase ):
    class Data:
        def __init__(self):
            pass

    def __init__( self, getTempoCallback, getBeatsCallback, playTickCallback, updatePageCallback, volumeFunctions, trackIDs ):
        TrackPlayerBase.__init__( self, self.getTempo, self.getBeats, playTickCallback, volumeFunctions, trackIDs )
        
        #to pickle
        self.pageTempoDictionary = {}
        #to pickle
        self.pageBeatsDictionary = {}
        self.getCurrentTempoCallback = getTempoCallback
        self.getCurrentBeatsCallback = getBeatsCallback
        self.updatePageCallback = updatePageCallback
        #to pickle
        self.trackIDs = trackIDs
        
        self.playingTune = False
        self.selectedPageIDs = set()
        
        #to pickle
        self.tunePages = []
        #to pickle
        self.currentPageIndex = -1
        #to pickle
        self.currentPageID = -1
        self.pageDictionary = {} #map: [ pageID : [ onset : events ] ]
        #to pickle
        self.trackDictionary = {} #map [ trackID : [ pageID : events ] ]

        #initialize dictionary
        for trackID in trackIDs:
            self.trackDictionary[ trackID ] = {}

    def addPage( self, pageID ):
        self.pageDictionary[ pageID ] = {}
        
        for trackID in self.trackIDs:
            self.trackDictionary[ trackID ][ pageID ] = []

    def setPage( self, index, pageID ):
        self.tunePageOrder[ index ] = pageID

    def setCurrentPageIndex( self, pageIndex ):
        if self.currentPageIndex != pageIndex:
            self.currentPageIndex = pageIndex
            
            self.eventDictionary = self.pageDictionary[ self.tunePages[ pageIndex ] ]
                
            TrackPlayerBase.handleReachedEndOfPage( self )

    def setPlayPage( self, pageID ):
        self.playingTune = False
        self.currentPageID = pageID
        
        self.update()
        self.updatePageCallback()
        
    def setPlayTune( self, pageIndex ):
        self.playingTune = True
        self.currentPageIndex = pageIndex
       
        self.update()
        self.updatePageCallback()

    #-----------------------------------
    # playback overrides
    #-----------------------------------
    def handleReachedEndOfPage( self ):
        if self.playingTune:
            if self.currentPageIndex >= len( self.tunePages ) - 1:
                self.setCurrentPageIndex( 0 )
            else:
                self.setCurrentPageIndex( self.currentPageIndex + 1 )

        TrackPlayerBase.handleReachedEndOfPage( self )
        
        #if Constants.NUMBER_OF_PAGES > 1:
        if self.playingTune:
            self.updatePageCallback()

    #-----------------------------------
    # add/remove/update methods
    #-----------------------------------
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

        if self.playingTune:
            self.eventDictionary = self.pageDictionary[ self.tunePages[ self.currentPageIndex ] ]
        else:
            self.eventDictionary = self.pageDictionary[ self.currentPageID ]
        
    def updatePage( self, trackID, pageID, events = [] ):
        if self.trackDictionary.has_key( trackID ) and self.trackDictionary[ trackID ].has_key( pageID ):
            del self.trackDictionary[ trackID ][ pageID ]
        
        self.addPage( trackID, pageID, events )
    
    #-----------------------------------
    # tempo/beats-per-page methods
    #-----------------------------------        
    def getTempo( self ):
        # TODO: hack temporaire
        #return self.pageTempoDictionary[ self.currentPageID ]
        return self.getCurrentTempoCallback()

    def setTempo( self, tempo ):
        for pageID in self.pageTempoDictionary.keys():
            self.setTempoForPage( tempo, pageID )

    def setTempoForPage( self, tempo, pageID ):
        self.pageTempoDictionary[ pageID ] = tempo
    
    def getBeats( self ):
# TODO: hack temporaire
        return self.getCurrentBeatsCallback()
#        return self.pageBeatsDictionary[ self.currentPageID ]
    
    def setBeats( self, beats ):
        self.setBeatsForPage( self, beats, self.tunePages[ currentPageIndex ] )
        
    def setBeatsPerPage( self, beats, pageID ):
        self.pageBeatsDictionary[ beats ]

    #-----------------------------------
    # misc methods
    #-----------------------------------    
    def toggleSelectPage( self, pageID ):
        toggle( self.selectedPageIDs, pageID )

    def getEvents( self, trackID ):
        if self.playingTune:
            return self.getEventsForPage( trackID, self.tunePages[ self.currentPageIndex ] )
        else:
            return self.getEventsForPage( trackID, self.currentPageID )

    def getEventsForPage( self, trackID, pageID ):
        return self.trackDictionary[ trackID ][ pageID ]
    
    # data is a tuple ( trackID, instrumentName )
    def setInstrument( self, data ):
        trackID = data[0]
        instrument = data[1]
        
        self.trackInstruments[ trackID ] = instrument
        
        if self.trackDictionary.has_key( trackID ):
            for pageID in self.trackDictionary[ trackID ].keys():
                for event in self.getEventsForPage( trackID, pageID ):
                    event.instrument = instrument
                        
    def getCurrentPageID( self ):
        if self.playingTune:
            return self.tunePages[ self.currentPageIndex ]
        else:
            return self.currentPageID

    #
    # serialization
    #
    def serialize(self, path):
        print "serialize the stuff to ", path
        pickle.dump(self.trackDictionary, open(path, 'w'))
      
    def unserialize(self, path):
        print "un serialize the stuff from ", path
        self.trackDictionary = pickle.load(open(path, 'r'))
        self.update()

