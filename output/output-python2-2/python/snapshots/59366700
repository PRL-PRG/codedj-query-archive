#!/usr/bin/python
#-*- coding: utf-8 -*-

import sys
import Queue

from PyQt4 import QtCore, QtGui

from gui_abstract import Gui
import event_type

class QGui(Gui, QtGui.QWidget):
    def __init__(self, q_app_gui, q_gui_app):
        # work in progress..
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication(sys.argv)
        QtGui.QWidget.__init__(self, None)
        quit_button = QtGui.QPushButton("Quit")
        quit_button.setFont(QtGui.QFont("Times", 18, QtGui.QFont.Bold))

        self.connect(quit_button, QtCore.SIGNAL("clicked()"),
                     self._endApplication)

        self.connect(quit_button, QtCore.SIGNAL("clicked()"),
                     QtGui.qApp, QtCore.SLOT("quit()"))

        self.label = QtGui.QLabel()
        gridLayout = QtGui.QGridLayout()
        gridLayout.addWidget(quit_button, 0, 0)
        gridLayout.addWidget(self.label, 1, 0)
        self.setLayout(gridLayout)

        timer = QtCore.QTimer(self)
        self.connect(timer,QtCore.SIGNAL("timeout()"), self._processIncoming)
        timer.start(200)

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP,""))

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            self.label.setText(msg)
        except Queue.Empty:
            pass

    def mainLoop(self):
        self.setGeometry(100, 100, 500, 355)
        self.show()
        sys.exit(self.app.exec_())