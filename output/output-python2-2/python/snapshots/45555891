#!/usr/bin/python
"""
Example of paste.urlmap
"""
from paste import httpserver, urlmap

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

def foo_app(environ, start_response):
    """
    Foo app, shows some content only if 'foo' found in path, err 404 otherwise
    """
    if 'foo' in environ['PATH_INFO']:
        start_response('200 OK', [('Content-type', 'text/html')])
        content = ['<html><body><h1>Foo app</h1>'] + \
                  ['<p>We\'ve found <code>foo</code> in path</p>']
    else:
        start_response('404 Not found', [('Content-type', 'text/html')])
        content = ['<html><body><h1>404: Not found</h1>'] + \
                  ['<p>Foo not found in path</p>']
    return content

mapping = urlmap.URLMap()
# maps /foo to foo_app (i.e. /foo shows 404; /foo/foo shows some content
mapping['/foo'] = foo_app
# maps /bar to sample_app
mapping['/bar'] = sample_app

httpserver.serve(mapping, host="127.0.0.1", port=5000)
