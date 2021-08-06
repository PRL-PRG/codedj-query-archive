#!/usr/bin/python

#~ import pygtk

#~ pygtk.require('2.0')

import warnings
warnings.simplefilter('ignore', Warning)

import gtk
import gobject

from lib.Clipboard import CClipboard
from lib.Gui.common import CApplication, argument
import os.path

from lib.Project import CProject
from lib.Project import CRecentFiles

from lib.Gui import CfrmSplash, CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave, CfrmOptions, CfrmException

from lib.config import config
from lib.consts import SPLASH_TIMEOUT

__version__ = '1.0-beta'

class Application(CApplication):
    windows = (CfrmSplash, CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave, CfrmOptions, CfrmException)
    glade = os.path.join(config['/Paths/Gui'], 'gui.glade')
    main_window = 'frmMain'
    textdomain = 'uml_fri'
    localespath = config['/Paths/Locales']

    project = None
    canopen = True
    
    def __init__(self):
        self.recentFiles = CRecentFiles()
        CApplication.__init__(self)
        
        self.clipboard = CClipboard()
        
        gobject.timeout_add(SPLASH_TIMEOUT, self.GetWindow('frmSplash').Hide)
    
    @argument("-o", "--open", True)
    def DoOpen(self, value):
        "Opens selected project file"
        if self.canopen:
            self.GetWindow('frmMain').LoadProject(value, False)
            self.canopen = False
            
    
    @argument("-n", "--new", True)
    def DoNew(self, value):
        "Creates new project from template"
        if self.canopen:
            self.GetWindow('frmMain').LoadProject(value, True)
            self.canopen = False
    
    @argument()
    def DoArguments(self, *files):
        "File to open"
        if self.canopen:
            self.GetWindow('frmMain').LoadProject(files[0], False)
            self.canopen = False
    
    def GetRecentFiles(self):
        return self.recentFiles
    
    def ProjectInit(self):
        if self.project is None:
            self.project = CProject()
            
    def ProjectDelete(self):
        self.project = None
        
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
        CApplication.Quit(self)
        self.recentFiles.SaveRecentFiles()

Application().Main()
