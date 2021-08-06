import csnd
import os
import socket
import select
import sys
import threading
import time
import bisect

from sugar import env
import Config

from Util.CSoundNote import CSoundNote  #maybe not actually used, but dependence is there.  All notes are assumed to be CSoundNotes
from Generation.GenerationConstants import GenerationConstants

class Sound:
    #PRIVATE
    DRIFT = 0.01  #careful about changing this... coordinate with instrument 5777
    def loop_work(self, sleeptime):
        def next( ) :
            time_time = time.time()
            #tickhorizon is tick where we'll be after range_sec
            tickhorizon = self.getTick( self.range_sec + time_time, False )  
            time0_time = self.time0 - self.time_start + self.DRIFT

            if tickhorizon < 0 : return []
            if len(self.notes) == 0 : return []

            def cache_cmd(secs_per_tick, amplitude, pitch, inst, trackId, duration, tied, fullDuration, overlap, attack, decay, reverbSend, filterType, filterCutoff, pan ):
                if inst[0:4] == 'drum':
                    if pitch in GenerationConstants.DRUMPITCH:
                        key = GenerationConstants.DRUMPITCH[ pitch ]
                    else: 
                        key = pitch

                    if inst == 'drum1kit':
                        inst = Config.DRUM1INSTRUMENTS[ key ]
                    if inst == 'drum2kit':
                        inst = Config.DRUM2INSTRUMENTS[ key ]
                    if inst == 'drum3kit':
                        inst = Config.DRUM3INSTRUMENTS[ key ]
                    pitch = 1

                else:
                    pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]

                    # condition for tied notes
                    if Config.INSTRUMENTS[ inst ].csoundInstrumentId  == 101  and tied and fullDuration:
                        duration= -1.0
                    # condition for overlaped notes
                    if Config.INSTRUMENTS[ inst ].csoundInstrumentId == 102 and overlap:
                        duration += 1.0

                attack = max( 0.002, duration * attack)
                decay  = max( 0.002, duration * decay)

                rval = Config.PLAY_NOTE_COMMAND_MINUS_DELAY % \
                    ( Config.INSTRUMENTS[ inst ].csoundInstrumentId, 
                        trackId, 
                        '%f', #delay,
                        duration, 
                        pitch, 
                        reverbSend,
                        amplitude, 
                        pan,
                        Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ inst ].instrumentId,
                        attack,
                        decay,
                        filterType, filterCutoff )
                return rval

            def getText(i, secs_per_tick, time_offset):
                (onset,note,cache,z) = self.notes[i]
                if cache == '' or note.nchanges != z :
                    self.notes[i] = \
                            (
                                onset,
                                note, 
                                cache_cmd( 
                                    secs_per_tick, 
                                    note.amplitude, # * track-level mixer rate
                                    note.pitch,
                                    note.instrumentFlag,
                                    note.trackId,
                                    note.duration * self.secs_per_tick,
                                    note.tied,
                                    note.fullDuration,
                                    note.overlap,
                                    note.attack,
                                    note.decay, 
                                    note.reverbSend, 
                                    note.filterType, 
                                    note.filterCutoff,
                                    note.pan),
                                note.nchanges
                            )
                rval = self.notes[i][2] % float(onset * self.secs_per_tick + time_offset) 
                return rval

            prev_secs = (self.loops * self.duration) * self.secs_per_tick
            rval = []
            while self.notes[self.hIdx][0] + self.loops * self.duration < tickhorizon:
                rval.append ( getText(self.hIdx, self.secs_per_tick, prev_secs + time0_time ) )
                self.hIdx += 1
                if self.hIdx == len(self.notes):
                    self.hIdx  = 0
                    self.loops += 1
                    prev_secs  += self.duration * self.secs_per_tick

            return rval

        #thread.start_new_thread( testtimer, (0,) )
        m = 0.0
        while self.thread_continue:
            t0 = time.time()
            time.sleep(sleeptime)
            t1 = time.time()
            if t1 - t0 > 2.0 * sleeptime : 
                print 'critical lagginess: ', t1 - t0
            if m < t1 - t0:
                m = t1 - t0
                print t1, ' timer max = ', m
            cmds = self.next()
            for c in cmds: 
                self.perf.InputMessage( '' )

    def __init__(self, orc, range_sec, ticks_per_sec ):
        self.orc = orc
        self.up = False
        self.csound = csnd.Csound()

        self.ticks_per_sec = ticks_per_sec                  # ticks last this long
        self.secs_per_tick = 1.0 / ticks_per_sec            # precomputed inverse
        self.range_sec  = range_sec                         # notes are checked-for, this many seconds in advance

        self.duration = 0                                   # number of ticks in playback loop
        self.loops = 0                                      # number of elapsed loops
        self.notes = []                                     # sorted list of (onset, noteptr, cache)

        self.time0 = time.time() + 1000000                  # the real time at which tick == 0 (sometimes retro-active)
        self.thread_continue = 1
        self.thread = thread.start_new_thread( loop_work, (self,0.040) )

    def uninit(self):
        self.thread_continue = 0
        self.thread.join()
        if self.up : self.lower()

    def micRecording( self, table ):
        mess = Config.MIC_RECORDING_COMMAND % table
        self.sendText( mess )

    def load_mic_instrument( self, inst ):
        home_path = env.get_profile_path() + Config.PREF_DIR
        fileName = home_path + '/' + inst
        instrumentId = Config.INSTRUMENT_TABLE_OFFSET + int(fileName[-1]) + 6
        mess = Config.LOAD_INSTRUMENT_COMMAND % ( instrumentId, fileName )
        self.sendText( mess )

    def startTime(self):
        if not self.up : 
            debug_print (1, "ERROR: Sound::startTime, performance thread isn't up yet.")
            return 
        self.perf.InputMessage('i 5999 0.0 60000000')
        self.time_start = time.time()
        # if a note event is sent to csound before or simultaneous to this one, then it will not play correctly.
        # thus we sleep right here, to (ideally) let csound pick up the message.
        # NB: match this to the constant in the instrument 5777 of the csound orcestra
        time.sleep(0.1)

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

    def raise( self ):
        if self.up : 
            debug_print(3, 'Sound::raise() already up.')
            return
        self.up = True
        self.perf   = csnd.CsoundPerformanceThread(self.csound)
        self.csound.Compile( self.orc )
        self.perf.Play()
        self.load_instruments()
        debug_print(5, 'Sound::raise succeeded')

    def lower(self):
        if not self.up :
            debug_print(3, 'Sound::lower() already down.')
            return
        self.up = False
        self.sendText( Config.UNLOAD_TABLES_COMMAND  )
        self.perf.Stop()
        rval = self.perf.Join()
        self.csound.Reset()
        debug_print(5, 'Sound::lower() succeeded')

    def setMasterVolume(self, volume):
        self.csound.SetChannel('masterVolume',volume )

    def inputMessage(self, txt):
        self.perf.InputMessage(txt)

    def loop_setTick( self, tick ):
        time_time = time.time()
        self.time0 = time_time - tick * self.secs_per_tick
        self.loops = tick // self.duration
        self.hIdx = bisect.bisect_left(self.notes, tick - self.duration * self.loops )

    def loop_setRate( self, ticks_per_sec):
        if ticks_per_sec != self.ticks_per_sec:
            secs_per_tick = 1.0 / ticks_per_sec

            time_time = time.time()
            curtick = self.getTick( time_time, False )
            curticktime = curtick * self.secs_per_tick + self.time0

            self.ticks_per_sec = ticks_per_sec
            self.secs_per_tick = secs_per_tick
            self.time0 = curticktime - curtick * secs_per_tick
            self.notes = [ (o,n,'',z) for (o,n,c,z) in self.notes ]  #clear cache
            self.loops = 0

    def loop_setDuration( self, duration ):
        self.time0 += self.loops * self.duration * self.secs_per_tick
        self.loops = 0
        self.duration = duration

    def loop_getTick(self, t, domod): #t is for time
        if domod : 
            return ( int( ( t - self.time0 ) * self.ticks_per_sec ) ) % self.duration
        else     :
            return ( int( ( t - self.time0 ) * self.ticks_per_sec ) )

    def loop_insert( self, notes):
        def insertMany():
            self.notes += [ ( notes[i][0], notes[i][1], '', 0 ) for i in xrange(len(notes)) ]
            self.notes.sort()
        def insertFew():
            for i in xrange(len(notes)): 
                t = (notes[i][0], notes[i][1],'',0)
                l = bisect.bisect_left(self.notes, t )
                self.notes.insert(l, t)

        if len(notes) >= 1:
            insertMany()
        else:
            insertFew()
        self.hIdx = bisect.bisect_left(self.notes, self.getTick(self.range_sec + time.time(), True))

    def loop_remove(self, note):
        def removeFew():
            i = 0
            while i < len(self.notes):
                if self.notes[i][1] in note:
                    del self.notes[i]
                else:
                    i += 1

        def removeMany():
            self.notes = [t for t in self.notes if t[1] not in note]

        if len(idset) >= 0:  #just guessing here, should do some timing tests to see if this is good or no
            removeMany()
        else:
            removeFew()
        self.hIdx = bisect.bisect_left(self.notes, self.getTick(self.range_sec + time.time(), True))

    def loop_clear(self):
        self.notes = []
