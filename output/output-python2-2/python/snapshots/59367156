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
import time
import socket
import unittest

sys.path.append('..')
sys.path.append('../configobj')
sys.path.append('../../resources')

import communication
from devclient.gui import SocketToCore


class GuiMock(object):

    def __init__(self):
        self._warning = None
        self._text = {}
        self._text['FatalError'] = ''

    def connect(self, widget, signal, callback):
        pass

    def _readDataFromCore(self):
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


if __name__ == '__main__':
    unittest.main()

