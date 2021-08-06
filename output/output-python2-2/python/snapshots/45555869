#!/usr/bin/env python
# Trivial WSGI server, based on wsgiref's simpleserver
# it depends on wsgiref (http://cheeseshop.python.org/pypi/wsgiref)
# Pythy <the.pythy@gmail.com>

from wsgiref import simple_server, validate

class TrivialWSGIServer(object):
    def __init__(self, app):
        self.app = app
        self.server = simple_server.WSGIServer(
            ('', 8080),
            simple_server.WSGIRequestHandler,
        )
        self.server.set_app(validate.validator(self.app))

    def serve(self):
        self.server.serve_forever()

def runner(app):
    TrivialWSGIServer(app).serve()

if __name__ == '__main__':
    import sys
    import helper
    
    app = helper.get_arg(sys.argv, "Usage: trivial_wsgi_server.py package.wsgi.app")
    runner(app)

