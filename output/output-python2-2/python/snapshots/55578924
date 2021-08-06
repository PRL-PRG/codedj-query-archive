import socket
import os
from Framework.Constants import Constants

#----------------------------------------------------------------------
# A CSound client used to send messages to the CSound server
# CSoundClient is a singleton
#----------------------------------------------------------------------
class CSoundClient( object ):
    def __init__( self, serverAddress, serverPort, clientID ):
        self.clientID = clientID
        self.socket = socket.socket()
        self.socket.connect( ( serverAddress, serverPort ) )
        self.initializeClient()
	
    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        self.socket.send( text )

    def initializeClient(self):
        sndfiles = []

        for currentFile in os.listdir(Constants.CSOUND_SOUNDS_DIR):
            if not currentFile[0] == ".":
                sndfiles.append( currentFile )

        sndfiles.sort()
        print sndfiles

        for i in range(len(sndfiles)):
            currentFile = sndfiles[i]
            fileName = Constants.CSOUND_SOUNDS_DIR + "/" + currentFile
            mess = "perf.InputMessage('f%d 0 0 -1 \"%s\" 0 0 0')\n" % (Constants.INSTRUMENT_TABLE_OFFSET + i, fileName)
            self.sendText( mess )

CSoundClient = CSoundClient( 'localhost', 40002, os.getpid() )
