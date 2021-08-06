import string,time
import subprocess
import threading
from Sensor import Sensor
import Driver
from os import curdir, sep
import cherrypy

def Handler(environ, start_response):
    start_response('200 OK', [('Content-Type', 'text/plain')])
    path = environ['REQUEST_URI']
    try:
        bits = string.split(path, "/")
        if len(bits) > 4:
            id = bits[1]
            getset = bits[2]
            param = bits[3]
            value = bits[4]
            for f in Driver.Drivers:
                if f[0] == int(id):
                    if getset == "set":
                        f[1].setParameter(param, value)
                        yield "OK"
                        return
                    elif getset == "get":
                        params = f[1].getParameters()
                        for p in params:
                            if p[0] == param:
                                yield str(p[1])
                                return
        yield "Error"
        return
    except IOError:
         yield 'File Not Found'

class ServerThread(threading.Thread):
    def run(self):
        cherrypy.config.update({'server.socket_host': '0.0.0.0',
                        'server.socket_port': 8090,
                       })
        cherrypy.tree.graft(Handler, '/')
       
        cherrypy.engine.start()
        cherrypy.engine.block()


class HTTP_API(Sensor):
    def __init__(self):
        Sensor.__init__(self)
        ServerThread().start()
