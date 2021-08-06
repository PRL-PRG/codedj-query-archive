#!/usr/bin/python
#-*- coding: utf-8 -*-

import sys
import Queue

from PyQt4 import QtCore, QtGui

from gui_abstract import Gui
from gui_qt_ui import Ui_DevClient
import event_type

class QGui(Gui, QtGui.QMainWindow, Ui_DevClient):
    def __init__(self, q_app_gui, q_gui_app):
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication(sys.argv)
        Gui.__init__(self)
        QtGui.QMainWindow.__init__(self)
        self.setupUi(self)

        QtCore.QObject.connect(self.actionEsci, QtCore.SIGNAL("activated()"),
                               self._endApplication)

        timer = QtCore.QTimer(self)
        self.connect(timer, QtCore.SIGNAL("timeout()"), self._processIncoming)
        timer.start(200)

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP,""))

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            self.textOutput.append(msg)
        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
