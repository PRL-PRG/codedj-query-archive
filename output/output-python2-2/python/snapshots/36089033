#!/usr/bin/env python
#
# This file is in the public domain.
import sys
from distutils.core import setup, Command
import pydelicious


class Test(Command):
    description = 'Run pydelicious API tests.'
    user_options = []

    def initialize_options(self): pass

    def finalize_options(self): pass

    def run(self):
        from tests import main
        main.test_api()


# TODO: setuptools is said to do dependencies but that needs to be figured out further
#from setuptools import setup
#requires = ['feedparser']
#if sys.version[:2] == (2, 4):
#    requires += ["elementtree >= 1.2"]
#
#elif sys.version[:2] == (2, 6):
#    pass # integrated into the standard library as xml.etree.

# TODO: need to see this work...
dependency_links = [
    "http://feedparser.org/feedparser.py#egg=feedparser-latest"
]


### distutils setup

setup(
    cmdclass = {
        'test': Test,
    },
    name = 'pydelicious',
    version = pydelicious.__version__,
    license = 'BSD',
    description = pydelicious.__description__,
    long_description = pydelicious.__long_description__,

    author = pydelicious.__author__,
    author_email = pydelicious.__author_email__,
    url = pydelicious.__url__,

    py_modules = [ 'pydelicious' ],
#    requires = requires
)
