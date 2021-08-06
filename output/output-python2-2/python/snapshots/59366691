#!/usr/bin/python
#-*- coding: utf-8 -*-

import telnetlib

from socket_abstract import Socket

class TSocket(Socket):
    def __init__(self):
        Socket.__init__(self)
        self.t = telnetlib.Telnet()

    def connect(self, host, port):
        self.t.open(host, port)
        self.connected = 1

    def read(self):
        try:
            return self.t.read_very_eager()
        except EOFError:
           self.disconnect()
           return ''

    def write(self, msg):
        self.t.write(msg + "\n")

    def disconnect(self):
        if self.connected:
            self.t.close()
            self.connected = 0


