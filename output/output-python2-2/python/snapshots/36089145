# -*- coding: iso-8859-1 -*-

##
# License: pydelicious is released under the bsd license. 
# See 'license.txt' for more informations.
#

dev = 0
if dev:
    raise Exception, \
          ("""
This does not work as intended right now.
Dont use it. Will be fixed.""")

from distutils.core import setup
import os
import pydelicious as delicious

setup(
    name='pydelicious',
    version=delicious.VERSION,
    author=delicious.AUTHOR,
    author_email=delicious.AUTHOR_EMAIL,
    url=delicious.PROJECT_URL,
    description=delicious.DESCRIPTION,
    long_description=delicious.LONG_DESCRIPTION,
    packages=['pydeliciouslibs', 'pydeliciouslibs.elementtree','pydeliciouslibs.feedparser'],
    py_modules=['pydelicious'],
    )

