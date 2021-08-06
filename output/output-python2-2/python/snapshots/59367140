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
from PyQt4.QtCore import QEvent, Qt, QLocale, QVariant, QObject
from PyQt4.QtCore import SIGNAL, PYQT_VERSION_STR, QT_VERSION_STR
from PyQt4.QtGui import QApplication, QIcon, QLineEdit
from PyQt4.QtGui import QMessageBox, QShortcut, QKeySequence
from PyQt4.QtNetwork import QHostAddress, QTcpSocket, QTcpServer, QAbstractSocket

import storage
import messages
import exception
import gui_option
from conf import config
from alias import Alias
from history import History
from viewer import getViewer
from servers import getServer
from utils import terminateProcess, startProcess
from gui_src.gui import Ui_dev_client
from constants import PUBLIC_VERSION, PROJECT_NAME

logger = logging.getLogger('gui')


class SocketToCore(QObject):
    """
    Provide a socket interface used to exchange message with `Core`.
    """

    def __init__(self, widget, cfg_file, timeout=.2):
        """
        Create the `SocketToCore` instance.

        :Parameters:
          widget : QWidget
            the parent widget, used to display messages
          cfg_file : str
            the path of configuration file
          timeout : int
            the timeout of socket operations (in seconds)
        """

        QObject.__init__(self)
        self._w = widget
        self._timeout = timeout * 1000
        self._server = QTcpServer()
        self._startCore(cfg_file)
        self._setupSignal()

    def _startCore(self, cfg_file):
        self._server.listen()
        port = self._server.serverPort()

        p = startProcess(['python',
                        join(config['devclient']['path'], 'core.py'),
                        '--config=%s' % cfg_file,
                        '--port=%d' % port])
        self._pid = p.pid

        if not self._server.waitForNewConnection(500)[0]:
            logger.error('SocketToCore: ' + self._server.errorString())
            self._w.displayWarning(PROJECT_NAME, self._w._text['FatalError'])
            raise exception.IPCError()
        self._s = self._server.nextPendingConnection()

    def _setupSignal(self):
        self.connect(self._s, SIGNAL("readyRead()"), SIGNAL("readyRead()"))
        self.connect(self._s, SIGNAL("error(QAbstractSocket::SocketError)"),
                     self._commError)

    def _commError(self, error):
        # Timeout error is managed by _readData
        if self._s.error() != QAbstractSocket.SocketTimeoutError:
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
        self._s.close()

    def __del__(self):
        terminateProcess(self._pid)


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
        self._w = widget
        self.user = unicode(widget.list_account.currentText())
        storage.setOption('default_account', self.user, id_conn)
        self._save_account = storage.option('save_account')
        self._cmd_pwd = server.cmd_password
        self._id_conn = id_conn
        self.cmd_counter = 0
        self._commands = []

    def register(self, text):
        self.cmd_counter += 1

        if self.cmd_counter == self._cmd_pwd - 1:
            echo_mode = QLineEdit.Password
        else:
            echo_mode = QLineEdit.Normal

        self._w.text_input.lineEdit().setEchoMode(echo_mode)

        if not self.user and self._save_account \
           and self.cmd_counter <= self._cmd_pwd:
            self._commands.append(text)
            if self.cmd_counter == self._cmd_pwd:
                storage.saveAccount(self._commands, self._id_conn,
                                    self._cmd_pwd - 1)
                return True
        return False


