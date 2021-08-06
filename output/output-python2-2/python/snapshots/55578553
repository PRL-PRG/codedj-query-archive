import pickle
import time

from TrackPlayerBase import TrackPlayerBase

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

class PagePlayer( TrackPlayerBase ):

    DEFAULT_BEATS_PER_PAGE = 4

    def __init__( self, trackIDs, updateTickCallback, updatePageCallback ):
        TrackPlayerBase.__init__( self, trackIDs )
        
        self.updatePageCallback = updatePageCallback
        self.updateTickCallback = updateTickCallback

        #to pickle
        self.beatsPerPage = self.DEFAULT_BEATS_PER_PAGE
        self.pageBeatsDictionary = {}
        self.trackIDs = trackIDs
        self.tunePages = []      #list of pageID keys
        self.currentPageIndex = -1
        self.currentPageID = -1
        self.trackDictionary = {} #map [ trackID : [ pageID : events ] ]
        self.selectedPageIDs = set()
        
        self.pageDictionary = {} #map: [ pageID : [ onset : events ] ]
        self.playingTune = False
        self.needPageCall = False

        #initialize dictionary
        for trackID in trackIDs:
            self.trackDictionary[ trackID ] = {}

    def addPage( self, pageID ):
        self.pageDictionary[ pageID ] = {}
        self.pageBeatsDictionary[ pageID ] = self.DEFAULT_BEATS_PER_PAGE
        
        for trackID in self.trackIDs:
            self.trackDictionary[ trackID ][ pageID ] = []

    def setPage( self, index, pageID ):
        self.tunePageOrder[ index ] = pageID

    def getTunePages( self ):
        return self.tunePages

    def setCurrentPageIndex( self, pageIndex ):
        if self.currentPageIndex != pageIndex:
            self.currentPageIndex = pageIndex
            
            self.eventDictionary = self.pageDictionary[ self.tunePages[ pageIndex ] ]
            #NOTE: this function is so slow (60ms on my machine), and runs in X (high priority) 
            #      that it fux up playback
            #      we use this flag to put off updating the page until hookClock
            self.needPageCall = True

    def setPlayPage( self, pageID ):
        self.playingTune = False
        self.currentPageID = pageID
        
        self.updatePageDictionary()
        self.updatePageCallback()
        
    def setPlayTune( self, pageIndex ):
        self.playingTune = True
        self.currentPageIndex = pageIndex
       
        self.updatePageDictionary()
        self.updatePageCallback()

    def getSelectedPageIDs( self ):
        return self.selectedPageIDs

    #-----------------------------------
    # playback overrides
    #-----------------------------------

    def hookTick( self ) :
        #pass up the hierarchy
        TrackPlayerBase.hookTick( self )

        if self.currentTick >= Constants.TICKS_PER_BEAT * self.getBeats():

            #reset to a new page
            self.currentTick = 0
            if self.playingTune:
                if self.currentPageIndex >= len( self.tunePages ) - 1:
                    self.setCurrentPageIndex( 0 )
                else:
                    self.setCurrentPageIndex( self.currentPageIndex + 1 )
#            print 'hookTick: tunePages', self.tunePages
#           print 'hookTick: self.currentPageIndex', self.tunePages

    def hookClock( self ):
        TrackPlayerBase.hookClock( self )
        if self.needPageCall: 
            self.updatePageCallback()
            self.needPageCall = False
        fraction = float(self.currentTick) / float(Constants.TICKS_PER_BEAT * self.getBeats())
        self.updateTickCallback( fraction )

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
        
    #TODO this should be removed when TrackPlayer (and TrackPlayerBase) get removed
    # we should then always call updatePageDictionary
    def update( self ):
        self.updatePageDictionary()
        
    def updatePageDictionary( self ):
        self.clear()
        
        for pageID in self.pageDictionary.keys():
            self.pageDictionary[ pageID ].clear()
            
            for trackID in self.trackIDs.difference( self.mutedTrackIDs ):
                self.addMultipleToDictionary( self.trackDictionary[ trackID ][ pageID ], self.pageDictionary[ pageID ] )

        if self.playingTune:
            self.eventDictionary = self.pageDictionary[ self.tunePages[ self.currentPageIndex ] ]
        else:
            self.eventDictionary = self.pageDictionary[ self.currentPageID ]
        
    def updatePage( self, trackID, pageID, events = [] ):
        print 'pagePlayer::updatePage ', pageID
        if self.trackDictionary.has_key( trackID ) and self.trackDictionary[ trackID ].has_key( pageID ):
            del self.trackDictionary[ trackID ][ pageID ]
        
        self.addPage( trackID, pageID, events )
    
    #-----------------------------------
    # tempo/beats-per-page methods
    #-----------------------------------        
    
    def getBeats( self ):
        return self.pageBeatsDictionary[ self.currentPageID ]
    
    def setBeats( self, beats ):
        self.setBeatsForPage( self, beats, self.tunePages[ currentPageIndex ] )
        
    def setBeatsPerPage( self, beats, pageID ):
        self.pageBeatsDictionary[ pageID ] = beats

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
                        
    def getTrackInstruments( self ):
        return self.trackInstruments

    def getTrackDictionary( self ):
        return self.trackDictionary

    def setTrackDictionary( self, dict):
        self.trackDictionary = dict

    def getSelectedTrackIDs( self ):
        return self.selectedTrackIDs

    def getCurrentPageID( self ):
        if self.playingTune:
            return self.tunePages[ self.currentPageIndex ]
        else:
            return self.currentPageID

    #
    # serialization
    #
    def serialize(self, f):
        TrackPlayerBase.serialize(self, f)

        pickle.dump( self.pageBeatsDictionary, f )
        pickle.dump( self.trackIDs, f )
        pickle.dump( self.tunePages, f )
        pickle.dump( self.currentPageIndex, f )
        pickle.dump( self.currentPageID, f )
        pickle.dump( self.trackDictionary, f )
        pickle.dump( self.selectedPageIDs, f )
        pickle.dump( self.playingTune, f)

        print 'PagePlayer::serialize tunePages', self.tunePages
        print 'PagePlayer::serialize currentPageIndex', self.currentPageIndex
        print 'PagePlayer::serialize currentPageID', self.currentPageID
        print 'PagePlayer::serialize playingTune', self.playingTune

    def unserialize(self, f):
        TrackPlayerBase.unserialize(self, f )

        self.pageBeatsDictionary = pickle.load( f )
        self.trackIDs = pickle.load( f )
        self.tunePages = pickle.load( f )
        self.currentPageIndex = pickle.load( f )
        self.currentPageID = pickle.load( f )
        self.trackDictionary = pickle.load( f ) 
        self.selectedPageIDs = pickle.load( f )
        self.playingTune = pickle.load(f)

        print 'PagePlayer::unserialize tunePages', self.tunePages
        print 'PagePlayer::unserialize currentPageIndex', self.currentPageIndex
        print 'PagePlayer::unserialize currentPageID', self.currentPageID
        print 'PagePlayer::unserialize playingTune', self.playingTune

        self.updatePageDictionary()
