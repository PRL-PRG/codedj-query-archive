import signal
import time
import sys
import pygtk
pygtk.require( '2.0' )
import gtk

import Config
import Util.CSoundClient as CSoundClient
from   Util.Profiler import TP
from   Player.StandalonePlayer import StandAlonePlayer
from   Edit.MainWindow import MainWindow


#csnd = CSoundClient.CSoundClientSocket( Config.SERVER_ADDRESS, Config.SERVER_PORT, os.getpid() )
#csnd = CSoundClient.CSoundClientPerf( '/usr/share/olpc-csound-server/univorc.csd' )
csnd = CSoundClient.CSoundClientPerf( Config.TAM_TAM_ROOT + '/Resources/univorc.csd' )


csnd.initialize(True)
csnd.setMasterVolume(100.0)
CSoundClient.CSoundClient = csnd   #Dodgy move: TODO: remove this global variable.

if __name__ == "__main__":     
    def run_sugar_mode():
        tamtam = StandAlonePlayer(csnd)
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
        csnd.initialize(False)
        print 'GOT BACK FROM UNINIT'
        sys.exit(0)

from sugar.activity.Activity import Activity
from sugar import env
import os, shutil
class TamTam(Activity):
    def __init__(self):
        Activity.__init__(self)
        
        home_path = env.get_profile_path() + Config.PREF_DIR
        if not os.path.isdir(home_path):
            os.mkdir(home_path)
            os.system('chmod 0777 ' + home_path + ' &')
            for snd in ['mic1','mic2','mic3','mic4','lab1','lab2','lab3','lab4']:
                shutil.copyfile(Config.SOUNDS_DIR + '/' + snd , home_path + '/' + snd)
                os.system('chmod 0777 ' + home_path + '/' + snd + ' &')
        
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
        self.tamtam.csnd.startTime() 
        self.tamtam.noteLooper.startTime()
    
    def handleFocusOut(self, event, data=None):
        if self.tamtam.synthLabWindowOpen(): return
        csnd.initialize(False)

    def do_quit(self, arg2):
        csnd.initialize(False)
        del self.tamtam

