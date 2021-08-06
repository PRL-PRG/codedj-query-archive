import csnd
import os
import socket
import select
import sys
import threading
import time
from sugar import env

import Config

class CSoundClientBase:
    def __init__(self, orc):
        self.orc = orc
        self.on = False
        self.csound = csnd.Csound()

    def micRecording( self, table ):
        mess = Config.MIC_RECORDING_COMMAND % table
        self.sendText( mess )

    def load_mic_instrument( self, inst ):
        home_path = env.get_profile_path() + Config.PREF_DIR
        fileName = home_path + '/' + inst
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + int(fileName[-1]) + 6
        mess = Config.LOAD_INSTRUMENT_COMMAND % ( instrumentId, fileName )
        self.sendText( mess )

    def load_instruments( self ):
        home_path = env.get_profile_path() + Config.PREF_DIR
        for instrumentSoundFile in Config.INSTRUMENTS.keys():
            if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
                fileName = home_path + '/' + instrumentSoundFile
            else:
                fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
            mess = Config.LOAD_INSTRUMENT_COMMAND % ( instrumentId, fileName )
            self.sendText( mess )

    def startTime(self):
        self.sendText("perf.InputMessage('i 5999 0.0  60000000')")
        # if any other message arrives to csound at the same time as this one, 
        # then the global variables will not be set up right in the orcestra
        #
        # NB: match this to the constant in the instrument 5777 of the csound orcestra
        time.sleep(0.1)

    def initialize( self, init = True ):
        if init:
            if self.on : return
            self.on = True
            self.perf   = csnd.CsoundPerformanceThread(self.csound)
            self.csound.Compile( self.orc )
            self.perf.Play()
            self.load_instruments()
            print 'CSoundClient = True'
        else:
            if not self.on : return
            self.on = False
            #self.csound.SetChannel('udprecv.0.on', 0)
            #print Config.UNLOAD_TABLES_COMMAND
            self.sendText( Config.UNLOAD_TABLES_COMMAND  )
            #print 'PERF STOP'
            self.perf.Stop()
            #print 'SLEEP'
            #time.sleep(1)
            #print 'JOIN'
            #time.sleep(1)
            rval = self.perf.Join()
            #print 'Join returned ', rval
            #del self.perf
            #time.sleep(1)
            #print 'STOP'
            #self.csound.Stop()
            #print 'RESET'
            self.csound.Reset()
            print 'CSoundClient = False'
            #careful how much cleaning up we do... don't cause a segault!
            # better to leave a segfault for the automatic cleanning at the end of the prog
            
            #self.csound.Cleanup()
            #print 'STOPPED'
            #time.sleep(1)
            #del self.csound
            #print 'DELETED'
            #time.sleep(1)

    def setMasterVolume(self, volume):
        self.csound.SetChannel('masterVolume',volume )

    def sendText(self, txt):
        #print txt
        perf = self.perf
        csound = self.csound
        if 'csound' in txt:
            print txt
            import sys
            sys.exit(0)
        exec txt

