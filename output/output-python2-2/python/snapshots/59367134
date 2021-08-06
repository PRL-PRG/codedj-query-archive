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

import os
import sys
import time
import shutil
import socket
import unittest

from PyQt4.QtGui import QComboBox, QApplication, QLineEdit

sys.path.append('..')
sys.path.append('../configobj')
sys.path.append('../../resources')

import communication
import devclient.storage as storage
from devclient.conf import config
from devclient.gui import SocketToCore, AccountManager


class GuiMock(object):

    def __init__(self):
        self._warning = None
        self._text = {}
        self._text['FatalError'] = ''
        self.text_input = QComboBox()
        self.text_input.setEditable(True)
        self.list_account = QComboBox()

    def connect(self, widget, signal, callback):
        pass

    def displayWarning(self, title, message):
        self._warning = (title, message)


def fakeStartCore(self, cfg_file):
    self._server.listen()
    port = self._server.serverPort()
    self._client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    self._client.connect(("localhost", port))
    self._server.waitForNewConnection(500)
    self._s = self._server.nextPendingConnection()


def fakeDel(self):
    pass


class TestSocketToCore(unittest.TestCase, communication.TestSocket):
    def startCommunication(self):
        SocketToCore._startCore = fakeStartCore
        SocketToCore.__del__ = fakeDel
        s_core = SocketToCore(GuiMock(), '')
        return s_core, s_core._client


class ServerFake:
    cmd_password = 2


class TestAccountManager(unittest.TestCase):

    def __init__(self, methodName='runTest'):
        super(TestAccountManager, self).__init__(methodName)
        if not QApplication.instance():
            self.app = QApplication([])

        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        storage.loadStorage()

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)

    def emptyAccounts(self):
        conn = (0, 'name', 'host', 111)
        storage.setOption('save_account', 1)
        storage.addConnection(list(conn))
        account = AccountManager(GuiMock(), ServerFake, 1)
        self.assert_(storage.accounts(1) == [])

    def testEchoMode(self):
        conn = (0, 'name', 'host', 111)
        storage.addConnection(list(conn))
        mock = GuiMock()
        account = AccountManager(mock, ServerFake, 1)
        self.assert_(mock.text_input.lineEdit().echoMode() == QLineEdit.Normal)
        account.register("john")
        self.assert_(mock.text_input.lineEdit().echoMode() == QLineEdit.Password)
        account.register("johnpwd")
        self.assert_(mock.text_input.lineEdit().echoMode() == QLineEdit.Normal)

    def testNoSaveAccount(self):
        conn = (0, 'name', 'host', 111)
        storage.addConnection(list(conn))
        account = AccountManager(GuiMock(), ServerFake, 1)
        account.register("john")
        account.register("johnpwd")
        self.assert_(storage.accounts(1) == [])

    def testSaveAccount(self):
        conn = (0, 'name', 'host', 111)
        storage.setOption('save_account', 1)
        storage.addConnection(list(conn))
        account = AccountManager(GuiMock(), ServerFake, 1)
        account.register("john")
        account.register("johnpwd")
        self.assert_(storage.accounts(1) == ["john"])

    def testSaveAccount2(self):
        conn = (0, 'name', 'host', 111)
        storage.setOption('save_account', 1)
        storage.addConnection(list(conn))
        account = AccountManager(GuiMock(), ServerFake, 1)
        account.register("john")
        account.register("johnpwd")
        self.assert_(storage.accountDetail(1, "john") == ["john", "johnpwd"])

    def testDefaultAccount(self):
        conn = (0, 'name', 'host', 111)
        storage.setOption('save_account', 1)
        storage.addConnection(list(conn))
        self.assert_(storage.option('default_account', 1) == '')
        mock = GuiMock()
        mock.list_account.addItem('john')
        account = AccountManager(mock, ServerFake, 1)
        self.assert_(storage.option('default_account', 1) == 'john')


if __name__ == '__main__':
    unittest.main()

