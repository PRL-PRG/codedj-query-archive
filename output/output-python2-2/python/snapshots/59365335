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

import sys
import struct
import os.path
import cPickle
import logging

from PyQt4 import QtCore, QtGui, QtNetwork
from PyQt4.QtCore import SIGNAL, Qt, QLocale
from PyQt4.QtGui import QApplication, QMessageBox
from PyQt4.QtNetwork import QHostAddress

import storage
import messages
import gui_option
from conf import config
from gui_ui import Ui_dev_client
from history import History
from mud_type import getMudType, ComponentFactory
from constants import PUBLIC_VERSION, PROJECT_NAME

logger = logging.getLogger('gui')


class SocketToCore(object):
    """
    Provide a socket interface used to exchange message with `Core`.
    """

    def __init__(self, widget, port=7890, timeout=.2):
        """
        Create the `SocketToCore` instance.
        
        :Parameters:
          widget : QWidget
            the parent widget, used to display messages
          port : int
            the port used to establish a connection with `Core`
          timeout : int
            the timeout of socket operations (in seconds)
        """

        self._w = widget
        self._timeout = timeout * 1000
        self._s = QtNetwork.QTcpSocket()
        self._s.connectToHost(QHostAddress(QHostAddress.LocalHost), port)
        if not self._s.waitForConnected(self._timeout):
            # FIX: the core process must to be killed
            self._commError()
            return
        self._setupSignal()

    def _setupSignal(self):
        self._w.connect(self._s, SIGNAL("readyRead()"),
                        self._w._readDataFromCore)
        self._w.connect(self._s, SIGNAL("error(QAbstractSocket::SocketError)"),
                        self._commError)

    def _commError(self, error=None):
        logger.error('SocketToCore: ' + self._s.errorString())
        self._w._displayWarning(PROJECT_NAME, self._w._text['FatalError'])

    def _readData(self, size):
        """
        Read data, blocking until (for a max of timeout) all data is available.
        
        :Parameters:
          size : int
            the length of data to read

        :return: data if it is available, None otherwise
        """

        for wait in (True, False):
            if self._s.bytesAvailable() < size:
                if wait:
                    # waitForReadyRead might cause UI to freeze
                    self._s.waitForReadyRead(self._timeout)
                else:
                    return None

        return self._s.read(size)
        
    def read(self):
        """
        Read a message.

        :return: a tuple of the form (<message type>, <message>)
        """

        def exit_clean():
            # waste all data available to restore format data for next messages
            self._s.read(self._s.bytesAvailable())
            return (messages.UNKNOWN, '')

        size = self._readData(struct.calcsize("L"))
        if size is None:
            return exit_clean()
        try:
            size = struct.unpack('>l', size)[0]
        except struct.error:
            return exit_clean()

        if size < 0:
            return exit_clean()

        data = self._readData(size)
        if data is None:
            return exit_clean()

        try:
            return cPickle.loads(data)
        except cPickle.BadPickleGet:
            return (messages.UNKNOWN, '')

    def availableData(self):
        return self._s.bytesAvailable() > 0

    def write(self, cmd, message):
        """
        Send a message.

        :Parameters:
          cmd : int
            the message type

          message : object
            the message to sent
        """

        buf = cPickle.dumps((cmd, message))
        self._s.write(struct.pack('>l', len(buf)))
        self._s.write(buf)
        self._s.flush()  # prevent buffering

    def disconnect(self):
        self._s.disconnectFromHost()

    def __del__(self):
        self.disconnect()


