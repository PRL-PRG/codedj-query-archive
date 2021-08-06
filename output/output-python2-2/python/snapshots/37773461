#!/usr/bin/python

import pygtk

pygtk.require('2.0')

import warnings
warnings.simplefilter('ignore', Warning)

import gtk

from lib.Clipboard import CClipboard
import lib.Gui.common
import os.path

from lib.Project import CProject
from lib.Project import CRecentFiles

from lib.Gui import CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave


class Application(lib.Gui.common.CApplication):
    windows = (CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave)
    glade = os.path.join(os.path.dirname(__file__), 'gui', 'gui.glade')
    main_window = 'frmMain'
    
    project = None
    
    def __init__(self):
        lib.Gui.common.CApplication.__init__(self)
        self.clipboard = CClipboard()
        self.recentFiles = CRecentFiles()
    
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
