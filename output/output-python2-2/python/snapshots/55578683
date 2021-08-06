import pickle

from EventPlayer import EventPlayer
from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundNote import CSoundNote

class TrackPlayerBase( EventPlayer ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, trackIDs ):
        EventPlayer.__init__( self )
        
        self.trackIDs = set( trackIDs )
        self.selectedTrackIDs = set()
        self.mutedTrackIDs = set()
        self.trackInstruments = {} #maps trackIDs to instrumentNames
        self.trackVolumes = {} #maps trackIDs to floats (volume)

        CSoundNote.getVolumeCallback = self.getTrackVolume

        for id in self.trackIDs : 
            if id == 0 :
                self.trackInstruments[ id ] = CSoundConstants.FLUTE
            elif id == 1 :
                self.trackInstruments[ id ] = CSoundConstants.HHC
            elif id == 2 :
                self.trackInstruments[ id ] = CSoundConstants.SNARE
            elif id == 3 : 
                self.trackInstruments[ id ] = CSoundConstants.BD
            else :
                self.trackInstruments[ id ] = CSoundConstants.FLUTE

            self.trackVolumes[ id ] = Constants.DEFAULT_VOLUME


    def getTrackVolume( self, trackID ):
        if self.trackVolumes.has_key( trackID ):
            return self.trackVolumes[ trackID ]
        else:
            return Constants.DEFAULT_VOLUME

    #-----------------------------------
    # toggle methods
    #-----------------------------------        
    def toggleSelectTrack( self, trackID ):
        self.toggle( self.selectedTrackIDs, trackID )
        self.update()
    
    def toggleMuteTrack( self, trackID ):
        self.toggle( self.mutedTrackIDs, trackID )
        self.update()
        
    def toggle( self, set, object ):
        if object in set:
            set.discard( object )
        else:
            set.add( object )
            
    #-----------------------------------
    # misc methods
    #-----------------------------------                    
    def getActiveTrackIDs( self ):
        if len( self.selectedTrackIDs ) == 0:
            return self.trackIDs
        else:
            return self.selectedTrackIDs

    # data is a tuple ( trackID, instrumentName )
    def setInstrument( self, data ):
        trackID = data[0]
        instrument = data[1]
        for event in self.getEvents( trackID ):
            event.instrument = instrument

        self.trackInstruments[ trackID ] = instrument
        
    def update( self ):
       raise NotImplementedError

    #
    # serialization
    #
    def serialize(self, f):
        EventPlayer.serialize(self, f)

        pickle.dump( self.pageBeatsDictionary, f )
        pickle.dump( self.trackIDs, f )
        pickle.dump( self.selectedTrackIDs, f )
        pickle.dump( self.mutedTrackIDs, f )
        pickle.dump( self.trackInstruments, f )
        pickle.dump( self.trackVolumes, f )
        

    def unserialize(self, f):
        EventPlayer.unserialize(self, f )

        self.pageBeatsDictionary = pickle.load( f )
        self.trackIDs = pickle.load( f )
        self.selectedTrackIDs = pickle.load( f )
        self.mutedTrackIDs = pickle.load( f )
        self.trackInstruments = pickle.load( f )
        self.trackVolumes = pickle.load( f )
