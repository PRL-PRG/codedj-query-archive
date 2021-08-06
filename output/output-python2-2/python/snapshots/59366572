#!/usr/bin/python
#-*- coding: utf-8 -*-

import re
import sys
import Queue

from PyQt4 import QtCore, QtGui

from gui_ui import Ui_DevClient
import event_type
import viewer
import gui_option

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

        self.connect(self.actionOption, QtCore.SIGNAL("triggered()"),
                               self._showOption)

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

    def _showOption(self):
        opt = gui_option.GuiOption(self)
        opt.show()

    def _connect(self):
        self.q_gui_app.put((event_type.CONNECT, ""))

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP, ""))

    def _sendText(self):
        self.q_gui_app.put((event_type.MSG, unicode(self.textInput.text())))
        self.textInput.clear()

    def _setOutputColors(self, bg, fg):
        """
        Set output default colors.
        """

        style = str(self.textOutput.styleSheet())
        m = re.search('QTextEdit\s*{(.*)}', style)
        if m:
            oldstyle = m.group(1)
            tmp = [el.split(':') for el in oldstyle.split(';')]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        if bg: d['background-color'] = '#' + bg
        if fg: d['color'] = '#' + fg

        newstyle = ';'.join([k + ':' + v for k,v in d.iteritems()])

        if oldstyle:
            self.textOutput.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            self.textOutput.setStyleSheet('QTextEdit {%s}' % style)

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if cmd == event_type.MODEL:
                text, bg, fg = self.mainViewer.process(msg)
                self.textOutput.insertHtml(text)
                self.textOutput.moveCursor(QtGui.QTextCursor.End)
                if bg or fg:
                    self._setOutputColors(bg, fg)

        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
