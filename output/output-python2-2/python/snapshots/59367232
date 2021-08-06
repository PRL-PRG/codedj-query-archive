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
import zlib
import time
import socket
import unittest
import subprocess
from random import randint

sys.path.append('..')

import communication
import devclient.exception as exception
from devclient.core import SocketToServer, SocketToGui
from devclient.core import IAC, DONT, DO, WONT, WILL, SB, SE, MCCP2, MCCP


def start_server_echo():
    port = randint(2000, 10000)
    p = subprocess.Popen(['python', '-u', 'server_echo.py',
                         '--port=%d' % port],
                         stdout=subprocess.PIPE)
    try:
        buf = p.stdout.read(6) # read READY\n from stdout
    except IOError:
        time.sleep(.5)
    return port


class TestSocketToServer(unittest.TestCase):

    def testNoConnection(self):
        s = SocketToServer()
        self.assert_(not s.connected)

    def testConnectionEstablished(self):
        port = start_server_echo()
        s = SocketToServer()
        s.connect("localhost", port)
        self.assert_(s.connected)
        s.write("quit")

    def testConnectionRefused(self):
        s = SocketToServer()
        self.assertRaises(exception.ConnectionRefused,
                          s.connect,
                          "localhost", 7890)
        self.assert_(not s.connected)

    def testReadError(self):
        port = start_server_echo()
        s = SocketToServer()
        s.connect("localhost", port)
        s.write("quit")
        time.sleep(.1)
        self.assertRaises(exception.ConnectionLost, s.read)
        self.assert_(s.connected)

    def testData(self):
        port = start_server_echo()
        s = SocketToServer()
        s.connect("localhost", port)
        s.write("hello")
        time.sleep(.1)
        self.assert_(s.read() == "hello\n")
        s.write('quit')

    def testDisconnect(self):
        port = start_server_echo()
        s = SocketToServer()
        s.connect("localhost", port)
        s.write('quit')
        s.disconnect()
        self.assert_(not s.connected)


class TestMCCP(unittest.TestCase):
    def start_connection(self, port):
        root = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        root.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        root.bind(("localhost", port))
        root.listen(1)
        client = SocketToServer()
        client.connect("localhost", port)
        server = root.accept()[0]
        return (root, server, client)

    def sendMccpV2Data(self, socket, data):
        socket.send(IAC + WILL + MCCP2)
        socket.send(IAC + SB + MCCP2 + IAC + SE)
        c = zlib.compressobj()
        to_send = c.compress(data)
        to_send += c.flush()
        socket.send(to_send)

    def testMccpV1(self):
        root, server, client = self.start_connection(randint(2000, 10000))
        server.send(IAC + WILL + MCCP)
        server.send(IAC + SB + MCCP + WILL + SE)
        c = zlib.compressobj()
        to_send = c.compress('hello')
        to_send += c.flush()
        server.send(to_send)
        self.assert_(client.read() == 'hello')

    def testMccpV2(self):
        root, server, client = self.start_connection(randint(2000, 10000))
        self.sendMccpV2Data(server, 'hello')
        self.assert_(client.read() == 'hello')

    def testMulti(self):
        root, server, client = self.start_connection(randint(2000, 10000))
        server.send('hello ')
        self.sendMccpV2Data(server, 'world')
        self.assert_(client.read() == 'hello world')

    def testMulti2(self):
        root, server, client = self.start_connection(randint(2000, 10000))
        self.sendMccpV2Data(server, 'hello ')
        server.send('world')
        self.assert_(client.read() == 'hello world')

    def testMulti3(self):
        root, server, client = self.start_connection(randint(2000, 10000))
        self.sendMccpV2Data(server, 'hello ')
        self.sendMccpV2Data(server, 'world')
        self.assert_(client.read() == 'hello world')


class TestSocketToGui(unittest.TestCase, communication.TestSocket):

    def startCommunication(self):
        port = randint(2000, 10000)
        s_gui = SocketToGui(port)
        s_mock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s_mock.connect(('localhost', port))
        s_gui.accept()
        return s_gui, s_mock


if __name__ == '__main__':
    unittest.main()
