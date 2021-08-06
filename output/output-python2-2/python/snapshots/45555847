#!/usr/bin/env python
# tap.py: TwistedPythy application
# Pythy <the.pythy@gmail.com>

from twisted.application import internet, service
from twisted.python import usage

from TwistedPythy import proto, clients

class Options(usage.Options):

    optParameters = [
        ['listen-port', 'l', 3000, 'Port, listening to'],
        ['delay', 'd', 5, 'Delay for dummy client'],        
        ['encoding', 'e', 'utf-8', 'Transport encoding'],
        ]

def makeService(config):
    a = service.Application('TwistedPythy')
    client = clients.UnicodeDummyClient()
    client.pause_time = config['delay']
    factory = proto.AsyncUnicodePythyFactory(client, config['encoding'])
    tp_service = internet.TCPServer(config["listen-port"], factory)
    
    return tp_service
