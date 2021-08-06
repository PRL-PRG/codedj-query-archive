#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
dejumble: presents an organized view of the contents of a directory.

dejumblefs is a FUSE that lets the user mount a virtual directory with the
contents of the original directory, or the results of filtering the original
filesystem. Files can be presented to the user organized by filetype, access
time, metadata, etc. This search an be a shell script (find, locate, etc) or
many other backends as a xesam filter.
"""

classifiers = """\
Development Status :: 4 - Beta
Intended Audience :: Developers
License :: OSI Approved :: GNU General Public License (GPL)
Programming Language :: Python
Topic :: Filesystems
Topic :: Software Development :: Libraries :: Python Modules
Operating System :: Linux
Operating System :: MacOS :: MacOS X
Operating System :: POSIX
"""

from distutils.core import setup

doclines = __doc__.splitlines()

setup(name='dejumblefs',
      version = '0.8',
      packages = ['dejumblefs', 'dejumblefs.filters', 'dejumblefs.caches',
                  'dejumblefs.organizers'],
      package_dir = {'': ''},
      package_data = {'dejumblefs': ['conf/*.conf', 'conf/*.sql'],
                      'dejumblefs.testdata': ['*.*']},
      scripts = ['scripts/dejumble'],
      maintainer = 'César Izurieta',
      maintainer_email = 'cesar@caih.org',
      url = 'http://code.google.com/p/dejumble',
      license = 'http://www.gnu.org/copyleft/gpl.html',
      platforms = ['unix', 'linux', 'mac'],
      description = doclines[0],
      classifiers = filter(None, classifiers.splitlines()),
      long_description = '\n'.join(doclines[2:]),
)
