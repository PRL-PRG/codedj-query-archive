#!/usr/bin/python
#-*- coding: utf-8 -*-

import sys
import Queue

from PyQt4 import QtCore, QtGui

from gui_ui import Ui_DevClient
import event_type, viewer

class Gui(QtGui.QMainWindow, Ui_DevClient):
    """
    The Gui class written with Qt, that inherits the real gui interface
    designed by Qt-designer.
    """

    def __init__(self, q_app_gui, q_gui_app):
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication([])
        QtGui.QMainWindow.__init__(self)
        self.setupUi(self)

        self.connect(self.actionExit, QtCore.SIGNAL("triggered()"),
                               self._endApplication)

        self.connect(self.actionConnect, QtCore.SIGNAL("triggered()"),
                               self._connect)

        self.connect(self.textInput, QtCore.SIGNAL("returnPressed()"),
                               self._sendText)

        timer = QtCore.QTimer(self)
        self.connect(timer, QtCore.SIGNAL("timeout()"), self._processIncoming)
        timer.start(100)

        self.textInput.setFocus()
        self.mainViewer = viewer.Viewer()

    def closeEvent(self, event):
        self._endApplication()
        event.accept()

    def _connect(self):
        self.q_gui_app.put((event_type.CONNECT, ""))

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP, ""))

    def _sendText(self):
        self.q_gui_app.put((event_type.MSG, unicode(self.textInput.text())))
        self.textInput.clear()

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if cmd == event_type.MODEL:
                self.textOutput.insertHtml(self.mainViewer.process(msg))
                self.textOutput.moveCursor(QtGui.QTextCursor.End)
        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
