"""This little shim wires Aspen up to serve Django.
"""
import os
import sys

import aspen
from django.core.handlers.wsgi import WSGIHandler


DJANGO_SETTINGS_MODULE = 'stephane.settings' # change if you want


# Tell Django which settings to use.
# ==================================
# If you don't like the default project name above, I don't blame you. Change
# it, or set DJANGO_SETTINGS_MODULE explicitly in aspen's environment.

if not os.environ.has_key('DJANGO_SETTINGS_MODULE'):
    os.environ['DJANGO_SETTINGS_MODULE'] = DJANGO_SETTINGS_MODULE
sys.path.insert(0, aspen.paths.root)


# Instantiate Django's WSGI handler.
# ==================================
# This is then wired up to the site root in __/etc/apps.conf.

django = WSGIHandler()
