import socket
import os
from Framework.CSound.CSoundConstants import CSoundConstants

#----------------------------------------------------------------------
# A CSound client used to send messages to the CSound server
# CSoundClient is a singleton
#----------------------------------------------------------------------
class CSoundClient( object ):
    def __init__( self, serverAddress, serverPort, clientID ):
        self.clientID = clientID
        self.socket = socket.socket()
        self.serverInfo = ( serverAddress, serverPort )

    def setMasterVolume(self, volume):
        self.sendText("csound.SetChannel('masterVolume', %f)\n" % volume)
        
    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        self.socket.send( text )

    def initialize( self ):
        self.socket.connect( self.serverInfo )
        self.initializeInstruments()
    
    def initializeInstruments( self ):
        for instrumentSoundFile in CSoundConstants.INSTRUMENTS.keys():
            fileName = CSoundConstants.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentID = CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ instrumentSoundFile ]
            mess = CSoundConstants.LOAD_INSTRUMENT_COMMAND % ( instrumentID, fileName )
            self.sendText( mess )

CSoundClient = CSoundClient( 'localhost', 40002, os.getpid() )
