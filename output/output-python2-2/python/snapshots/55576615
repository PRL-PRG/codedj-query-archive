import pickle
import time
import bisect

import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

import Config
from Util.CSoundNote import CSoundNote  #maybe not actually used, but dependence is there.  All notes are assumed to be CSoundNotes
from Generation.GenerationConstants import GenerationConstants

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class NoteLooper:

    #PRIVATE
    DRIFT = 0.01  #careful about changing this... coordinate with instrument 5777

    #PUBLIC

    def __init__( self, range_sec, ticks_per_sec ):
        self.ticks_per_sec = ticks_per_sec                  # ticks last this long
        self.secs_per_tick = 1.0 / ticks_per_sec            # precomputed inverse
        self.range_sec  = range_sec                         # notes are checked-for, this many seconds in advance

        self.duration = 0                                   # number of ticks in playback loop
        self.loops = 0                                      # number of elapsed loops
        self.notes = []                                     # sorted list of (onset, noteptr, cache)

        self.time0 = time.time() + 1000000                  # the real time at which tick == 0 (sometimes retro-active)

        #self.time_start                                    # remember to call NoteLooper.startTime
                                                            # at the same time as you call 
                                                            # CSoundClient.startTime()
    def setTick( self, tick ):
        time_time = time.time()
        self.time0 = time_time - tick * self.secs_per_tick
        self.loops = tick // self.duration
        self.hIdx = bisect.bisect_left(self.notes, tick - self.duration * self.loops )

    def setRate( self, ticks_per_sec):
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

    def setDuration( self, duration ):
        self.time0 += self.loops * self.duration * self.secs_per_tick
        self.loops = 0
        self.duration = duration

    def getTick(self, t, domod): #t is for time
        if domod : 
            return ( int( ( t - self.time0 ) * self.ticks_per_sec ) ) % self.duration
        else     :
            return ( int( ( t - self.time0 ) * self.ticks_per_sec ) )

    def next( self ) :
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

    def insert( self, notes):
        def insertMany():
            self.notes += [ ( notes[i][0], notes[i][1], '', 0 ) for i in xrange(len(notes)) ]
            self.notes.sort()
        def insertFew():
            for i in xrange(len(notes)): 
                t = (notes[i][0], notes[i][1],'',0)
                l = bisect.bisect_left(self.notes, t )
                self.notes.insert(l, t)

        if len(notes) > 6:
            insertMany()
        else:
            insertFew()
        self.hIdx = bisect.bisect_left(self.notes, self.getTick(self.range_sec + time.time(), True))

    def remove(self, note):
        def removeFew():
            i = 0
            while i < len(self.notes):
                if self.notes[i][1] in note:
                    del self.notes[i]
                else:
                    i += 1

        def removeMany():
            self.notes = [t for t in self.notes if t[1] not in note]

        if len(idset) > 6:  #just guessing here, should do some timing tests to see if this is good or no
            removeMany()
        else:
            removeFew()
        self.hIdx = bisect.bisect_left(self.notes, self.getTick(self.range_sec + time.time(), True))

    def clear(self):
        self.notes = []

    def startTime(self):
        self.time_start = time.time()
