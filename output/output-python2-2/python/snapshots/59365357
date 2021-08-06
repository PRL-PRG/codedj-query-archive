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

__version__ = "$Revision:$"[11:-2]
__docformat__ = 'restructuredtext'

import sys
import time
import socket
import random
import unittest

from PyQt4.QtCore import QCoreApplication

sys.path.append('..')

import communication
from devclient.gui import SocketToCore


def eventWrap(classname):

    def add_event_process(func):
        if QCoreApplication.instance():
            qApp = QCoreApplication.instance()
        else:
            qApp = QCoreApplication([])

        def wrapper(*args, **kw):
            qApp.processEvents()
            try:
                return func(*args, **kw)
            finally:
                qApp.processEvents()

        return wrapper

    for n, f in vars(classname).iteritems():
        if not n.startswith('_') or n == '__init__':
            setattr(classname, n, add_event_process(getattr(classname, n)))


class GuiMock(object):

    def connect(self, widget, signal, callback):
        pass

    def _readDataFromCore(self):
        pass

    def _commError(self, error):
        pass


class TestSocketToCore(unittest.TestCase, communication.TestSocket):

    def startCommunication(self):
        port = random.randint(2000, 10000)
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind(("localhost", 7890))
        s.listen(1)
        s_core = SocketToCore(GuiMock())
        return s_core, s.accept()[0]


if __name__ == '__main__':
    if not socket.getdefaulttimeout():
        socket.setdefaulttimeout(1)
    eventWrap(SocketToCore)
    unittest.main()

