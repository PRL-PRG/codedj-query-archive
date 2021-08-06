import socket
import os

#----------------------------------------------------------------------
# A CSound client used to send messages to the CSound server
# CSoundClient is a singleton
#----------------------------------------------------------------------
class CSoundClient( object ):
    def __init__( self, serverAddress, serverPort, clientID ):
        self.clientID = clientID
        self.socket = socket.socket()
        #self.socket.connect( ( serverAddress, serverPort ) )
	
    def sendText( self, text ):
        print "Sending to CSound server: %s" % text
        #self.socket.send( text )

CSoundClient = CSoundClient( 'localhost', 40002, os.getpid() )
