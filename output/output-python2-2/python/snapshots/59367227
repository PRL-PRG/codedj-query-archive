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
import cPickle
import logging
from os import mkdir
from glob import glob
from time import strftime
from os.path import join, exists

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import QEvent, Qt, QLocale, QVariant
from PyQt4.QtCore import SIGNAL, PYQT_VERSION_STR, QT_VERSION_STR
from PyQt4.QtGui import QApplication, QIcon
from PyQt4.QtGui import QMessageBox, QShortcut, QKeySequence
from PyQt4.QtNetwork import QHostAddress, QTcpSocket

import messages
import exception
import gui_option
from conf import config
from alias import Alias
from history import History
from viewer import getViewer
from servers import getServer
from storage import Storage, Option, adjustSchema
from gui_src.gui import Ui_dev_client
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
        self._s = QTcpSocket()
        self._s.connectToHost(QHostAddress(QHostAddress.LocalHost), port)
        if not self._s.waitForConnected(self._timeout):
            self._commError()
        self._setupSignal()

    def _setupSignal(self):
        self._w.connect(self._s, SIGNAL("readyRead()"),
                        self._w._readDataFromCore)
        self._w.connect(self._s, SIGNAL("error(QAbstractSocket::SocketError)"),
                        self._commError)

    def _commError(self, error=None):
        logger.error('SocketToCore: ' + self._s.errorString())
        self._w.displayWarning(PROJECT_NAME, self._w._text['FatalError'])
        raise exception.IPCError()

    def _readData(self, size):
        """
        Read data, blocking until (for a max of timeout) all data is available.

        :Parameters:
          size : int
            the length of data to read

        :return: data if it is available, None otherwise
        """

        while self._s.bytesAvailable() < size and \
              self._s.waitForReadyRead(self._timeout):
            pass

        if self._s.bytesAvailable() < size:
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


class GameLogger(object):

    encoding = "ISO-8859-1"

    def __init__(self, server_name, preferences):
        if not preferences or not preferences[3] or not server_name:
            return

        dir_name = join(config['logger']['path'], server_name)
        try:
            if not exists(dir_name):
                mkdir(dir_name)

            self.fd = open(join(dir_name, strftime("%Y-%m-%d_%H-%M.log")), 'a+')
        except IOError:
            logger.warning('GameLogger: unable to open log file')

    def write(self, model):
        if hasattr(self, 'fd'):
            self.fd.write(model.original_text.encode(self.encoding))

    def __del__(self):
        if hasattr(self, 'fd'):
            self.fd.flush()
            self.fd.close()


class AccountManager(object):

    def __init__(self, widget, server, id_conn):
        s = Storage()
        self.user = unicode(widget.list_account.currentText())
        s.setOption(Option.DEFAULT_ACCOUNT, self.user, id_conn)
        self._save_account = s.option(Option.SAVE_ACCOUNT, 0)
        self._num_cmds = server.cmds_account
        self._cmd_user = server.cmd_username
        self._id_conn = id_conn
        self._commands = []

    def register(self, text):
        if not self.user and self._save_account \
           and len(self._commands) < self._num_cmds:
            self._commands.append(text)
            if len(self._commands) == self._num_cmds:
                Storage().saveAccount(self._commands, self._id_conn,
                                      self._cmd_user)
                return True
        return False


class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with `Qt`_, that inherits the real gui interface
    designed by `Qt-designer`_.

