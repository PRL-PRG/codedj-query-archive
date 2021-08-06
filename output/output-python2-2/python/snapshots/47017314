#!/usr/bin/env python

#    Copyright (C) 2006  Cesar Izurieta  <cesar@ecuarock.net>
#
#    This program can be distributed under the terms of the GNU LGPL.
#    See the file COPYING.
#

import os
import stat
import errno
import logging
import time

import fuse
from fuse import Fuse

import SearchFS.handlers
from SearchFS.handlers import *

fuse.fuse_python_api = (0, 2)

logger = logging.getLogger('searchfs')

def pathpart(path):
    return path.rsplit('/', 1)[-1]

def flag2mode(flags):
    md = {os.O_RDONLY: 'r', os.O_WRONLY: 'w', os.O_RDWR: 'w+'}
    m = md[flags & (os.O_RDONLY | os.O_WRONLY | os.O_RDWR)]

    if flags | os.O_APPEND:
        m = m.replace('w', 'a', 1)

    return m

class SearchFS(Fuse):
    class SearchFSStat(fuse.Stat):
        def __init__(self):
            self.st_mode = 0
            self.st_ino = 0
            self.st_dev = 0
            self.st_nlink = 0
            self.st_uid = 0
            self.st_gid = 0
            self.st_size = 0
            self.st_atime = 0
            self.st_mtime = 0
            self.st_ctime = 0

    def main(self, *a, **kw):
        logger.debug('main');
        global server
        server = self 
        self.file_class = self.SearchResultFile
        self.handler = getHandler(self.handler, self.query)
        return Fuse.main(self, *a, **kw)

    def getattr(self, path):
        if path == '/':
            st = self.SearchFSStat()
            st.st_mode = stat.S_IFDIR | 0755
            st.st_nlink = 2
            return st
        else:
            logger.debug('getattr(' + path + ')');
            return os.lstat(self.handler.realpath(path))

    def readdir(self, path, offset):
        logger.debug('readdir(' + path + ', ' + str(offset) + ')');
        for filename, realpath in self.handler.filelist(path):
            logger.debug('readdir ' + filename + ', ' + realpath);
            yield fuse.Direntry(filename[1:])

    def readlink(self, path):
        logger.debug('readlink(' + path + ')');
        return os.readlink(self.handler.realpath(path))

    def unlink(self, path):
        logger.debug('unlink(' + path + ')');
        os.unlink(self.handler.realpath(path))

    def rename(self, path, path1):
        logger.debug('rename(' + path + ', ' + path1 + ')');
        pathpart0 = pathpart(path)
        pathpart1 = pathpart(path1)
        # FIXME: This won't work
        if pathpart0 == pathpart1:
            os.rename(self.handler.realpath(path), path1)
        else:
            return -errno.ENOENT

    def chmod(self, path, mode):
        logger.debug('chmod(' + path + ', ' + str(mode) + ')');
        os.chmod(self.handler.realpath(path), mode)

    def chown(self, path, user, group):
        logger.debug('chown(' + path + ', ' + str(user) + ', ' + str(group) + ')');
        os.chown(self.handler.realpath(path), user, group)

    def truncate(self, path, len):
        logger.debug('truncate(' + path + ', ' + str(len) + ')');
        f = open(self.handler.realpath(path), "a")
        f.truncate(len)
        f.close()

    def utime(self, path, times):
        logger.debug('utime(' + path + ', ' + str(times) + ')');
        os.utime(self.handler.realpath(path), times)

    def access(self, path, mode):
        logger.debug('access(' + path + ', ' + mode + ')');
        if not os.access(self.handler.realpath(path), mode):
            return -errno.EACCES

    class SearchResultFile(object):
        def __init__(self, path, flags, *mode):
            logger.debug('SearchResultFile(' + path + ', ' + flags + ', ' + mode + ')');
            self.file = os.fdopen(os.open(server.handler.realpath(path), flags, *mode), 
                                  flag2mode(flags))
            self.fd = self.file.fileno()

        def read(self, length, offset):
            logger.debug('read(' + str(length) + ', ' + str(offset) + ')');
            self.file.seek(offset)
            return self.file.read(length)

        def write(self, buf, offset):
            logger.debug('write([buf], ' + str(offset) + ')');
            self.file.seek(offset)
            self.file.write(buf)
            return len(buf)

        def release(self, flags):
            logger.debug('release(' + str(flags) + ')');
            self.file.close()

        def _fflush(self):
            if 'w' in self.file.mode or 'a' in self.file.mode:
                self.file.flush()

        def fsync(self, isfsyncfile):
            self._fflush()
            if isfsyncfile and hasattr(os, 'fdatasync'):
                os.fdatasync(self.fd)
            else:
                os.fsync(self.fd)

        def flush(self):
            logger.debug('flush()');
            self._fflush()
            os.close(os.dup(self.fd))

        def fgetattr(self):
            logger.debug('fgetattr()');
            return os.fstat(self.fd)

        def ftruncate(self, len):
            logger.debug('ftruncate(' + str(len) + ')');
            self.file.truncate(len)


