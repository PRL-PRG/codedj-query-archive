#!/usr/bin/python
"""
Example of paste.wsgiwrappers
"""
from paste import httpserver, wsgiwrappers

def advanced_app(environ, start_response):
    """
    App that shows your preferred languages
    """
    request = wsgiwrappers.WSGIRequest(environ)
    params = request.params
    langs = request.languages
    
    response = wsgiwrappers.WSGIResponse("<html><body><h1>Advanced WSGI app in action</h1>")
    response.write("<p>Your preferred languages are: %s</p>" % langs)
    response.write("<p>Params of your query are: %s</p>" % params)
    response.write("</html></body>")
    
    return response(environ, start_response)

httpserver.serve(advanced_app, host="127.0.0.1", port=5000)