class ConnectionManager(QObject):
    def __init__(self, widget, cfg_file):

        QObject.__init__(self)
        self.conn_name = None
        """the name of server connected or None"""

        self._w = widget

        self._s_core = SocketToCore(widget, cfg_file)
        """the interface with `Core`, an instance of `SocketToCore`"""

        self._viewer = None
        """The instance of `Viewer` used to show data arrived from `Core`"""

        self._history = History()

        self._game_logger = None

        self._account = None

        self._preferences = storage.preferences()
        self.connect(self._s_core, SIGNAL("readyRead()"),
                     self._readDataFromCore)

    def _checkModifier(self, event, mod):
        """
        Check keyboard's modifier.
        """

        return int((event.modifiers() & mod) == mod)

    def _getKeySeq(self, event):
        """
        Given a keyboard event, return a tuple of its components.

        :Parameters:
          event : QKeyEvent
            the keyboard event

        :return: a tuple of the form (shift, alt, ctrl, keycode)
        """

        s = self._checkModifier(event, Qt.ShiftModifier)
        a = self._checkModifier(event, Qt.AltModifier)
        c = self._checkModifier(event, Qt.ControlModifier)
        return (s, a, c, event.key())

    def eventFilter(self, event):
        if event.type() == QEvent.KeyPress and self.conn_name and \
           event.key() not in (Qt.Key_Shift, Qt.Key_Control, Qt.Key_Meta,
                               Qt.Key_Alt):

            key_seq = self._getKeySeq(event)
            for m in self._macros:
                if m[1:] == key_seq:
                    self._s_core.write(messages.MSG, m[0])
                    self._appendEcho(m[0])
                    return True

            # Ctrl-C is used to copy selected text of text_output to clipboard
            if self._checkModifier(event, Qt.ControlModifier) and \
               not self._checkModifier(event, Qt.ShiftModifier) and \
               not self._checkModifier(event, Qt.AltModifier) and \
               event.key() == Qt.Key_C:
                self._viewer.copySelectedText()
                return True

        return False

    def startConnection(self, id_conn):
        conn = [el for el in storage.connections() if el[0] == id_conn][0]

        # AccountManager is built here to get the custom prompt from the user
        self._account = AccountManager(self._w, getServer(*conn[2:4]), id_conn)
        msg = conn[1:4] + storage.prompt(conn[0], self._account.user)
        self._s_core.write(messages.CONNECT, msg)

    def reloadPreferences(self):
        self._preferences = storage.preferences()
        self._game_logger = GameLogger(self.conn_name, self._preferences)

    def reloadConnData(self, conn_name):
        if self.conn_name and self.conn_name == conn_name:
            self._macros = storage.macros(conn_name)
            self._alias = Alias(conn_name)

            c = storage.connection(conn_name)
            prompt = [p for p in storage.prompt(c[0], self._account.user) if p]
            self._s_core.write(messages.CUSTOM_PROMPT, prompt)

    def _connEstablished(self, conn_name):
        self.conn_name = conn_name
        conn = storage.connection(conn_name)
        server = getServer(*conn[2:4])
        self._history.clear()
        self._alias = Alias(conn_name)
        custom_prompt = [p for p in storage.prompt(conn[0], self._account.user)
                         if p]
        self._viewer = getViewer(self._w, server, custom_prompt)
        self._macros = storage.macros(conn_name)
        self._game_logger = GameLogger(conn_name, self._preferences)
        storage.setOption('default_connection', conn[0])

        if self._account.user:
            commands = storage.accountDetail(conn[0], self._account.user)

            for cmd in commands:
                self._account.cmd_counter += 1
                self._s_core.write(messages.MSG, cmd)

    def disconnect(self):
        self.conn_name = None
        self._s_core.write(messages.END_APP, "")
        self._s_core.disconnect()

    def _appendEcho(self, text):
        if not self._preferences[0]:
            text = '<br>'
        else:
            text = '<span style="color:%s">%s</span><br>' % \
                (self._preferences[1], text)

        self._viewer.appendHtml(text)

    def sendText(self, text):
        self._s_core.write(messages.MSG, self._alias.check(text))
        self._appendEcho(text)
        self._history.add(text)

        return self._account.register(text)

    def history(self):
        return self._history.get()

    def historyPrev(self):
        return self._history.getPrev()

    def historyNext(self):
        return self._history.getNext()

    def _readDataFromCore(self):
        while self._s_core.availableData():
            cmd, msg = self._s_core.read()
            if cmd == messages.MODEL:
                self._game_logger.write(msg)
                self._viewer.process(msg)
                self._w.update()
            elif cmd == messages.CONN_REFUSED:
                self._w.displayWarning(self._w._text['Connect'],
                                       self._w._text['ConnError'])
            elif cmd == messages.CONN_ESTABLISHED:
                self._connEstablished(msg[0])
            elif cmd == messages.CONN_LOST:
                self._w.displayWarning(self._w._text['Connect'],
                                       self._w._text['ConnLost'])
                self.conn_name = None
            elif cmd == messages.CONN_CLOSED:
                self.conn_name = None
            elif cmd == messages.UNKNOWN:
                logger.warning('SocketToCore: Unknown message')


class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with `Qt`_, that inherits the real gui interface
    designed by `Qt-designer`_.

