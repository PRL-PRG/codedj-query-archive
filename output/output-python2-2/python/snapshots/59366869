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

import tarfile
from optparse import OptionParser
from socket import setdefaulttimeout
from shutil import copyfile, rmtree, copymode
from urllib2 import urlopen, HTTPError, URLError
from os import mkdir, chdir, walk, getcwd, makedirs
from os.path import basename, join, exists, abspath, normpath, dirname


class UpdaterError(Exception):

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

def newVersion(client_url):
    # TODO
    return True

def downloadFile(client_url, timeout):

    setdefaulttimeout(timeout)
    try:
        u = urlopen(client_url)
    except HTTPError:
        raise UpdaterError('Error on downloading file: ' + client_url)
    except URLError:
        raise UpdaterError('Url format error: ' + client_url)
    except IOError:
        raise UpdaterError('Timeout on download file: ' + client_url)

    filename = basename(client_url)
    fd = open(filename, 'wb+')
    fd.write(u.read())
    fd.close()

def uncompressClient(filename):
    tar = tarfile.open(filename)
    base_dir = normpath(tar.getnames()[0])
    tar.extractall()
    tar.close()
    return base_dir

def replaceOldVersion(root_dir, base_dir, ignore_list):

    chdir(base_dir)
    for root, dirs, files in walk('.'):
        for f in files:
            filename = normpath(join(root, f))
            if filename in ignore_list:
                print 'skip file: %s' % filename
                continue

            destfile = join(root_dir, filename)
            print '%s file:' % ('add', 'replace')[exists(destfile)], destfile
            if not exists(dirname(destfile)):
                print 'create directory:', dirname(destfile)
                makedirs(dirname(destfile))
            copyfile(filename, destfile)
            copymode(filename, destfile)

def updateClient():
    o = parseOption()
    client_url = "https://www.develer.com/~aleister/devclient/devclient.tgz"
    if o.url:
       client_url = o.url

    start_dir = getcwd()
    tmp_dir = join(start_dir, 'temp')
    # the root directory of client
    root_dir = abspath(join(start_dir, '../..'))
    # files to be skipped, with path relative to root_dir
    ignore_list = []
    if not exists(tmp_dir):
        mkdir(tmp_dir)

    chdir(tmp_dir)
    try:
        if newVersion(client_url):
            downloadFile(client_url, o.timeout)
            base_dir = uncompressClient(basename(client_url))
            replaceOldVersion(root_dir, base_dir, ignore_list)
            chdir(start_dir)
    except UpdaterError, e:
            print e
    finally:
        rmtree(tmp_dir)

if __name__ == '__main__':
    updateClient()