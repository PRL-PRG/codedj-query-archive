#!/usr/bin/python
import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import time
import sys

from GUI.Core.MainWindow import MainWindow
from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundServer import CsoundServerMult
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants

import lltimer

class TimerWithCallback :
    def __init__(self,msecs,typ) :
        self.typ=typ
        if typ=="gtk":
            gobject.timeout_add( msecs, self.printTime)
        elif typ=="pthread" :
            lltimer.timeout_add(msecs,self.printTime)
        else :
            print "Unknown timer type.",typ,"Cannot continue"
            sys.exit(0)
            
        self.maxerr=0;
        self.cumerr=0;
        self.ctr=0;
        self.msecs=msecs
        self.lasttime=-1
        self.starttime = time.time()
    def printTime(self) :
        if self.lasttime==-1 :
            self.lasttime=time.time()
        else :
            currtime = time.time()
            diff = currtime-self.lasttime
            self.lasttime = currtime

            if currtime>(self.starttime+2) :  #print errors but ignore first 2 seconds
                err= (diff*1000)-self.msecs
                if abs(err)>self.maxerr :
                    self.maxerr=abs(err)
                self.ctr+=1
                self.cumerr+=abs(err)            
                print "%4.2f %s mserr=%4.2fms mxerr=%4.2fms meanerr=%4.2fms" % \
                ((currtime-self.starttime),self.typ,err,self.maxerr,(self.cumerr/self.ctr))
            else :
                print "Not timing first 2 seconds"


            self.genDumbNote()

        return True




    def genDumbNote(self) :
        print "Genning note"
        # duration for CSound is in seconds
        newPitch = 60
        newDuration = 100
        newAmplitude = 100
        instr = CSoundConstants.FLUTE
        trackId= 1
        reverbSend = 0
        pan = 0.5
        return CSoundConstants.PLAY_NOTE_COMMAND % ( CSoundConstants.INSTRUMENTS[ instr ].csoundInstrumentID, 
                                                     trackId, 
                                                     newDuration, 
                                                     newPitch, 
                                                     reverbSend,
                                                     newAmplitude, 
                                                     pan, 
                                                     CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ instr ].instrumentID )


        
if __name__ == "__main__": 
    CSoundClient.initialize()
    tamTam = MainWindow()


    if len(sys.argv)<2 or not (sys.argv[1]=="gtk" or sys.argv[1]=="pthread" or sys.argv[1]=="none"):
        print "Usage timetest.py <gtk|pthread|none>"
        print "Note: you must be root or suid to benefit from pthread enhanced timing"
        sys.exit(0)

    if not sys.argv[1]=="none" :
        t=TimerWithCallback(1000,sys.argv[1])
    else :
        print "No timer started"
    gtk.main()

