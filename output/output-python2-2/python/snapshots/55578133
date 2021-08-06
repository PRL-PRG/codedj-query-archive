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
    # TODO this should get started outside of TamTam (perhaps by Sugar?)
    # start the CSoundServer
    def do_quit(event, param):
        CSoundClient.sendText('off()')
        print 'do_quit()  waiting'
        #we know that quitting doesn't really work
        time.sleep(0.5)
        os.kill(pid, signal.SIGKILL)
        time.sleep(0.3)
        os.wait()
        print '... phew!'

    try :
        pid = os.fork()

        if pid > 0 :
            time.sleep(1)
            CSoundClient.initialize()
            CSoundClient.setMasterVolume(100)
            tamtam = StandAlonePlayer()
            #tamtam = gtk.Button("adsf")
            mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
            mainwin.set_title('TamTam Player')
            mainwin.set_resizable(False)
            mainwin.connect('destroy', do_quit, pid)
            mainwin.connect('destroy' , tamtam.destroy )
            mainwin.connect( "key-press-event", tamtam.keyboardStandAlone.onKeyPress )
            mainwin.connect( "key-release-event", tamtam.keyboardStandAlone.onKeyRelease )
            mainwin.add(tamtam)
            tamtam.show()
            mainwin.show()
            gtk.main()
        else:
            server = CsoundServerMult( ( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT ) )
            server.interpret()

    except OSError, e: 
        print >>sys.stderr, "fork failed: %d (%s)" % (e.errno, e.strerror) 
        sys.exit(1)

from sugar.activity.Activity import Activity
class TamTam(Activity):
    def __init__(self):
        Activity.__init__(self)

        CSoundClient.initialize()
        CSoundClient.setMasterVolume(100)

        self.tamtam = StandAlonePlayer()
        self.connect('destroy', self.do_quit)
        self.add(self.tamtam)
        self.tamtam.show()
        self.set_title('TamTam')
        self.set_resizable(False)
        self.connect('destroy', do_quit, pid)
        self.connect('destroy' , tamtam.destroy )
        self.connect( "key-press-event", tamtam.keyboardStandAlone.onKeyPress )
        self.connect( "key-release-event", tamtam.keyboardStandAlone.onKeyRelease )
        self.add(self.tamtam)
        self.tamtam.show()
        self.show()

    def do_quit(self):
        del self.tamtam


