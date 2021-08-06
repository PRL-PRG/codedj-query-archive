#!/usr/bin/python2.4

import pygtk
pygtk.require( '2.0' )
import gtk


from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundServer import CsoundServerMult
from GUI.Core.MainWindow import MainWindow
import os
import sys
import signal
import time

from GUI.StandalonePlayer import StandAlonePlayer

if __name__ == "__main__": 
    CSoundClient.initialize(True)
    CSoundClient.setMasterVolume(100)
    tamtam = StandAlonePlayer()
    #tamtam = gtk.Button("adsf")
    mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
    mainwin.set_title('TamTam Player')
    mainwin.set_resizable(False)
    mainwin.connect('destroy' , gtk.main_quit )
    mainwin.connect( "key-press-event", tamtam.keyboardStandAlone.onKeyPress )
    mainwin.connect( "key-release-event", tamtam.keyboardStandAlone.onKeyRelease )
    mainwin.add(tamtam)
    tamtam.show()
    mainwin.show()
    gtk.main()
    CSoundClient.initialize(False)
    sys.exit(0)

from sugar.activity.Activity import Activity
class TamTam(Activity):
    def __init__(self):

        Activity.__init__(self)

        CSoundClient.initialize(True)
        CSoundClient.setMasterVolume(100)

        self.tamtam = StandAlonePlayer()
        self.connect('destroy', self.do_quit)
        self.add(self.tamtam)
        self.tamtam.show()
        self.set_title('TamTam')
        self.set_resizable(False)
        self.connect( "key-press-event", self.tamtam.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.tamtam.keyboardStandAlone.onKeyRelease )

    def do_quit(self, arg2):
        CSoundClient.initialize(False)
        del self.tamtam
