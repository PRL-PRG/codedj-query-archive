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
                self.trackInstruments[ id ] = CSoundConstant.FLUTE

            self.trackVolumes[ id ] = Constants.DEFAULT_VOLUME

    def getTrackVolume( self, trackID ):
        return self.trackVolumes[ trackID ]

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
        if len( self.selectedTrackIDs ) != 0:
            return self.selectedTrackIDs
        else:
            return self.trackIDs.difference( self.mutedTrackIDs )

    # data is a tuple ( trackID, instrumentName )
    def setInstrument( self, data ):
        trackID = data[0]
        instrument = data[1]
        for event in self.getEvents( trackID ):
            event.instrument = instrument

        self.trackInstruments[ trackID ] = instrument
        
    def update( self ):
       raise NotImplementedError

