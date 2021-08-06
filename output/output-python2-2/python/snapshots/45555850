#!/usr/bin/python
"""
Example of paste.cgitb_cacher middleware
"""
from paste import httpserver, cgitb_catcher

def erroneous_app(environ, start_response):
    """
    App that raises exception on /error path
    """
    if environ['PATH_INFO'] == '/error':
        raise Exception("Some error occured here")
    
    start_response('200 OK', [('Content-Type', 'text/html')])
    return ['<html><body><h1>Erroreous application</h1>'] + \
           ['<p>Go to <a href="/error">/error</a> to see exception</p>'] + \
           ['</body></html>']

app = cgitb_catcher.CgitbMiddleware(erroneous_app, display=True)
httpserver.serve(app, host="127.0.0.1", port=5000)
