import pickle
import time
import bisect

import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.Core.Profiler import TP

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class NoteLooper:

    #PRIVATE

    def dirty_track(self, track):
        for i in range(len(self.notes)): 
            (o,n,c) =  self.notes[i]
            if n['trackID'] == track:
                self.notes[i] = (o,n,'')


    #PUBLIC

    def __init__( self, range_sec, ticks_per_sec, inst, tvol, mute ):
        self.ticks_per_sec = ticks_per_sec                  # ticks last this long
        self.secs_per_tick = 1.0 / ticks_per_sec            # precomputed inverse
        self.range_sec  = range_sec                         # notes are checked-for, this many seconds in advance
        self.range_tick = int( range_sec * ticks_per_sec )  # same, but in ticks

        self.inst = inst                                    # instrument for each track
        self.tvol = tvol                                    # volume for each track 
        self.mute = mute                                    # pre-amp for track volume

        if len(inst) != len(tvol): print 'ERROR: NoteLooper::__init__() invalid args'
        if len(inst) != len(mute): print 'ERROR: NoteLooper::__init__() invalid args'

        self.duration = 0                                   # number of ticks in playback loop
        self.notes = []                                     # sorted list of (onset, noteptr, cache)

        self.tick0 = 0                                      # the tick at which playback started
        self.time0 = time.time() + 1000000                  # the real time at which playback started

    def setRate( self, ticks_per_sec):
        if ticks_per_sec != self.ticks_per_sec:
            t = time.time()
            secs_per_tick = 1.0 / ticks_per_sec

            if t > self.time0:
                self.tick0 +=  int( (t - self.time0) * ticks_per_sec)
            self.time0 = t
            self.ticks_per_sec = ticks_per_sec
            self.secs_per_tick = secs_per_tick
            self.range_tick = ticks_per_sec * self.range_sec
            self.notes = [ (o,n,'') for (o,n,c) in self.notes ]  #clear cache

    def setDuration( self, duration ):
        self.duration = duration

    def getCurrentTick(self, future , domod , t): #t is for time
        if domod : return ( self.tick0 + int( ( t + future - self.time0 ) * self.ticks_per_sec ) ) % self.duration
        else     : return ( self.tick0 + int( ( t + future - self.time0 ) * self.ticks_per_sec ) )

    def setVolume(self, track,vol):
        if self.tvol[track] != vol:
            self.tvol[track] = vol
            self.dirty_track(track)

    def setInstrument(self, track, inst):
        if self.inst[track] != inst:
            self.inst[track] = inst
            self.dirty_track(track)
    def setMute(self, track, m):
        if self.mute[track] != m:
            self.mute[track] = m
            self.dirty_track(track)

    def next( self ) :
        TP.ProfileBegin("NL::next")

        TP.ProfileBegin("NL::next A")
        time_time = time.time()
        tickhorizon = self.getCurrentTick( self.range_sec, False, time_time )  #tick where we'll be after range_sec

        if tickhorizon < self.tick0 : return []
        TP.ProfileEnd("NL::next A")

        def cache_cmd(secs_per_tick, amplitude, pitch, iflag, trackId, duration, tied, fullDuration, overlap, attack, decay, reverbSend, filterType, filterCutoff, pan ):
            TP.ProfileBegin("NL::next::cache_cmd")
            if self.inst[ trackId ] == 'drum1kit':
                if pitch in GenerationConstants.DRUMPITCH:
                     pitch = GenerationConstants.DRUMPITCH[ pitch ]

                iflag = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                pitch = 1
            else:
                return None
                iflag = self.inst[ trackId ]
                pitch = GenerationConstants.TRANSPOSE[ pitch - 24 ]

            # condition for tied notes
            if CSoundConstants.INSTRUMENTS[ iflag ].csoundInstrumentID  == 101  and tied and fullDuration:
                duration= -1.0
            # condition for overlaped notes
            if CSoundConstants.INSTRUMENTS[ iflag ].csoundInstrumentID == 102 and overlap:
                duration += 1.0

            attack = max( 0.002, duration * attack)
            decay  = max( 0.002, duration * decay)

            rval = CSoundConstants.PLAY_NOTE_COMMAND_MINUS_DELAY % \
                ( CSoundConstants.INSTRUMENTS[ iflag ].csoundInstrumentID, 
                    trackId, 
                    '%f', #delay,
                    duration, 
                    pitch, 
                    reverbSend,
                    amplitude, 
                    pan,
                    CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ iflag ].instrumentID,
                    attack,
                    decay,
                    filterType, filterCutoff )
            TP.ProfileEnd("NL::next::cache_cmd")
            return rval

        def getText(i, secs_per_tick, time_offset):
            TP.ProfileBegin("NL::next::getText")
            (onset,note,cache) = self.notes[i]
            if cache == '' :
                self.notes[i] = ( onset, note, 
                        cache_cmd( secs_per_tick, 
                            note['amplitude'] * self.tvol[note['trackID']] * self.mute[note['trackID']],
                            note['pitch'],
                            note['instrumentFlag'],
                            note['trackID'],
                            note['duration'] * self.secs_per_tick,
                            note['tied'],
                            note['fullDuration'],
                            note['overlap'],
                            note['attack'],
                            note['decay'], 
                            note['reverbSend'], 
                            note['filterType'], 
                            note['filterCutoff'],
                            note['pan']))
            if self.notes[i][2] == None:
                rval = ''
            else :
                rval = self.notes[i][2] % float(onset * self.secs_per_tick + time_offset) 
            TP.ProfileEnd("NL::next::getText")
            return rval

        TP.ProfileBegin("NL::next B")
        if self.tick0 != 0: 
            print self.tick0
            raise 'tick0 != 0'
        TP.ProfileEnd("NL::next B")

        #find the right end of the buffer
        if tickhorizon <= self.duration:
            TP.ProfileBegin("NL::next C")
            hIdx = self.hIdx
            self.hIdx = hIdxMax = bisect.bisect_left(self.notes, (tickhorizon,))
            rlag = self.time0 - time_time
            rlist = [(i, rlag ) for i in range(hIdx, hIdxMax)]
            TP.ProfileEnd("NL::next C")
        else:
            TP.ProfileBegin("NL::next D")
            self.hIdx = hIdxMax = bisect.bisect_left(self.notes, (self.duration,))
            rlag = self.time0 - time_time
            rlist = [(i, rlag ) for i in range(self.hIdx, hIdxMax)]

            while tickhorizon > self.duration:   #loop back to tick0 == 0
                rlag       += (self.duration - self.tick0) * self.secs_per_tick
                self.time0 += (self.duration - self.tick0) * self.secs_per_tick
                self.tick0 = 0
                tickhorizon -= self.duration
                self.hIdx = hIdxMax = bisect.bisect_left(self.notes, (min(tickhorizon, self.duration), 0))
                rlist += [(i,rlag) for i in range(hIdxMax)]
                
            TP.ProfileEnd("NL::next D")
        TP.ProfileBegin("NL::next E")
        rval = [ getText(i, self.secs_per_tick, looplag) for (i,looplag) in rlist ] 
        TP.ProfileEnd("NL::next E")
        TP.ProfileEnd("NL::next")
        return rval

    def setTick( self, tick ):
        self.time0 = time.time() + self.range_sec
        self.tick0 = tick % self.duration
        self.hIdx = bisect.bisect_left(self.notes, self.tick0)

    def insert( self, notes):
        def insertMany():
            self.notes += [ ( notes[i][0], notes[i][1], '' ) for i in xrange(len(notes)) ]
            self.notes.sort()
        def insertFew():
            for i in xrange(len(notes)): 
                t = (notes[i][0], notes[i][1],'')
                l = bisect.bisect_left(self.notes, t )
                self.notes.insert(l, t)
                print 't',t

        if len(notes) > 6:
            insertMany()
        else:
            insertFew()
        print self.notes

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

    def clear(self):
        self.notes = []


