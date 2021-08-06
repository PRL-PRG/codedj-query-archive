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
import socket
import struct
import cPickle

import messages
import mud_type
import exception
from model import *
from alias import Alias
from mud_type import getMudType, ComponentFactory

class SocketToServer(object):
    """
    Provide a socket interface to Mud server.
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

    def getSocket(self):
        return self.t.get_socket()

    def read(self):
        """
        Read data from socket and return a unicode string.
        """

        try:
            return unicode(self.t.read_very_eager(), self.encoding)
        except (EOFError, socket.error), e:
            raise exception.ConnectionLost()

    def write(self, msg):
        self.t.write(msg.encode(self.encoding) + "\n")

    def disconnect(self):
        if self.connected:
            self.t.close()
            self.connected = 0

    def __del__(self):
        self.disconnect()


class SocketToGui(object):
    """
    Provide a socket interface to `Gui` part of client.
    """

    def __init__(self, port=7890):
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.s.bind(('localhost', port))
        self.s.listen(1)

    def getSocket(self):
        return self.s

    def accept(self):
        self.conn, addr = self.s.accept()
        return self.conn

    def read(self):
        """
        Read a message.

        :return: a tuple of the form (<message type>, <message>)
        """

        size = self.conn.recv(struct.calcsize("L"))
        try:
            size = struct.unpack('>l', size)[0]
        except struct.error:
            return (messages.UNKNOWN, '')

        if size < 0:
            return (messages.UNKNOWN, '')

        data = []
        while size > 0:
            data.append(self.conn.recv(min(4096, size)))
            size -= len(data[-1])

        return cPickle.loads(''.join(data))

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
        self.conn.send(struct.pack('>l', len(buf)))
        self.conn.send(buf)

class Core(object):
    """
    Main class for the core part of client.
    """

    def __init__(self, port):
        """
        Create the `Core` instance.
        """

        self.s_server = SocketToServer()
        """the interface with mud server, an instance of `SocketToServer`"""

        self.s_gui = SocketToGui(port)
        """the interface with gui part, an instance of `SocketToGui`"""

        self.alias = None
        """the `Alias` instance, used to replace alias with text to send"""

        self.parser = None
        """the `Parser` instance, used to parse data from server"""

        self.bg = None
        """the background color"""

        self.fg = None
        """the text color"""

    def _reloadConnData(self, conn):
        """
        Reload all data rely on connection.

        :Parameters:
          conn : str
            the name of connection
        """

        self.alias = Alias(conn)

    def _readDataFromGui(self, sock_watched):
        """
        Read data from `Gui`

        :Parameters:
          sock_watched : list
            the list of socket watched for events

        :return: a boolean equal to False when client must exit.
        """

        cmd, msg = self.s_gui.read()
        if cmd == messages.MSG and self.s_server.connected:
            self.s_server.write(self.alias.check(msg))
        elif cmd == messages.END_APP:
            self.s_server.disconnect()
            return False
        elif cmd == messages.RELOAD_CONN_DATA:
            self._reloadConnData(msg)
        elif cmd == messages.CONNECT:
            if self.s_server.connected:
                sock_watched.remove(self.s_server.getSocket())
                self.s_gui.write(messages.CONN_CLOSED, '')
                self.s_server.disconnect()
            try:
                self.s_server.connect(*msg[1:])
            except exception.ConnectionRefused:
                self.s_gui.write(messages.CONN_REFUSED, "")
            else:
                sock_watched.append(self.s_server.getSocket())
                self.s_gui.write(messages.CONN_ESTABLISHED, msg)

            mud = getMudType(*msg[1:])
            self.parser = ComponentFactory(mud).parser()
            self.alias = Alias(msg[0])
            self.bg = None
            self.fg = None

        return True

    def _readDataFromServer(self, sock_watched):
        try:
            data = self.s_server.read()
        except exception.ConnectionLost:
            sock_watched.remove(self.s_server.getSocket())
            self.s_gui.write(messages.CONN_LOST, '')
            self.s_server.disconnect()
        else:
            if data:
                model = self.parser.buildModel(data, self.bg, self.fg)
                if model.bg_color != self.bg:
                    self.bg = model.bg_color

                if model.fg_color != self.fg:
                    self.fg = model.fg_color

                self.s_gui.write(messages.MODEL, model)

    def mainLoop(self):
        """
        Realize the main loop of core.

        Manage `SocketToServer` input/output and take care of exchange messages
        with the `Gui` part via `SocketToGui` instance object.
        """

        sock_watched = [self.s_gui.getSocket()]

        while 1:
            try:
                r, w, e = select.select(sock_watched, [], [])
            except select.error, e:
                # FIX
                break
            except socket.error, e:
                # FIX
                break

            for s in r:
                if s == self.s_server.getSocket():
                    self._readDataFromServer(sock_watched)
                # connection request from Gui
                elif s == self.s_gui.getSocket():
                    sock_watched.append(self.s_gui.accept())
                elif not self._readDataFromGui(sock_watched):
                    break
