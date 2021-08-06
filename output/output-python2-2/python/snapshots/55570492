import locale
locale.setlocale(locale.LC_NUMERIC, 'C')
import signal , time , sys , os, shutil
import pygtk
pygtk.require( '2.0' )
import gtk

import gobject
import time

import common.Config as Config
from   common.Util.CSoundClient import new_csound_client
from   common.Util.Profiler import TP

from   Jam.JamMain import JamMain
from   common.Util.Trackpad import Trackpad
from   gettext import gettext as _
import commands
from sugar.activity import activity

class TamTamJam(activity.Activity):
    def __init__(self, handle):
        # !!!!!! initialize threading in gtk !!!!!
        # ! this is important for the networking !
        # !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        gtk.gdk.threads_init()

        activity.Activity.__init__(self, handle)
        self.ensure_dirs()

        color = gtk.gdk.color_parse(Config.WS_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)

        self.set_title('TamTam Jam')
        self.set_resizable(False)

        self.trackpad = Trackpad( self )

        self.preloadTimeout = None

        self.connect('notify::active', self.onActive)
        self.connect('destroy', self.onDestroy)

        #load the sugar toolbar
        self.toolbox = activity.ActivityToolbox(self)
        self.set_toolbox(self.toolbox)

        self.activity_toolbar = self.toolbox.get_activity_toolbar()

        self.toolbox.show()

        self.trackpad.setContext('jam')
        self.jam = JamMain(self)
        self.connect('key-press-event', self.jam.onKeyPress)
        self.connect('key-release-event', self.jam.onKeyRelease)
        #self.modeList[mode].regenerate()

        self.set_canvas( self.jam )

        self.jam.onActivate(arg = None)
        self.show()

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

    def onActive(self, widget = None, event = None):
        if widget.props.active == False:
            Config.logwrite(1, 'Jam.onActivate disconnecting csound')
            csnd = new_csound_client()
            csnd.connect(False)
        else:
            Config.logwrite(1, 'Jam.onActivate connecting csound')
            csnd = new_csound_client()
            csnd.connect(True)

    def onKeyPress(self, widget, event):
        pass

    def onKeyRelease(self, widget, event):
        pass

    def onDestroy(self, arg2):
        if Config.DEBUG: print 'DEBUG: TamTam::onDestroy()'
        os.system('rm -f ' + Config.PREF_DIR + '/synthTemp*')

        self.jam.onDestroy()

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
        self.ensure_dir(Config.SNDS_DIR)
        self.ensure_dir(Config.SNDS_INFO_DIR)
        self.ensure_dir(Config.SCRATCH_DIR)

        if not os.path.isdir(Config.PREF_DIR):
            os.mkdir(Config.PREF_DIR)
            os.system('chmod 0777 ' + Config.PREF_DIR + ' &')
            for snd in ['mic1','mic2','mic3','mic4','lab1','lab2','lab3','lab4', 'lab5', 'lab6']:
                shutil.copyfile(Config.SOUNDS_DIR + '/' + snd , Config.SNDS_DIR + '/' + snd)
                os.system('chmod 0777 ' + Config.SNDS_DIR + '/' + snd + ' &')

    def read_file(self,file_path):
        self.jam.handleJournalLoad(file_path)

    def write_file(self,file_path):
        self.jam.handleJournalSave(file_path)
