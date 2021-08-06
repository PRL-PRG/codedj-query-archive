import pickle
import time

import pygtk
pygtk.require( '2.0' )
import gtk 
import gobject

from Constants import Constants
from CSoundClient import CSoundClient
from CSoundConstants import CSoundConstants

#------------------------------------------------------------------------------
# A base class used to play a collection of Events at their respective onsets
#------------------------------------------------------------------------------
class NoteLooper:
    def __init__( self, duration, horizon, ticks_per_second, timeout_delay ):
        self.horizon  = horizon  # in seconds
        self.duration = duration  # the duration of the loop, in ticks (compare, timeduration)
        self.ticks_per_second = ticks_per_second
        self.seconds_per_tick = 1.0 / ticks_per_second
        self.playing  = False
        self.timeout_delay = timeout_delay

        self.tempo = ticks_per_second * 60 / Constants.TICKS_PER_BEAT

        self.notes = []
        self.hIdx = 0

        self.time0 = 0           # the anchor time
        self.tick0 = 0           # the current tick at time0

    def getCurrentTick(self, future = 0, domod = True):
        if domod : return ( self.tick0 + (time.time() + future - self.time0) * self.ticks_per_second ) % self.duration
        else       return ( self.tick0 + (time.time() + future - self.time0) * self.ticks_per_second )

    def seekTick( self, tick ):
        self.time0 = time.time()
        self.tick0 = tick % self.duration
        if self.playing : self.hIdx = lsearch(notes, self.tick0 + self.ticks_per_second * self.horizon)

    def setTickRate( self, rate ):
        t = time.time()
        self.tick0 +=  (t - self.time0) * self.ticks_per_second
        self.time0 = t
        self.ticks_per_second = rate
        self.seconds_per_tick = 1.0 / rate

        self.tempo = ticks_per_second * 60 / Constants.TICKS_PER_BEAT

    def setDuration( self, duration ):
        self.duration = duration
        return

    def setPlaying( self, tf = True ):
        # hack for shutOff tied notes when stop playing ( don't work when tracks are selected, probably not for mute... 
        def shutOff( ):
            for track in range( Constants.NUMBER_OF_TRACKS ):
                for i in range( 3 ):
                    csoundInstrument = i + 101
                    CSoundClient.sendText( CSoundConstants.PLAY_NOTE_OFF_COMMAND % ( csoundInstrument, track ) )
                      
        if self.playing == tf :
            pass
        else
            if tf :    # start
                self.time0 = time.time()
                self.hIdx  = self.tick0
                self.onTimeout()
                self.playbackTimeout = gobject.timeout_add( int ( 1000 * self.timeout_delay) , self.onTimeout )
            else  :    # stop
                self.tick0 = getCurrentTick()
                gobject.source_remove( self.playbackTimeout )
                shutOff()

    def onTimeout( self ) :
        tickhorizon = self.getCurrentTick( self.horizon, False )
        time_time = time.time()
        sendbuf = ""

        while 1 :
            (tick, note) = notes[hIdx]
            if tick >= tickhorizon : break

            dtick = tick - self.tick0
            if dtick < 0 : raise "negative dtick!"
            dtime = dtick * self.seconds_per_tick
            sendbuf += note.getText(self.tempo, time0 + dtime - time_time )

            self.hIdx = self.hIdx + 1

            if self.hIdx == len(notes) :   # looping around, re-anchoring at time 0
                self.hIdx = 0
                tickhorizon -= self.duration
                self.time0 = self.time0 + (self.duration - self.tick0) * self.ticks_per_second
                self.tick0 = 0

        if self.send_buffer != "" :
            CSoundClient.sendText( self.send_buffer ) 

        return True

    def insertNote( self, onset, note ):
        pass

    def removeNote( self, onset, note ):
        pass

