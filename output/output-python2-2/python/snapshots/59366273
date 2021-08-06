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

import copy
import Queue

import exception
import event_type
import mud_type
from socket import Socket
from alias import Alias
from mud_type import getMudType, ComponentFactory

class Application(object):
    """
    Main class for the application part of client.
    """

    def __init__(self, q_app_gui, q_gui_app):
        """
        Create the `Application` instance.

        :Parameters:
          q_app_gui : Queue
            a Queue used to send message from `Application` to `Gui`
          q_gui_app : Queue
            a Queue used to send message from `Gui` to `Application`
        """

        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.sock = Socket()
        self.alias = None
        self.parser = None

    def _reloadConnData(self, conn):
        """
        Reload all data rely on connection and propagate message of reloading.

        :Parameters:
          conn : str
            the name of connection
        """

        self.alias = Alias(conn)

    def mainLoop(self):
        """
        Realize the main loop of application.

        Manage `Socket` input/output and take care of exchange messages with
        the `Gui` part.
        """

        while 1:

            if self.sock.connected:
                data = self.sock.read()
                if not self.sock.connected:
                    self.q_app_gui.put((event_type.CONN_CLOSED, ""))
                elif data:
                    self.parser.parse(data)
                    self.q_app_gui.put((event_type.MODEL,
                                        copy.deepcopy(self.parser.model)))
            try:
                cmd, msg = self.q_gui_app.get(0)
                if cmd == event_type.MSG and self.sock.connected:
                    self.sock.write(self.alias.check(msg))
                elif cmd == event_type.END_APP:
                    self.sock.disconnect()
                    return
                elif cmd == event_type.RELOAD_CONN_DATA:
                    self._reloadConnData(msg)
                elif cmd == event_type.CONNECT:
                    if self.sock.connected:
                        self.sock.disconnect()

                    try:
                        self.sock.connect(*msg[1:])
                    except exception.ConnectionRefused:
                        self.q_app_gui.put((event_type.CONN_REFUSED, ""))
                    else:
                        self.q_app_gui.put((event_type.CONN_ESTABLISHED, msg))

                    mud = getMudType(*msg[1:])
                    self.parser = ComponentFactory(mud).parser()
                    self.alias = Alias(msg[0])

            except Queue.Empty:
                pass
