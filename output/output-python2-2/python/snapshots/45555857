#!/usr/bin/python
"""
Example of paste.fixture
"""
from paste import fixture, lint
import unittest

def testing_app(environ, start_response):
    """
    Simple WSGI-app, which we will test
    """
    start_response('200 OK', [('Content-Type', 'text/plain'),
                              ('X-Powered-By', 'Python Paste')])
    return ["Paste is a collection of useful tools for WSGI"]

class WSGIAppTestCase(unittest.TestCase):
    """
    Example of paste.fixture powered unit-tests
    """
    def setUp(self):
        self.test_app = fixture.TestApp(lint.middleware(testing_app))
    
    def test_headers(self):
        response = self.test_app.get('/path/to/url')
        self.assertEquals(response.header('X-Powered-By'), 'Python Paste')
    
    def test_content(self):
        response = self.test_app.get('/path/to/url')
        # similar to response.mustcontain('WSGI'), but more pythonic
        self.assert_('WSGI' in response)


unittest.main()