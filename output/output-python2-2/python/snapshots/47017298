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

import SearchFS.providers
from SearchFS.providers import *
import SearchFS.util
from SearchFS.util import *

fuse.fuse_python_api = (0, 2)

logger = logging.getLogger('searchfs')


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
        global server
        logger.info(_('Initializing SearchFS'));
        server = self 
        self.file_class = self.SearchResultFile
        self.provider = getFileListProvider(self.provider, self.query)
        self.originaldir = os.open(self.fuse_args.mountpoint, os.O_RDONLY)
        try:
            result = Fuse.main(self, *a, **kw)
        except fuse.FuseError:
            result = -errno.ENOENT 
            logger.info(_('Finalizing SearchFS'))
        os.close(self.originaldir)
        return result

    def fsinit(self):
        os.fchdir(self.originaldir)

    def getattr(self, path):
        if path == '/':
            return os.lstat('.')
        else:
            logger.debug('getattr(' + path + ')')
            return os.lstat(self.provider.realpath(path))

    def readdir(self, path, offset):
        logger.debug('readdir(' + path + ')')
        for filename in self.provider.filelist(path):
            yield fuse.Direntry(filename[1:])

    def readlink(self, path):
        return os.readlink(self.provider.realpath(path))

    def unlink(self, path):
        os.unlink(self.provider.realpath(path))

    def rename(self, path, path1):
        pathpart0 = pathpart(path)
        pathpart1 = pathpart(path1)
        # FIXME: This won't work
        if pathpart0 == pathpart1:
            os.rename(self.provider.realpath(path), path1)
        else:
            return -errno.ENOENT

    def chmod(self, path, mode):
        os.chmod(self.provider.realpath(path), mode)

    def chown(self, path, user, group):
        os.chown(self.provider.realpath(path), user, group)

    def truncate(self, path, len):
        f = open(self.provider.realpath(path), 'a')
        f.truncate(len)
        f.close()

    def utime(self, path, times):
        os.utime(self.provider.realpath(path), times)

    def access(self, path, mode):
        if not os.access(self.provider.realpath(path), mode):
            return -errno.EACCES

    class SearchResultFile(object):
        def __init__(self, path, flags, *mode):
            self.file = os.fdopen(os.open(server.provider.realpath(path), flags, *mode), 
                                  flag2mode(flags))
            self.fd = self.file.fileno()

        def read(self, length, offset):
            self.file.seek(offset)
            return self.file.read(length)

        def write(self, buf, offset):
            self.file.seek(offset)
            self.file.write(buf)
            return len(buf)

        def release(self, flags):
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
            self._fflush()
            os.close(os.dup(self.fd))

        def fgetattr(self):
            return os.fstat(self.fd)

        def ftruncate(self, len):
            self.file.truncate(len)


