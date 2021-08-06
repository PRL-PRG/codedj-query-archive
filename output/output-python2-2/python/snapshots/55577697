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

from Framework.Core.Profiler import TP

if __name__ == "__main__": 
    def run_sugar_mode():
        CSoundClient.initialize(True)
        CSoundClient.setMasterVolume(100)
        tamtam = StandAlonePlayer()
        #tamtam = gtk.Button("adsf")
        mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
        #mainwin.set_size_request(1200,600)
        mainwin.set_title('miniTamTam')
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
        TP.Profile("TT::init")
        TP.Profile("TT::init::csound")
        CSoundClient.initialize(True)
        CSoundClient.setMasterVolume(100)
        TP.Profile("TT::init::csound")
        TP.Profile("TT::init::mainwin")
        TP.Profile("TT::init::mainwin::init")
        tamtam = MainWindow()
        TP.Profile("TT::init::mainwin::init")
        TP.Profile("TT::init::mainwin::connect")
        mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
        mainwin.set_title('TamTam Player')
        mainwin.set_geometry_hints( None, 1200, 900, 1200, 900, 1200, 900 )
        #mainwin.fullscreen() # don't need to specify full screen, it seem to sit properly anyway
        mainwin.set_resizable(False)
        mainwin.connect('destroy' , tamtam.destroy )
        #mainwin.connect( "configure-event", tamtam.handleConfigureEvent )
        mainwin.connect( "key-press-event", tamtam.onKeyPress )
        mainwin.connect( "key-release-event", tamtam.onKeyRelease )
        mainwin.connect( "delete_event", tamtam.delete_event )
        mainwin.add(tamtam)
        TP.Profile("TT::init::mainwin::add")
        TP.Profile("TT::init::mainwin")
        TP.Profile("TT::init::show")
        tamtam.show()
        mainwin.show()
        TP.Profile("TT::init::show")
        TP.Profile("TT::init")
        gtk.main()
        CSoundClient.initialize(False)
        sys.exit(0)

    if len(sys.argv) > 1 and sys.argv[1] == 'edit':
        run_edit_mode()
    else:
        run_sugar_mode()

from sugar.activity.Activity import Activity
class TamTam(Activity):
    def __init__(self):

        Activity.__init__(self)

        self.tamtam = StandAlonePlayer()
        self.connect('focus_in_event',self.handleFocusIn)
        self.connect('focus_out_event',self.handleFocusOut)
        self.connect('destroy', self.do_quit)
        self.add(self.tamtam)
        self.tamtam.show()
        self.set_title('TamTam')
        self.set_resizable(False)
        self.connect( "key-press-event", self.tamtam.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", self.tamtam.keyboardStandAlone.onKeyRelease )
    
    def handleFocusIn(self, event, data=None):
        CSoundClient.initialize(True)
        CSoundClient.setMasterVolume(100)
    
    def handleFocusOut(self, event, data=None):
        CSoundClient.initialize(False)

    def do_quit(self, arg2):
        CSoundClient.initialize(False)
        del self.tamtam
