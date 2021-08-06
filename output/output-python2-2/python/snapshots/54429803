"""This little shim wires Aspen up to serve Django.
"""
import os
import sys

from django.core.handlers.wsgi import WSGIHandler


os.environ['DJANGO_SETTINGS_MODULE'] = 'stephane.settings'


class Django(WSGIHandler):
    def __init__(self, config):
        sys.path.insert(0, config.paths.root)
        WSGIHandler.__init__(self)
