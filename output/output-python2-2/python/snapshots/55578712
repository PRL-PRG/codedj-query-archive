#!/usr/bin/python
import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import time

from GUI.Core.MainWindow import MainWindow
from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundServer import CsoundServerMult

import lltimer

class GTKTimerWithCallback :
    def __init__(self,msecs) :
        gobject.timeout_add( msecs, self.printTime)
        self.maxerr=0;
        self.cumerr=0;
        self.ctr=0;
        self.msecs=msecs
        self.lasttime=-1

    def printTime(self) :
        if self.lasttime==-1 :
            self.lasttime=time.time()
        else :
            currtime = time.time()
            diff = currtime-self.lasttime
            self.lasttime = currtime
            err= (diff*1000)-self.msecs
            if err>self.maxerr :
                self.maxerr=err
            self.ctr+=1
            self.cumerr+=abs(err)            
            if abs(err)>1:
                print "GTK ms error",err,"mx",self.maxerr,"mean",(self.cumerr/self.ctr)

        return True



class PThreadTimerWithCallback :
    def __init__(self,msecs) :
        lltimer.timeout_add(msecs,self.printTime)
        self.maxerr=0;
        self.cumerr=0;
        self.ctr=0;
        self.msecs=msecs        
        self.lasttime=-1;
        
    def printTime(self) :
        if self.lasttime==-1 :
            self.lasttime=time.time()
        else :
            currtime = time.time()
            diff = currtime-self.lasttime
            self.lasttime = currtime
            err= (diff*1000)-self.msecs
            if err>self.maxerr :
                self.maxerr=err
            self.ctr+=1
            self.cumerr+=abs(err)            
            if abs(err)>5 :
                print "PThread ms error",err,"mx",self.maxerr,"mean",(self.cumerr/self.ctr)
       
        return True



        
if __name__ == "__main__": 
    CSoundClient.initialize()
    tamTam = MainWindow()

    #t1 = GTKTimerWithCallback(10)

    t2 = PThreadTimerWithCallback(100)

    #start the gtk event loop
    
    gtk.main()

