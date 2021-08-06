import csnd
import os
import socket
import select
import sys
import threading
import time

from Framework.CSound.CSoundConstants import CSoundConstants

#----------------------------------------------------------------------
# A CSound client used to send messages to the CSound server
# CSoundClient is a singleton
#----------------------------------------------------------------------
class CSoundClientBase:
    def setMasterVolume(self, volume):
        self.sendText("csound.SetChannel('masterVolume', %f)\n" % volume)

    def micRecording( self, table ):
        mess = CSoundConstants.MIC_RECORDING_COMMAND % table
#        print mess
        self.sendText( mess )

    def load_instruments( self ):
        for instrumentSoundFile in CSoundConstants.INSTRUMENTS.keys():
            fileName = CSoundConstants.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentID = CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ instrumentSoundFile ].instrumentID
            mess = CSoundConstants.LOAD_INSTRUMENT_COMMAND % ( instrumentID, fileName )
            self.sendText( mess )

    def sendText(self, txt):
        raise 'noImpl'
    def initialize(self, flag):
        raise 'noImpl'
        

class CSoundClientSocket( CSoundClientBase ):
    def __init__( self, serverAddress, serverPort, clientID ):
        self.clientID = clientID
        self.serverInfo = ( serverAddress, serverPort )
        self.connected = False

    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        try:
            self.socket.send( text )
        except socket.error:
            if CSoundConstants.SERVER_REQUIRED : 
                print 'ERROR: no CSound server. Ignoring message: %s' % text

    def initialize( self, init = True ):
        if init :
            n = CSoundConstants.INIT_ATTEMPTS
            self.socket = socket.socket()
            self.connected = False
            while n > 0 and not self.connected:
                try:
                    self.socket.connect( self.serverInfo )
                    self.connected = True
                    self.load_instruments()
                except socket.error:
                    if CSoundConstants.SERVER_REQUIRED : 
                        print 'ERROR: no CSound server. Ignoring connection request.'
                    time.sleep(CSoundConstants.INIT_DELAY)
                    n = n - 1
        else : #un-init
            self.sendText( CSoundConstants.UNLOAD_TABLES_COMMAND  )
            del self.socket

class CSoundClientPerf( CSoundClientBase ):
    def __init__(self, orc):
        self.orc = orc
    def initialize( self, init = True ):
        if init:
            self.csound = csnd.Csound()
            self.perf   = csnd.CsoundPerformanceThread(self.csound)
            self.csound.Compile( self.orc )
            self.perf.Play()
            self.load_instruments()
        else:
            self.csound.SetChannel('udprecv.0.on', 0)
            self.perf.Stop()
            self.perf.Join()                                    
            self.csound.Reset()
            self.csound = 0

    def sendText(self, txt):
        #print txt
        perf = self.perf
        csound = self.csound
        exec txt

class CSoundClientPipe( CSoundClientBase ):
    def __init__(self, orc):
        self.orc = orc
    def initialize( self, init = True ):
        if init:
            (self.child_out, self.child_in) = os.popen2('csound ' + self.orc)
        else:
            self.child_in.close()
            self.child_out.close()

    def sendText(self, txt):
        str = txt[19:-3]
        if len(str) == 0: return
        print 'tosend:[%s]' % (str,)
        self.child_out.write(str)

#singleton access object
if False:
    CSoundClient = CSoundClientSocket( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT, os.getpid() )
elif False:
    CSoundClient = CSoundClientPerf( '/usr/share/olpc-csound-server/univorc.csd' )
else:
    CSoundClient = CSoundClientPerf( '/home/james/tamtam/Resources/univorc.csd' )



