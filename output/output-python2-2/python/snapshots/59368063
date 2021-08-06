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

import struct
import cPickle

import devclient.messages as messages


def read_data(socket):
    size = socket.recv(struct.calcsize("L"))
    size = struct.unpack('>l', size)[0]
    data = []
    while size > 0:
        data.append(socket.recv(min(4096, size)))
        size -= len(data[-1])
    return cPickle.loads(''.join(data))

def write_data(socket, data):
    buf = cPickle.dumps(data)
    socket.send(struct.pack('>l', len(buf)))
    socket.sendall(buf)


class TestSocket(object):

    def startCommunication(self):
        raise NotImplementedError

    def testWriteData(self):
        s_test, s_mock = self.startCommunication()
        s_test.write(messages.MODEL, 'hello')
        self.assert_((messages.MODEL, 'hello') == read_data(s_mock))

    def testReadData(self):
        s_test, s_mock = self.startCommunication()
        write_data(s_mock, (messages.END_APP, ''))
        self.assert_((messages.END_APP, '') == s_test.read())

    def testMalformedData(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send(struct.pack('>l', -10))
        self.assert_((messages.UNKNOWN, '') == s_test.read())

    def testMalformedData2(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send(struct.pack('>l', 5))
        s_mock.send('hello')
        self.assert_((messages.UNKNOWN, '') == s_test.read())

    def testMalformedData3(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send(struct.pack('>l', 10))
        s_mock.send('hello')
        self.assert_((messages.UNKNOWN, '') == s_test.read())

    def testMalformedData4(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send('hello')
        self.assert_((messages.UNKNOWN, '') == s_test.read())

    def testMalformedData5(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send('h')
        self.assert_((messages.UNKNOWN, '') == s_test.read())

    def testMalformedData6(self):
        s_test, s_mock = self.startCommunication()
        s_mock.send(struct.pack('>l', 5))
        s_mock.send('hello')
        self.assert_((messages.UNKNOWN, '') == s_test.read())
        write_data(s_mock, (messages.END_APP, ''))
        self.assert_((messages.END_APP, '') == s_test.read())

