#!/usr/bin/python
#-*- coding: utf-8 -*-

import telnetlib

class Socket(object):
    """
    Provide an asynchronous interface to socket operation.
    """

    encoding = "ISO-8859-1"

    def __init__(self):
        self.connected = 0
        self.t = telnetlib.Telnet()

    def connect(self, host, port):
        self.t.open(host, port)
        self.connected = 1

    def read(self):
        """
        Read data from socket without blocking and return a unicode string.
        """

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

