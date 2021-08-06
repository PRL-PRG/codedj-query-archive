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



class GTKTimerWithCallback :
    def __init__(self) :
        self.timer = gobject.timeout_add( 100, self.printTime)
        self.lasttime=-1
        
    def printTime(self) :
        if self.lasttime==-1 :
            self.lasttime=time.time()
        else :
            currtime = time.time()
            diff = currtime- self.lasttime
            self.lasttime = currtime
            print (diff*1000)-100
        return True



        
if __name__ == "__main__": 
    CSoundClient.initialize()
    tamTam = MainWindow()
    t = GTKTimerWithCallback()

    
    #start the gtk event loop

    
    gtk.main()

