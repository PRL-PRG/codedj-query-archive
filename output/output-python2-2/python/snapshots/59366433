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

import telnetlib
import select

import exception

class Socket(object):
    """
    Provide an asynchronous interface to socket operation.
    """

    encoding = "ISO-8859-1"

    def __init__(self):
        self.connected = 0
        self.t = telnetlib.Telnet()

    def connect(self, host, port):
        try:
            self.t.open(host, port)
        except:
            raise exception.ConnectionRefused()
        self.connected = 1

    def read(self):
        """
        Read data from socket (wait a maximum of 0.1s) and return a unicode
        string.
        """

        # As there is only one socket it is possible to use telnetlib function
        # instead socket function. This allow to avoid parsing of IAC and Co..
        select.select([self.t.get_socket()], [], [], .1)

        try:
            return unicode(self.t.read_very_eager(), self.encoding)
        except EOFError:
            self.disconnect()
            return unicode('')

    def write(self, msg):
        self.t.write(msg.encode(self.encoding) + "\n")

    def disconnect(self):
        if self.connected:
            self.t.close()
            self.connected = 0

    def __del__(self):
        self.disconnect()

