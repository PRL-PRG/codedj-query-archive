from paste.deploy.converters import asbool, asint

class DemoApp(object):
    """
    Demo WSGI app
    """
    def __init__(self, foo, bar, baz, author, description):
        self.foo = foo
        self.bar = bar
        self.baz = baz
        self.author = author
        self.description = description

    def __call__(self, environ, start_response):
        start_response('200 Ok', [('Content-Type', 'text/plain')])
        params = ['\t%s (%s): %r\n' % (p, type(getattr(self, p)).__name__, getattr(self, p)) 
                  for p in ('foo', 'bar', 'baz')]
        env = ['\t%s ==> %s\n' % (i, environ[i]) for i in sorted(environ.keys())]
        return [
            ' ==== Demo app by %s ==== \n' % self.author,
            ' .:[ %s ]:.\n' % self.description,
            '\nHere the params:\n',
	    '-=-=-=-=-=-=-=-=-=-=-=-=-=-\n' ] + params + [
            '\nHere the environ:\n',
	    '-=-=-=-=-=-=-=-=-=-=-=-=-=-\n'] + env
    
def demo_app_factory(global_conf, **local_conf):
    """
    Example of PasteDeploy app factory using DemoApp as target WSGI application
    """
    conf = global_conf.copy()
    conf.update(local_conf)
    return DemoApp(
        conf.get('foo') or '--absent--',
        conf.get('bar') or '--absent--',
        asbool(conf.get('baz')),
        conf.get('author') or 'N/A',
        conf.get('description') or '--absent--')

def paste_demo_server_factory(global_conf, **local_conf):
    """
    Example of PasteDeploy server factory using paste's httpserver
    """
    conf = global_conf.copy()
    conf.update(local_conf)
    port = asint(conf.get('port') or 8080)
    host = conf.get('host') or '127.0.0.1'
    def server(app):
        from paste import httpserver
        httpserver.serve(app, host, port)
    return server

def wsgiref_demo_server_factory(global_conf, **local_conf):
    """
    Example of PasteDeploy server factory using wsgiref's simple_server
    """

    conf = global_conf.copy()
    conf.update(local_conf)
    port = asint(conf.get('port') or 8080)
    host = conf.get('host') or '127.0.0.1'
    def server(app):
        from wsgiref import simple_server
        httpd = simple_server.make_server(host, port, app)
        httpd.serve_forever()
    return server