class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with Qt, that inherits the real gui interface
    designed by Qt-designer.
    """

    def __init__(self, port):

        if QApplication.instance():
            self.app = QApplication.instance()
        else:
            self.app = QApplication([])

        self.app.setStyle(QtGui.QStyleFactory.create("Cleanlooks"))
        self._installTranslator()
        QtGui.QMainWindow.__init__(self)
        self.setupUi(self)
        self._setupSignal()
        self._translateText()
        self.setupLogger()

        self.s_core = SocketToCore(self, port)
        """the interface with `Core`, an instance of `SocketToCore`"""

        self.history = History()
        self._default_style = unicode(self.text_output.styleSheet())

        self.setWindowTitle(PROJECT_NAME + ' ' + PUBLIC_VERSION)
        self.connected = None
        self.text_input.setCompleter(None)
        self.text_input.installEventFilter(self)
        self.text_output.installEventFilter(self)
        self.text_output.setFocusProxy(self.text_input)

    def setupLogger(self):
        """
        Setup the root logger from configuration params.
        """

        level = {'CRITICAL': logging.CRITICAL,
                 'ERROR': logging.ERROR,
                 'WARNING': logging.WARNING,
                 'INFO': logging.INFO,
                 'DEBUG': logging.DEBUG }

        logging.basicConfig(level=level[config['logger']['level']],
                            format='%(asctime)s %(levelname)s %(message)s',
                            datefmt='%d %b %Y %H:%M:%S',
                            stream=sys.stdout)

    def _setupSignal(self):
        self.connect(self.action_connect, SIGNAL("triggered()"),
                     self._connect)

        self.connect(self.action_option, SIGNAL("triggered()"),
                     self._showOption)

        QtGui.QShortcut(QtGui.QKeySequence(Qt.Key_Up), self, self._onKeyUp)
        QtGui.QShortcut(QtGui.QKeySequence(Qt.Key_Down), self, self._onKeyDown)

        QtGui.QShortcut(QtGui.QKeySequence(Qt.Key_Enter), self, self._sendText)
        QtGui.QShortcut(QtGui.QKeySequence(Qt.Key_Return), self, self._sendText)

        QtGui.QShortcut(QtGui.QKeySequence(Qt.ALT + Qt.Key_Q), self, self.close)

    def _getKeySeq(self, event):
        """
        Given a keyboard event, return a tuple of its components.

        :Parameters:
          event : QKeyEvent
            the keyboard event

        :return: a tuple of the form (shift, alt, ctrl, keycode)
        """

        def _checkModifier(value, mod):
            """
            Check keyboard's modifier.
            """

            return int((value & mod) == mod)

        s = _checkModifier(event.modifiers(), Qt.ShiftModifier)
        a = _checkModifier(event.modifiers(), Qt.AltModifier)
        c = _checkModifier(event.modifiers(), Qt.ControlModifier)
        return (s, a, c, event.key())

    def eventFilter(self, target, event):
        if event.type() == QtCore.QEvent.KeyPress and self.connected and \
           event.key() not in (Qt.Key_Shift, Qt.Key_Control, Qt.Key_Meta,
                               Qt.Key_Alt):

            key_seq = self._getKeySeq(event)

            for m in self.macros:
                if m[1:] == key_seq:
                    self.s_core.write(messages.MSG, m[0])
                    return True
        return False

    def _onKeyUp(self):
        self.text_input.setCurrentIndex(0)
        self.text_input.setItemText(0, self.history.getPrev())

    def _onKeyDown(self):
        self.text_input.setCurrentIndex(0)
        self.text_input.setItemText(0, self.history.getNext())

    def _installTranslator(self):
        """
        Translate application according to system locale
        """

        locale = str(QLocale.system().name())
        fn = os.path.join(config['translation']['path'], locale + '.qm')
        if os.path.exists(fn):
            self.translator = QtCore.QTranslator()
            self.translator.load(fn)
            QApplication.installTranslator(self.translator)

    def _translateText(self):
        self._text = {}
        self._text['Connect'] = QApplication.translate("dev_client", "Connect",
            None, QApplication.UnicodeUTF8)

        self._text['NoConn'] = QApplication.translate("dev_client",
            "There aren't connections defined", None, QApplication.UnicodeUTF8)

        self._text['ConnError'] = QApplication.translate("dev_client",
            "Unable to establish connection", None, QApplication.UnicodeUTF8)

        self._text['Yes'] = QApplication.translate("dev_client", "Yes",
            None, QApplication.UnicodeUTF8)

        self._text['No'] = QApplication.translate("dev_client", "No",
            None, QApplication.UnicodeUTF8)

        self._text['CloseConfirm'] = QApplication.translate("dev_client",
            "Really quit?", None, QApplication.UnicodeUTF8)

        self._text['CloseConn'] = QApplication.translate("dev_client",
            "Really close connection?", None, QApplication.UnicodeUTF8)

        self._text['FatalError'] = QApplication.translate("dev_client",
            "Fatal Error, please restart your client", None,
            QApplication.UnicodeUTF8)

        self._text['ConnLost'] = QApplication.translate("dev_client",
            "Connection lost", None, QApplication.UnicodeUTF8)

        self._text['NotConnected'] = QApplication.translate("dev_client",
            "Client is not connected", None, QApplication.UnicodeUTF8)

    def closeEvent(self, event):
        if self.connected:
            if not self._displayQuestion(PROJECT_NAME,
                                         self._text['CloseConfirm']):
                event.ignore()
                return

        self.s_core.write(messages.END_APP, "")
        self.s_core.disconnect()
        event.accept()

    def _showOption(self):
        opt = gui_option.GuiOption(self)
        self.connect(opt, SIGNAL("connectReq(int)"), self._connect)
        self.connect(opt, SIGNAL("reloadConnData(QString)"),
                     self._reloadConnData)
        opt.show()

    def _connect(self, id_conn = None):
        if self.connected:
            if not self._displayQuestion(self._text['Connect'],
                                         self._text['CloseConn']):
                return

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

        self.s_core.write(messages.CONNECT, conn[0][1:4])

    def _reloadConnData(self, conn):
        """
        Reload all data rely on connection and propagate message of reloading.

        :Parameters:
          conn : str
            the name of connection
        """

        if self.connected and self.connected == conn:
            self.macros = storage.Storage().macros(self.connected)
            self.s_core.write(messages.RELOAD_CONN_DATA, unicode(conn))

    def _startConnection(self, host, port):
        self.history.clear()

        comp_factory = ComponentFactory(getMudType(host, port))
        self.viewer = comp_factory.viewer(self)
        self.viewer.resetOutputColors(self._default_style)
        self.macros = storage.Storage().macros(self.connected)

    def _sendText(self):
        if self.connected:
            text = unicode(self.text_input.currentText())
            self.history.add(text)
            self.s_core.write(messages.MSG, text)
            hist = self.history.get()
            hist.reverse()
            self.text_input.clear()
            self.text_input.addItem('')
            self.text_input.addItems(hist)
            self.text_input.setCurrentIndex(0)
            self.text_input.setItemText(0, '')
        else:
            self._displayWarning(PROJECT_NAME, self._text['NotConnected'])

    def _readDataFromCore(self):

        while self.s_core.availableData():
            cmd, msg = self.s_core.read()
            if cmd == messages.MODEL:
                self.viewer.process(msg)
            elif cmd == messages.CONN_REFUSED:
                self._displayWarning(self._text['Connect'],
                                     self._text['ConnError'])
            elif cmd == messages.CONN_ESTABLISHED:
                self.connected = msg[0]
                self._startConnection(*msg[1:])
            elif cmd == messages.CONN_LOST:
                self._displayWarning(self._text['Connect'],
                                     self._text['ConnLost'])
                self.connected = None
            elif cmd == messages.CONN_CLOSED:
                self.connected = None
            elif cmd == messages.UNKNOWN:
                logger.warning('SocketToCore: Unknown message')

    def _displayQuestion(self, title, message):
        box = QMessageBox(self)
        box.setWindowTitle(title)
        box.setText(message)
        yes = box.addButton(self._text['Yes'], QMessageBox.AcceptRole)
        no = box.addButton(self._text['No'], QMessageBox.RejectRole)
        box.setDefaultButton(no)
        box.setEscapeButton(no)
        box.exec_()
        return box.clickedButton() == yes

    def _displayWarning(self, title, message):
        QMessageBox.warning(self, title, message)

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
