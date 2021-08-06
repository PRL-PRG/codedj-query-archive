#!/usr/bin/python

#~ import pygtk

#~ pygtk.require('2.0')

import warnings
warnings.simplefilter('ignore', Warning)

import gtk
import gobject

from lib.Clipboard import CClipboard
import lib.Gui.common
import os.path

from lib.Project import CProject
from lib.Project import CRecentFiles

from lib.Gui import CfrmSplash, CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave

from lib.config import config
from lib.consts import SPLASH_TIMEOUT

__version__ = '1.0-alpha'

class Application(lib.Gui.common.CApplication):
    windows = (CfrmSplash, CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave)
    glade = os.path.join(config['/Paths/Gui'], 'gui.glade')
    main_window = 'frmMain'
    textdomain = 'uml_fri'
    localespath = config['/Paths/Locales']
    
    project = None
    
    def __init__(self):
        lib.Gui.common.CApplication.__init__(self)
        self.clipboard = CClipboard()
        self.recentFiles = CRecentFiles()
        gobject.timeout_add(SPLASH_TIMEOUT, self.GetWindow('frmSplash').Hide)
    
    def GetRecentFiles(self):
        return self.recentFiles
    
    def ProjectInit(self):
        if self.project is None:
            self.project = CProject()
    
    def GetProject(self):
        return self.project
    
    def GetClipboard(self):
        return self.clipboard
    
    def cw_FileChooserWidget(self, str1, str2, int1, int2):
        if str1:
            action = getattr(gtk, 'FILE_CHOOSER_ACTION_%s'%str1.upper())
        else:
            action = gtk.FILE_CHOOSER_ACTION_OPEN
        widget = gtk.FileChooserWidget(action)
        widget.show()
        return widget
    
    def Quit(self):
        lib.Gui.common.CApplication.Quit(self)
        self.recentFiles.SaveRecentFiles()
    
    Project = property(GetProject)
    Clipboard = property(GetClipboard)
    
    

Application().Main()
