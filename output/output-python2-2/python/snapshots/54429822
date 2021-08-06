"""

add the parent of the root dir to pythonpath
configure settings
wrap django.core.handlers.wsgi

"""
import os
import sys

from django.core.handlers.wsgi import WSGIHandler


# Define a class that Aspen will instantiate.
# ===========================================
# This is for Aspen 0.6; in 0.7, you'll be able to import config directly from 
# aspen, so this would be simpler.

class Django(WSGIHandler):
    def __init__(self, config):
        sys.path.insert(0, config.paths.root)
        os.environ['DJANGO_SETTINGS_MODULE'] = 'hotclub.settings'
        WSGIHandler.__init__(self)
