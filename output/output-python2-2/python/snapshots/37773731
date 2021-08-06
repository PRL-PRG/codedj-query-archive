#!/usr/bin/python

import pygtk

pygtk.require('2.0')

import gtk

import lib.Gui.common
import os.path

from lib.Project import CProject
from lib.Gui import CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave
from lib.Storages import open_storage

from lib.consts import ROOT_PATH, VERSIONS_PATH, DIAGRAMS_PATH, ELEMENTS_PATH, CONNECTIONS_PATH

class Application(lib.Gui.common.CApplication):
    windows = (CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation, CfrmOpen, CfrmSave)
    glade = os.path.join(os.path.dirname(__file__), 'gui', 'gui.glade')
    main_window = 'frmMain'
    
    def __init__(self):
        #projekt view, pametova reprezentacia
        #Project = DebugAttribute('Project')
        self.Storage = open_storage(os.path.join(ROOT_PATH, 'etc', 'uml'))
        self.Project = CProject(self.Storage)
        lib.Gui.common.CApplication.__init__(self)
        
    
    def cw_FileChooserWidget(self, str1, str2, int1, int2):
        if str1:
            action = getattr(gtk, 'FILE_CHOOSER_ACTION_%s'%str1.upper())
        else:
            action = gtk.FILE_CHOOSER_ACTION_OPEN
        widget = gtk.FileChooserWidget(action)
        widget.show()
        return widget

Application().Main()
