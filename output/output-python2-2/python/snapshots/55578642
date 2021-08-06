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

    def micRecording( self, table ):
        mess = CSoundConstants.MIC_RECORDING_COMMAND % table
        print mess
        self.sendText( mess )
        
    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        try:
            self.socket.send( text )
        except socket.error:
            if CSoundConstants.SERVER_REQUIRED : print 'ERROR: no CSound server. Ignoring message: %s' % text

    def initialize( self ):
        try:
            self.socket.connect( self.serverInfo )
            self.initializeInstruments()
        except socket.error:
            if CSoundConstants.SERVER_REQUIRED : print 'ERROR: no CSound server. Ignoring connection request.'
    
    def initializeInstruments( self ):
        for instrumentSoundFile in CSoundConstants.INSTRUMENTS.keys():
            fileName = CSoundConstants.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentID = CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ instrumentSoundFile ].instrumentID
            mess = CSoundConstants.LOAD_INSTRUMENT_COMMAND % ( instrumentID, fileName )
            self.sendText( mess )

CSoundClient = CSoundClient( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT, os.getpid() )
