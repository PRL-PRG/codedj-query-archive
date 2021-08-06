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

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class NoteLooper:

    #PRIVATE

    def dirty_track(self, t):
        for i in range(len(self.notes)): 
            if self.notes[i]['trackID'] == track:
                self.cache[i] = ''


    #PUBLIC

    def __init__( self, range_sec, ticks_per_sec, inst, tvol, mute ):
        self.range_sec  = range_sec 
        self.ticks_per_sec = ticks_per_sec
        self.secs_per_tick = 1.0 / ticks_per_sec
        self.range_tick = int( range_sec * ticks_per_sec )

        self.inst = inst
        self.tvol = tvol
        self.mute = mute

        self.duration = 0  # the duration of the loop, in ticks (compare, timeduration)
        self.notes = []      #sorted list of (onset, noteptr, cache)

    def setRate( self, ticks_per_sec):
        if ticks_per_sec != self.ticks_per_sec:
            t = time.time()
            secs_per_tick = 1.0 / ticks_per_sec

            self.tick0 +=  int( (t - self.time0) * ticks_per_sec)
            self.time0 = t
            self.ticks_per_sec = ticks_per_sec
            self.secs_per_tick = secs_per_tick
            self.range_tick = ticks_per_sec * self.range_sec
            self.notes = [ (o,n,'') for (o,n,c) in self.notes ]

    def setDuration( self, duration ):
        self.duration = duration

    def getCurrentTick(self, future , domod , t): #t is for time
        if domod : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_sec) ) % self.duration
        else     : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_sec) )

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
        time_time = time.time()
        if time_time < self.time0 : return ''

        def cache_cmd(note, secs_per_tick, preamp):
            if self.inst[ note['trackID'] ] == 'drum1kit':
                if GenerationConstants.DRUMPITCH.has_key( note['pitch'] ):
                    #print note['pitch']
                    note['pitch'] = GenerationConstants.DRUMPITCH[ note['pitch'] ]

                note['instrumentFlag'] = CSoundConstants.DRUM1INSTRUMENTS[ note['pitch'] ]
                newPitch = 1
            else:
                note['instrumentFlag'] = self.inst[ note['trackID'] ]
                newPitch = GenerationConstants.TRANSPOSE[ note['pitch'] - 24 ]

            duration = secs_per_tick * note['duration']

            # condition for tied notes
            if CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID  == 101  and note['tied'] and note['fullDuration']:
                duration = -1.0
            # condition for overlaped notes
            if CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID == 102 and note['overlap']:
                duration += 1.0

            newAmplitude = note['amplitude'] * preamp

            newAttack = duration * note['attack']
            if newAttack <= 0.002:
                newAttack = 0.002

            newDecay = duration * note['decay']
            if newDecay <= 0.002:
                newDecay = 0.002

            return CSoundConstants.PLAY_NOTE_COMMAND_MINUS_DELAY % \
                ( CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID, 
                    note['trackID'], 
                    '%f', #delay,
                    duration, 
                    newPitch, 
                    note['reverbSend'], 
                    newAmplitude, 
                    note['pan'], 
                    CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].instrumentID,
                    newAttack,
                    newDecay,
                    note['filterType'],
                    note['filterCutoff'] )

        def getText(i, secs_per_tick):
            (onset,note,cache) = self.notes[i]
            delay = (onset - self.tick0) * self.secs_per_tick - time_time + self.time0
            if delay < 0.0 : 
                print 'ERROR: you cant send note with negative delay', delay
                return ''
            preamp = self.tvol[note['trackID']] * self.mute[note['trackID']]
            if preamp == 0.0 :
                return ''
            if self.inst[ note['trackID'] ] == 'drum1kit':
                if self.notes[i][1]['instrumentFlag'] == 'drum1chine': 
                    print 'WARNING: NoteLooper::next() skipping instance of drum1chine'
                    return ''
            if cache == '' :
                self.notes[i] = (onset,note,cache_cmd( note, secs_per_tick, preamp ))
            return self.notes[i][2] % float(delay)

        tickhorizon = self.getCurrentTick( self.range_sec, False, time_time )

        #find the right end of the buffer
        if tickhorizon <= self.duration:
            hIdxMax = bisect.bisect_left(self.notes, (tickhorizon,0))
        else:
            hIdxMax = bisect.bisect_left(self.notes, (self.duration,0))

        buf = ["\n".join([ getText(i, self.secs_per_tick) for i in range(self.hIdx, hIdxMax)])]

        #print [ self.notes[i][0] for i in range(self.hIdx, hIdxMax)]

        while tickhorizon > self.duration:
            tickhorizon -= self.duration
            hIdxMax = bisect.bisect_left(self.notes, (tickhorizon, 0))
            self.time0 += (self.duration - self.tick0) * self.secs_per_tick
            self.tick0 = 0
            buf.append("\n".join([ getText(i, self.secs_per_tick) for i in range(hIdxMax)]))
            #print [ self.notes[i][0] for i in range(hIdxMax)]

        self.hIdx = hIdxMax
        
        if len(buf) == 1: return buf[0]
        else:             return ''.join(buf)

    def setTick( self, tick ):
        self.time0 = time.time() + self.range_sec
        self.tick0 = tick % self.duration
        self.hIdx = lsearch(self.notes, self.tick0 + self.ticks_per_sec * self.horizon)

    def insert( self, notes):
        def insertMany():
            self.notes += [ ( notes[i][0], notes[i][1], '' ) for i in xrange(len(notes)) ]
            self.notes.sort()
        def insertFew():
            for i in xrange(len(notes)): 
                bisect.insert_left(self.notes, (notes[i][0], notes[i][1],'') )

        if len(onset) > 6:
            insertMany()
        else:
            insertFew()

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


