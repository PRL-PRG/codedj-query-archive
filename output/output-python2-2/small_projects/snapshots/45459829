#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
simpleevent: A simple event class for python

Use it as follows:
from simpleevent import Event


class TestClassWithEvent:

    def __init__(self):
        self.onsomeevent = Event()

    def someevent(self):
        self.onsomeevent("param")


def someeventhandler1(param):
    print("Handler 1 = %s" % param)


def someeventhandler2(param):
    print("Handler 2 = %s" % param)

test = TestClassWithEvent()
test.onsomeevent += someeventhandler1
test.onsomeevent += someeventhandler2
test.someevent()

"""

classifiers = """\
Development Status :: 4 - Beta
Intended Audience :: Developers
License :: OSI Approved :: GNU General Public License (GPL)
Programming Language :: Python
Topic :: Software Development :: Libraries :: Python Modules
"""

from distutils.core import setup

doclines = __doc__.splitlines()

setup(name='simpleevent',
      version = '0.1',
      py_modules = ['simpleevent'],
      maintainer = 'César Izurieta',
      maintainer_email = 'cesar@caih.org',
      url = 'http://code.google.com/p/simpleevent',
      license = 'http://www.gnu.org/copyleft/gpl.html',
      platforms = ['unix', 'linux', 'mac', 'win'],
      description = doclines[0],
      classifiers = filter(None, classifiers.splitlines()),
      long_description = '\n'.join(doclines[2:]),
)
