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

    def __init__( self, duration, horizon, tick0, vols, ticks_per_second, notes ):
        self.time0 = time.time()
        self.tick0 = tick0

        self.horizon  = horizon  # in seconds
        self.duration = duration  # the duration of the loop, in ticks (compare, timeduration)
        self.ticks_per_second = ticks_per_second
        self.seconds_per_tick = 1.0 / ticks_per_second

        self.vols = vols
        self.notes = notes
        self.hIdx = bisect.bisect_left(notes, (tick0,0) )

        self.dirty_all()

    def setRate( self, secs_per_tick):
        if secs_per_tick != self.secs_per_tick:
            t = time.time()
            self.tick0 +=  int( (t - self.time0) * self.ticks_per_second)
            self.time0 = t
            self.ticks_per_second = ticks_per_second
            self.seconds_per_tick = 1.0 / ticks_per_second
            self.dirty_all()

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

    def next( self, trackVols, secs_per_tick ) :

        if trackVols != self.trackvols :
            self.trackvols = trackVols
            self.dirty_all()

        time_time = time.time()
        tickhorizon = self.getCurrentTick( self.horizon, False, time_time )
        print 'thoriz', tickhorizon

        #find the right end of the buffer
        hIdxMax = bisect.bisect_left(self.notes, (tickhorizon,0))
        sendlist = self.notes[self.hIdx: hIdxMax]

        while tickhorizon > self.duration:
            tickhorizon -= self.duration
            hIdxMax = bisect.bisect_left(self.notes, (tickhorizon, 0))
            sendlist = sendlist + self.notes[0:hIdxMax]
            self.time0 += (self.duration - self.tick0) * self.seconds_per_tick
            self.tick0 = 0

        hIdx = hIdxMax

        return reduce( 
                lambda buf, (onset, note): 
                buf + note_getText(note, 
                    vol[note['track']], 
                    self.seconds_per_tick, 
                    (onset - time0) * seconds_per_tick - time_time + time0),
                sendlist, "" )


    def getCurrentTick(self, future = 0, domod = True, t = time.time()):
        if domod : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_second) ) % self.duration
        else     : return ( self.tick0 + int( (t + future - self.time0) * self.ticks_per_second) )

    def seekTick( self, tick ):
        self.time0 = time.time()
        self.tick0 = tick % self.duration
        if self.playing : self.hIdx = lsearch(self.notes, self.tick0 + self.ticks_per_second * self.horizon)

    def insertNote( self, onset, note ):
        raise 'not impl'

    def removeNote( self, onset, note ):
        raise 'not impl'

