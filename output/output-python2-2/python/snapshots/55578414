import pickle
import time
import bisect

import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Note import *

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class NoteLooper:
    #PRIVATE

    def dirty_all(self):
        def asdf (note) :
            note['dirty'] = True
        map( lambda (onset, note) : asdf(note), self.notes)


    #PUBLIC

    def __init__( self, duration, range_sec, tick0, vols, ticks_per_sec, notes ):
        self.time0 = time.time()
        self.tick0 = tick0

        self.range_sec  = range_sec 
        self.range_tick = int( range_sec * ticks_per_sec )
        self.duration = int(duration)  # the duration of the loop, in ticks (compare, timeduration)
        self.ticks_per_sec = ticks_per_sec
        self.secs_per_tick = 1.0 / ticks_per_sec

        self.vols = vols
        self.notes = notes
        self.hIdx = bisect.bisect_left(notes, (tick0,0) )

        #self.dirty_all()

    def setRate( self, ticks_per_sec):
        if ticks_per_sec != self.ticks_per_sec:
            t = time.time()
            secs_per_tick = 1.0 / ticks_per_sec

            self.tick0 +=  int( (t - self.time0) * ticks_per_sec)
            self.time0 = t
            self.ticks_per_sec = ticks_per_sec
            self.secs_per_tick = secs_per_tick
            self.range_tick = ticks_per_sec * self.range_sec
            #self.dirty_all()

    def setVols( self, vols ):
        raise 'not implented'
        #for t in len(vols):
            #if vols[t] != self.vols[t]:
                #self.vols[t] = vols[t]
        #tracks = filter( lambda i  : vols[i] != self.vols[i], range(len(vols)))
        #self.vols = vols
        #self.dirty_all()  # could just dirty

    def setDuration( self, duration ):
        self.duration = duration

    def getCurrentTick(self, future = 0, domod = True, t = time.time()):
        if domod : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_sec) ) % self.duration
        else     : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_sec) )

    def next( self ) :

        time_time = time.time()
        tickhorizon = self.getCurrentTick( self.range_sec, False, time_time )

        #find the right end of the buffer
        hIdxMax = bisect.bisect_left(self.notes, (tickhorizon,0))
        sendlist = self.notes[self.hIdx: hIdxMax]

        tempo = self.ticks_per_sec / 12 * 60

        buf0 = reduce( 
                lambda buf, (onset, note): 
                buf + note.getText( tempo, (onset - self.tick0) * self.secs_per_tick - time_time + self.time0),
                #note_getText(note, 
                #    self.vols[note.track], 
                #    self.secs_per_tick, 
                #    (onset - self.time0) * self.secs_per_tick - time_time + self.time0),
                sendlist, "" )

        buf1 = ''

        #print 'len sendlist', len(sendlist)
        while tickhorizon > self.duration:
            tickhorizon -= self.duration
            print map( lambda (o,n) : o, self.notes)
            hIdxMax = bisect.bisect_left(self.notes, (tickhorizon, 0))
            sendlist = self.notes[0:hIdxMax]
            self.time0 += (self.duration - self.tick0) * self.secs_per_tick
            self.tick0 = 0
            buf1 = reduce( 
                    lambda buf, (onset, note): 
                    buf + note.getText( tempo, (onset - self.tick0) * self.secs_per_tick - time_time + self.time0),
                    #note_getText(note, 
                    #    self.vols[note.track], 
                    #    self.secs_per_tick, 
                    #    (onset - self.time0) * self.secs_per_tick - time_time + self.time0),
                    sendlist, "" )

        self.hIdx = hIdxMax

        return buf0 + buf1

    def seekTick( self, tick ):
        self.time0 = time.time()
        self.tick0 = tick % self.duration
        if self.playing : self.hIdx = lsearch(self.notes, self.tick0 + self.ticks_per_sec * self.horizon)

    def insertNote( self, onset, note ):
        raise 'not impl'

    def removeNote( self, onset, note ):
        raise 'not impl'

