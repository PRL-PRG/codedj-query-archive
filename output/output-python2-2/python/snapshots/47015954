#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
cryo.py: A simple python DB persistence library.
"""

from distutils.core import setup
import os

try:
    # Just for development to be able to do sudo python setup.py develop
    import py2app
except ImportError:
    pass

_classifiers = """\
Development Status :: 4 - Beta
Intended Audience :: Developers
License :: OSI Approved :: GNU General Public License (GPL)
Programming Language :: Python
Topic :: Software Development :: Libraries :: Python Modules
"""

examplepath = os.path.join('docs', 'example.py')
doclines = __doc__.strip().splitlines()

setup(name='cryo',
      version='0.1',
      packages=['cryo'],
      data_files=[('docs', [examplepath])],
      maintainer='César Izurieta',
      maintainer_email='cesar@caih.org',
      url='http://code.google.com/p/cryo',
      license='http://www.gnu.org/copyleft/gpl.html',
      platforms=['unix', 'linux', 'mac', 'win'],
      description=doclines[0],
      classifiers=filter(None, _classifiers.splitlines()),
      long_description='\n'.join(doclines[2:])
)
