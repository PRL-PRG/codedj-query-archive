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

        self.connect(self.actionExit, QtCore.SIGNAL("activated()"),
                               self._endApplication)

        self.connect(self.actionConnect, QtCore.SIGNAL("activated()"),
                               self._connect)

        self.connect(self.textInput, QtCore.SIGNAL("returnPressed()"),
                               self._sendText)

        timer = QtCore.QTimer(self)
        self.connect(timer, QtCore.SIGNAL("timeout()"), self._processIncoming)
        timer.start(100)

        self.textInput.setFocus()

    def _connect(self):
        self.q_gui_app.put((event_type.CONNECT,""))

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP,""))

    def _sendText(self):
        self.q_gui_app.put((event_type.MSG, self.textInput.displayText()))
        self.textInput.clear()

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if msg:
                self.textOutput.append(msg)
        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
