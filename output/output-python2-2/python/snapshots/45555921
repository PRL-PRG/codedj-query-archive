#!/usr/bin/python
"""
Example of paste.errordocument
"""
from paste import httpserver, urlmap, errordocument

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

def err404_app(environ, start_response):
    """
    Simple WSGI app that shows message '404 error'.
    Notice that this app must return '200 OK' response
    """
    start_response('200 OK', [('Content-type', 'text/html')])
    return ["<html><body><h1>Err 404: Object not found in %s</h1></body></html>" % environ['paste.recursive.old_path_info']]

mapping = urlmap.URLMap()
mapping['/foo'] = foo_app
mapping['/err404'] = err404_app

# handle 404 errors by /err404
error_handled = errordocument.forward(mapping, codes={404:'/err404'})

httpserver.serve(error_handled, host="127.0.0.1", port=5000)
