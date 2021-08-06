#!/usr/bin/python2.4

import pygtk
pygtk.require( '2.0' )
import gtk


from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundServer import CsoundServerMult
import os
import sys
import signal
import time

from GUI.StandalonePlayer import StandAlonePlayer
from GUI.Core.MainWindow import MainWindow

if __name__ == "__main__": 
    def run_sugar_mode():
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
    def run_edit_mode():
        CSoundClient.initialize(True)
        CSoundClient.setMasterVolume(100)
        tamtam = MainWindow()
        mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
        mainwin.set_title('TamTam Player')
        mainwin.set_resizable(False)
        mainwin.connect('destroy' , tamtam.destroy )
        mainwin.connect( "configure-event", tamtam.handleConfigureEvent )
        mainwin.connect( "key-press-event", tamtam.onKeyPress )
        mainwin.connect( "key-release-event", tamtam.onKeyRelease )
        mainwin.connect( "delete_event", tamtam.delete_event )
        mainwin.set_border_width(10)
        mainwin.set_geometry_hints( None, 855, Constants.NUMBER_OF_TRACKS * 50 + 200, 900, Constants.NUMBER_OF_TRACKS * 300 + 200 )
        mainwin.add(tamtam)
        tamtam.show()
        mainwin.show()
        gtk.main()
        CSoundClient.initialize(False)
        sys.exit(0)

    mode = os.getenv('TAMTAM_MODE')
    if None == mode:
        run_sugar_mode()
    elif mode == 'sugar': 
        run_sugar_mode()
    elif mode == 'edit':
        run_edit_mode()
    else:
        print "If you're going to define TAMTAM_MODE, please make it either 'sugar' or 'edit'"
        sys.exit(1)

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
