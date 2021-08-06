import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants
from Framework.Core.TrackPlayer import TrackPlayer
from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.Generator import Generator

class TamTamSimple:
    def __init__( self ):

        self.tempo = 60
        self.beatsPerPage = 4

        self.trackPlayer = TrackPlayer( self.getTempo, 
                                        self.getBeatsPerPage,
                                        self.updatePositionIndicator, 
                                        Constants.NUMBER_OF_TRACKS )

        self.generator = Generator()
        self.trackPlayer.generate(GenerationParameters())

        self.trackPlayer.startPlayback()
        gtk.main()
      
    def generate( self, generationParameters ):
        for trackID in self.trackIDs:
            self.updateTrack( trackID, self.generator.generate( generationParameters, trackID ) )

    def getTempo(self):
        return self.tempo

    def getBeatsPerPage(self):
        return self.beatsPerPage

    def updatePositionIndicator(self, currentTick):
        pass
