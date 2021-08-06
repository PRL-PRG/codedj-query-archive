#!/usr/bin/python
#-*- coding: utf-8 -*-

import re
import sys
import Queue

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import SIGNAL
from PyQt4.QtGui import QApplication

import viewer
import storage
import gui_option
import event_type
from conf import config
from gui_ui import Ui_dev_client

class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with Qt, that inherits the real gui interface
    designed by Qt-designer.
    """

    def __init__(self, q_app_gui, q_gui_app):
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication([])
        self._installTranslator()

        QtGui.QMainWindow.__init__(self)
        self.setupUi(self)

        self.connect(self.action_exit, SIGNAL("triggered()"),
                     self._endApplication)

        self.connect(self.action_connect, SIGNAL("triggered()"),
                     self._connect)

        self.connect(self.action_option, SIGNAL("triggered()"),
                     self._showOption)

        self.connect(self.text_input, SIGNAL("returnPressed()"),
                     self._sendText)

        timer = QtCore.QTimer(self)
        self.connect(timer, SIGNAL("timeout()"), self._processIncoming)
        timer.start(10)

        self.text_input.setFocus()
        self.mainViewer = viewer.Viewer()

    def _installTranslator(self):
        self.translator = QtCore.QTranslator()
        self.translator.load(config['translation']['path'])
        QtGui.QApplication.installTranslator(self.translator)

    def closeEvent(self, event):
        self._endApplication()
        event.accept()

    def _showOption(self):
        opt = gui_option.GuiOption(self)
        opt.show()

    def _connect(self):
        connections = storage.Storage().connections()
        if connections:
            conn = [el for el in connections if el[3] == 1]
            # if is not defined a default connection take the first
            if not conn:
                conn = connections

            self.q_gui_app.put((event_type.CONNECT, (conn[0][1], conn[0][2])))
        else:
            window = QApplication.translate("dev_client", "Connect",
                                            None, QApplication.UnicodeUTF8)

            msg = QApplication.translate("dev_client",
                                "There aren't connections defined",
                                None, QApplication.UnicodeUTF8)
            QtGui.QMessageBox.warning(self, window, msg)

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP, ""))

    def _sendText(self):
        self.q_gui_app.put((event_type.MSG, unicode(self.text_input.text())))
        self.text_input.clear()

    def _setOutputColors(self, bg, fg):
        """
        Set output default colors.
        """

        style = unicode(self.text_output.styleSheet())
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
            self.text_output.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            self.text_output.setStyleSheet('QTextEdit {%s}' % style)

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if cmd == event_type.MODEL:
                text, bg, fg = self.mainViewer.process(msg)
                self.text_output.moveCursor(QtGui.QTextCursor.End)
                self.text_output.insertHtml(text)
                self.text_output.moveCursor(QtGui.QTextCursor.End)
                if bg or fg:
                    self._setOutputColors(bg, fg)

        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
