#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import sys
import tarfile
from os import makedirs, chdir, getcwd, walk
from os.path import exists, abspath, dirname, join, normpath, splitext, isfile

_SELF_DIR = dirname(sys.argv[0])
"""directory of the module itself"""

_ROOT_DIR = abspath(join(getcwd(), _SELF_DIR, '../..'))
"""the root directory of project"""

sys.path.append(join(getcwd(), _SELF_DIR, '..'))
from devclient import __version__
"""the public version of client"""

_ADMITTED_EXT = ('py', 'ui', 'cfg', 'qrc', 'pro', 'qm', 'ts', 'GPL', 'msg')
"""the admitted extension"""

_PROJECT_NAME = 'devclient'
"""the project name"""

_DIST_DIR = abspath(join(_SELF_DIR, 'dist'))
"""the directory of distribution's file"""


def createVersionFile():
    """Create the file that contain the version of client"""

    fd = open(_PROJECT_NAME + '.version', 'w+')
    fd.write(__version__)
    fd.close()

def createArchive():
    """
    Create the archive (compressed with bzip2) that contain the client.
    """

    def is_admitted(name):
        if '.svn' in name:
            return False

        if isfile(name) and splitext(name)[1][1:] not in _ADMITTED_EXT:
            return False
        return True

    tar = tarfile.open(_PROJECT_NAME + '.tar.bz2', 'w:bz2')
    old_dir = getcwd()
    chdir(_ROOT_DIR)
    for root, dirs, files in walk('.'):
        for d in dirs:
            dirname = normpath(join(root, d))
            if is_admitted(dirname):
                tar.add(dirname, join(_PROJECT_NAME, dirname), False)
        for f in files:
            filename = normpath(join(root, f))
            if is_admitted(filename):
                tar.add(filename, join(_PROJECT_NAME, filename))
    tar.close()
    chdir(old_dir)

def createDist():
    if not exists(_DIST_DIR):
        makedirs(_DIST_DIR)

    chdir(_DIST_DIR)
    createVersionFile()
    createArchive()

if __name__ == '__main__':
    createDist()
