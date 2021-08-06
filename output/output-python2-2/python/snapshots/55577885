import socket
import os
import time
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
#        print mess
        self.sendText( mess )
        
    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        try:
            self.socket.send( text )
        except socket.error:
            if CSoundConstants.SERVER_REQUIRED : print 'ERROR: no CSound server. Ignoring message: %s' % text

    def initialize( self, init = True ):
        if init :
            n = CSoundConstants.INIT_ATTEMPTS
            connected = False
            while n > 0 and not connected:
                try:
                    self.socket.connect( self.serverInfo )
                    for instrumentSoundFile in CSoundConstants.INSTRUMENTS.keys():
                        fileName = CSoundConstants.SOUNDS_DIR + "/" + instrumentSoundFile
                        instrumentID = CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ instrumentSoundFile ].instrumentID
                        mess = CSoundConstants.LOAD_INSTRUMENT_COMMAND % ( instrumentID, fileName )
                        self.sendText( mess )
                        time.sleep(0.02)
                    connected = True
                except socket.error:
                    if CSoundConstants.SERVER_REQUIRED : print 'ERROR: no CSound server. Ignoring connection request.'
                    time.sleep(CSoundConstants.INIT_DELAY)
                    n = n - 1
        else : #un-init
            self.sendText( CSoundConstants.UNLOAD_TABLES_COMMAND  )
            del self.socket

CSoundClient = CSoundClient( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT, os.getpid() )
