#!/usr/bin/python

import pygtk

pygtk.require('2.0')

import lib.Gui.common
import os.path

from lib.Gui import CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation
from lib.Versions import CVersionFactory
from lib.Diagrams import CDiagramFactory
from lib.Elements import CElementFactory
from lib.Connections import CConnectionFactory
from lib.Storages import open_storage

from lib.consts import ROOT_PATH, VERSIONS_PATH, DIAGRAMS_PATH, ELEMENTS_PATH, CONNECTIONS_PATH

class Application(lib.Gui.common.CApplication):
    windows = (CfrmMain, CfrmAbout, CfrmProperties, CfrmAttribute, CfrmOperation)
    glade = os.path.join(os.path.dirname(__file__), 'gui', 'gui.glade')
    main_window = 'frmMain'
    
    def __init__(self):
        self.Storage = open_storage(os.path.join(ROOT_PATH, 'etc', 'uml'))
        self.DiagramFactory = CDiagramFactory(self.Storage, DIAGRAMS_PATH)
        self.ElementFactory = CElementFactory(self.Storage, ELEMENTS_PATH)
        self.ConnectionFactory = CConnectionFactory(self.Storage, CONNECTIONS_PATH)
        self.VersionFactory = CVersionFactory(self.Storage, VERSIONS_PATH)
        self.version = self.VersionFactory.GetVersion('UML 1.4')
        lib.Gui.common.CApplication.__init__(self)
        self.Project = None
        

Application().Main()
