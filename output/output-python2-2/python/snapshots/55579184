import csnd
import os
import socket
import select
import sys
import threading
import time

import Config

#----------------------------------------------------------------------
# A CSound client used to send messages to the CSound server
# CSoundClient is a singleton
#----------------------------------------------------------------------
CSoundClient = None
class CSoundClientBase:
    def setMasterVolume(self, volume):
        self.sendText("csound.SetChannel('masterVolume', %f)\n" % volume)

    def micRecording( self, table ):
        mess = Config.MIC_RECORDING_COMMAND % table
        self.sendText( mess )

    def load_mic_instrument( self, inst ):
        fileName = Config.PREF_DIR + '/' + inst
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + int(fileName[-1]) + 6
        mess = Config.LOAD_INSTRUMENT_COMMAND % ( instrumentId, fileName )
        self.sendText( mess )

    def load_instruments( self ):
        for instrumentSoundFile in Config.INSTRUMENTS.keys():
            if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
                fileName = Config.PREF_DIR + '/' + instrumentSoundFile
            else:
                fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
            mess = Config.LOAD_INSTRUMENT_COMMAND % ( instrumentId, fileName )
            self.sendText( mess )

    def startTime(self):
        raise 'dont do this anymore'
        self.sendText("perf.InputMessage('i 5999 0.0  60000000')")
        # if any other message arrives to csound at the same time as this one, 
        # then the global variables will not be set up right in the orcestra
        #
        # NB: match this to the constant in the instrument 5777 of the csound orcestra
        time.sleep(0.1)

    def sendText(self, txt):
        raise 'noImpl'

    def connect(self, flag):
        raise 'noImpl'

    def destroy(self):
        pass



class CSoundClientSocket( CSoundClientBase ):
    def __init__( self, serverAddress, serverPort, clientId ):
        self.clientId = clientId
        self.serverInfo = ( serverAddress, serverPort )
        self.connected = False

    def sendText( self, text ):
        #print "Sending to CSound server: %s" % text        
        try:
            self.socket.send( text )
        except socket.error:
            if Config.SERVER_REQUIRED : 
                print 'ERROR: no CSound server. Ignoring message: %s' % text

    def connect( self, init = True ):
        if init :
            n = Config.INIT_ATTEMPTS
            self.socket = socket.socket()
            self.connected = False
            while n > 0 and not self.connected:
                try:
                    self.socket.connect( self.serverInfo )
                    self.connected = True
                    self.load_instruments()
                except socket.error:
                    if Config.SERVER_REQUIRED : 
                        print 'ERROR: no CSound server. Ignoring connection request.'
                    time.sleep(Config.INIT_DELAY)
                    n = n - 1
        else : #un-init
            self.sendText( Config.UNLOAD_TABLES_COMMAND  )
            del self.socket

class CSoundClientPerf( CSoundClientBase ):
    def __init__(self, orc):
        self.orc = orc
        self.on = False
        self.csound = csnd.Csound()
    def connect( self, init = True ):
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

class CSoundClientPipe( CSoundClientBase ):
    def __init__(self, orc):
        self.orc = orc
    def connect( self, init = True ):
        if init:
            (self.child_out, self.child_in) = os.popen2('csound ' + self.orc)
        else:
            self.child_in.close()
            self.child_out.close()

    def sendText(self, txt):
        str = txt[19:-3]
        if len(str) == 0: return
        #print 'tosend:[%s]' % (str,)
        self.child_out.write(str)

from Util.Clooper.sclient import *

class CSoundClientPlugin( CSoundClientBase ):
    def setMasterVolume(self, volume):
        self.masterVolume = volume
        if self.on:
            sc_setMasterVolume(volume)

    def setTrackpadX( self, value ):
        trackpadX = value
        sc_setTrackpadX(trackpadX)

    def setTrackpadY( self, value ):
        trackpadY = value
        sc_setTrackpadY(trackpadY)

    def micRecording( self, table ):
        sc_inputMessage( Config.CSOUND_MIC_RECORD % table )

    def load_mic_instrument( self, inst ):
        fileName = Config.PREF_DIR + '/' + inst
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + int(fileName[-1]) + 6
        sc_inputMessage(Config.CSOUND_LOAD_INSTRUMENT % (instrumentId, fileName))

    def __init__(self, orc):
        sc_initialize(orc)
        self.on = False
        self.masterVolume = 80.0

    def load_instruments( self ):
        for instrumentSoundFile in Config.INSTRUMENTS.keys():
            if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
                fileName = Config.PREF_DIR + '/' + instrumentSoundFile
            else:
                fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
            instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
            sc_inputMessage( Config.CSOUND_LOAD_INSTRUMENT % (instrumentId, fileName) )

    def connect( self, init = True ):
        def reconnect():
