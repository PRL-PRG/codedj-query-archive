#!/usr/bin/python
# Simple Nevow WSGI application
# It depends on Nevow (http://divmod.org/trac/wiki/DivmodNevow)
# Pythy <the.pythy@gmail.com>

from nevow import rend, loaders, wsgi, tags, inevow

class NevowPage(rend.Page):
    addSlash = True
    docFactory = loaders.stan(
        tags.html[
            tags.head[tags.title['Nevow WSGI hello app']],
            tags.body[
                tags.h1(id='title')['Nevow WSGI hello app'],
                tags.p(id='welcome')['Welcome to the Nevow (WSGI powered). Just show your environment.'],
                tags.p(id='environment')[tags.invisible(render=tags.directive('environ'))]
            ]
        ]
    )

    def render_environ(self, context, data):
        environ = inevow.IRequest(context).environ
        sorted_keys = environ.keys()
        sorted_keys.sort()
        inner = [tags.li[k, " =&gt; ", str(environ[k])] for k in sorted_keys]
        return tags.ul[inner]

app = wsgi.createWSGIApplication(NevowPage())

if __name__ == '__main__':
    import sys
    import helper
    
    server = helper.get_arg(sys.argv, "Usage: nevow_wsgi_app.py package.wsgi.server_callable")
    server(app)
