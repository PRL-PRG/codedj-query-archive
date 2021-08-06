#!/usr/bin/python
#-*- coding: utf-8 -*-

import telnetlib

class Socket(object):
    """
    Provide an asynchronous interface to socket operation.
    """

    def __init__(self):
        self.connected = 0
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

    def __del__(self):
        self.disconnect()

