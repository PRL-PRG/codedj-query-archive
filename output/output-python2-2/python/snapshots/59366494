#!/usr/bin/python
#-*- coding: utf-8 -*-

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

