import signal , time , sys , os, shutil
import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from  Util.CSoundClient import new_csound_client
from   Util.Profiler import TP

from   miniTamTam.miniTamTamMain import miniTamTamMain
from   Edit.MainWindow import MainWindow
from   Welcome import Welcome
from   SynthLab.SynthLabWindow import SynthLabWindow
from Util.Trackpad import Trackpad

try :
    from sugar.activity.Activity import Activity
except ImportError:
    from FActivity import FakeActivity as Activity

if not os.path.isdir(Config.PREF_DIR):
    os.mkdir(Config.PREF_DIR)
    os.system('chmod 0777 ' + Config.PREF_DIR + ' &')
    for snd in ['mic1','mic2','mic3','mic4','lab1','lab2','lab3','lab4']:
        shutil.copyfile(Config.SOUNDS_DIR + '/' + snd , Config.PREF_DIR + '/' + snd)
        os.system('chmod 0777 ' + Config.PREF_DIR + '/' + snd + ' &')


class TamTam(Activity):
    # TamTam is the topmost container in the TamTam application
    # At all times it has one child, which may be one of
    # - the welcome screen
    # - the mini-tamtam 
    # - the synth lab
    # - edit mode

    def __init__(self, handle, mode='welcome'):
        Activity.__init__(self, handle)
        
        color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.set_title('TamTam')
        self.set_resizable(False)

        self.trackpad = Trackpad( self )

        self.connect('focus_in_event',self.onFocusIn)
        self.connect('focus_out_event',self.onFocusOut)
        self.connect('destroy', self.onDestroy)
        self.connect( "key-press-event", self.onKeyPress )
        self.connect( "key-release-event", self.onKeyRelease )

        self.modeList = {
                'welcome': Welcome(self.set_mode), 
                'mini': miniTamTamMain(self.set_mode),
                'edit': MainWindow(self.set_mode),
                'synth': SynthLabWindow( self.set_mode, 86, None)
                }
        self.mode = mode

        self.add(self.modeList[self.mode])
        self.modeList[ self.mode ].onActivate()
        self.show_all()

    def doNothing(): #a callback function to appease SynthLab
        pass
    def set_mode(self, mode):
        print 'DEBUG: TamTam::set_mode from', self.mode, 'to', mode
        if mode in self.modeList:
            self.remove( self.modeList[ self.mode ] )
            self.modeList[ self.mode ].onDeactivate()
            self.mode = mode
            self.add(    self.modeList[ self.mode ] )
            self.modeList[ self.mode ].onActivate()
            self.modeList[ self.mode ].show()
        else:
            print 'DEBUG: TamTam::set_mode invalid mode:', mode

    def onFocusIn(self, event, data=None):
        print 'DEBUG: TamTam::onFocusOut in TamTam.py'
        csnd = new_csound_client()
        csnd.connect(True)
        #csnd.load_instruments()
    
    def onFocusOut(self, event, data=None):
        print 'DEBUG: TamTam::onFocusOut in TamTam.py'
        csnd = new_csound_client()
        csnd.connect(False)

    def onKeyPress(self, widget, event):
        print 'DEBUG: TamTam::onKeyPress in TamTam.py'
        if event.state == gtk.gdk.MOD1_MASK:
            key = event.hardware_keycode
            if key == 58:    #M
                self.set_mode('mini')
                return
            elif key == 39:  #S
                self.set_mode('synth')
                return
            elif key == 25:  #W
                self.set_mode('welcome')
                return
            elif key == 53:  #X
                self.destroy()
                return
        self.modeList[ self.mode ].onKeyPress(widget, event)

    def onKeyRelease(self, widget, event):
        print 'DEBUG: TamTam::onKeyRelease in TamTam.py'
        self.modeList[ self.mode ].onKeyRelease(widget, event)
        pass

    def onDestroy(self, arg2):
        print 'DEBUG: TamTam::onDestroy()'
        os.system('rm -f ' + Config.PREF_DIR + '/synthTemp*')

        for m in self.modeList: 
            self.modeList[m].onDestroy()

        csnd = new_csound_client()
        csnd.connect(False)
        csnd.destroy()

        gtk.main_quit()


if __name__ == "__main__":     
    def run_non_sugar_mode(mode):
        mainwin = TamTam(None,mode)
        gtk.main()
        
    if len(sys.argv) > 1 :
        run_non_sugar_mode(sys.argv[1])
    else:
        run_non_sugar_mode('welcome')
    
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

