import csnd
import os
import socket
import select
import sys
import threading
import time
import array

import Config

from Generation.GenerationConstants import GenerationConstants
from Util.Clooper.aclient import *
from Util import NoteDB

class _CSoundClientPlugin:
    def __init__(self, orc):
        sc_initialize(orc)
        self.on = False
        self.masterVolume = 80.0
        self.periods_per_buffer = 2

    def __del__(self):
        self.connect(False)
        sc_destroy()


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
            if sc_start(self.periods_per_buffer) : 
                print 'ERROR connecting'
            else:
                self.on = True
        def disconnect():
            if sc_stop() : 
                print 'ERROR connecting'
            else:
                self.on = False

        if init and not self.on :
            reconnect()
        if not init and self.on :
            disconnect()

    def destroy( self ):
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
    def loopDelete(self, dbnote):
        sc_loop_delScoreEvent( (dbnote.page << 16) + dbnote.id)
    def loopDelete1(self, page, id):
        sc_loop_delScoreEvent( (page << 16) + id)
    def loopStart(self):
        sc_loop_playing(1)
    def loopPause(self):
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

    def loopUpdate(self, note, parameter, value):
        page = note.page
        id = note.id
        if (parameter == NoteDB.PARAMETER.ONSET):
            print 'INFO: updating onset', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 1, value)
        elif (parameter == NoteDB.PARAMETER.PITCH):
            print 'INFO: updating pitch', (page<<16)+id, value
            pitch = value
            instr = note.cs.instrumentFlag
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
            else:
                pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]
            sc_loop_updateEvent( (page<<16)+id, 3, pitch)
        elif (parameter == NoteDB.PARAMETER.AMPLITUDE):
            print 'INFO: updating amp', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 5, value)
        elif (parameter == NoteDB.PARAMETER.DURATION):
            print 'INFO: updating duration', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 2, value)
        else:
            print 'ERROR: loopUpdate(): unsupported parameter change'
    def loopPlay(self, dbnote):
        qid = (dbnote.page << 16) + dbnote.id
        sc_loop_addScoreEvent( qid, 1, 'i', self.csnote_to_array(dbnote.cs))
    def play(self, csnote, secs_per_tick):
        a = self.csnote_to_array(csnote)
        a[self.DURATION] = a[self.DURATION] * secs_per_tick
        a[self.ATTACK] = max(a[self.ATTACK]*a[self.DURATION], 0.002)
        a[self.DECAY] = max(a[self.DECAY]*a[self.DURATION], 0.002)
        sc_scoreEvent( 'i', a)

    def csnote_to_array(self, csnote):
        return self.csnote_to_array1(
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

    INSTR_TRACK=0
    ONSET=1
    DURATION=2
    PITCH=3
    AMPLITUDE=5
    ATTACK=8
    DECAY=9
    def csnote_to_array1( self, onset, 
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

            if instr in Config.DRUMKITS:
                instr = Config.DRUMSINSTRUMENTSDICT[Config.DRUMKITS.index(instr)][ key ]

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


_Client = None

def new_csound_client():
    global _Client
    if _Client == None:
        _Client = _CSoundClientPlugin( Config.TAM_TAM_ROOT + '/Resources/univorc.csd' )
        _Client.connect(True)
        _Client.setMasterVolume(100.0)
        _Client.load_instruments()
        time.sleep(0.2)
    return _Client
