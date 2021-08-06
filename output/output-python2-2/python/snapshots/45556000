#!/usr/bin/python
"""
Example of paste.debug.prints
"""
from paste import httpserver
from paste.debug import prints

def sample_app_prints(environ, start_response):
    """
    App that shows environment (with some debugging prints)
    """
    print "start response with 200 OK status"
    start_response('200 OK', [('Content-type', 'text/html')])
    print "retrieve environ keys"
    sorted_keys = environ.keys()
    print "sort keys"
    sorted_keys.sort()
    print "construct content"
    content = ['<html><body><h1>Trivial WSGI app in action</h1>'] + \
              ['<p>Sample WSGI application. Just show your environment.</p><p><ul>'] + \
              ['<li> %s : %s</li>' % (str(k), str(environ[k])) for k in sorted_keys] + \
              ['</ul></p></body></html>']
    print "return content"
    return content

app = prints.PrintDebugMiddleware(sample_app_prints)
httpserver.serve(app, host="127.0.0.1", port=5000)
