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

"""
The module to manage auto updating of client.
"""

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import sys
import tarfile
import sqlite3
from filecmp import cmp
from optparse import OptionParser
from socket import setdefaulttimeout
from ConfigParser import SafeConfigParser
from shutil import copyfile, rmtree, copymode
from urllib2 import urlopen, HTTPError, URLError
from os import chdir, walk, getcwd, makedirs, rename, sep
from os.path import basename, splitext, split, abspath
from os.path import exists, join, normpath, dirname

_SELF_MODULE = basename(sys.argv[0])
"""name of the module itself"""

_SELF_DIR = dirname(sys.argv[0])
"""directory of the module itself"""

sys.path.append(join(getcwd(), _SELF_DIR, '..'))
from devclient import __version__
"""the public version of client"""

_ROOT_DIR = abspath(join(getcwd(), _SELF_DIR, '../..'))
"""the root directory of client"""

_TMP_DIR = abspath(join(getcwd(), _SELF_DIR, 'temp'))
"""temp directory where store data for the process of updating"""

_CONFIG_FILE = join(abspath(_SELF_DIR), 'updater.cfg')
"""the configuration file"""

_UPDATE_DB_FILE = join(abspath(_SELF_DIR), 'updater.sql')
"""the file that contains the changes to apply at db"""

_DB_FILE = join(_ROOT_DIR, 'data/storage/db.sqlite')
"""the database file"""


class UpdaterError(Exception):
    """
    Base class for the exceptions of updater.
    """

    def __init__(self, msg):
        self.msg = msg

    def __str__(self):
        return self.msg

def newVersion(client_version):
    """
    Check existance of a new version to download.

    :Parameters:
      client_version : str
        the url of file that contains the version of client downloadable.
    """

    try:
        online_str = downloadFile(client_version).strip()
        online_version = map(int, online_str.split('.'))
    except UpdaterError:
        print 'Unknown online version, download new version'
        return True

    local_version = map(int, __version__.split('.'))
    print 'online version:', online_str, 'local version:', __version__
    return online_version > local_version

def downloadFile(url, timeout=2):
    """
    Download a file from url and return its data.

    :Parameters:
      url : str
        the url of file to download
      timeout : int
        timeout to wait before raising an error
    """

    setdefaulttimeout(timeout)
    try:
        u = urlopen(url)
    except HTTPError:
        raise UpdaterError('Unable to download file: %s' % url)
    except URLError:
        raise UpdaterError('Url malformed: %s' % url)
    except IOError:
        raise UpdaterError('Timeout on download file: %s' % url)

    return u.read()

def downloadClient(client_url, timeout=2):
    """
    Download the client from an url and save it in the filesystem.

    :Parameters:
      client_url : str
        the url of file to download
      timeout : int
        timeout to wait before raising error
    """

    data = downloadFile(client_url, timeout)
    filename = basename(client_url)
    fd = open(filename, 'wb+')
    fd.write(data)
    fd.close()

def uncompressClient(filename):
    """
    Extracting the client

    :Parameters:
      filename : str
        the name of the client
    """

    try:
        tar = tarfile.open(filename)
        name = normpath(tar.getnames()[0])
        base_dir = name[:name.find(sep)]
        tar.extractall()
        tar.close()
    except tarfile.ReadError:
        raise UpdaterError('Archive malformed')
    return base_dir

def replaceOldVersion(root_dir, base_dir, ignore_list):
    """
    Replace the old files of installed client with the new files.

    :Parameters:
        root_dir : str
          the root directory of the client installed
        base_dir : str
          the base directory of the new version of client
        ignore_list : list
          the list of files to be skipped
    """

    chdir(base_dir)
    for root, dirs, files in walk('.'):
        for f in files:
            source = normpath(join(root, f))
            if source in ignore_list:
                d, f = split(source)
                name, ext = splitext(f)
                dest = join(root_dir, d, name + '_ignore' + ext)
                if exists(dest) and cmp(source, dest):
                    continue
                print 'skip file: %s, save into %s' % (source, dest)
            else:
                dest = join(root_dir, source)
                if exists(dest) and cmp(source, dest):
                    continue

                # FIX: this check should be done from the root dir of client
                if basename(source) == _SELF_MODULE:
                    d, f = split(dest)
                    name, ext = splitext(f)
                    rename(dest, join(d, name + '_old' + ext))
                    print 'replace file: %s (old version backupped)' % source
                else:
                    print '%s file:' % ('add', 'replace')[exists(dest)], source

            if not exists(dirname(dest)):
                print 'create directory:', dirname(dest)
                makedirs(dirname(dest))
            copyfile(source, dest)
            copymode(source, dest)

def updateDatabase():
    local_version = map(int, __version__.split('.'))

    cp = SafeConfigParser()
    cp.read(_UPDATE_DB_FILE)
    statements = {}
    for s in cp.sections():
        ver = map(int, s.split('.'))
        if ver > local_version:
            statements[tuple(ver)] = sorted(cp.items(s))

    conn = sqlite3.connect(_DB_FILE, isolation_level=None)
    c = conn.cursor()

    for v, s in sorted(statements.items()):
        for label, sql in s:
            print 'Execute query:', sql
            c.execute(sql)

def updateClient():
    cp = SafeConfigParser()
    cp.read(_CONFIG_FILE)
    config = {}
    for s in cp.sections():
        config[s] = dict(cp.items(s))

    if not int(config['main']['update']):
        print 'Update disabled!'
        return 0

    retcode = 1
    ignore_list = map(normpath, config['files']['ignore'].split(','))
    if not exists(_TMP_DIR):
        makedirs(_TMP_DIR)

    chdir(_TMP_DIR)
    try:
        if newVersion(config['client']['version']):
            downloadClient(config['client']['url'])
            base_dir = uncompressClient(basename(config['client']['url']))
            replaceOldVersion(_ROOT_DIR, base_dir, ignore_list)
            updateDatabase()
    except UpdaterError, e:
        print 'ERROR:', e
    else:
        retcode = 0
        print 'Update successfully complete!'
    finally:
        chdir(_ROOT_DIR)  # change directory is required to remove the temp dir
        rmtree(_TMP_DIR)

    return retcode

if __name__ == '__main__':
    sys.exit(updateClient())
