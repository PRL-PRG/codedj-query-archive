#!/usr/bin/env python
# -*- coding: koi8-r -*- 
# twistedpythy_run.py: runner for TwistedPythy
# Pythy <the.pythy@gmail.com>

import sys

from twisted.internet import reactor
from twisted.python import log

from TwistedPythy import proto, clients

log.startLogging(sys.stderr)
client = clients.DummyClient()
client.pause_time = 20
factory = proto.AsyncPythyFactory(client, 'utf-8')
reactor.listenTCP(3000, factory)
reactor.run()
