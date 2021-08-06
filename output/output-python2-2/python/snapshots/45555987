#!/usr/bin/env python
# Example of usage twisted.web2 as WSGI-server
# It depends on twisted (http://twistedmatrix.com)
# Pythy <the.pythy@gmail.com>

from twisted.internet import reactor
from twisted.web2 import wsgi, channel, server

class TwistedWSGIServer(object):
    def __init__(self, app):
        self.app = app
        self.wsgi_res = wsgi.WSGIResource(app)
        self.site = server.Site(self.wsgi_res)
        self.factory = channel.HTTPFactory(self.site)

    def serve(self):
        reactor.listenTCP(8080, self.factory)
        reactor.run()

def runner(app):
    TwistedWSGIServer(app).serve()

if __name__ == '__main__':
    import sys
    import helper
    
    app = helper.get_arg(sys.argv, "Usage: twisted_wsgi_server.py package.wsgi.app")
    runner(app)
