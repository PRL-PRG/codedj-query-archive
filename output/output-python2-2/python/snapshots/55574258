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
    def __init__(self, orc, logpath=""):
        sc_initialize(orc, logpath)
        self.on = False
        #self.masterVolume = 80.0
        self.periods_per_buffer = 2

    def __del__(self):
        self.connect(False)
        sc_destroy()


    def setMasterVolume(self, volume):
        #self.masterVolume = volume
        if self.on:
            sc_setMasterVolume(volume)

    def setTrackVolume( self, volume, trackId ):
        self.trackVolume = volume
        sc_setTrackVolume(volume, trackId)

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

    def load_synth_instrument( self, inst ):
        fileName = Config.PREF_DIR + '/' + inst
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + int(fileName[-1]) + 85
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
                if (Config.DEBUG > 0) : print 'ERROR connecting'
            else:
                self.on = True
        def disconnect():
            if sc_stop() : 
                if (Config.DEBUG > 0) : print 'ERROR connecting'
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
        if (Config.DEBUG > 3) : print 'INFO: loop tempo: %f -> %f' % (t, 60.0 / (Config.TICKS_PER_BEAT * t))
        sc_loop_setTickDuration( 60.0 / (Config.TICKS_PER_BEAT * t))

    def loopDeactivate(self, note = None):
        if note == None:
            sc_loop_deactivate_all()
        else:
            if (Config.DEBUG > 0) : print 'ERROR: deactivating a single note is not implemented'

    def loopUpdate(self, note, parameter, value,cmd):
        page = note.page
        id = note.id
        if note.cs.mode == 'mini':
            instrument_offset = 0
        elif note.cs.mode == 'edit':
            if Config.INSTRUMENTSID[note.cs.instrumentId].soundClass == 'drum':
                instrument_offset = 0
            else:
                instrument_offset = 100
        if (parameter == NoteDB.PARAMETER.ONSET):
            if (Config.DEBUG > 2): print 'INFO: updating onset', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 1, value, cmd)
        elif (parameter == NoteDB.PARAMETER.PITCH):
            if (Config.DEBUG > 2): print 'INFO: updating pitch', (page<<16)+id, value
            pitch = value
            if Config.INSTRUMENTSID[note.cs.instrumentId].kit != None:
                instrument = Config.INSTRUMENTSID[note.cs.instrumentId].kit[pitch]
                csoundInstId = instrument.csoundInstrumentId
                csoundTable  = Config.INSTRUMENT_TABLE_OFFSET + instrument.instrumentId
                if (Config.DEBUG > 2): print 'INFO: updating drum instrument (pitch)', (page<<16)+id, instrument.name, csoundInstId
                sc_loop_updateEvent( (page<<16)+id, 0, (csoundInstId + instrument_offset) + note.track * 0.01, -1 )
                sc_loop_updateEvent( (page<<16)+id, 7, csoundTable  , -1 )
                pitch = 1
            else:
                pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]
            sc_loop_updateEvent( (page<<16)+id, 3, pitch, cmd)
        elif (parameter == NoteDB.PARAMETER.AMPLITUDE):
            if (Config.DEBUG > 2): print 'INFO: updating amp', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 5, value, cmd)
        elif (parameter == NoteDB.PARAMETER.DURATION):
            if (Config.DEBUG > 2): print 'INFO: updating duration', (page<<16)+id, value
            sc_loop_updateEvent( (page<<16)+id, 2, value, cmd)
        elif (parameter == NoteDB.PARAMETER.INSTRUMENT):
            pitch = note.cs.pitch
            instrument = Config.INSTRUMENTSID[value]
            if instrument.kit != None:
                instrument = instrument.kit[pitch]
            csoundInstId = instrument.csoundInstrumentId
            csoundTable  = Config.INSTRUMENT_TABLE_OFFSET + instrument.instrumentId
            loopStart = instrument.loopStart
            loopEnd = instrument.loopEnd
            crossDur = instrument.crossDur
            if (Config.DEBUG > 2): print 'INFO: updating instrument', (page<<16)+id, instrument.name, csoundInstId
            sc_loop_updateEvent( (page<<16)+id, 0, (csoundInstId + instrument_offset) + note.track * 0.01, cmd )
            sc_loop_updateEvent( (page<<16)+id, 7, csoundTable, -1 )
            sc_loop_updateEvent( (page<<16)+id, 12, loopStart, -1 )
            sc_loop_updateEvent( (page<<16)+id, 13, loopEnd, -1 )
            sc_loop_updateEvent( (page<<16)+id, 14, crossDur , -1 )
        else:
            if (Config.DEBUG > 0): print 'ERROR: loopUpdate(): unsupported parameter change'

    def loopPlay(self, dbnote, active):
        qid = (dbnote.page << 16) + dbnote.id
        sc_loop_addScoreEvent( qid, 1, active, 'i', self.csnote_to_array(dbnote.cs))

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
                csnote.attack,
                csnote.decay,
                csnote.reverbSend,
                csnote.filterType,
                csnote.filterCutoff,
                csnote.tied,
                csnote.instrumentId,
                csnote.mode)

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
            attack = 0.002, 
            decay = 0.098, 
            reverbSend = 0.1, 
            filterType = 0, 
            filterCutoff = 1000,
            tied = False,
            instrumentId = Config.INSTRUMENTS["flute"].instrumentId,
            mode = 'edit' ):

        instrument = Config.INSTRUMENTSID[instrumentId]
        if instrument.kit != None:
            instrument = instrument.kit[pitch]
            pitch = 1
            time_in_ticks = 0
        else:
            pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]
            time_in_ticks = 1

        instrument_offset = 0
        # condition for tied notes
        if instrument.csoundInstrumentId  == Config.INST_TIED  and tied and mode == 'mini':
            duration = -1
            instrument_offset = 0
        elif instrument.csoundInstrumentId == Config.INST_TIED and not tied and mode == 'mini':
            instrument_offset = 0
        elif instrument.csoundInstrumentId == Config.INST_TIED and mode == 'edit':
            instrument_offset = 100

        if instrument.csoundInstrumentId == Config.INST_SIMP and mode == 'mini':
            instrument_offset = 0
        elif instrument.csoundInstrumentId == Config.INST_SIMP and mode == 'edit':
            if instrument.soundClass == 'drum':
                instrument_offset = 0
            else:
                instrument_offset = 100

        a = array.array('f')
        a.extend( [
                 (instrument.csoundInstrumentId + trackId + instrument_offset) + trackId * 0.01,
                 onset,
                 duration,
                 pitch,
                 reverbSend,
                 amplitude,
                 pan,
                 Config.INSTRUMENT_TABLE_OFFSET + instrument.instrumentId,
                 attack,
                 decay,
                 filterType,
                 filterCutoff,
                 instrument.loopStart,
                 instrument.loopEnd,
                 instrument.crossDur ])
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
