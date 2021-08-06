#!/usr/bin/python
"""
Example of paste.debug.profile
"""
from paste import httpserver
from paste.debug import profile

def sample_app(environ, start_response):
    """
    App that shows environment
    """
    start_response('200 OK', [('Content-type', 'text/html')])
    sorted_keys = environ.keys()
    sorted_keys.sort()
    content = ['<html><body><h1>Trivial WSGI app in action</h1>'] + \
              ['<p>Sample WSGI application. Just show your environment.</p><p><ul>'] + \
              ['<li> %s : %s</li>' % (str(k), str(environ[k])) for k in sorted_keys] + \
              ['</ul></p></body></html>']
    return content

app = profile.ProfileMiddleware(sample_app, None)
httpserver.serve(app, host="127.0.0.1", port=5000)