#            def load_instruments( ):
#                for instrumentSoundFile in Config.INSTRUMENTS.keys():
#                    if instrumentSoundFile[0:3] == 'mic' or instrumentSoundFile[0:3] == 'lab':
#                        fileName = Config.PREF_DIR + '/' + instrumentSoundFile
#                    else:
#                        fileName = Config.SOUNDS_DIR + "/" + instrumentSoundFile
#                    instrumentId = Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ instrumentSoundFile ].instrumentId
#                    sc_inputMessage( Config.CSOUND_LOAD_INSTRUMENT % (instrumentId, fileName) )

            if sc_start() : 
                print 'ERROR connecting'
            else:
                self.on = True
                sc_setMasterVolume(self.masterVolume)
                #load_instruments()
                time.sleep(0.2)
        def disconnect():
            if sc_stop() : 
                print 'ERROR connecting'
            else:
                self.on = False

        if init and not self.on :
            reconnect()
        if not init and self.on :
            disconnect()

    def destroy( self):
        self.connect(False)
        sc_destroy()

    def inputMessage(self,msg):
        sc_inputMessage(msg)

    def sendText(self, txt):
        print 'WARNING: replacing sendText() with inputMessage(%s)' % txt[19:-3]
        sc_inputMessage( txt[19:-3] )

    def loopSet_onset_note(self, onset_note):
        sc_loop_clear()
        for (o,n) in onset_note:
            n.playLoop()                   # a special non-documented CSoundNote function!

    def loopClear(self):
        sc_loop_clear()
    def loopDelete(dbnote):
        sc_loop_delEvent(dbnote.page << 16 + dbnote.id)
    def loopDelete1(page, id):
        sc_loop_delEvent(page << 16 + id)
    def loopStart(self):
        sc_loop_playing(1)
    def loopStop(self):
        sc_loop_playing(0)
    def loopSetTick(self,t):
        sc_loop_setTick(t)
    def loopGetTick(self):
        return sc_loop_getTick()
    def loopSetNumTicks(self,n):
        sc_loop_setNumTicks(n)
    def loopSetTickDuration(self,d):
        sc_loop_setTickDuration(d)
    def loopSetTempo(self,t):
        print 'INFO: loop tempo: %f -> %f' % (t, 60.0 / (Config.TICKS_PER_BEAT * t))
        sc_loop_setTickDuration( 60.0 / (Config.TICKS_PER_BEAT * t))

    def loopPlay(self, dbnote):
        print 'skipping loopPlay'
        return

        qid = dbnote.page << 16 + dbnote.id
        sc_loop_addScoreEvent( qid, True, 'i', csnote_to_array(dbnote.cs))
    def play(self, dbnote, secs_per_tick):
        print 'skipping play'
        return
        a = csnote_to_array(dbnote.cs)
        a[1] = a[0] * secs_per_tick
        a[8] = max(a[8]*a[1], 0.002)
        a[9] = max(a[9]*a[1], 0.002)
        cs_addScoreEvent( 'i', a)

    def csnote_to_array(csnote):
        return csnote_to_array1(
                csnote.onset, 
                csnote.pitch,
                csnote.amplitude,
                csnote.pan,
                csnote.duration, 
                csnote.trackId, 
                csnote.fullDuration, 
                csnote.attack,
                csnote.decay,
                csnote.reverbSend,
                csnote.filterType,
                csnote.filterCutoff,
                csnote.tied,
                csnote.overlap,
                csnote.instrumentFlag)

    def csnote_to_array1( onset, 
            pitch, 
            amplitude, 
            pan, 
            duration, 
            trackId, 
            fullDuration = False, 
            attack = 0.002, 
            decay = 0.098, 
            reverbSend = 0.1, 
            filterType = 0, 
            filterCutoff = 1000,
            tied = False,
            overlap = False,
            instr = Config.FLUTE  ):

        if instr[0:4] == 'drum':
            if pitch in GenerationConstants.DRUMPITCH:
                key = GenerationConstants.DRUMPITCH[ pitch ]
            else: 
                key = pitch

            if instr == 'drum1kit':
                instr = Config.DRUM1INSTRUMENTS[ key ]
            if instr == 'drum2kit':
                instr = Config.DRUM2INSTRUMENTS[ key ]
            if instr == 'drum3kit':
                instr = Config.DRUM3INSTRUMENTS[ key ]
            pitch = 1
            time_in_ticks = 0
        else:
            pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]

            # condition for tied notes
            if Config.INSTRUMENTS[ instr ].csoundInstrumentId  == 101  and tied and fullDuration:
                duration= -1.0
            # condition for overlaped notes
            if Config.INSTRUMENTS[ instr ].csoundInstrumentId == 102 and overlap:
                duration += 1.0
            time_in_ticks = 1

        # condition for tied notes
        if Config.INSTRUMENTS[ instr].csoundInstrumentId  == Config.INST_TIED  and tied and fullDuration:
            duration = -1
        # condition for overlaped notes
        if Config.INSTRUMENTS[ instr ].csoundInstrumentId == Config.INST_PERC and overlap:
            duration = duration + 1.0

        a = array.array('f')
        a.extend( [
                 Config.INSTRUMENTS[ instr ].csoundInstrumentId + trackId * 0.01,
                 onset,
                 duration,
                 pitch,
                 reverbSend,
                 amplitude,
                 pan,
                 Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[instr].instrumentId,
                 attack,
                 decay,
                 filterType,
                 filterCutoff,
                 Config.INSTRUMENTS[ instr ].loopStart,
                 Config.INSTRUMENTS[ instr ].loopEnd,
                 Config.INSTRUMENTS[ instr ].crossDur ])
        return a