.. _Qt: http://doc.trolltech.com/4.3/index.html
.. _Qt-designer: http://doc.trolltech.com/4.3/designer-manual.html
    """

    def __init__(self, cfg_file):

        if QApplication.instance():
            self.app = QApplication.instance()
        else:
            self.app = QApplication([])

        self.app.setStyle(QtGui.QStyleFactory.create("Cleanlooks"))
        self._installTranslator()
        QtGui.QMainWindow.__init__(self)
        self.setupLogger()
        self._translateText()
        self.setupUi(self)
        self._conn_manager = ConnectionManager(self, cfg_file)
        self._setEventFilter()

        logger.debug('PyQt version: %s, Qt version: %s' %
            (PYQT_VERSION_STR, QT_VERSION_STR))

        self._loadConnections()
        self._setupSignal()

    def _loadConnections(self):
        connections = storage.connections()
        def_conn = storage.option('default_connection')
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
        def_account = storage.option('default_account', id_conn)
        selected = 0
        for i, a in enumerate(storage.accounts(id_conn)):
            self.list_account.addItem(a)
            if a == def_account:
                selected = i + 1
        self.list_account.setCurrentIndex(selected)

    def _setEventFilter(self):
        self.text_input.installEventFilter(self)
        self.text_output.installEventFilter(self)
        self.text_input.lineEdit().installEventFilter(self)
        self.text_output_noscroll.installEventFilter(self)

    def setupUi(self, w):
        Ui_dev_client.setupUi(self, w)
        self.setWindowTitle(PROJECT_NAME + ' ' + PUBLIC_VERSION)
        self.text_input.setCompleter(None)
        self.text_output_noscroll.setVisible(False)

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
        self.connect(self.toggle_splitter, clicked, self._toggleSplitter)

        self.connect(self.list_conn,
                     SIGNAL("currentIndexChanged(int)"),
                     self._loadAccountsFromIdx)

        QShortcut(QKeySequence(Qt.Key_Up), self, self._onKeyUp)
        QShortcut(QKeySequence(Qt.Key_Down), self, self._onKeyDown)

        QShortcut(QKeySequence(Qt.Key_Enter), self, self._sendText)
        QShortcut(QKeySequence(Qt.Key_Return), self, self._sendText)

        QShortcut(QKeySequence(Qt.ALT + Qt.Key_Q), self, self.close)

    def _toggleSplitter(self):
        no_scroll = self.text_output_noscroll
        no_scroll.setVisible(not no_scroll.isVisible())

    def eventFilter(self, target, event):
        return self._conn_manager.eventFilter(event)

    def _onKeyUp(self):
        self.text_input.setCurrentIndex(0)
        self.text_input.setItemText(0, self._conn_manager.historyPrev())

    def _onKeyDown(self):
        self.text_input.setCurrentIndex(0)
        self.text_input.setItemText(0, self._conn_manager.historyNext())

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
        if self._conn_manager.conn_name:
            if not self._displayQuestion(PROJECT_NAME,
                                         self._text['CloseConfirm']):
                event.ignore()
                return

        self._conn_manager.disconnect()
        event.accept()

    def _showOption(self):
        opt = gui_option.GuiOption(self)
        self.connect(opt, SIGNAL("reloadConnData(QString)"),
                     self._reloadConnData)
        self.connect(opt, SIGNAL("reloadPreferences()"),
                     self._conn_manager.reloadPreferences)
        opt.show()

    def _connect(self):
        connections = storage.connections()
        if self._conn_manager.conn_name:
            if not self._displayQuestion(self._text['Connect'],
                                         self._text['CloseConn']):
                return

        if not connections:
            self.displayWarning(self._text['Connect'], self._text['NoConn'])
            return

        data = self.list_conn.itemData(self.list_conn.currentIndex())
        self._conn_manager.startConnection(data.toInt()[0])

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

        self._conn_manager.reloadConnData(unicode(conn))

    def _sendText(self):
        if not self._conn_manager.conn_name:
            self.displayWarning(PROJECT_NAME, self._text['NotConnected'])
            return

        text = unicode(self.text_input.currentText())
        if self._conn_manager.sendText(text):
            conn = storage.connection(self._conn_manager.conn_name)
            self._loadAccounts(conn[0])
        self._manageLineInput(text)

    def _manageLineInput(self, text):
        hist = self._conn_manager.history()
        hist.reverse()
        self.text_input.clear()
        self.text_input.addItem('')
        self.text_input.addItems(hist)
        self.text_input.setCurrentIndex(0)
        if not storage.preferences()[2]:
            text = ''
        self.text_input.setItemText(0, text)
        self.text_input.lineEdit().selectAll()

    def _displayQuestion(self, title, message):
        b = QMessageBox.question(self, title, message,
                                 QMessageBox.Yes, QMessageBox.No)
        return b == QMessageBox.Yes

    def displayWarning(self, title, message):
        QMessageBox.warning(self, title, message)

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
