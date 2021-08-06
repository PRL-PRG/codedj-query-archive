#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
simpleevent.py: A simple event class for python
"""

classifiers = """\
Development Status :: 4 - Beta
Intended Audience :: Developers
License :: OSI Approved :: GNU General Public License (GPL)
Programming Language :: Python
Topic :: Software Development :: Libraries :: Python Modules
"""

from distutils.core import setup

doclines = __doc__.strip().splitlines()
example = open("example.py")
doclines.extend(['\n', '\n'])
doclines.extend(example.readlines())
example.close()

setup(name='simpleevent',
      version = '0.1',
      py_modules = ['simpleevent'],
      maintainer = u'César Izurieta',
      maintainer_email = 'cesar@caih.org',
      url = 'http://code.google.com/p/simpleevent',
      license = 'http://www.gnu.org/copyleft/gpl.html',
      platforms = ['unix', 'linux', 'mac', 'win'],
      description = doclines[0],
      classifiers = filter(None, classifiers.splitlines()),
      long_description = '\n'.join(doclines[2:]),
)