.. _Qt: http://doc.trolltech.com/4.3/index.html
.. _Qt-designer: http://doc.trolltech.com/4.3/designer-manual.html
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
        self.setupLogger()
        self._translateText()

        self.s_core = SocketToCore(self, port)
        """the interface with `Core`, an instance of `SocketToCore`"""

        self.viewer = None
        """The instance of `Viewer` used to show data arrived from `Core`"""

        self.history = History()

        self.connected = None
        """the name of server connected or None"""

        logger.debug('PyQt version: %s, Qt version: %s' %
            (PYQT_VERSION_STR, QT_VERSION_STR))

        adjustSchema()
        self.preferences = Storage().preferences()
        self._loadConnections()
        self._setupSignal()

    def _loadConnections(self):
        connections = Storage().connections()
        def_conn = Storage().option(Option.DEFAULT_CONNECTION, 0)
        selected = 0
        for i, el in enumerate(connections):
            self.list_conn.addItem(el[1], QVariant(el[0]))
            if el[0] == def_conn:
                selected = i
        self.list_conn.setCurrentIndex(selected)
        if connections:
            self._loadAccounts(def_conn if def_conn else connections[0][0])

    def _loadAccountsFromIdx(self, idx):
         id_conn = self.list_conn.itemData(idx).toInt()[0]
         self._loadAccounts(id_conn)

    def _loadAccounts(self, id_conn):
        self.list_account.clear()
        self.list_account.addItem('')
        def_account = Storage().option(Option.DEFAULT_ACCOUNT, '', id_conn)
        selected = 0
        for i, a in enumerate(Storage().accounts(id_conn)):
            self.list_account.addItem(a)
            if a == def_account:
                selected = i + 1
        self.list_account.setCurrentIndex(selected)

    def setupUi(self, w):
        Ui_dev_client.setupUi(self, w)

        self.setWindowTitle(PROJECT_NAME + ' ' + PUBLIC_VERSION)
        self.text_input.setCompleter(None)
        self.text_input.installEventFilter(self)
        self.text_output.installEventFilter(self)
        self.text_input.lineEdit().installEventFilter(self)

        screen = QtGui.QDesktopWidget().screenGeometry()
        size = self.geometry()
        self.move((screen.width() - size.width()) / 2,
                  (screen.height() - size.height()) / 2)

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
        clicked = SIGNAL("clicked()")
        self.connect(self.button_connect, clicked, self._connect)
        self.connect(self.button_option, clicked, self._showOption)

        self.connect(self.list_conn,
                     SIGNAL("currentIndexChanged(int)"),
                     self._loadAccountsFromIdx)

        QShortcut(QKeySequence(Qt.Key_Up), self, self._onKeyUp)
        QShortcut(QKeySequence(Qt.Key_Down), self, self._onKeyDown)

        QShortcut(QKeySequence(Qt.Key_Enter), self, self._sendText)
        QShortcut(QKeySequence(Qt.Key_Return), self, self._sendText)

        QShortcut(QKeySequence(Qt.ALT + Qt.Key_Q), self, self.close)

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

        if event.type() == QEvent.KeyPress and self.connected and \
           event.key() not in (Qt.Key_Shift, Qt.Key_Control, Qt.Key_Meta,
                               Qt.Key_Alt):

            key_seq = self._getKeySeq(event)

            for m in self.macros:
                if m[1:] == key_seq:
                    self.s_core.write(messages.MSG, m[0])
                    self._appendEcho(m[0])
                    return True

        elif event.type() == QEvent.MouseButtonPress and \
            event.button() == Qt.LeftButton and self.viewer and \
            unicode(self.viewer.selectedText()):
            text = unicode(self.text_input.currentText())
            text += unicode(self.viewer.selectedText())
            self.text_input.setItemText(0, text)
            self.viewer.clearSelection()
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

        locale = str(QLocale.system().name())[:2]
        self._translators = {}
        files = glob(join(config['translation']['path'], '*_' + locale + '.qm'))
        for fn in files:
            self._translators[fn] = QtCore.QTranslator()
            self._translators[fn].load(fn)
            QApplication.installTranslator(self._translators[fn])

    def _translateText(self):
        self._text = {}
        execfile(join(config['devclient']['path'], 'gui.msg') , self._text)

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
        self.connect(opt, SIGNAL("reloadConnData(QString)"),
                     self._reloadConnData)
        self.connect(opt, SIGNAL("reloadPreferences()"),
                     self._reloadPreferences)
        opt.show()

    def _connect(self):
        s = Storage()
        connections = s.connections()
        if self.connected:
            if not self._displayQuestion(self._text['Connect'],
                                         self._text['CloseConn']):
                return

        if not connections:
            self.displayWarning(self._text['Connect'], self._text['NoConn'])
            return

        data = self.list_conn.itemData(self.list_conn.currentIndex())
        id_conn = data.toInt()[0]
        conn = [el for el in connections if el[0] == id_conn][0]
        self.account = AccountManager(self, getServer(*conn[2:4]), id_conn)
        msg = conn[1:4] + s.prompt(id_conn, self.account.user)
        self.s_core.write(messages.CONNECT, msg)

    def _reloadPreferences(self):
        self.preferences = Storage().preferences()
        self.game_logger = GameLogger(self.connected, self.preferences)

    def _reloadConnData(self, conn):
        """
        Reload all data rely on connection and propagate message of reloading.

        :Parameters:
          conn : str
            the name of connection
        """

        if not conn:
            self.list_conn.blockSignals(True)
            self.list_conn.clear()
            self._loadConnections()
            self.list_conn.blockSignals(False)

        if self.connected and self.connected == conn:
            s = Storage()
            self.macros = s.macros(self.connected)
            self.alias = Alias(self.connected)

            id_conn = s.getIdConnection(self.connected)
            prompt = [p for p in s.prompt(id_conn, self.account.user) if p]
            self.s_core.write(messages.CUSTOM_PROMPT, prompt)

    def _startConnection(self, host, port):
        s = Storage()
        id_conn = s.getIdConnection(self.connected)
        server = getServer(host, port)
        self.history.clear()
        self.alias = Alias(self.connected)
        custom_prompt = [p for p in s.prompt(id_conn, self.account.user) if p]
        self.viewer = getViewer(self, server, custom_prompt)
        self.macros = s.macros(self.connected)
        self.game_logger = GameLogger(self.connected, self.preferences)
        s.setOption(Option.DEFAULT_CONNECTION, id_conn)

        if self.account.user:
            commands = s.accountDetail(id_conn, self.account.user)

            for cmd in commands:
                self.s_core.write(messages.MSG, cmd)

    def _appendEcho(self, text):
        if not self.preferences or not self.preferences[0]:
            text = '<br>'
        else:
            text = '<span style="color:%s">%s</span><br>' % \
                (self.preferences[1], text)

        self.viewer.appendHtml(text)

    def _sendText(self):
        if not self.connected:
            self.displayWarning(PROJECT_NAME, self._text['NotConnected'])
            return

        text = unicode(self.text_input.currentText())
        if self.account.register(text):
            id_conn = Storage().getIdConnection(self.connected)
            self._loadAccounts(id_conn)

        self.s_core.write(messages.MSG, self.alias.check(text))
        self._appendEcho(text)
        self.history.add(text)
        self._manageLineInput(text)

    def _manageLineInput(self, text):
        hist = self.history.get()
        hist.reverse()
        self.text_input.clear()
        self.text_input.addItem('')
        self.text_input.addItems(hist)
        self.text_input.setCurrentIndex(0)
        if not self.preferences or not self.preferences[2]:
            text = ''
        self.text_input.setItemText(0, text)
        self.text_input.lineEdit().selectAll()

    def _readDataFromCore(self):

        while self.s_core.availableData():
            cmd, msg = self.s_core.read()
            if cmd == messages.MODEL:
                self.game_logger.write(msg)
                self.viewer.process(msg)
                self.update()
            elif cmd == messages.CONN_REFUSED:
                self.displayWarning(self._text['Connect'],
                                    self._text['ConnError'])
            elif cmd == messages.CONN_ESTABLISHED:
                self.connected = msg[0]
                self._startConnection(*msg[1:])
            elif cmd == messages.CONN_LOST:
                self.displayWarning(self._text['Connect'],
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
        box.setIcon(QMessageBox.Question)
        yes = box.addButton(self._text['Yes'], QMessageBox.AcceptRole)
        yes.setIcon(QIcon(":/images/button-yes.png"))
        no = box.addButton(self._text['No'], QMessageBox.RejectRole)
        no.setIcon(QIcon(":/images/button-no.png"))
        box.setDefaultButton(no)
        box.setEscapeButton(no)
        box.exec_()
        return box.clickedButton() == yes

    def displayWarning(self, title, message):
        QMessageBox.warning(self, title, message)

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
