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
from filecmp import cmp
from optparse import OptionParser
from socket import setdefaulttimeout
from shutil import copyfile, rmtree, copymode
from urllib2 import urlopen, HTTPError, URLError
from os import mkdir, chdir, walk, getcwd, makedirs, rename
from os.path import basename, splitext, split, abspath
from os.path import exists, join, normpath, dirname

sys.path.append(join(getcwd(), dirname(sys.argv[0]), '..'))
from devclient import __version__
"""the public version of client"""

_ROOT_DIR = abspath(join(getcwd(), dirname(sys.argv[0]), '../..'))
"""the root directory of client"""

_TMP_DIR = abspath(join(getcwd(), dirname(sys.argv[0]), 'temp'))
"""temp directory where store data for the process of updating"""

_SELF_MODULE = abspath(sys.argv[0])[len(_ROOT_DIR) + 1:]
"""path of the module itself"""


class UpdaterError(Exception):
    """
    Base class for the exceptions of updater.
    """

    def __init__(self, msg):
        self.msg = msg

    def __str__(self):
        return self.msg


def parseOption():

    parser = OptionParser()
    parser.add_option('-u', '--url', help='the url of client')
    parser.add_option('-t', '--timeout', type='int', default=2,
                      help='timeout to retrieve the file (default %default)')
    o, args = parser.parse_args()
    return o

def newVersion(client_version):
    """
    Check existance of a new version to download.

    :Parameters:
      client_version : str
        the url of file that contains the version of client downloadable.
    """

    try:
        online_version = map(int, downloadFile(client_version, 2).split('.'))
    except UpdaterError:
        print 'Unknown online version, download new version'
        return True

    local_version = map(int, __version__.split('.'))
    print 'online version:', online_version, 'local version:', local_version
    return online_version > local_version

def downloadFile(url, timeout):
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
        raise UpdaterError('Error on downloading file: %s' % url)
    except URLError:
        raise UpdaterError('Url format error: %s' % url)
    except IOError:
        raise UpdaterError('Timeout on download file: %s' % url)

    return u.read()

def downloadClient(client_url, timeout):
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

    tar = tarfile.open(filename)
    base_dir = normpath(tar.getnames()[0])
    tar.extractall()
    tar.close()
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
                dest = join(root_dir, d, 'ignore_ver.' + f)
                if exists(dest) and cmp(source, dest):
                    continue
                print 'skip file: %s, save into %s' % (source, dest)
            else:
                dest = join(root_dir, source)
                if exists(dest) and cmp(source, dest):
                    continue

                if source == _SELF_MODULE:
                    d, f = split(dest)
                    name, ext = splitext(f)
                    rename(dest, join(d, name + '_old' + ext))
                    print 'replace file: %s (old version backupped)' % source
                else:
                    print '%s file:' % ('add', 'replace')[exists(source)], \
                        source

            if not exists(dirname(dest)):
                print 'create directory:', dirname(dest)
                makedirs(dirname(dest))
            copyfile(source, dest)
            copymode(source, dest)

def updateClient():
    o = parseOption()
    # FIX: remove all the url from code
    base_url = "https://www.develer.com/~aleister/devclient/"
    client_url = join(base_url, "devclient.tar.bz2")
    client_version = join(base_url, "devclient.version")
    if o.url:
       client_url = o.url

    # files to be skipped, with path relative to root_dir
    ignore_list = []
    ignore_list = map(normpath, ignore_list)
    if not exists(_TMP_DIR):
        mkdir(_TMP_DIR)

    chdir(_TMP_DIR)
    try:
        if newVersion(client_version):
            downloadClient(client_url, o.timeout)
            base_dir = uncompressClient(basename(client_url))
            replaceOldVersion(_ROOT_DIR, base_dir, ignore_list)
    except UpdaterError, e:
        print e
    else:
        print 'Update successfully complete!'
    finally:
        chdir(_ROOT_DIR)  # change directory is required to remove the temp dir
        rmtree(_TMP_DIR)

if __name__ == '__main__':
    updateClient()