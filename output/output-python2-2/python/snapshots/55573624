
import signal , time , sys , os, shutil
import pygtk
pygtk.require( '2.0' )
import gtk

import gobject
import time

import Config
from   Util.CSoundClient import new_csound_client
from   Util.Profiler import TP

from   Util.InstrumentPanel import InstrumentPanel
from   miniTamTam.miniTamTamMain import miniTamTamMain
from   Edit.MainWindow import MainWindow
from   Welcome import Welcome
from   SynthLab.SynthLabWindow import SynthLabWindow
from   Util.Trackpad import Trackpad
import commands

if __name__ != '__main__':
    try: 
        from sugar.activity.activity import Activity
        if Config.DEBUG: print 'using sugar Activity'
    except ImportError:
        from FActivity import FakeActivity as Activity
        if Config.DEBUG: print 'using fake activity'
else:
        from FActivity import FakeActivity as Activity
        if Config.DEBUG: print 'using fake activity'


class TamTam(Activity):
    # TamTam is the topmost container in the TamTam application
    # At all times it has one child, which may be one of
    # - the welcome screen
    # - the mini-tamtam 
    # - the synth lab
    # - edit mode

    def __init__(self, handle, mode='welcome'):
        Activity.__init__(self, handle)

        self.ensure_dirs()
        
        color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.set_title('TamTam')
        self.set_resizable(False)

        self.trackpad = Trackpad( self )

        self.preloadTimeout = None

        self.connect('focus_in_event',self.onFocusIn)
        self.connect('focus_out_event',self.onFocusOut)
        self.connect('destroy', self.onDestroy)
        self.connect( "key-press-event", self.onKeyPress )
        self.connect( "key-release-event", self.onKeyRelease )

        self.mode = None
        self.modeList = {}

        self.instrumentPanel = InstrumentPanel( force_load = False )
        self.preloadList = [ self.instrumentPanel ]

        if self._shared_activity: # if we're joining a shared activity force mini
            self.set_mode("mini")
        else:
            self.set_mode(mode)

    def onPreloadTimeout( self ):
        if Config.DEBUG > 4: print "TamTam::onPreloadTimeout", self.preloadList

        t = time.time()
        if self.preloadList[0].load( t + 0.100 ): # finished preloading this object
            self.preloadList.pop(0)
            if not len(self.preloadList):
                if Config.DEBUG > 1: print "TamTam::finished preloading", time.time() - t
                self.preloadTimeout = False
                return False # finished preloading everything

        if Config.DEBUG > 4: print "TamTam::preload returned after", time.time() - t

        return True

    def doNothing(): #a callback function to appease SynthLab
        pass
    def set_mode(self, mode, arg = None):
        if Config.DEBUG: print 'DEBUG: TamTam::set_mode from', self.mode, 'to', mode

        if self.mode != None:
            self.modeList[ self.mode ].onDeactivate()
            self.remove( self.modeList[ self.mode ] )

        self.mode = None
        self.trackpad.setContext(mode)

        if mode == 'welcome':
            if not (mode in self.modeList):
                self.modeList[mode] = Welcome(self.set_mode)
            self.mode = mode
            if len( self.preloadList ): 
                self.preloadTimeout = gobject.timeout_add( 300, self.onPreloadTimeout )
        elif self.preloadTimeout:
            gobject.source_remove( self.preloadTimeout )
            self.predrawTimeout = False
 

        if mode == 'mini':
            if not (mode in self.modeList):
                self.modeList[mode] = miniTamTamMain(self, self.set_mode)
            else:
                self.modeList[mode].regenerate()
            if self.instrumentPanel in self.preloadList:
                self.instrumentPanel.load() # finish loading
            self.modeList[mode].setInstrumentPanel( self.instrumentPanel )
            self.mode = mode
        if mode == 'edit':
            if not (mode in self.modeList):
                self.modeList[mode] = MainWindow(self.set_mode)
            if self.instrumentPanel in self.preloadList:
                self.instrumentPanel.load() # finish loading
            self.modeList[mode].setInstrumentPanel( self.instrumentPanel )
            self.mode = mode
        if mode == 'synth':
            if not (mode in self.modeList):
                self.modeList[mode] = SynthLabWindow(self.set_mode, None)
            self.mode = mode

        if self.mode == None:
            print 'DEBUG: TamTam::set_mode invalid mode:', mode
        else:
            try:
                self.set_canvas( self.modeList[ self.mode ] )
            except:
                self.add( self.modeList[ self.mode ] )
            self.modeList[ self.mode ].onActivate(arg)
            self.show()

    def onFocusIn(self, event, data=None):
        if Config.DEBUG > 3: print 'DEBUG: TamTam::onFocusOut in TamTam.py'
        csnd = new_csound_client()
        csnd.connect(True)
        if self.mode == 'synth':
            self.modeList[ self.mode ].updateSound()
            self.modeList[ self.mode ].updateTables()
        #csnd.load_instruments()
    
    def onFocusOut(self, event, data=None):
        if Config.DEBUG > 3: print 'DEBUG: TamTam::onFocusOut in TamTam.py'
        csnd = new_csound_client()
        csnd.connect(False)

    def onKeyPress(self, widget, event):
        if Config.DEBUG > 5: print 'DEBUG: TamTam::onKeyPress in TamTam.py'
        if event.state == gtk.gdk.MOD1_MASK:
            key = event.hardware_keycode
            if key == 58:    #M
                self.set_mode('mini')
                return
            elif key == 49:#39:  S
                #self.set_mode('synth')
                os.spawnlp(os.P_NOWAIT,'/usr/share/activities/TamTam.activity/cnee','/usr/share/activities/TamTam.activity/cnee', '--record', '--keyboard', '--mouse', '--stop-key', 'h', '--out-file', '/home/olpc/test.xnl')
                return
            elif key == 10:#25:  W
                #self.set_mode('welcome')
                os.spawnlp(os.P_NOWAIT,'/usr/share/activities/TamTam.activity/cnee','/usr/share/activities/TamTam.activity/cnee', '--replay', '--keyboard', '--mouse', '--file', '/home/olpc/test.xnl')
                return
            elif key == 53:  #X
                self.destroy()
                return
        self.modeList[ self.mode ].onKeyPress(widget, event)

    def onKeyRelease(self, widget, event):
        if Config.DEBUG > 5: print 'DEBUG: TamTam::onKeyRelease in TamTam.py'
        self.modeList[ self.mode ].onKeyRelease(widget, event)
        pass

    def onDestroy(self, arg2):
        if Config.DEBUG: print 'DEBUG: TamTam::onDestroy()'
        os.system('rm -f ' + Config.PREF_DIR + '/synthTemp*')

        for m in self.modeList: 
            if self.modeList[m] != None:
                self.modeList[m].onDestroy()

        csnd = new_csound_client()
        csnd.connect(False)
        csnd.destroy()

        gtk.main_quit()

    def ensure_dir(self, dir, perms=0777, rw=os.R_OK|os.W_OK):
        if not os.path.isdir( dir ):
            try:
                os.makedirs(dir, perms)
            except OSError, e:
                print 'ERROR: failed to make dir %s: %i (%s)\n' % (dir, e.errno, e.strerror)
        if not os.access(dir, rw):
            print 'ERROR: directory %s is missing required r/w access\n' % dir

    def ensure_dirs(self):
        self.ensure_dir(Config.TUNE_DIR)
        self.ensure_dir(Config.SYNTH_DIR)

        if not os.path.isdir(Config.PREF_DIR):
            os.mkdir(Config.PREF_DIR)
            os.system('chmod 0777 ' + Config.PREF_DIR + ' &')
            for snd in ['mic1','mic2','mic3','mic4','lab1','lab2','lab3','lab4', 'lab5', 'lab6']:
                shutil.copyfile(Config.SOUNDS_DIR + '/' + snd , Config.PREF_DIR + '/' + snd)
                os.system('chmod 0777 ' + Config.PREF_DIR + '/' + snd + ' &')
                
    def read_file(self,file_path):
        if self.modeList['edit']:
            self.modeList['edit'].handleJournalLoad(file_path)
    def write_file(self,file_path):
        if self.modeList['edit']:
            self.modeList['edit'].handleJournalSave(file_path)


if __name__ == "__main__":     
    if len(sys.argv) > 1 :
        mainwin = TamTam(None, sys.argv[1])
    else:
        mainwin = TamTam(None, 'welcome')
        
    gtk.gdk.threads_init()
    gtk.main()
    
    sys.exit(0)








    def run_edit_mode():
        tamtam = MainWindow()
        mainwin = gtk.Window(gtk.WINDOW_TOPLEVEL)
        mainwin.set_title('TamTam Player')
        display = mainwin.get_display()
        screen = gtk.gdk.Display.get_default_screen(display)
        mainwin.set_geometry_hints( None, screen.get_width(), screen.get_height(), screen.get_width(), screen.get_height(), screen.get_width(), screen.get_height() )
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

