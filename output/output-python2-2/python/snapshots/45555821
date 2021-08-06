#/usr/bin/env python
# -*- coding: koi8-r -*-
# proto.py: Pythy's demo protocol
# Pythy <the.pythy@gmail.com>

from twisted.internet import protocol, threads
from twisted.protocols import basic
from twisted.python import log

class PythyProto(basic.LineReceiver):
    """Simple text demo protocol"""

    def connectionMade(self):
        log.msg("got connection from %s" % str(self.transport.client))

    def connectionLost(self, reason):
        log.msg("connection from %s lost" % str(self.transport.client))

    def lineReceived(self, line):
        log.msg("data received from %s: `%s'" % (str(self.transport.client), line))
        if line == '':
            # для тестовых целей
            self.sendLine("Good bye")
            self.transport.loseConnection()
            return
        agr = line[10:15]
        client = self.factory.clients.getClient(agr)
        self.sendAnswer(client)

    def sendAnswer(self, client):
        packet = "dummypacketmaker012345678%s" % client
        self.sendLine(packet)

    def sendLine(self, line):
        assert isinstance(line, basestring)
        line = line.replace('\r', '').replace('\n', '')
        log.msg("data send: %s" % line)
        line = line + "\r\n"
        self.transport.write(line)


class AsyncPythyProto(PythyProto):
    """Simple text demo protocol with async (deferred) method for fetching data"""

    def lineReceived(self, line):
        log.msg("data received from %s: `%s'" % (str(self.transport.client), line))
        if line == '':
            self.sendLine("Good bye")
            self.transport.loseConnection()
            return
        agr = line[10:15]
        deferred = threads.deferToThread(self.factory.clients.getClient, agr)
        deferred.addCallback(self._cbGetClient)

    def _cbGetClient(self, result):
        self.sendAnswer(result)

class AsyncUnicodePythyProto(AsyncPythyProto):
    """
    Simple text demo protocol with async method for fetching data
    and internal data in unicode 
    """

    def lineReceived(self, line):
        assert(line, str)
        log.msg("data received from %s: `%s'" % (str(self.transport.client), line))
        if line == '':
            self.sendLine(u"Good bye")
            self.transport.loseConnection()
            return
        # str -> unicode
        line = line.decode(self.factory.encoding)
        agr = line[10:15]
        deferred = threads.deferToThread(self.factory.clients.getClient, agr)
        deferred.addCallback(self._cbGetClient)

    def sendAnswer(self, client):
        assert(client, unicode)
        packet = u'\u0442\u0435\u0441\u0442' + client
        self.sendLine(packet)

    def sendLine(self, line):
        assert isinstance(line, unicode)
        line = line.replace('\r', '').replace('\n', '')
        # unicode -> str
        line = line.encode(self.factory.encoding)
        log.msg("data send: %s" % line)
        line = line + "\r\n"
        self.transport.write(line)
    

class PythyFactory(protocol.ServerFactory):
    protocol = PythyProto   # class, not instance!

    def __init__(self, clients, transport_encoding):
        self.clients = clients
        self.encoding = transport_encoding

class AsyncPythyFactory(PythyFactory):
    protocol = AsyncPythyProto

class AsyncUnicodePythyFactory(AsyncPythyFactory):
    protocol = AsyncUnicodePythyProto
