"""This little shim wires Aspen up to serve Django.
"""
import os
import sys

import aspen
from django.core.handlers.wsgi import WSGIHandler


os.environ['DJANGO_SETTINGS_MODULE'] = 'stephane.settings'
sys.path.insert(0, aspen.paths.root)


django = WSGIHandler()

