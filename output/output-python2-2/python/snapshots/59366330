#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import re
import sys
import Queue

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import SIGNAL
from PyQt4.QtGui import QApplication

import storage
import gui_option
import event_type
from conf import config
from gui_ui import Ui_dev_client
from history import History
from mud_type import getMudType, ComponentFactory
from constants import PUBLIC_VERSION, PROJECT_NAME

class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with Qt, that inherits the real gui interface
    designed by Qt-designer.
    """

    def __init__(self, q_app_gui, q_gui_app):
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication([])
        self.app.setStyle(QtGui.QStyleFactory.create("Cleanlooks"))
        self._installTranslator()

        self.history = History()

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

        QtGui.QShortcut(QtGui.QKeySequence(QtCore.Qt.Key_Up),
                        self, self._onKeyUp)

        QtGui.QShortcut(QtGui.QKeySequence(QtCore.Qt.Key_Down),
                        self, self._onKeyDown)

        timer = QtCore.QTimer(self)
        self.connect(timer, SIGNAL("timeout()"), self._processIncoming)
        timer.start(10)

        self._translateText()
        self.setWindowTitle(PROJECT_NAME + ' ' + PUBLIC_VERSION)

    def _onKeyUp(self):
        if self.text_input.hasFocus():
            self.text_input.setText(self.history.getPrev())

    def _onKeyDown(self):
        if self.text_input.hasFocus():
            self.text_input.setText(self.history.getNext())

    def _installTranslator(self):
        self.translator = QtCore.QTranslator()
        self.translator.load(config['translation']['path'])
        QtGui.QApplication.installTranslator(self.translator)

    def _translateText(self):
        self._text = {}
        self._text['Connect'] = QApplication.translate("dev_client", "Connect",
            None, QApplication.UnicodeUTF8)

        self._text['NoConn'] = QApplication.translate("dev_client",
            "There aren't connections defined", None, QApplication.UnicodeUTF8)

        self._text['ConnError'] = QApplication.translate("dev_client",
            "Unable to establish connection", None, QApplication.UnicodeUTF8)

        self._text['AlreadyConn'] = QApplication.translate("dev_client",
            "Connection already established", None, QApplication.UnicodeUTF8)

    def closeEvent(self, event):
        self._endApplication()
        event.accept()

    def _showOption(self):
        opt = gui_option.GuiOption(self)
        self.connect(opt, SIGNAL("connectReq(int)"), self._connect)
        opt.show()

    def _connect(self, id_conn = None):
        connections = storage.Storage().connections()
        if not connections:
            self._displayWarning(self._text['Connect'], self._text['NoConn'])
            return

        if id_conn:
            conn = [el for el in connections if el[0] == id_conn]
        else:
            conn = [el for el in connections if el[4] == 1]
            # if is not defined a default connection take the first
            if not conn:
                conn = connections

        name = conn[0][1]
        host = conn[0][2]
        port = conn[0][3]

        self.q_gui_app.put((event_type.CONNECT, (name, host, port)))

    def _startConnection(self, host, port):
        self.history.clear()

        comp_factory = ComponentFactory(getMudType(host, port))
        self.viewer = comp_factory.viewer(self)
        self.text_input.setFocus()

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP, ""))

    def _sendText(self):
        self.history.add(self.text_input.text())
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
            tmp = [el.split(':') for el in oldstyle.split(';') if el]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        if bg: d['background-color'] = '#' + bg
        if fg: d['color'] = '#' + fg

        newstyle = ';'.join([k + ':' + v for k, v in d.iteritems()])

        if oldstyle:
            self.text_output.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            self.text_output.setStyleSheet('QTextEdit {%s}' % style)

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if cmd == event_type.MODEL:
                self.viewer.process(msg)
            elif cmd == event_type.CONN_REFUSED:
                 self._displayWarning(self._text['Connect'],
                                      self._text['ConnError'])
            elif cmd == event_type.ALREADY_CONN:
                 self._displayWarning(self._text['Connect'],
                                      self._text['AlreadyConn'])
            elif cmd == event_type.CONN_ESTABLISHED:
                 self._startConnection(*msg)


        except Queue.Empty:
            pass

    def _displayWarning(self, title, message):
        QtGui.QMessageBox.warning(self, title, message)

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
