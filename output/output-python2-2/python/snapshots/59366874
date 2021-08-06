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
from urllib import urlopen
from optparse import OptionParser
from shutil import copyfile, rmtree
from socket import setdefaulttimeout
from os import mkdir, chdir, walk, getcwd
from os.path import basename, join, exists, abspath


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
    except IOError:
        print 'Timeout on download file:', client_url
        exit()

    # TODO: manage 404 error
    filename = basename(client_url)
    fd = open(filename, 'wb+')
    fd.write(u.read())
    fd.close()

def uncompressClient(filename):
    tar = tarfile.open(filename)
    tar.extractall()
    tar.close()

def replaceOldVersion(root_dir):
    tmp_dir = 'devclient'
    for root, dirs, files in walk(tmp_dir):
        for f in files:
            filename = join(root, f)
            if exists(join(root_dir, filename)):
                print 'replace file:', join(root_dir, filename)
            else:
                print 'add file:', join(root_dir, filename)
            copyfile(filename, join(root_dir, filename))

def updateClient():
    o = parseOption()
    client_url = "https://www.develer.com/~aleister/devclient/devclient.tgz"
    if o.url:
       client_url = o.url

    start_dir = getcwd()
    tmp_dir = join(start_dir, 'temp')
    # the parent directory of client
    root_dir = abspath(join(start_dir, '../../..'))
    if not exists(tmp_dir):
        mkdir(tmp_dir)

    chdir(tmp_dir)
    if newVersion(client_url):
        downloadFile(client_url, o.timeout)
    uncompressClient(basename(client_url))
    replaceOldVersion(root_dir)
    chdir(start_dir)
    rmtree(tmp_dir)

if __name__ == '__main__':
    updateClient()