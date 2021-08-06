import signal
import time
import sys
import pygtk
pygtk.require( '2.0' )
import gtk

import Framework.CSound.CSoundClient as CSoundClient
from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.CSound.CSoundServer import CsoundServerMult

from GUI.StandalonePlayer import StandAlonePlayer
from GUI.Core.MainWindow import MainWindow

from Framework.Core.Profiler import TP


#csnd = CSoundClient.CSoundClientSocket( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT, os.getpid() )
#csnd = CSoundClient.CSoundClientPerf( '/usr/share/olpc-csound-server/univorc.csd' )
csnd = CSoundClient.CSoundClientPerf( Constants.TAM_TAM_ROOT + '/Resources/univorc.csd' )


csnd.initialize(True)
csnd.setMasterVolume(100.0)
CSoundClient.CSoundClient = csnd



if __name__ == "__main__": 
    def run_sugar_mode():
        tamtam = StandAlonePlayer(csnd)
        #tamtam = gtk.Button("adsf")
        mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
        color = gtk.gdk.color_parse('#FFFFFF')
        mainwin.modify_bg(gtk.STATE_NORMAL, color)
        #mainwin.set_size_request(1200,700)
        mainwin.set_title('miniTamTam')
        mainwin.set_resizable(False)
        mainwin.connect('destroy' , gtk.main_quit )
        mainwin.connect( "key-press-event", tamtam.keyboardStandAlone.onKeyPress )
        mainwin.connect( "key-release-event", tamtam.keyboardStandAlone.onKeyRelease )
        mainwin.add(tamtam)
        tamtam.show()
        mainwin.show()
        gtk.main()

    def run_edit_mode():
        tamtam = MainWindow(csnd)
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
        tamtam.show()
        mainwin.show()
        gtk.main()

    if len(sys.argv) > 1 and sys.argv[1] == 'edit':
        if False:
            import hotshot
            prof = hotshot.Profile("some_stats")
            prof.runcall(run_edit_mode)
            prof.close()
        else:
            run_edit_mode()
        csnd.initialize(False)
        print 'GOT BACK FROM UNINIT'
        sys.exit(0)
    else:
        run_sugar_mode()
        sys.exit(0)


from sugar.activity.Activity import Activity
class TamTam(Activity):
    def __init__(self):

        Activity.__init__(self)
        
        color = gtk.gdk.color_parse('#FFFFFF')
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.tamtam = StandAlonePlayer(csnd)
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
        csnd.initialize(True)
        csnd.setMasterVolume(100)
    
    def handleFocusOut(self, event, data=None):
        #csnd.initialize(False)
        pass

    def do_quit(self, arg2):
        csnd.initialize(False)
        del self.tamtam

